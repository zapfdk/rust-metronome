use std::time::Duration;

mod io;
mod metronome;
mod settings;

fn main() {
    let metronome: metronome::Metronome = io::ask_metronome_settings();

    println!("♩ = {:?}", metronome);
    let worker = metronome.start();
    loop {}
}
