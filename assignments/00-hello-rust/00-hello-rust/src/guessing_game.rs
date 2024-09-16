#![warn(missing_docs)]

//! A simple guessing game.
//! A computer will generate a random number between 1 and 100.
//! At each guess, the computer will report if the number is too high or too low.
//! The player will guess the number until they get it right.

use rand::Rng;
use std::cmp::Ordering;
use std::io;

/// Get the user's input from stdin.
/// @returns i32: the user's guess
/// @error panics if the input is not a valid number
fn get_input() -> i32 {
    println!("Please input your guess");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    match input.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid entry."),
    }
}

/// Runs the guessing game.
/// Reads and outputs from stdin/stdout.
/// See module description for more information on the game rules.
fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    loop {
        let guess = get_input();
        print!("You guessed: {}. ", guess);
        
        // Compare the guess to the secret number
        // Outptut if the guess is too high or too low to stdout
        // Let the user continue guessing until they get it right
        match secret_number.cmp(&guess) {
            Ordering::Equal => {
                println!("That is correct!");
                break;
            }
            Ordering::Greater => println!("You're guess is too low."),
            Ordering::Less => println!("You're guess is too high."),
        }
    }
}
