use crate::games::guess;

pub fn parse_guess(number: Option<String>) {
    if let Some(num_str) = number {
        match num_str.trim().parse::<i32>(){
            Ok(num) => {
                if (1..=10).contains(&num) {
                    guess::guess(num);
                } else {
                    println!("⚠️ Number out of range. Please enter a value between 1 and 10.");
                }
            }
            Err(_) => {
                println!("❌ Invalid input '{}'. Please enter a number between 1 and 10.", num_str);
                println!("or type 'hnt guess' to enter interactive mode.");
            }
        }
    } else {
        guess::start();
    }
}