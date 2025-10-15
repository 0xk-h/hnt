use cliclack::{ outro, select};
use colored::*;
use std::io::{self, Write};
use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::collections::{HashMap, HashSet};
use include_dir::{Dir, include_dir};

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn check(path: &Path, force: Option<bool>) -> bool {

    if !path.exists() || is_empty(path) {
        return true;
    }

    if force == Some(true) {
        fs::remove_dir_all(path).unwrap_or_else(|e| {
            eprintln!("Failed to remove existing directory: {}", e);
            std::process::exit(1);
        });
        return true;
    }

    if force == Some(false) {
        eprintln!("{}","Operation cancelled: target directory has existing files. Pass -f to overwrite.".red());
        return false;
    }

        let ans = select("Current directory is not empty. Please choose how to proceed:")
            .item(0, "Cancel operation", "")
            .item(1, "Remove existing files and continue", "")
            .item(2, "Igonre files and continue", "")
            .interact()
            .unwrap();

        match ans {
            1 => {
                fs::remove_dir_all(path).unwrap_or_else(|e| {
                    eprintln!("Failed to remove existing directory: {}", e);
                    std::process::exit(1);
                });
                return true;
            }
            2 => {
                return true;
            }
            _ => {
                let _ = outro("Operation canceled.".red());
                return false;
            }
        }
}

fn is_empty(dir: &Path) -> bool {
    fs::read_dir(dir).map(|mut entries| entries.next().is_none()).unwrap_or(true)
}

pub fn copy(src: &str, dest: &Path, file_replacements: &HashMap<String, HashMap<&str, &str>>, skip: &HashSet<String>, rename: &HashMap<String, String> ) -> io::Result<()> {
    let dir = TEMPLATES.get_dir(src).ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, format!("Directory '{}' not found", src))
    })?;

    for file in dir.files() {
        let mut path = dest.join(file.path().strip_prefix(src).unwrap());

        if let Some(parent) = path.parent() {
            println!("{}",format!("Creating directory: {}", parent.display()).bold().red());
            fs::create_dir_all(parent)?;
        }

        if let Some(file_name) = path.file_name().and_then(|x| x.to_str()) {
            
            if skip.contains(file_name) {
                println!("Skipping {}", file_name);
            }
            else if let Some(replacements) = file_replacements.get(file_name) {
                copy_with_replacement(&path, file.contents(), replacements)?;
            } else {
                if let Some(new_name) = rename.get(file_name) {
                    println!("Renaming {} to {}", file_name, new_name);
                    path.set_file_name(new_name);
                }
                println!("{}",format!("Creating file: {}", path.display()).bold().yellow());
                let mut out_file = File::create(path)?;
                out_file.write_all(file.contents())?;
            }
        }
    }

    for subdir in dir.dirs() {
        let sub_dest = dest.join(subdir.path().strip_prefix(src).unwrap());
        copy(subdir.path().to_str().unwrap(), &sub_dest, &file_replacements, &skip, &rename)?;
    }

    Ok(())
}

fn copy_with_replacement(path: &PathBuf, file_contents: &[u8], replacements: &HashMap<&str, &str>) -> io::Result<()> {

    let contents = std::str::from_utf8(file_contents)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    let mut result = contents.to_string();
    for (from, to) in replacements.iter() {
        result = result.replace(from, to);
    }

    println!("{}",format!("Creating file with replacements: {}", path.display()).bold().blue());
    let mut out_file = File::create(path)?;
    out_file.write_all(result.as_bytes())?;

    Ok(())
}
