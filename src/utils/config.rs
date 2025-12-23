use cliclack::confirm;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::init::prompts::ProjectConfig;

#[derive(Serialize, Deserialize, Debug)]
pub struct HntConfig {
    pub api: ApiConfig,
    pub init_defaults: InitDefaults,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ApiConfig {
    pub default_ai: String,
    pub gemini_api_key: String,
    pub openrouter_api_key: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitDefaults {
    pub frontend: Option<String>,
    pub backend: Option<String>,
    pub use_tailwind: bool,
    pub git_init: bool,
}

impl HntConfig {
    // Load config file from ~/.hnt/config.toml
    pub fn load() -> Self {
        let path = HntConfig::config_path();

        if !path.exists() {
            // Default HntConfig
            let cfg = Self::default_config();
            cfg.save();
            return cfg;
        }

        let content = fs::read_to_string(&path).expect("Failed to read config file");

        match toml::from_str::<HntConfig>(&content) {
            Ok(cfg) => cfg,
            Err(e) => {
                eprintln!("Failed to parse config file: {}", e);
                eprintln!("This may be due to an old or incompatible config version.");

                HntConfig::reset_config()
            }
        }
    }

    // Save config toml
    fn save(&self) {
        let path = Self::config_path();
        fs::create_dir_all(path.parent().unwrap()).expect("Failed to create .hnt directory");
        let toml = toml::to_string(self).expect("Failed to serialize config");
        fs::write(&path, toml).expect("Failed to write config file");
    }

    // Update AI api key
    pub fn update_ai_key(ai: &str, new_key: &str) {
        let mut cfg = Self::load();
        match ai {
            "gemini" => cfg.api.gemini_api_key = new_key.to_string(),
            "open_router" => cfg.api.openrouter_api_key = new_key.to_string(),
            _ => panic!("Invalid AI type"),
        }
        cfg.save();
    }

    pub fn update_default_ai(default_ai: &str) {
        let mut cfg = Self::load();
        cfg.api.default_ai = default_ai.to_string();
        cfg.save();
    }

    // Gets home dir
    fn config_path() -> PathBuf {
        let home = dirs::home_dir().expect("Cannot find home directory");
        return home.join(".hnt/config.toml");
    }

    // Default HntConfig
    pub fn default_config() -> Self {
        HntConfig {
            api: ApiConfig {
                default_ai: String::from("gemini"),
                gemini_api_key: String::from(""),
                openrouter_api_key: String::from(""),
            },
            init_defaults: InitDefaults {
                frontend: None,
                backend: None,
                use_tailwind: true,
                git_init: true,
            },
        }
    }

    // Update init defaults
    pub fn update_init_defaults(cfg: &ProjectConfig) {
        let mut current = Self::load();

        current.init_defaults = InitDefaults {
            frontend: match &cfg.frontend {
                Some(f) => Some(f.clone()),
                None => current.init_defaults.frontend,
            },
            backend: match &cfg.backend {
                Some(b) => Some(b.clone()),
                None => current.init_defaults.backend,
            },
            use_tailwind: cfg.use_tailwind,
            git_init: cfg.git_init,
        };

        current.save();
    }

    // reset config file
    pub fn reset_config() -> Self {
        let reset =
            confirm("Do you want to reset the config to defaults? (Old config will be backed up)")
                .initial_value(true)
                .interact()
                .unwrap_or(true);

        if reset {
            let path = HntConfig::config_path();

            // Backup old config
            let backup_path = path.with_extension("toml.bak");
            fs::copy(&path, &backup_path).expect("Failed to backup old config");

            // Create new config
            let cfg = Self::default_config();
            cfg.save();
            println!(
                "New default config created. Old config backed up as {:?}",
                backup_path
            );
            cfg
        } else {
            println!("Keeping the old config file as-is. Exiting.");
            std::process::exit(1);
        }
    }
}
