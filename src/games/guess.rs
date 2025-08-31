use std::io::{self, Write};

fn print_banner() {
    println!( r#"
 (`─').─>           <─. (`─')_ (`─')      (`─')  _   (`─')  
 (OO )__      .─>      ╲( OO) )( OO).─>   ( OO).─╱<─.(OO )  
,──. ,'─',──.(,──.  ,──./ ,──╱ /    '._  (,──────.,──────,) 
│  │ │  ││  │ │(`─')│   ╲ │  │ │'──...__) │  .───'│   /`. ' 
│  `─'  ││  │ │(OO )│  . '│  │)`──.  .──'(│  '──. │  │_.' │ 
│  .─.  ││  │ │ │  ╲│  │╲    │    │  │    │  .──' │  .   .' 
│  │ │  │╲  '─'(_ .'│  │ ╲   │    │  │    │  `───.│  │╲  ╲  
`──' `──' `─────'   `──'  `──'    `──'    `──────'`──' '──' 

          🎲 Welcome to the Guessing Game! 🎲
"#
    );
}

pub fn guess(number: i32) {
    let secret = rand::random_range(1..=10);

    println!("The secret number is: {}", secret);

    let replies = [
        "✨ Big brain energy! You're unstopppable! ✨",
        "🔥 Poggers! Nailed it in one go!",
        "😎 Absolute legend move! Respect!",
    ];
    if number == secret {
        println!("You win!🎉");
        let reply = replies[rand::random_range(0..replies.len())];
        println!("{}", reply);
    } else {
        println!("❌ You lose! Better luck next time.");
    }
}

pub fn start() {
    print_banner();
    println!("Guess a number between 1-10, or 'q' to quit.");

    let secret = rand::random_range(1..=10);
    let mut guess = 0;

    loop {
        print!("Your guess: ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        let quits = ["q", "Q", "quit", "Quit", "QUIT"];
        if quits.contains(&input) {
            println!("👋 Quitting game...");
            break;
        }

        match input.parse::<i32>() {
            Ok(num) => {
                if !(1..=10).contains(&num) {
                    println!("⚠️ Please enter a number between 1 and 10, or 'q' to quit.");
                }
                if num == secret {
                    guess += 1;
                    println!("🎉 Correct! The secret number was {}.", num);

                    let reply = match guess {
                        1 => "🌿 First try? Go touch some grass, dude!",
                        2..=3 => "😏 Not bad… you're getting the hang of it!",
                        4 => "😎 Decent… not bad for a rookie, ngl.",
                        5 => "🤔 Average vibes… did someone hit the slow-mo button?",
                        6..=7 => "😵 Bruh… did you forget this was a game or nah?",
                        _ => "😂 Iconic struggle… even a snail was judging your pace!",
                    };
                    println!("✅ You guessed in {} attempt(s)!", guess);
                    println!("{}", reply);

                    break;
                } else if num < secret {
                    guess += 1;
                    println!("❌ Too low! Try again.");
                } else {
                    guess += 1;
                    println!("❌ Too high! Try again.");
                }
            }
            Err(_) => println!("⚠️ Invalid input. Enter a number between 1-10, or 'q' to quit."),
        }
    }

    println!("Game over!");
}