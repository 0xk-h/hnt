use clap::{Parser, ValueEnum};
use heck::ToKebabCase;
use std::env;
use std::fmt::Debug;
use std::path::Path;

use super::prompts::ProjectConfig;
use crate::init;
use crate::init::fs_ops;
use crate::utils::config::HntConfig;

#[derive(Debug, Parser)]
pub struct InitArgs {
    #[arg(short, long, default_value_t = false, group = "mode_group")]
    yes: bool,

    #[arg(short, long, default_value_t = false, group = "mode_group")]
    quick: bool,

    project_name: Option<String>,

    #[arg(long)]
    frontend: Option<FrontendLang>,

    #[arg(long)]
    backend: Option<BackendLang>,

    #[arg(long)]
    tailwind: bool,

    #[arg(long)]
    git: bool,

    #[arg(long)]
    shadcn: bool,

    #[arg(short, long)]
    force: bool,
    // #[arg( long="skip-install",default_value_t=true, action = clap::ArgAction::Set)]
    // skip_install: bool,
}

#[derive(Clone, Debug, ValueEnum)]
#[clap(rename_all = "kebab-case")]
enum FrontendLang {
    React,
    ReactTs,
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
    println!("validating the prompt");

    if args.frontend.is_none() && args.backend.is_none() && !args.yes {
        init::wizard::wizard(args.quick, args.project_name.clone(), args.force);
        return;
    }

    let cfg = HntConfig::load();

    if cfg.init_defaults.frontend.is_none()
        && cfg.init_defaults.backend.is_none()
        && args.frontend.is_none()
        && args.backend.is_none()
    {
        eprintln!(
            "Missing both frontend and backend — Run `hnt config` to set defaults or remove --yes for interactive setup."
        );
        return;
    }

    let name: String = match args.project_name {
        Some(ref name) if !name.trim().is_empty() && name.trim() != "." => {
            let name = args.project_name.clone().unwrap();
            let path = Path::new(&name);
            if !fs_ops::check(&path, Some(args.force)) {
                return;
            }
            name
        }
        _ => {
            let path = env::current_dir().expect("Failed to get current directory");
            if !fs_ops::check(&path, Some(args.force)) {
                return;
            }
            String::from(".")
        }
    };

    let cfg = to_project_config(&cfg, args, name);

    println!("Creating project with config: {:?}", cfg);

    init::scaffold::scaffold(cfg);
}

fn to_project_config(cfg: &HntConfig, args: &InitArgs, name: String) -> ProjectConfig {
    let frontend: Option<String> =
        enum_to_kebab(&args.frontend).or(cfg.init_defaults.frontend.clone());

    let backend: Option<String> =
        enum_to_kebab(&args.backend).or(cfg.init_defaults.backend.clone());

    // Determine project type
    let project_type = match (&frontend, &backend) {
        (Some(_), Some(_)) => "Fullstack".to_string(),
        (Some(_), None) => "Frontend".to_string(),
        (None, Some(_)) => "Backend".to_string(),
        _ => "Unknown".to_string(),
    };

    init::prompts::ProjectConfig {
        name,
        project_type,
        frontend,
        backend,
        use_tailwind: args.tailwind || cfg.init_defaults.use_tailwind,
        git_init: args.git || cfg.init_defaults.git_init,
    }
}

fn enum_to_kebab<T: Debug>(opt: &Option<T>) -> Option<String> {
    opt.as_ref().map(|t| format!("{:?}", t).to_kebab_case())
}
