use inquire::Confirm;
use colored::*;
use std::env;

use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::init::project_summary::print_project_summary;
// use crate::init::fs_ops;

pub fn scaffold_project(yes: bool , project_name: Option<String>) {

    let config: ProjectConfig = match project_name {
        Some(name) => {
            let name = name.trim();
            if name.is_empty() {
                println!("Project name cannot be empty.");
                return;
            } else if name == "." {
                // Use current dir name
                let path = env::current_dir()
                    .expect("Failed to get current directory");
                let project_name = path
                    .file_name()
                    .expect("Failed to get directory name")
                    .to_string_lossy()
                    .to_string();
                get_project_config(Some(project_name), yes)
            } else {
                get_project_config(Some(name.to_string()), yes)
            }
        }
        None => {
            get_project_config(None, yes)
        }
    };

    print_project_summary(&config);

    let res = Confirm::new("Do you want to proceed with this configuaration?")
        .with_default(true)
        .prompt()
        .unwrap_or(false);    

    if !res {
        println!("{}","Project initialization terminated.".red().bold());
        return;
    }

}
