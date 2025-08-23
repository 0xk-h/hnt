
pub fn guess(number: Option<i32>) {
    let secret = rand::random_range(1..=10);

    if let Some(number) = number {
        println!("You guessed the number: {}", number);
        println!("The secret number is: {}", secret);
        if number == secret {
            println!("You win!🎉");
        } else {
            println!("You lose!");
        }
    } else {
        println!("No number was provided.");
    }
}