use cliclack::{ outro, select};
use colored::*;
use std::fs;
use std::path::Path;

pub fn check(path: &Path, force: Option<bool>) -> bool {

    if is_empty(path) {
        return true;
    }

    if force == Some(true) {
        // do ops
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