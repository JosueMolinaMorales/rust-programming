// Reading input from user. io library
use std::io;
use rand::Rng;
use std::cmp::Ordering;

/*
    By default rust has items defined in the standard library that are in scope of every program
    This is called the prelude.
    Things not in the prelude need to be brought into scope using the keyword 'use'
*/

fn main() {
    println!("Guess the number!");

    /*
        '1..101' is a range expression where the format is: start..end. Includes start but not end
        '1..=100' will include the end
    */
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number);

    println!("Please input your guess.");

    /* 
        'let' creates a variable. In rust, variables are immutable by default
        'mut' makes a variable mutable
        String::new() is a function that returns a new instance of a String
        '::' in '::new' indeicates that new is an associated function of the String type
        associated function is a function thats implemented on a type.
        'new()' creates a new empty string
    */
    let mut guess = String::new();

    /*
        if io wasnt importing using 'use std::io', the 'stdin()' could still be used by
        std::io::stdin()()
        'stin()' returns an instance of 'std::io::Stdin'
        'read_line()' reads the users input and appends it to the variable passed in. Not overwritting the
        content of the variable
        '&' denotes passing by reference. Variable references are immutable by default.
        '&mut' makes the variable mutable. '&guess' would be immutable reference
    */
    io::stdin()
        .read_line(&mut guess) // Returns 'result' type that is an enum of 'Ok' and 'Err'
        .expect("Failed to read line"); // Handles potential failures

    /*
        Rust allows us to shadow the previous value of guess with a new one
        Shadowing lets us reuse the guess variable name rather than forcing us to create two unique variable name
    */
    let guess: u32 = guess.trim().parse().expect("Please type a number!");
    println!("You guessed: {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small!"),
        Ordering::Greater => println!("Too big!"),
        Ordering::Equal => println!("You win!")
    }
}
