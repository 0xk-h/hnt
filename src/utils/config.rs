use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct HntConfig {
    pub api: ApiConfig,
    pub init_defaults: InitDefaults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiConfig {
    pub gemini_api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitDefaults {
    pub frontend: String,
    pub backend: String,
    pub use_tailwind: bool,
    pub git_init: bool,
    pub use_shadcn: bool,
}

impl HntConfig {
    // Load config file from ~/.hnt/config.toml
    pub fn load() -> Self {
        let path = HntConfig::config_path();

        if !path.exists() {
            // Default HntConfig
            let cfg = HntConfig {
                api: ApiConfig {
                    gemini_api_key: String::from(""),
                },
                init_defaults: InitDefaults {
                    frontend: String::from("React"),
                    backend: String::from("Express.js"),
                    use_tailwind: true,
                    git_init: true,
                    use_shadcn: false,
                },
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
        cfg.api.gemini_api_key = new_key.to_string();
        cfg.save();
    }

    // Gets home dir
    fn config_path() -> PathBuf {
        let home = dirs::home_dir().expect("Cannot find home directory");
        return home.join(".hnt/config.toml");
    }
}
