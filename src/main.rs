mod games;
mod git;
mod utils;
mod ai;

use clap::{Parser, Subcommand};
use tokio;

#[tokio::main]
async fn main() {

    // let cfg = utils::config::HntConfig::load();
    #[derive(Parser)]
    #[command(
        name = "hnt",
        author = "Kishore Kumar",
        version = "1.0.0",
        about = "Dev productivity cli tool"
    )]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        // guessing game
        Guess {
            number: Option<String>,
        },
        // git commands
        Push {
            #[arg(short = 'u', long = "set-upstream", default_value_t = false)]
            set_upstream: bool,

            #[arg(short = 'A', long, default_value_t = false)]
            ai: bool,

            #[arg(short = 'n', long = "dry-run", default_value_t = false)]
            dry_run: bool,

            #[arg(trailing_var_arg = true)]
            input : Vec<String>,
        },
        // AI commands
        Ai {
            #[arg(short,long, group = "ai")]
            key: Option<String>,

            #[arg(group = "ai")]
            prompt: Option<String>,

            #[arg(short, long,requires = "prompt", default_value_t = false)]
            full: bool,
        },
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Guess { number } => {
            games::guess_parser::parse_guess(number);
        }
        Commands::Push { input, set_upstream, ai, dry_run } => {
            if set_upstream && input.len() != 2 {
                eprintln!("Error: -u requires exactly 2 arguments: <msg> <branch>");
                return;
            }
            git::push::push(&input, set_upstream, ai, dry_run).await;
        }
        Commands::Ai { key, prompt, full } => {
            if let Some(new_key) = key {
                ai::update_api_key::key(&new_key);
            } else if let Some(p) = prompt {
                let res = match ai::call_ai::ai(&p).await {
                    Ok(json) => json,
                    Err(_err) => {
                        eprintln!("Error could not reach AI service. Check your internet connection and try again");
                        return;
                    }
                };
                if full {
                    println!("{}", serde_json::to_string_pretty(&res).unwrap());
                } else {
                    let output = if let Some(text) = res["candidates"][0]["content"]["parts"][0]["text"].as_str() {
                        text
                    } else if let Some(err) = res["error"]["message"].as_str() {
                        err
                    } else {
                        "⚠️ No output found"
                    };

                    println!("{}", output);

                }
            } else {
                println!("⚠️ Please provide either an AI key with --key or a prompt.");
            }
        }
        //_ => println!("Command not recognized. Please enter a valid command"),
    }
}