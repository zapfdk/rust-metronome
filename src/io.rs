use crate::metronome::Metronome;
use std::io;

pub fn ask_metronome_settings() -> Metronome {
    let mut bpm = String::new();
    let mut time_signature = String::new();

    println!("Please enter the BPM for ♩ = (0-65536):");
    io::stdin().read_line(&mut bpm).expect("You failed!");

    println!("Please enter the time signature, (Format: \"x/y\"):");
    io::stdin()
        .read_line(&mut time_signature)
        .expect("You failed!");

    let bpm: u16 = match bpm.trim().parse() {
        Ok(num) => num,
        Err(_) => 0,
    };

    let numbers: Vec<u8> = time_signature
        .trim()
        .split("/")
        .map(|x| x.parse().expect("Error parsing time signature"))
        .collect();

    Metronome::new(bpm, numbers[0], numbers[1])
}
