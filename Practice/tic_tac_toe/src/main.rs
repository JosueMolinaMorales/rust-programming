use std::io;
use table::Choice;

pub mod table;

fn main() {
    let mut ttt_table = table::Table::new();
    
    // Print Welcome
    println!("Welcome to Tic-Tac-Toe!");
    // Player 1 Name
    let mut input = String::new();
    let player_one: Choice;

    println!("Player 1 please enter your X or O: ");
    loop {
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim().to_ascii_uppercase();
                if input == String::from("X") {
                    player_one = Choice::X;
                } else if input == String::from("O") {
                    player_one = Choice::O;
                } else {
                    println!("Please enter either X or O");
                    continue;
                }
                break;
            },
            Err(_) => {
                println!("There was an error. Please try again");
                continue;
            }
        }
    }
    // Player 2 Name
    let player_two = match player_one {
        Choice::X => Choice::O,
        Choice::O => Choice::X
    };

    println!("Player 2 you are {}", player_two);

    // Other Variables
    let mut player_choice;
    let mut curr_player = 1;

    // Loop
    'outer: loop {
        ttt_table.print_table();

        if curr_player == 1 {
            println!("Player one please enter your choice! (1-9)");
        } else if curr_player == 2 {
            println!("Player two please enter your choice! (1-9)");
        }

        loop {
            let curr_choice = if curr_player == 1 { player_one } else { player_two };
            player_choice = String::new();
            match io::stdin().read_line(&mut player_choice) {
                Ok(_) => {
                    let player_choice: usize = player_choice.trim().parse().unwrap();
                    if player_choice < 1 || player_choice > 9 {
                        println!("Please enter a number between 1-9");
                        continue;
                    }
                    if !ttt_table.add_choice(curr_choice, player_choice-1) {
                        println!("This position is taken! Pick again!");
                        continue;
                    }
                    if ttt_table.check_for_winner(curr_choice) {
                        println!("Player {} wins!" , curr_player);
                        break 'outer;
                    }
                    if ttt_table.check_for_draw() {
                        println!("Its a draw!");
                        break 'outer;
                    }
                    break;
                },
                Err(_) => {
                    println!("There was an error. Please try again");
                    continue;
                }
            };
            
        }
        curr_player = if curr_player == 1 { 2 } else { 1 }; 
    }

    ttt_table.print_table();
    
}
