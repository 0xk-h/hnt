use crate::init::prompts::{ProjectConfig, get_project_config};
use crate::init::project_summary::print_project_summary;
use std::env;

pub fn scaffold_project(yes: bool , project_name: Option<String>) {

    let config: ProjectConfig = match project_name {
        Some(name) => {
            let name = name.trim();
            if name.is_empty() {
                println!("Project name cannot be empty.");
                return;
            } else if name == "." {
                // Use current dir name
                let current_dir = env::current_dir()
                    .expect("Failed to get current directory");
                let project_name = current_dir
                    .file_name()
                    .expect("Failed to get directory name")
                    .to_string_lossy()
                    .to_string();
                get_project_config(Some(project_name))
            } else {
                get_project_config(Some(name.to_string()))
            }
        }
        None => {
            get_project_config(None)
        }
    };

    if yes {
        println!("Skipping prompts and using default configuration.");
    }

    print_project_summary(&config);

}
