use crate::ai::commit::commit_msg;
use std::process::{Command, Stdio};

pub fn push(inputs: &[String], set_upstream: bool, ai: bool, dry_run: bool) {

    if ai {
        let msg = commit_msg(dry_run);
        if dry_run {
            return;
        } else {
            println!("AI generated commit message: {}", msg);
        }
    }
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
            if set_upstream {
                run_git(&["push", "--set-upstream", "origin", "HEAD"]);
            } else {
                run_git(&["push", "origin", &branch]);
            }
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