// use std::process::Command;
use std::fs;
use std::env;
use std::path::{PathBuf};
use std::collections::{HashMap, HashSet};
use colored::*;

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

    let mut replacements = HashMap::new();
    replacements.insert("index.html", index_replacements);

    let mut skip: HashSet<&str> = HashSet::from([".gitkeep"]);
    if config.project_type != "Fullstack" {
        skip.insert("message.js");
        skip.insert("DemoFullstack.jsx");
    } else {
        skip.insert("DemoFrontendOnly.jsx");
    }

    let mut rename = HashMap::new();
    if config.project_type != "Fullstack"{
        rename.insert("DemoFrontendOnly.jsx", "Demo.jsx");
    } else {
        rename.insert("DemoFullstack.jsx", "Demo.jsx");
    }

    let mut src = String::from("");
    if let Some(frontend) = &config.frontend {
        if frontend == "react"  {
            src.push_str("react");
        } else if frontend == "react-ts" {
            src.push_str("react-ts");
        }
    }
    if config.use_tailwind{
        if src.is_empty() {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, "No frontend specified"));
        }
        src.push_str("/tailwind");
    } else {
        src.push_str("/base");
    }
    print!("Using template: {}\n", src);
    println!("replacements: {:?}\n skip: {:?}\n rename: {:?}", replacements, skip, rename);

    println!("{}","Creating project".bold().green());

    copy(&src, &path, &replacements, &skip, &rename)?;


    println!("Project created successfully at {:?}", path);

    Ok(())
}
