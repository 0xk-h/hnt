use super::project_summary::print_next_steps;
use super::prompts::ProjectConfig;
use crate::generator;

pub fn scaffold(config: ProjectConfig) {
    if let Some(frontend) = &config.frontend {
        let res = match frontend.as_str() {
            "react" | "react-ts" => generator::react::create(&config),
            "vanilla" | "vanilla-ts" => generator::vanilla::create(&config),
            _ => {
                println!("Frontend template '{}' is not yet supported.", frontend);
                std::process::exit(1);
            }
        };
        if let Err(e) = res {
            eprintln!("Failed to scaffold frontend '{}': {}", frontend.as_str(), e);
            return;
        }
    }

    if let Some(backend) = &config.backend {
        let res = match backend.as_str() {
            "express" | "express-ts" => generator::express::create(&config),
            "fastapi" => generator::fastapi::create(&config),
            "gin" => generator::gin::create(&config),
            "axum" => generator::axum::create(&config),
            _ => {
                println!("Backend template '{}' is not yet supported.", backend);
                std::process::exit(1);
            }
        };
        if let Err(e) = res {
            eprintln!("Failed to scaffold backend '{}': {}", backend.as_str(), e);
            return;
        }
    }

    print_next_steps(&config.name);
}
