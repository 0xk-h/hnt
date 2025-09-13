use crate::init::prompts::ProjectConfig;

pub fn scaffold_project() {
    // Get project configuration from prompts
    let config: ProjectConfig = crate::init::prompts::get_project_config();

println!("✅ Project configuration:");
println!("Project name: {}", config.name);
println!("Project type: {}", config.project_type);

// Frontend
if let Some(ref fe) = config.frontend {
    println!("Frontend framework: {}", fe);
}

// Backend
if let Some(ref be) = config.backend {
    println!("Backend framework: {}", be);
}

// Language
if let Some(ref lang) = config.language {
    println!("Language: {}", lang);
}

// TypeScript / Tailwind
println!("Use TypeScript: {}", config.use_typescript);
println!("Use Tailwind: {}", config.use_tailwind);

// Database
if let Some(ref db) = config.database {
    println!("Database: {}", db);
}

// Git
println!("Initialize Git repo: {}", config.git_init);

// UI Library
if config.use_shadcn {
    println!("UI component library: shadcn/ui");
} else {
    println!("UI component library: None");
}

println!("\nNext steps:");
println!("1. cd {}", config.name);
println!("2. Install dependencies (npm/pip/cargo/go depending on project)");
println!("3. Start building your project!");

}
