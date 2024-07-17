use serde::{Deserialize, Serialize};
use std::io::{self, BufReader};
use std::fs::{File};

#[derive(Serialize, Deserialize, Default)]
pub struct Config {
    pub selected_version: Option<String>,
}

impl Config {
    pub fn load() -> io::Result<Self> {
        let config_path = config_path();

        if let Ok(file) = File::open(&config_path) {
            let reader = BufReader::new(file);
            let config = serde_json::from_reader(reader)?;
            Ok(config)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> io::Result<()> {
        let config_path = config_path();
        let file = File::create(&config_path)?;
        serde_json::to_writer(file, self)?;
        Ok(())
    }
}

fn config_path() -> String {
    let username = whoami::username();
    format!("C:\\Users\\{}\\Documents\\rust_java_launcher\\config.json", username)
}
