use clap::{Parser, Subcommand};
use tokio;

mod games;
mod git;
mod utils;
mod ai;
mod init;
mod generator;

#[tokio::main]
async fn main() {

    #[derive(Parser)]
    #[command(
        name = "hnt",
        author = "Kishore Kumar",
        version = "1.7.3",
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

            #[arg(short, long, requires = "prompt", default_value_t = false)]
            full: bool,
        },

        //scaffold new project
        Init(init::validate::InitArgs),

        //Config {}
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Guess { number } => {
            games::guess_parser::parse_guess(number);
        }
        Commands::Push { input, set_upstream, ai, dry_run } => {
            git::push::push(&input, set_upstream, ai, dry_run).await;
        }
        Commands::Ai { key, prompt, full } => {
            ai::handler::handle_prompt(key, prompt, full).await;
        }
        Commands::Init(init_args)    => {
            init::validate::validate(&init_args);
        }
        //_ => println!("Command not recognized. Please enter a valid command"),
    }
}