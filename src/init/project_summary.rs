use crate::init::prompts::ProjectConfig;

// Only for debugging
pub fn print_project_summary(config: &ProjectConfig) {
    println!("\n✅ Project configuration:");
    println!("  Project name: {}", config.name);
    println!("  Project type: {}", config.project_type);

    // Frontend
    if let Some(ref ln) = config.frontend {
        println!("  Frontend framework: {}", ln);
    }

    // Backend
    if let Some(ref ln) = config.backend {
        println!("  Backend framework: {}", ln);
    }

    // Tailwind
    println!("  Use Tailwind: {}", config.use_tailwind);

    // Git
    println!("  Initialize Git repo: {}", config.git_init);
}

pub fn print_next_steps(name: &str) {
    println!("\nNext steps:");
    println!("1. cd {}", name);
    println!("2. Install dependencies (npm/pip/cargo/go depending on project)");
    println!("3. Start building your project!");
    println!();
}
