use crate::utils::config::HntConfig;
use colored::*;

pub fn key(new_key: &str) {
    HntConfig::update_ai_key(new_key);
    println!("{}", "AI key updated!".bold().green());
}
