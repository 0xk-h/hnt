use crate::ai::commit::commit_msg;
use std::process::{Command, Stdio};
use inquire::{Select, Text};
use colored::*;

pub async fn push(inputs: &[String], set_upstream: bool, ai: bool, dry_run: bool) {
    if dry_run {
        println!("Dry run mode enabled.");
    }

    run_git(&["add", "."]);

    let msg = if ai {
        let generated:Option<String> = commit_msg().await;
        if let Some(msg) = &generated {
            let mut commits: Vec<String> = match serde_json::from_str(&msg) {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("Failed to parse JSON: {}", err);
                    return;
                }
            };
            commits.push(String::from("Other"));

            let choice = Select::new("Pick a commit message:", commits.clone())
                .prompt()
                .unwrap();

            for commit in &commits {
                if commit == &choice {
                    println!("  {}",commit.blue());
                } else {
                    println!("  {}", commit);
                }
            }

            let final_choice = if choice == "Other" {
                Text::new("Enter your custom commit message:")
                    .prompt()
                    .unwrap()
            } else {
                choice
            };
        
            final_choice
        } else {
            return;
        }
    } else {
        if inputs.is_empty() {
            eprintln!("❌ No commit message provided. Use -A to let AI generate one.");
            return;
        } else {
            inputs[0].clone()
        }
    };
    println!("✅ Using commit message => {}", &msg);

    if dry_run {
        return;
    }

    run_git(&["commit", "-m", &msg]);

    let branch = if inputs.len() >= 2 { inputs[1].as_str() } else { "HEAD" };
    if set_upstream {
        run_git(&["push", "--set-upstream", "origin", &branch]);
    } else {
        run_git(&["push", "origin", &branch]);
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