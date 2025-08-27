use std::process::{Command, Stdio};

pub fn run_git(args: &[&str]) {
    let status = Command::new("git")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Unexpected error occured while running git command");

    if !status.success() {
        eprintln!("❌ Git command failed: git {:?}", args);
    }
}
