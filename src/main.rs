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
            number: Option<String>,
        },
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Guess { number } => {
            if let Some(num_str) = number {
                match num_str.trim().parse::<i32>(){
                    Ok(num) => {
                        if (1..=10).contains(&num) {
                            games::guess::guess(num);
                        } else {
                            println!("⚠️ Number out of range. Please enter a value between 1 and 10.");
                        }
                    }
                    Err(_) => {
                        println!("❌ Invalid input '{}'. Please enter a number between 1 and 10.", num_str);
                        println!("or type 'hnt guess' to enter interactive mode.");
                    }
                }
            } else {
                games::guess::start();
            }
        }
    }
}
