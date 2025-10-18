use colored::*;
use std::io::{self, Write};

fn print_banner() {
    println!(
        "{}",
        r#"
 (`─').─>           <─. (`─')_ (`─')      (`─')  _   (`─')
 (OO )__      .─>      ╲( OO) )( OO).─>   ( OO).─╱<─.(OO )
,──. ,'─',──.(,──.  ,──./ ,──╱ /    '._  (,──────.,──────,)
│  │ │  ││  │ │(`─')│   ╲ │  │ │'──...__) │  .───'│   /`. '
│  `─'  ││  │ │(OO )│  . '│  │)`──.  .──'(│  '──. │  │_.' │
│  .─.  ││  │ │ │  ╲│  │╲    │    │  │    │  .──' │  .   .'
│  │ │  │╲  '─'(_ .'│  │ ╲   │    │  │    │  `───.│  │╲  ╲
`──' `──' `─────'   `──'  `──'    `──'    `──────'`──' '──'
"#
        .bold()
        .bright_purple()
    );
    println!(
        "{}{}",
        " ".repeat(12),
        "🎲 Welcome to the Guessing Game! 🎲"
            .bold()
            .bright_magenta()
    );
    println!();
}

pub fn guess(number: i32) {
    let secret = rand::random_range(1..=10);

    println!("The secret number is: {}", secret);

    let replies = [
        "✨ Big brain energy! You're unstopppable!",
        "🔥 Poggers! Nailed it in one go!",
        "😎 Absolute legend move! Respect!",
    ];
    if number == secret {
        println!("You win!");
        let reply = replies[rand::random_range(0..replies.len())];
        println!("{}", reply);
    } else {
        println!("You lose! Better luck next time.");
    }
}

pub fn start() {
    print_banner();
    println!(
        "{}",
        "Guess a number between 1-10, or 'q' to quit.".dimmed()
    );

    let secret = rand::random_range(1..=10);
    let mut guess = 0;

    loop {
        print!("{}", "Your guess: ".bold());
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let quits = ["q", "Q", "quit", "Quit", "QUIT"];
        if quits.contains(&input) {
            println!("{}", "Quitting game...".blue());
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => {
                if !(1..=10).contains(&num) {
                    println!("Please enter a number between 1 and 10, or 'q' to quit.");
                }
                if num == secret {
                    guess += 1;
                    println!(
                        "{}",
                        format!(
                            "🎉 {} The secret number was {}.",
                            "Correct!".bold(),
                            num.to_string().bold()
                        )
                        .green()
                    );

                    let reply = match guess {
                        1 => "🌿 First try? Go touch some grass, dude!",
                        2..=3 => "😏 Not bad… you're getting the hang of it!",
                        4 => "😎 Decent… not bad for a rookie, ngl.",
                        5 => "🤔 Average vibes… did someone hit the slow-mo button?",
                        6..=7 => "😵 Bruh… did you forget this was a game or nah?",
                        _ => "😂 Iconic struggle… even a snail was judging your pace!",
                    };
                    println!(
                        "{}",
                        format!("   You guessed in {} attempts!", guess.to_string().bold())
                            .yellow()
                    );
                    println!("{}", reply.bold().blue());
                    break;
                } else if num < secret {
                    guess += 1;
                    println!("{}", "Too low! Try again.".red());
                } else {
                    guess += 1;
                    println!("{}", "Too high! Try again.".red());
                }
            }
            Err(_) => println!(
                "{}",
                "Invalid input. Enter a number between 1-10, or 'q' to quit."
                    .bold()
                    .red()
            ),
        }
    }

    println!("{}", "Game over!".bold().bright_purple());
}
