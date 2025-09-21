use clap::{ Parser, ValueEnum };
use crate::init;

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false, group = "mode_group")]
    yes: bool,

    #[arg(short, long, default_value_t = false, group = "mode_group")]
    quick: bool,

    project_name: Option<String>,

    #[arg( long )]
    frontend: Option<FrontendLang>,

    #[arg( long )]
    backend: Option<BackendLang>,

    #[arg( long = "tailwind", default_value_t = true, group = "tailwind_group")]
    tailwind: bool,
    #[arg( long = "no-tailwind", group = "tailwind_group")]
    no_tailwind: bool,               // correct behaviour

    #[arg( long = "git", group = "git_group" )]
    git: bool,                       // correct behaviour    
    #[arg( long = "no-git", group = "git_group" )]
    no_git: bool,

    #[arg( long = "shadcn", group = "shadcn_group" )]
    shadcn: bool,
    #[arg( long = "no-shadcn", group = "shadcn_group" )]
    no_shadcn: bool,

    #[arg( short, long )]
    force: bool,

    #[arg( long = "skip-install", default_value_t = true, group = "skip_install_group" )]
    skip_install: bool,
    #[arg( long = "no-skip-install", group = "skip_install_group" )]
    no_skip_install: bool,
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
    println!("git: {}, no_git: {}", args.git, args.no_git);
    println!("{:?}", args);
    init::scaffold::scaffold_project(args.yes, args.project_name.clone());
}