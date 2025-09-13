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
    pub git_init: bool,
    pub use_shadcn: bool,
}

pub fn get_project_config(name:Option<String>) -> ProjectConfig {
    // 1. Project name
    let name = match name {
        Some(n) => n,
        None => Text::new("Enter project name:")
            .with_placeholder("my-awesome-project")
            .prompt()
            .unwrap_or_else(|_| String::from("Project007")),
    };

    // 2. Project type
    let project_types = vec!["Frontend", "Backend", "Fullstack"];
    let project_type = Select::new("Choose project type:", project_types)
        .prompt()
        .unwrap()
        .to_string();

    // 3. Frontend
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

    // 4. Backend
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

    // 5. Language (only for backend/fullstack)
    let language = if let Some(ref ln) = backend {
        if ln.contains("FastAPI") {
            Some(String::from("FastAPI"))
        } else if ln.contains("Gin") {
            Some(String::from("Go"))
        } else if ln.contains("Axum") {
            Some(String::from("Rust"))
        } else {
            // Express.js
            let use_ts = Confirm::new("Use TypeScript for Express backend?")
                .with_default(false)
                .prompt()
                .unwrap_or(false);
            Some(if use_ts { String::from("TypeScript") } else { String::from("JavaScript") })
        }
    } else {
        None
    };

    // 6. TypeScript (only for frontend/fullstack)
    let use_typescript = if let Some(ref ln) = frontend {
        if ln == "Vanilla" || ln == "Svelte" {
            false
        } else {
            Confirm::new("Use TypeScript for frontend?")
                .with_default(false)
                .prompt()
                .unwrap_or(false)
        }
    } else {
        false
    };

    // 6. TailwindCSS (only for frontend/fullstack)
    let use_tailwind = if frontend.is_some() {
        Confirm::new("Use Tailwind CSS?")
            .with_default(true)
            .prompt()
            .unwrap_or(true)
    } else {
        false
    };

    // 8. Git init
    let git_init = Confirm::new("Initialize a new git repository?")
        .with_default(true)
        .prompt()
        .unwrap_or(true);

    // 9. shadcnUI  (only for React/Next + Tailwind)
    let use_shadcn = if let Some(ref ln) = frontend {
        if (ln == "React" || ln == "Next.js") && use_tailwind {
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
        git_init,
        use_shadcn,
    }
}
