use crate::metronome::MetronomeSettings;

#[derive(Debug)]
struct Settings {
    last_metronome_settings: Vec<MetronomeSettings>,
}

impl Settings {
    pub fn save(&self, path: String) {}
    pub fn load(path: String) -> Settings {
        Settings {
            last_metronome_settings: Vec::new(),
        }
    }
}
