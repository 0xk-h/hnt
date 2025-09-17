use cliclack::{intro, outro, select};
use colored::*;
use std::fs;
use std::path::Path;

pub fn check(path: &Path) -> bool {

    if is_empty(path) {
        return true;
    }

    println!();
    let _ = intro("HNT Wizard".bold());

        let ans = select("Current directory is not empty. Please choose how to proceed:")
            .item(0, "Cancel operation", "")
            .item(1, "Remove existing files and continue", "")
            .item(2, "Igonre files and continue", "")
            .interact()
            .unwrap();

        match ans {
            1 => {
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