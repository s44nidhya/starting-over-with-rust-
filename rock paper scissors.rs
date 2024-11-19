use rand::Rng;
use std::io;

fn main() {
    println!("Welcome to Rock, Paper, Scissors!");

    let options = ["rock", "paper", "scissors"];
    loop {
        // Get user choice
        let user_choice = get_user_choice(&options);
        if user_choice.is_none() {
            println!("Invalid choice. Please type 'rock', 'paper', or 'scissors'.");
            continue;
        }
        let user_choice = user_choice.unwrap();

        // Generate computer's choice
        let computer_choice = options[rand::thread_rng().gen_range(0..options.len())];

        // Display choices
        println!("You chose: {}", user_choice);
        println!("Computer chose: {}", computer_choice);

        // Determine winner
        match determine_winner(user_choice, computer_choice) {
            Some("user") => println!("You win!"),
            Some("computer") => println!("Computer wins!"),
            _ => println!("It's a tie!"),
        }

        // Check if user wants to play again
        if !play_again() {
            println!("Thanks for playing!");
            break;
        }
    }
}

fn get_user_choice(options: &[&str]) -> Option<&str> {
    let mut input = String::new();
    println!("Choose your option: (rock, paper, scissors)");
    io::stdin().read_line(&mut input).unwrap();
    let choice = input.trim().to_lowercase();
    options.iter().find(|&&opt| opt == choice).copied()
}

fn determine_winner(user: &str, computer: &str) -> Option<&'static str> {
    match (user, computer) {
        ("rock", "scissors") | ("paper", "rock") | ("scissors", "paper") => Some("user"),
        ("rock", "paper") | ("paper", "scissors") | ("scissors", "rock") => Some("computer"),
        _ => None,
    }
}

fn play_again() -> bool {
    let mut input = String::new();
    println!("Do you want to play again? (yes or no)");
    io::stdin().read_line(&mut input).unwrap();
    input.trim().eq_ignore_ascii_case("yes")
}
