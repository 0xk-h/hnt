use crate::utils::git_run::run_git;

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