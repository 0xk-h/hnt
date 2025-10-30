use clap::{Parser, Subcommand};
use tokio;

mod ai;
mod games;
mod generator;
mod git;
mod init;
mod utils;
use utils::config_ops::ConfigOptions;

#[tokio::main]
async fn main() {
    #[derive(Parser)]
    #[command(
        name = "hnt",
        author = "Kishore Kumar",
        version = "2.0.0",
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

            #[arg(short = 'A', long = "ai", default_value_t = false)]
            ai: bool,

            #[arg(short = 'n', long = "dry-run", default_value_t = false)]
            dry_run: bool,

            // For commit msg and branch name
            #[arg(trailing_var_arg = true)]
            input: Vec<String>,
        },
        // AI commands
        Ai {
            #[arg(short, long, group = "ai")]
            key: Option<String>,

            #[arg(group = "ai")]
            prompt: Option<String>,

            #[arg(long, requires = "prompt", default_value_t = false)]
            full: bool,
        },

        //scaffold new project
        Init(init::validate::InitArgs),

        //Config {}
        Config {
            option: Option<ConfigOptions>,
        },
    }

    let cli = Cli::parse();

    match cli.command {
        Commands::Guess { number } => {
            games::guess_parser::parse_guess(number);
        }
        Commands::Push {
            input,
            set_upstream,
            ai,
            dry_run,
        } => {
            git::push::push(&input, set_upstream, ai, dry_run).await;
        }
        Commands::Ai { key, prompt, full } => {
            ai::handler::handle_prompt(key, prompt, full).await;
        }
        Commands::Init(init_args) => {
            init::validate::validate(&init_args);
        }
        Commands::Config { option } => match option {
            Some(opt) => {
                utils::config_ops::setup_default_config(opt);
            }
            None => {
                println!("No config option provided. Use --help for more information.");
            }
        },
        // _ => println!("Command not recognized. Please enter a valid command"),
    }
}
