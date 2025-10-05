use cliclack::{ outro, select};
use colored::*;
use std::io::{self, Write};
use std::fs::{self, File};
use std::path::Path;
use include_dir::{Dir, include_dir};

static TEMPLATES: Dir = include_dir!("$CARGO_MANIFEST_DIR/templates");

pub fn check(path: &Path, force: Option<bool>) -> bool {

    if is_empty(path) {
        return true;
    }

    if force == Some(true) {
        // remove
        return true;
    }

    if force == Some(false) {
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
                // remove all files
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

pub fn copy(src: &str, dest: &Path) -> io::Result<()> {
    let dir = TEMPLATES.get_dir(src).ok_or_else(|| {
        io::Error::new(io::ErrorKind::NotFound, format!("Directory '{}' not found", src))
    })?;

    for file in dir.files() {
        let path = dest.join(file.path().strip_prefix(src).unwrap());

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        if let Some(file_name) = path.file_name().and_then(|x| x.to_str()) {
            
            if file_name == ".gitkeep" {}
            else {
                let mut out_file = File::create(path)?;
                out_file.write_all(file.contents())?;
            }
        }
    }

    for subdir in dir.dirs() {
        let sub_dest = dest.join(subdir.path().strip_prefix(src).unwrap());
        copy(subdir.path().to_str().unwrap(), &sub_dest)?;
    }

    Ok(())
}
