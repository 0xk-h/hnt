use std::process::Command;
use crate::ai::call_ai;
use colored::*;

pub async fn commit_msg() -> Option<String> {

    let output = Command::new("git")
        .args(["diff", "--cached"])
        .output()
        .expect("Failed to run git diff --cached");

    if !output.status.success() {
        eprintln!("{}","git diff --cached failed".red());
        return None;
    }

    if output.stdout.is_empty() {
    eprintln!("{}","No changes detected. Modify files before running this command.".yellow());
    return None;
    }

    let diff = String::from_utf8_lossy(&output.stdout);

    let prompt = format!(
        "You are an assistant that generates conventional commit messages.\n\
         Based on the following git diff, suggest at most 5 possible commit messages.\n\
         \n\
         Return ONLY a valid JSON array of strings.\n\
         Do not include code fences, do not escape the strings, do not add any explanation.\n\
         \n\
         Example of the required format:\n\
         [\"feat: add login button\", \"fix: correct typo in README\"]\n\
         \n\
         Git diff:\n{}",
        diff
    );

    let res = match call_ai::ai(&prompt).await {
        Ok(json) => json,
        Err(err) => {
            let err = err.to_string();
            if err == "No API key found" {
                eprintln!("{}","No API key found. Please set it using `hnt ai --key <YOUR_API_KEY>`".bold().red());
            } else {
                eprintln!("Error could not reach AI service. Check your internet connection and try again");
            }
            return None;
        }
    };

    let output = if let Some(text) = res["candidates"][0]["content"]["parts"][0]["text"].as_str() {
        text
    } else if let Some(err) = res["error"]["message"].as_str() {
        eprintln!("{}",format!("AI Error: {}", err).bold().red());
        return None;
    } else {
        eprintln!("No output found for commit msg");
        return None;
    };

    println!("{}",output);

    Some(output.to_string())
}

