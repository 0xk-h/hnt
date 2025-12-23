use crate::ai;
use serde_json;

pub async fn handle_prompt(prompt: Option<String>, full: bool) {
    if let Some(p) = prompt {
        let res = match ai::call_ai::ai(&p).await {
            Ok(json) => json,
            Err(err) => {
                let err = err.to_string();
                if err == "No API key found" {
                    eprintln!(
                        "No API key found. Please set it using `hnt ai --key <YOUR_API_KEY>`"
                    );
                } else {
                    eprintln!(
                        "Error could not reach AI service. Check your internet connection and try again"
                    );
                }
                return;
            }
        };
        if full {
            println!("{}", serde_json::to_string_pretty(&res).unwrap());
        } else {
            let output =
                if let Some(text) = res["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                    text
                } else if let Some(err) = res["error"]["message"].as_str() {
                    err
                } else {
                    "No output found"
                };
            println!("{}", output);
        }
    } else {
        println!("Please provide either an AI key with --key or a prompt.");
    }
}
