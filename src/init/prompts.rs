use cliclack::{ input, confirm, select };
#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub project_type: String,
    pub frontend: Option<String>,
    pub backend: Option<String>,
    pub use_tailwind: bool,
    pub git_init: bool,
    pub use_shadcn: bool,
}

pub fn get_project_config(name:Option<String>, quick:bool) -> ProjectConfig {

    // 1. Project name
    let name = match name {
        Some(n) => n,
        None => input("Enter project name:")
            .placeholder("my-project")
            .interact()
            .unwrap_or_else(|_| String::from("my-project")),
    };

    // 2. Project type
    let project_types = vec![
        ("Frontend", "Frontend", ""),
        ("Backend", "Backend", ""),
        ("Fullstack", "Fullstack", "")
    ];
    let project_type = select("Choose project type:")
        .items(&project_types)
        .interact()
        .unwrap()
        .to_string();

    // 3. Frontend
    let frontend = if project_type == "Frontend" || project_type == "Fullstack" {
        let frontends = vec![
            ("react", "React", ""),
            ("next.js", "Next.js", ""),
            ("vanilla", "Vanilla", "HTML + CSS + JS"),
            ("svelte", "Svelte", "")
        ];
        Some(
            select("Choose frontend framework:")
                .items(&frontends)
                .interact()
                .unwrap()
                .to_string()
        )
    } else {
        None
    };

    // 4. Backend
    let backend = if project_type == "Backend" || project_type == "Fullstack" {
        let backends = vec![
            ("Node.js (Express.js)", "Express.js", "Node.js"),
            ("Python (FastAPI)", "FastAPI", "Python"),
            ("Go (Gin)", "Gin", "Golang"),
            ("Rust (Axum)", "Axum", "Rust")
        ];
        Some(
            select("Choose backend framework:")
                .items(&backends)
                .interact()
                .unwrap()
                .to_string()
        )
    } else {
        None
    };

    let backend = if let Some(ref ln) = backend {
        if ln.contains("FastAPI") {
            Some(String::from("fastapi"))
        } else if ln.contains("Gin") {
            Some(String::from("gin"))
        } else if ln.contains("Axum") {
            Some(String::from("axum"))
        } else {
            // Express.js
            if quick {
                Some(String::from("express"))
            } else {
                let use_ts = confirm("Use TypeScript for Express backend?")
                    .initial_value(false)
                    .interact()
                    .unwrap_or(false);
                Some(if use_ts { String::from("express-ts") } else { String::from("express") })
            }
        }
    } else {
        None
    };

    // 6. TypeScript (only for frontend/fullstack)
    let frontend = if let Some(ref ln) = frontend {
        if ln == "Svelte" || quick {
            Some(ln.to_string())
        } else {
            let use_ts = confirm("Use TypeScript for frontend?")
                .initial_value(false)
                .interact()
                .unwrap_or(false);
            Some(if use_ts { format!("{}-ts", ln) } else { ln.to_string() })
        }
    } else {
        None
    };

    // 7. TailwindCSS (only for frontend/fullstack)
    let use_tailwind = if quick {
        true
    } else if frontend.is_some() {
        confirm("Use TailwindCSS?")
            .initial_value(true)
            .interact()
            .unwrap_or(true)
    } else {
        false
    };

    // 8. Git init
    let git_init = if quick {
        true
    } else {
        confirm("Initialize a Git repository?")
            .initial_value(true)
            .interact()
            .unwrap_or(true)
    };

    // 9. shadcnUI  (only for React/Next + Tailwind)
    let use_shadcn = if quick {
        true
    } else if let Some(ref ln) = frontend {
        if (ln == "React" || ln == "Next.js") && use_tailwind {
            confirm("Use shadcn/ui component library?")
                .initial_value(true)
                .interact()
                .unwrap_or(true)
        } else {
            false
        }
    } else {
        false
    };

    ProjectConfig {
        name,
        project_type,
        frontend,
        backend,
        use_tailwind,
        git_init,
        use_shadcn,
    }
}
