// use crate::init::prompts::ProjectConfig;

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

pub fn print_next_steps(name: &str) {
    println!("\nNext steps:");
    let mut step: i8 = 1;
    if name != "." {
        println!("{}. cd {}", step, name);
        step += 1;
    }
    println!(
        "{}. Install dependencies (npm/pip/cargo/go depending on project)",
        step
    );
    println!("{}. Start building your project!", step + 1);
    println!();
}
