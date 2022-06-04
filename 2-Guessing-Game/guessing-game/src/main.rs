// Reading input from user. io library
use std::io;

/*
    By default rust has items defined in the standard library that are in scope of every program
    This is called the prelude.
    Things not in the prelude need to be brought into scope using the keyword 'use'
*/

fn main() {
    println!("Guess the number!");
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

    io::stin().read_line(&mut guess).expect("Failed to read line")

    println!("You guessed: {}", guess);
}
