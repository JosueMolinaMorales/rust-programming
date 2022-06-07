use rand::Rng;
use std::io;

const CHOICES: [&str; 3] = ["Rock", "Paper", "Scissors"];

fn main() {

    println!("Welcome to Rock Paper Scissors!");
    loop {
        let mut user_choice = String::default();
        let pc_choice = String::from(CHOICES[rand::thread_rng().gen_range(0..3)]);
        println!("
        1. Rock \n
        2. Paper \n
        3. Scissors \n
        Make your Choice!");

        io::stdin().read_line(&mut user_choice).expect("Error reading text");

        let user_choice: usize = match user_choice.trim().parse() {
            Ok(val) => {
                if val > 3 {
                    println!("ERROR: Please enter 1, 2, or 3");
                    continue;
                } 
                val
            },
            Err(_) => {
                println!("ERROR: Please enter a number");
                continue;
            }
        };

        let user_choice = String::from(CHOICES[user_choice-1]);

        println!("The PC choose {}! and you choose {}!", pc_choice, user_choice);

        if pc_choice == user_choice {
            println!("Its a draw!");
        } else if pc_choice == String::from("Rock") {
            if user_choice == String::from("Paper") {
                println!("You Win!");
            } else if user_choice == String::from("Scissors") {
                println!("You Lose!");
            }
        } else if pc_choice == String::from("Paper") {
            if user_choice == String::from("Rock") {
                println!("You Lose!");
            } else if user_choice == String::from("Scissors") {
                println!("You Win!");
            }
        } else { // pc_choice == Scissors
            if user_choice == String::from("Rock") {
                println!("You Win!");
            } else if user_choice == String::from("Paper") {
                println!("You Lose!");
            }
        }
        let mut user_choice = String::default();
        println!("Play again? Enter 'y'");
        io::stdin().read_line(&mut user_choice).expect("Error Reading Input");
        
        if user_choice.as_bytes()[0] as char != 'y' {
            break;
        }

    }
}


