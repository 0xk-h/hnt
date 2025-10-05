// use std::process::Command;
use std::fs;
use std::env;
use std::path::{PathBuf};

use crate::init::prompts::ProjectConfig;
use crate::utils::pkg_manager::detect_package_manager;
use crate::init::fs_ops::copy;

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

    let mut replacements = std::collections::HashMap::new();
    replacements.insert("{{project_name}}", &config.name as &str);
    if let Some(frontend) = &config.frontend {
        replacements.insert("{{frontend}}", frontend as &str);
    }

    copy("react/tailwind", &path)?;


    println!("Project created successfully at {:?}", path);

    Ok(())
}

// fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
//     if !dst.exists() {
//         fs::create_dir_all(dst)?;
//     }

//     for entry in fs::read_dir(src)? {
//         let entry = entry?;
//         let path = entry.path();
//         let dest = dst.join(entry.file_name());

//         if path.is_dir() {
//             copy_dir_all(&path, &dest)?;
//         } else {
//             fs::copy(&path, &dest)?;
//         }
//     }

//     Ok(())
// }
