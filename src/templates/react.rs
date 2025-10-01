use std::process::Command;
use std::fs;
use std::env;
use std::path::PathBuf;

use crate::init::prompts::ProjectConfig;
use crate::utils::pkg_manager::detect_package_manager;

const VITE_CONFIG: &str = r#"
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react(), tailwindcss()],
})
"#;

const DIRS: [&str; 6] = [
    "src/assets",
    "src/components",
    "src/pages",
    "src/hooks",
    "src/store",
    "src/utils",
];

const STYLE: &str = r#"
/* Coming soon... */
"#;

pub fn create(config: &ProjectConfig) -> std::io::Result<()> {
    println!(" Creating React project: {}", config.name);
    if !(detect_package_manager("npm")) {
        eprintln!("npm is not installed. Please install Node.js and npm to proceed.");
        std::process::exit(1);
    }

    if let Some(frontend) = &config.frontend {
        if frontend != "react" && frontend != "react-ts" {
            eprintln!("Unsupported frontend framework");
            std::process::exit(1);
        }
    }

    // vite with template react or react-ts
    Command::new("npm")
        .args(&["create", "vite@latest", &config.name, "--", "--template", config.frontend.as_ref().unwrap()])
        .status()
        .expect("Failed to run npm create vite");

    let path: PathBuf = if config.name == "." {
        env::current_dir()?
    } else {
        PathBuf::from(&config.name)
    };

    // install tailwindcss
    if config.use_tailwind {
        Command::new("npm")
            .current_dir(&path)
            .args(&["install", "-D", "tailwindcss", "@tailwindcss/vite"])
            .status()
            .expect("Failed to install Tailwind");

        fs::write(path.join("index.css"), "@import \"tailwindcss\";")?;

    } else {
        fs::write(path.join("index.css"), STYLE)?;
    }

    
    for dir in DIRS {
        fs::create_dir_all(path.join("src").join(dir))?;
    }

    fs::write(path.join("vite.config.js"), VITE_CONFIG)?;

// Command::new("npm")
//     .current_dir(&path)
//     .args(&["install", "class-variance-authority", "tailwind-variants", "lucide-react"])
//     .status()
//     .expect("Failed to install shadcn deps");
//     Command::new("npx")
//         .current_dir(&project_path)
//         .args(&["shadcn-ui", "init"])
//         .status()
//         .expect("Failed to init shadcn-ui");

    Ok(())
}
