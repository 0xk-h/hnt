use cliclack::{ intro };
use colored::*;
use std::env;

use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::init::fs_cleanup;
use crate::init::scaffold::scaffold;

pub fn wizard(skip: bool, project_name: Option<String>, force: bool) {

    println!();
    let _ = intro("HNT Wizard".bold());

    let config: ProjectConfig = match project_name {
        Some(name) => {
            let name = name.trim();
            if name.is_empty() {
                get_project_config(None, skip)

            } else if name == "." {
                let path = env::current_dir().unwrap();
                if !fs_cleanup::check(&path, if force { Some(true) } else { None }) {
                    return;
                } else {
                    get_project_config(Some(name.to_string()), skip)
                }

            } else {
                get_project_config(Some(name.to_string()), skip)
            }
        }
        None => {
            get_project_config(None, skip)
        }
    };

    scaffold(config, None);

    
}

