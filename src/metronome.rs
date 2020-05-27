use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use std::time::Instant;

use crate::audio_handling::AudioSettings;
use crate::audio_handling::TicToc;

pub fn calc_beat_delta(bpm: u16, lower: u8) -> Duration {
    let quarter_note_sec: f64 = 60f64 / bpm as f64;
    let factor: f64 = 4f64 / lower as f64;

    Duration::from_secs_f64(quarter_note_sec * factor)
}

#[derive(Debug, Copy, Clone)]
pub struct TimeSignature {
    upper: u8,
    lower: u8,
}

#[derive(Debug, Copy, Clone)]
pub struct MetronomeSettings {
    bpm: u16,
    time_signature: TimeSignature,
}

#[derive(Debug)]
pub enum MetronomeControls {
    Stop,
}

#[derive(Debug, Copy, Clone)]
pub struct Metronome {
    metronome_settings: MetronomeSettings,
    current_beat: u8,
    last_time_run: Instant,
    beat_delta: Duration,
}

impl Metronome {
    pub fn new(bpm: u16, upper: u8, lower: u8) -> Metronome {
        Metronome {
            metronome_settings: MetronomeSettings {
                bpm,
                time_signature: TimeSignature { upper, lower },
            },
            current_beat: 0,
            last_time_run: Instant::now(),
            beat_delta: calc_beat_delta(bpm, lower),
        }
    }
    fn next(&mut self) {
        self.current_beat = (self.current_beat + 1) % self.metronome_settings.time_signature.upper;
        self.last_time_run = std::time::Instant::now();
    }

    fn play_beat(self) {
        let audio_settings: AudioSettings = AudioSettings::load();
        match self.current_beat {
            0 => audio_settings.play(TicToc::Tic),
            _ => audio_settings.play(TicToc::Toc),
        }
    }

    pub fn start(mut self) -> Sender<MetronomeControls> {
        println!("Metronome started!");
        let (tx, rx) = channel();

        thread::spawn(move || loop {
            self.play_beat();
            let sleep_time = self.beat_delta - (Instant::now() - self.last_time_run);

            let msg = rx.recv_timeout(sleep_time);
            match msg {
                Ok(MetronomeControls::Stop) => break,
                Err(_) => (),
            }

            self.next();
        });

        tx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_beat_delta() {
        assert_eq!(calc_beat_delta(120, 4), Duration::from_millis(500));
        assert_eq!(calc_beat_delta(119, 4), Duration::from_secs_f64(0.50420168));
        assert_eq!(calc_beat_delta(119, 8), Duration::from_secs_f64(0.25210084));
    }

    #[test]
    fn test_metronome_next() {
        let mut metronome: Metronome = Metronome::new(120, 4, 4);
        assert_eq!(metronome.current_beat, 0);

        metronome.next();
        assert_eq!(metronome.current_beat, 1);
        metronome.next();
        metronome.next();
        assert_eq!(metronome.current_beat, 3);
        metronome.next();
        assert_eq!(metronome.current_beat, 0);
    }
}
