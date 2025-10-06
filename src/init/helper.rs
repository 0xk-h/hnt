use std::env;

pub fn get_name(name: &str) -> String {
    if name.trim() == "." {
        let current_dir = env::current_dir().expect("Failed to get current directory");
    
        return current_dir
            .file_name()
            .map(|name| name.to_string_lossy().to_string())
            .unwrap_or_default();
    } else {
        return name.to_string();
    }
}