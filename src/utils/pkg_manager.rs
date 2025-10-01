use std::process::Command;

pub fn detect_package_manager(manager: &str) -> bool {

    Command::new(manager).arg("--version").output().is_ok()
}