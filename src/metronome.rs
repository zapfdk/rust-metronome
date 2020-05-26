use std::sync::mpsc::{channel, RecvTimeoutError, Sender};
use std::thread;
use std::time::Duration;
use std::time::Instant;

pub fn calc_beat_delta(bpm: u16, lower: u8) -> Duration {
    let quarter_note_sec: f64 = 60f64 / bpm as f64;
    let factor: f64 = 4f64 / lower as f64;

    Duration::from_secs_f64(quarter_note_sec * factor)
}

#[derive(Debug)]
pub struct TimeSignature {
    upper: u8,
    lower: u8,
}

#[derive(Debug)]
pub struct MetronomeSettings {
    bpm: u16,
    time_signature: TimeSignature,
}

#[derive(Debug)]
pub enum MetronomeControls {
    Stop,
}

#[derive(Debug)]
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
                bpm: bpm,
                time_signature: TimeSignature { upper, lower },
            },
            current_beat: 0,
            last_time_run: Instant::now(),
            beat_delta: calc_beat_delta(bpm, lower),
        }
    }
    pub fn next(&mut self) {
        self.current_beat = (self.current_beat + 1) % self.metronome_settings.time_signature.upper;
        self.last_time_run = std::time::Instant::now();
    }

    pub fn start(mut self) -> Sender<MetronomeControls> {
        println!("Metronome started!");
        let (tx, rx) = channel();

        thread::spawn(move || loop {
            let msg = rx.recv_timeout(self.beat_delta);
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
