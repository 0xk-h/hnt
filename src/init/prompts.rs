use inquire::{Text, Select, Confirm};

#[derive(Debug)]
pub struct ProjectConfig {
    pub name: String,
    pub project_type: String,
    pub frontend: Option<String>,
    pub backend: Option<String>,
    pub language: Option<String>,
    pub use_typescript: bool,
    pub use_tailwind: bool,
    pub database: Option<String>,
    pub git_init: bool,
    pub use_shadcn: bool,
}

pub fn get_project_config() -> ProjectConfig {
    // 1. Project name
    let name = Text::new("Enter project name:")
        .prompt()
        .unwrap_or_else(|_| "my-project".to_string());

    // 2. Project type
    let project_types = vec!["Frontend", "Backend", "Fullstack"];
    let project_type = Select::new("Choose project type:", project_types)
        .prompt()
        .unwrap()
        .to_string();

    // 3. Frontend selection (if needed)
    let frontend = if project_type == "Frontend" || project_type == "Fullstack" {
        let frontends = vec!["Vanilla", "React", "Next.js", "Svelte"];
        Some(
            Select::new("Choose frontend framework:", frontends)
                .prompt()
                .unwrap()
                .to_string(),
        )
    } else {
        None
    };

    // 4. Backend selection (if needed)
    let backend = if project_type == "Backend" || project_type == "Fullstack" {
        let backends = vec!["Node.js (Express.js)", "Python (FastAPI)", "Go (Gin)", "Rust (Axum)"];
        Some(
            Select::new("Choose backend framework:", backends)
                .prompt()
                .unwrap()
                .to_string(),
        )
    } else {
        None
    };

    // 5. Language selection (frontend/backend)
    let language = if let Some(ref fe) = frontend {
        if fe == "Vanilla" || fe == "Svelte" {
            Some("JavaScript".to_string())
        } else {
            let use_ts = Confirm::new("Use TypeScript?")
                .with_default(false)
                .prompt()
                .unwrap_or(false);
            Some(if use_ts { "TypeScript".to_string() } else { "JavaScript".to_string() })
        }
    } else if let Some(ref be) = backend {
        if be.contains("FastAPI") {
            Some("Python".to_string())
        } else if be.contains("Gin") {
            Some("Go".to_string())
        } else if be.contains("Axum") {
            Some("Rust".to_string())
        } else {
            // Express.js
            let use_ts = Confirm::new("Use TypeScript?")
                .with_default(false)
                .prompt()
                .unwrap_or(false);
            Some(if use_ts { "TypeScript".to_string() } else { "JavaScript".to_string() })
        }
    } else {
        None
    };

    let use_typescript = language
        .as_ref()
        .map(|l| l.to_lowercase() == "typescript")
        .unwrap_or(false);

    // 6. TailwindCSS (only for frontend/fullstack)
    let use_tailwind = if frontend.is_some() {
        Confirm::new("Use Tailwind CSS?")
            .with_default(true)
            .prompt()
            .unwrap_or(true)
    } else {
        false
    };

    // 7. Database (backend/fullstack)
    let database = if backend.is_some() {
        let dbs = vec!["None", "PostgreSQL", "MongoDB"];
        Some(
            Select::new("Choose a database:", dbs)
                .prompt()
                .unwrap()
                .to_string(),
        )
    } else {
        None
    };

    // 8. Git init
    let git_init = Confirm::new("Initialize a new git repository?")
        .with_default(true)
        .prompt()
        .unwrap_or(true);

    // 9. shadcn/ui (optional, only React/Next + Tailwind)
    let use_shadcn = if let Some(ref fe) = frontend {
        if (fe == "React" || fe == "Next.js") && use_tailwind {
            Confirm::new("Add shadcn/ui component library?")
                .with_default(false)
                .prompt()
                .unwrap_or(false)
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
        language,
        use_typescript,
        use_tailwind,
        database,
        git_init,
        use_shadcn,
    }
}
