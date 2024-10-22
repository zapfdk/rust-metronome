mod audio_handling;
mod io;
mod metronome;
mod settings;

fn main() {
    audio_handling::check_audio_files();

    let metronome: metronome::Metronome = io::ask_metronome_settings();

    let worker = metronome.start();
    loop {}
}
