use rodio;
use std::io::BufReader;
use std::path::Path;

const TIC_PATH: &str = "./tic.wav";
const TOC_PATH: &str = "./toc.wav";
const AUDIO_FILE_PATHS: [&'static str; 2] = [TIC_PATH, TOC_PATH];

pub fn check_audio_files() -> bool {
    for file_path in AUDIO_FILE_PATHS.iter() {
        if !Path::new(&file_path).exists() {
            return false;
        }
    }
    true
}

// fn read_audio_file(file_path: &str) -> rodio::Decoder<BufReader<File>> {
//     rodio::Decoder::new(BufReader::new(File::open(file_path).unwrap())).unwrap()
// }

#[derive(Debug)]
pub enum TicToc {
    Tic,
    Toc,
}

pub struct AudioSettings {
    // tic: rodio::Decoder<BufReader<File>>,
    // toc: rodio::Decoder<BufReader<File>>,
    device: rodio::Device,
}

impl AudioSettings {
    pub fn load() -> AudioSettings {
        AudioSettings {
            // tic: read_audio_file(TIC_PATH),
            // toc: read_audio_file(TOC_PATH),
            device: rodio::default_output_device().unwrap(),
        }
    }
    pub fn play(self, what: TicToc) {
        let file_path = match what {
            TicToc::Tic => TIC_PATH,
            TicToc::Toc => TOC_PATH,
        };
        let audio_file = std::fs::File::open(file_path).unwrap();
        rodio::play_once(&self.device, BufReader::new(audio_file)).unwrap().detach();
        println!("{:?}", what);
    }
}
