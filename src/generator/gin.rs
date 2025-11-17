use std::collections::{HashMap, HashSet};
use std::env;
use std::fs;
use std::path::PathBuf;

use crate::init::fs_ops::copy;
use crate::init::helper::get_name;
use crate::init::prompts::ProjectConfig;

pub fn create(config: &ProjectConfig) -> std::io::Result<()> {
    if let Some(backend) = &config.backend {
        if backend != "gin" {
            eprintln!("Unsupported backend framework");
            std::process::exit(1);
        }
    }

    let mut path: PathBuf = if config.name == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&config.name)
    };
    if &config.project_type == "Fullstack" {
        path.push("backend");
    }
    if !path.exists() {
        fs::create_dir_all(&path)?;
    }

    let src = String::from("gin");

    let name: String = get_name(&config.name);

    let name_replacements: HashMap<&str, &str> = HashMap::from([("{{NAME}}", name.as_str())]);

    let mut replacements = HashMap::new();
    replacements.insert(String::from("go.mod"), &name_replacements);
    replacements.insert(String::from("router.go"), &name_replacements);
    replacements.insert(String::from("main.go"), &name_replacements);

    let skip: HashSet<String> = HashSet::from([String::from(".gitkeep")]);

    let mut rename = HashMap::new();

    rename.insert(String::from("_gitignore"), String::from(".gitignore"));

    copy(&src, &path, &replacements, &skip, &rename)?;

    Ok(())
}
