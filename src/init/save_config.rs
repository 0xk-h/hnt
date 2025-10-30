use clap::ValueEnum;

use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::utils::config::HntConfig;

#[derive(Clone, Debug, ValueEnum)]
pub enum ConfigOptions {
    Set,
}

pub fn setup_default_config(opt: ConfigOptions) {
    println!("Setting up default configuration for option: {:?}", opt);
    match opt {
        ConfigOptions::Set => {
            // Set up default configuration
            let cfg: ProjectConfig = get_project_config(None, false, false, true);
            println!("Default configuration set: {:?}", cfg);
            HntConfig::update_init_defaults(&cfg);
        }
    }
}
