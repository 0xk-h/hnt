// use std::process::Command;
use colored::*;
use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::init::fs_ops::copy;
use crate::init::helper::get_name;
use crate::init::prompts::ProjectConfig;
use crate::utils::pkg_manager::detect_package_manager;

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

    let mut path: PathBuf = if config.name == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&config.name)
    };
    if &config.project_type == "Fullstack" {
        path.push("frontend");
    }
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let mut end = String::from("");
    let mut src = String::from("");

    if let Some(frontend) = &config.frontend {
        if frontend == "react" {
            src.push_str("react");
            end.push_str("jsx");
        } else if frontend == "react-ts" {
            src.push_str("react-ts");
            end.push_str("tsx");
        }
    }

    if config.use_tailwind {
        if src.is_empty() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::Other,
                "No frontend specified",
            ));
        }
        src.push_str("/tailwind");
    } else {
        src.push_str("/base");
    }

    let name: String = get_name(&config.name);

    let index_replacements: HashMap<&str, &str> = HashMap::from([("{{NAME}}", name.as_str())]);

    let mut replacements = HashMap::new();
    replacements.insert(String::from("index.html"), index_replacements);

    let mut skip: HashSet<String> = HashSet::from([String::from(".gitkeep")]);
    if config.project_type != "Fullstack" {
        skip.insert(format!("message.{}", &end[..2]));
        skip.insert(format!("DemoFullstack.{}", &end));
    } else {
        skip.insert(format!("DemoFrontendOnly.{}", &end));
    }

    let mut rename = HashMap::new();
    if config.project_type != "Fullstack" {
        rename.insert(
            format!("DemoFrontendOnly.{}", &end),
            format!("Demo.{}", &end),
        );
    } else {
        rename.insert(format!("DemoFullstack.{}", &end), format!("Demo.{}", &end));
    }

    print!("Using template: {}\n", src);
    println!(
        "replacements: {:?}\n skip: {:?}\n rename: {:?}",
        replacements, skip, rename
    );

    println!("{}", "Creating project".bold().green());

    copy(&src, &path, &replacements, &skip, &rename)?;

    println!("Project created successfully at {:?}", path);

    Ok(())
}
