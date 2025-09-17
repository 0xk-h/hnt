use cliclack::{intro, outro, confirm};
use colored::*;
use std::env;

use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::init::project_summary::{print_project_summary, print_next_steps};
use crate::init::create;

pub fn scaffold_project(yes: bool , project_name: Option<String>) {

    println!();
    let _ = intro("HNT Wizard".bold());

    let config: ProjectConfig = match project_name {
        Some(name) => {
            let name = name.trim();
            if name.is_empty() {
                get_project_config(None, yes)
            } else if name == "." {
                // Use current dir name
                let path = env::current_dir()
                    .expect("Failed to get current directory");

                if !create::check(&path) {
                    return;
                }

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

    let res = confirm("Do you want to proceed with this configuaration?")
        .initial_value(true)
        .interact()
        .unwrap_or(false);   

    if !res {
        let _ = outro("Project initialization terminated".red());
        return;
    }

    let _ = outro("Scaffolding project...".green());

    print_next_steps(&config.name);

}
