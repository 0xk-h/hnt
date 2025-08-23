use clap::{Parser, Subcommand};
mod games;

fn main() {
    #[derive(Parser)]
    #[command(author, version, about)]
    struct Cli {
        #[command(subcommand)]
        command: Commands,
    }

    #[derive(Subcommand)]
    enum Commands {
        // guessing game
        Guess {
            number: Option<i32>,
            q: Option<String>,
        },
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Guess { number, q } => {
            if let Some(_) = q {
                println!("Quitting game");
            } else {
                games::guess::guess(number);
            }
        }
    }
    println!("The End");
}
