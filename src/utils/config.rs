use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct HntConfig {
    pub ai_api_key: String,
}

impl HntConfig {
    // Load config file from ~/.hnt/config.toml
    pub fn load() -> Self {
        let path = HntConfig::config_path();

        if !path.exists() {
            // Default HntConfig
            let cfg = HntConfig {
                ai_api_key: String::from(""),
            };
            cfg.save();
            return cfg;
        }

        let content = fs::read_to_string(&path).expect("Failed to read config file");
        return toml::from_str(&content).expect("Failed to parse config file");
    }

    // Save config toml
    fn save(&self) {
        let path = HntConfig::config_path();
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to create .hnt directory");
        let toml = toml::to_string(self).expect("Failed to serialize config");
        fs::write(&path, toml).expect("Failed to write config file");
    }

    // Update AI api key
    pub fn update_ai_key(new_key: &str) {
        let mut cfg = HntConfig::load();
        cfg.ai_api_key = new_key.to_string();
        cfg.save();
    }

    // Gets home dir
    fn config_path() -> PathBuf {
        let home = dirs::home_dir().expect("Cannot find home directory");
        return home.join(".hnt/config.toml");
    }
}
