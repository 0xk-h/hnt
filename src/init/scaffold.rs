use cliclack::{ outro, confirm };
use colored::*;

use super::project_summary::{print_next_steps, print_project_summary};
use super::prompts::ProjectConfig;
use crate::generator;

pub fn scaffold(config: ProjectConfig, install_deps: Option<bool>) {
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

    // Placeholder for future scaffold logic
    if let Some(frontend) = &config.frontend {
        let res = match frontend.as_str() {
            "react" | "react-ts" => {
                generator::react::create(&config)
            }
            _ => {
                println!("Frontend template '{}' is not yet supported.", frontend);
                std::process::exit(1);
            }
        };
        if let Err(e) = res {
            eprintln!("Failed to scaffold frontend '{}': {}", frontend.as_str(), e);
        }
    }

    print_project_summary(&config);
    
    print_next_steps(&config.name);


}
