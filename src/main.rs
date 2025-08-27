use clap::{Parser, Subcommand};
mod games;
mod git;
mod utils;

fn main() {
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
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Guess { number } => {
            games::guess_parser::parse_guess(number);
        }
        Commands::Push { input } => {
            git::push::push(&input);
        }
        //_ => println!("Command not recognized. Please enter a valid command"),
    }
}