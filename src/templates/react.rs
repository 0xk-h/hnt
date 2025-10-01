
use crate::init::prompts::ProjectConfig;

pub fn create(config: &ProjectConfig) -> std::io::Result<()> {
    println!(" Creating React project: {}", config.name);
    Ok(())

}
