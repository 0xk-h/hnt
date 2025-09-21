use cliclack::{ intro, confirm, outro };
use colored::*;
use std::env;

use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::init::project_summary::{print_project_summary, print_next_steps};
use crate::init::create;

pub fn wizard(skip: bool, project_name: Option<String>, force: bool) {

    println!();
    let _ = intro("HNT Wizard".bold());

    let config: ProjectConfig = match project_name {
        Some(name) => {
            let name = name.trim();
            if name.is_empty() {
                get_project_config(None, skip)
            } else if name == "." {
                // Use current dir name
                let path = env::current_dir()
                    .expect("Failed to get current directory");

                if !create::check(&path, if force { Some(true) } else { None }) {
                    return;
                }

                let project_name = path
                    .file_name()
                    .expect("Failed to get directory name")
                    .to_string_lossy()
                    .to_string();
                get_project_config(Some(project_name), skip)
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

pub fn scaffold(config: ProjectConfig, install_deps: Option<bool>) {
    // Placeholder for future scaffold logic

    match install_deps {
        Some(true) => {
            // install deps
        }
        Some(false) => {
            // skip install
        }
        None => {
            // Ask user
            let ans = confirm("Do you want to install dependencies now?")
                .initial_value(true)
                .interact()
                .unwrap_or(false);
            if ans {
                // install deps
            }
            let _ = outro("Scaffolding project...".green());
        }
    }

    print_project_summary(&config);
    
    print_next_steps(&config.name);


}

