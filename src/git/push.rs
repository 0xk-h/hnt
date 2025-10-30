use crate::ai::commit::commit_msg;
use cliclack::{input, select};
use colored::*;
use std::process::{Command, Stdio};

pub async fn push(inputs: &[String], set_upstream: bool, ai: bool, dry_run: bool) {
    if set_upstream && inputs.len() != 1 {
        eprintln!("Error: Missing branch name for '-u' (usage: -u <branch>)");
        return;
    }

    if dry_run {
        println!("{}", "Dry run mode enabled.".bold().cyan());
    }

    run_git(&["add", "."]);

    let msg = if ai {
        let generated: Option<String> = commit_msg().await;
        if let Some(msg) = &generated {
            let mut commits: Vec<String> = match serde_json::from_str(&msg) {
                Ok(c) => c,
                Err(err) => {
                    eprintln!("Failed to parse JSON: {}", err);
                    return;
                }
            };
            commits.push(String::from("Custom"));

            let items: Vec<(String, &str, &str)> = commits
                .iter()
                .map(|c| (c.clone(), c.as_str(), ""))
                .collect();

            let choice = select("Pick a commit message:")
                .items(&items)
                .interact()
                .unwrap();

            if choice == "Custom" {
                for commit in &commits {
                    if commit == &choice {
                        println!("{}  {} {}", "│".dimmed(), "●".green(), commit);
                    } else {
                        println!("{}  {} {}", "│".dimmed(), "○".dimmed(), commit.dimmed());
                    }
                }
            }

            let final_choice = if choice == "Custom" {
                input("Enter your custom commit message:")
                    .interact()
                    .unwrap_or_else(|err| {
                        eprintln!("{}", format!("Failed to read input {}", err).red());
                        std::process::exit(1);
                    })
            } else {
                choice
            };

            final_choice
        } else {
            return;
        }
    } else {
        if inputs.is_empty() {
            eprintln!("No commit message provided. Use -A to let AI generate one.");
            return;
        } else {
            inputs[0].clone()
        }
    };
    println!(
        "{}",
        format!("Using commit message - \"{}\"", &msg).yellow()
    );

    if dry_run {
        return;
    }

    run_git(&["commit", "-m", &msg]);

    let branch = if inputs.len() >= 2 {
        inputs[1].as_str()
    } else {
        "HEAD"
    };
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
