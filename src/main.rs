mod games;
mod git;
mod utils;
mod ai;

use clap::{Parser, Subcommand};

fn main() {

    // let cfg = utils::config::HntConfig::load();
    #[derive(Parser)]
    #[command(
        name = "hnt",
        author = "Kishore Kumar",
        version = "0.2.0",
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
            #[arg(short,long)]
            key: Option<String>,

            prompt: Option<String>,
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
        Commands::Ai { key, prompt } => {
            if let Some(new_key) = key {
                ai::update_api_key::key(&new_key);
            } else if let Some(_p) = prompt {
                println!("AI Prompt feature coming soon! Stay tuned. 🚀");
            } else {
                println!("⚠️ Please provide either an AI key with --key or a prompt.");
            }
        }
        //_ => println!("Command not recognized. Please enter a valid command"),
    }
}