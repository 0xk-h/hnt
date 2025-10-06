// use std::process::Command;
use std::fs;
use std::env;
use std::path::{PathBuf};
use std::collections::HashMap;

use crate::init::prompts::ProjectConfig;
use crate::utils::pkg_manager::detect_package_manager;
use crate::init::fs_ops::copy;
use crate::init::helper::get_name;

pub fn create(config: &ProjectConfig) -> std::io::Result<()> {
    if !(detect_package_manager("npm")) {
        eprintln!("npm is not installed. Please install Node.js and npm to proceed.");
        std::process::exit(1);
    }

    if let Some(frontend) = &config.frontend {
        if frontend != "react" && frontend != "react-ts" {
            eprintln!("Unsupported frontend framework");
            std::process::exit(1);
        }
    }

    let path: PathBuf = if config.name == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&config.name)
    };
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let name: String = get_name(&config.name);

    let index_replacements: HashMap<&str, &str> = HashMap::from([
        ("{{NAME}}", name.as_str())
    ]);

    let mut replacements = std::collections::HashMap::new();
    replacements.insert("index.html", index_replacements);

    copy("react/tailwind", &path, &replacements)?;


    println!("Project created successfully at {:?}", path);

    Ok(())
}
