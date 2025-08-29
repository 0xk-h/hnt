use std::process::{Command, Stdio};

pub fn push(inputs: &[String]) {
    match inputs {
        [] => {
            println!("No arguments provided for git.");
        }
        [msg] => {
            run_git(&["add", "."]);
            run_git(&["commit", "-m", &msg]);
            run_git(&["push"]);
        }
        [msg, branch] => {
            run_git(&["add", "."]);
            run_git(&["commit", "-m", &msg]);
            run_git(&["push", "origin", &branch]);
        }
        _ => {
            eprintln!("Too many arguments: {:?}", inputs);
        }
    }
}

fn run_git(args: &[&str]) {
    Command::new("git")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .expect("Unexpected error occured while running git command");
}