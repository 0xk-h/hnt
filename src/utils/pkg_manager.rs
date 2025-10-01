use std::process::Command;

fn detect_package_manager() -> String {
    let managers = ["bun", "pnpm", "yarn", "npm"];

    for i in managers.iter() {
        if Command::new(i).arg("--version").output().is_ok() {
            return i.to_string();
        }
    }

    panic!("No supported package manager found (bun, pnpm, yarn, npm).");
}