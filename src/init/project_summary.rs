use crate::init::prompts::ProjectConfig;

// Only for debugging
// pub fn print_project_summary(config: &ProjectConfig) {
//     println!("\n Project configuration:");
//     println!("  Project name: {}", config.name);
//     println!("  Project type: {}", config.project_type);
//     if let Some(ref ln) = config.frontend {
//         println!("  Frontend framework: {}", ln);
//     }
//     if let Some(ref ln) = config.backend {
//         println!("  Backend framework: {}", ln);
//     }
//     println!("  Use Tailwind: {}", config.use_tailwind);
//     println!("  Initialize Git repo: {}", config.git_init);
// }

pub fn print_next_steps(name: &str, cfg: &ProjectConfig) {
    let root = if name == "." { "" } else { name };

    println!("\nNext steps:\n");

    let frontend = cfg.frontend.as_deref();
    let backend = cfg.backend.as_deref();

    match (frontend, backend) {
        (Some(front), None) => {
            if root != "" {
                println!(" cd {}", root);
            }

            let finfo = frontend_info(front, cfg.use_tailwind);
            if !finfo.0.is_empty() {
                println!(" {}", finfo.0);
            }
            if !finfo.1.is_empty() {
                println!(" {}", finfo.1);
            }
        }

        (None, Some(back)) => {
            if root != "" {
                println!(" cd {}", root);
            }

            let binfo = backend_info(back);
            if !binfo.0.is_empty() {
                println!(" {}", binfo.0);
            }
            if !binfo.1.is_empty() {
                println!(" {}", binfo.1);
            }
        }

        (Some(front), Some(back)) => {
            println!("# Frontend (Terminal 1)");
            if root != "" {
                println!(" cd {}/frontend", root);
            } else {
                println!(" cd frontend")
            }

            let finfo = frontend_info(front, cfg.use_tailwind);
            if !finfo.0.is_empty() {
                println!(" {}", finfo.0);
            }
            if !finfo.1.is_empty() {
                println!(" {}", finfo.1);
            }
            println!();

            println!("# Backend (Terminal 2)");
            if root != "" {
                println!(" cd {}/backend", root);
            } else {
                println!(" cd backend")
            }

            let binfo = backend_info(back);
            if !binfo.0.is_empty() {
                println!(" {}", binfo.0);
            }
            if !binfo.1.is_empty() {
                println!(" {}", binfo.1);
            }
        }
        _ => {}
    }

    println!("\nStart building your project!\n");
}

fn frontend_info(front: &str, use_tailwind: bool) -> (&str, &str) {
    match front {
        "vanilla" if !use_tailwind => ("", "run index.html directly"),
        "vanilla" | "vanilla-ts" | "react" | "react-ts" => ("npm install", "npm run dev"),
        _ => ("", ""),
    }
}

fn backend_info(back: &str) -> (&str, &str) {
    match back {
        "express" | "express-ts" => ("npm install", "npm run dev"),
        "fastapi" => (
            "pip install -r requirements.txt",
            "uvicorn main:app --reload",
        ),
        "axum" => ("cargo build", "cargo run"),
        "gin" => ("go mod tidy", "go run cmd/server/main.go"),
        _ => ("", ""),
    }
}
