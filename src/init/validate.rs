use clap::{ Parser, ValueEnum };
use crate::init;

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false)]
    yes: bool,

    #[arg(short, long, default_value_t = false)]
    quick: bool,

    project_name: Option<String>,

    #[arg(trailing_var_arg = true)]
    frameworks: Vec<String>,

    #[arg( long )]
    frontend: Option<FrontendLang>,

    #[arg( long )]
    backend: Option<BackendLang>,

    #[arg( long, default_value_t = true, action = clap::ArgAction::SetTrue )]
    #[arg( long = "no-tailwind", action = clap::ArgAction::SetFalse )]
    tailwind: bool,

    #[arg( long, default_value_t = false, action = clap::ArgAction::SetTrue )]
    #[arg( long = "no-git", action = clap::ArgAction::SetFalse )]
    git: bool,

    #[arg( long, default_value_t = false, action = clap::ArgAction::SetTrue )]
    #[arg( long = "no-shadcn", action = clap::ArgAction::SetFalse )]
    shadcn: bool,

    #[arg( long, default_value_t = false )]
    force: bool,

    #[arg( long = "skip-install", default_value_t = false )]
    skip_install: bool,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum FrontendLang {
    React,
    ReactTs,
    Nextjs,
    NextjsTs,
    Svelte,
    Vanilla,
    VanillaTs,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum BackendLang {
    Express,
    ExpressTs,
    Fastapi,
    Gin,
    Axum,
}


pub fn validate(args: &InitArgs) {
    println!("From validate function");
    println!("{:?}", args);
    init::scaffold::scaffold_project(args.yes, args.project_name.clone());
}