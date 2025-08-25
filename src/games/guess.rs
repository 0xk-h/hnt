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

    println!("You guessed the number: {}", number);
    println!("The secret number is: {}", secret);
    if number == secret {
        println!("You win!🎉");
    } else {
        println!("You lose!");
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
                    println!("🎉 Correct! The secret number was {}. guessed in {} attempts", num, guess);

                    let reply = match guess {
                        1 => "🌿 First try? Go touch some grass, dude!",
                        2..=3 => "😏 Decent… still smells like beginner's luck tho.",
                        4..=5 => "🤡 Took you long enough, clown.",
                        6..=7 => "💀 Took 6-7 tries? Loser! Even a snail would be faster!",
                        _ => "😂 What's so hard? Even a toddler could do better!",
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