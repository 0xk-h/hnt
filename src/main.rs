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
        version = "0.3.0",
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
        }
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Guess { number } => {
            games::guess_parser::parse_guess(number);
        }
        Commands::Push { input } => {
            git::push::push(&input);
        }
        Commands::Ai { key, prompt, full } => {
            if let Some(new_key) = key {
                ai::update_api_key::key(&new_key);
            } else if let Some(p) = prompt {
                let res = match ai::call_ai::ai(&p).await {
                    Ok(json) => json,
                    Err(err) => {
                        eprintln!("Error calling AI: {}", err);
                        return;
                    }
                };

                if full {
                    println!("{}", serde_json::to_string_pretty(&res).unwrap());
                } else {
                    let output = res["candidates"][0]["content"]["parts"][0]["text"]
                        .as_str()
                        .unwrap_or("⚠️ No output found");

                    println!("{}", output);

                }
            } else {
                println!("⚠️ Please provide either an AI key with --key or a prompt.");
            }
        }
        //_ => println!("Command not recognized. Please enter a valid command"),
    }
}