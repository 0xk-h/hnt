use cliclack::{ intro, outro };
use colored::*;

use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::init::scaffold::scaffold;

pub fn wizard(skip: bool, project_name: Option<String>, force: bool) {

    println!();
    let _ = intro("HNT Wizard".bold());

    let config: ProjectConfig = match project_name {
        Some(name) => {
            let name = name.trim();
            if name.is_empty() {
                get_project_config(None, skip, force)

            } else {
                get_project_config(Some(name.to_string()), skip, force)

            }
        }
        None => {
            get_project_config(None, skip, force)
        }
    };

    let _ = outro("Scaffolding project...".green());

    scaffold(config);
    
}

