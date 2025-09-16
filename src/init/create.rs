use cliclack::{intro, outro, select};

pub fn check() -> bool {

    let _ = intro("My-app");

        let ans = select("Current directory is not empty. Please choose how to proceed:").item("value", 0, "batman")
            .interact()
            .unwrap();
    let _ = outro("Operation canceled.");

        match ans {
            "0" => {
                println!("❌ Operation canceled.");
                return false;
            }
            _ => return false,
        }
}

