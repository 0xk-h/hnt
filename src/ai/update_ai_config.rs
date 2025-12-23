use crate::utils::config::HntConfig;
use cliclack::{input, intro, select};
use colored::*;

pub fn update() {
    let _ = intro("HNT Wizard".bold());

    let options = vec![
        ("gemini", "Gemini", "update Gemini API key"),
        ("open_router", "Open Router", "update Open Router API key"),
        (
            "set_default",
            "Set Default AI",
            "set the default AI provider",
        ),
    ];

    let choice = select("Select an AI provider to update:")
        .items(&options)
        .interact()
        .unwrap()
        .to_string();

    if choice == "set_default" {
        let ai_options = vec![("gemini", "Gemini", ""), ("open_router", "Open Router", "")];
        let default_choice = select("Select the default AI provider:")
            .items(&ai_options)
            .interact()
            .unwrap();

        HntConfig::update_default_ai(default_choice);

        println!(
            "{}",
            format!("Default AI provider set to {}!", default_choice)
                .bold()
                .green()
        );
        return;
    }

    let new_key = input("Enter new API key:")
        .interact()
        .unwrap_or_else(|_| String::from(""));

    HntConfig::update_ai_key(&choice, &new_key);

    println!("{}", "AI key updated!".bold().green());
}
