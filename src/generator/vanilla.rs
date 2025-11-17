use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::init::fs_ops::copy;
use crate::init::helper::get_name;
use crate::init::prompts::ProjectConfig;

pub fn create(config: &ProjectConfig) -> std::io::Result<()> {
    if let Some(frontend) = &config.frontend {
        if frontend != "vanilla" && frontend != "vanilla-ts" {
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

    // let mut end = String::from("");
    let mut src = String::from("");

    if let Some(frontend) = &config.frontend {
        if frontend == "vanilla" {
            src.push_str("vanilla");
            // end.push_str("js");
        } else if frontend == "vanilla-ts" {
            src.push_str("vanilla-ts");
            // end.push_str("ts");
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

    let name_replacements: HashMap<&str, &str> = HashMap::from([("{{NAME}}", name.as_str())]);

    let mut replacements = HashMap::new();
    replacements.insert(String::from("index.html"), &name_replacements);
    replacements.insert(String::from("package.json"), &name_replacements);

    let skip: HashSet<String> = HashSet::from([String::from(".gitkeep")]);

    let rename: HashMap<String, String> = HashMap::new();

    copy(&src, &path, &replacements, &skip, &rename)?;

    Ok(())
}
