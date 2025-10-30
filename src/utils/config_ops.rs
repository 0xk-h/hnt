use clap::ValueEnum;

use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::utils::config::HntConfig;

#[derive(Clone, Debug, ValueEnum)]
pub enum ConfigOptions {
    Set,
    Reset,
}

pub fn setup_default_config(opt: ConfigOptions) {
    match opt {
        ConfigOptions::Set => {
            // Set up default config
            let cfg: ProjectConfig = get_project_config(None, false, false, true);
            HntConfig::update_init_defaults(&cfg);
        }
        ConfigOptions::Reset => {
            // Reset config file to defaults
            let _ = HntConfig::reset_config();
        }
    }
}
