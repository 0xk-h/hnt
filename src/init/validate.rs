use clap::{ Parser, ValueEnum };
use std::env;

use crate::init;
use crate::utils::config::HntConfig;
use crate::init::fs_cleanup;

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

    #[arg( long )]
    tailwind: bool,

    #[arg( long )]
    git: bool,

    #[arg( long )]
    shadcn: bool,

    #[arg( short, long )]
    force: bool,

    #[arg( long="skip-install",default_value_t=true, action = clap::ArgAction::Set)]
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
    if args.yes {
        let cfg = HntConfig::load();
        if !(cfg.init_defaults.frontend.is_empty() && cfg.init_defaults.backend.is_empty()) {
            eprintln!("Run `hnt config` first to set up defaults or use --quick to skip prompts.");
            return;
        }

        let name: String = match args.project_name {
            Some(ref name) if name.trim().is_empty() => {
                eprintln!("Invalid project name provided with --yes flag.");
                return;
            }
            Some(ref name) if name.trim() == "." => {
                let path = env::current_dir()
                    .expect("Failed to get current directory");

                if !fs_cleanup::check(&path, Some(false)) {
                    return;
                }

                path
                    .file_name()
                    .expect("Failed to get directory name")
                    .to_string_lossy()
                    .to_string()
            }
            None => {
                eprintln!("Project name is required when using --yes flag.");
                return;
            }
            _ => {
                args.project_name.clone().unwrap()
            }
            
        };


        let cfg = to_project_config(&cfg, name);

        init::scaffold::scaffold(cfg, Some(!args.skip_install));

    }

    if args.frontend.is_none() && args.backend.is_none() {
        init::scaffold::wizard(args.quick , args.project_name.clone(), args.force);
    }

    // from args to project config


}

fn to_project_config(args: &HntConfig, name: String) -> init::prompts::ProjectConfig {

    let frontend = Some(args.init_defaults.frontend.clone());
    let backend = Some(args.init_defaults.backend.clone());

    init::prompts::ProjectConfig {
        name,
        project_type: if frontend.is_some() && backend.is_some() {
            "Fullstack".to_string()
        } else if frontend.is_some() {
            "Frontend".to_string()
        } else {
            "Backend".to_string()
        },
        frontend,
        backend,
        use_tailwind: args.init_defaults.use_tailwind,
        git_init: args.init_defaults.git_init,
        use_shadcn: args.init_defaults.use_shadcn,
    }
}