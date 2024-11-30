#![allow(dead_code)]

//! Main binary compiling all the sample modules we are supposed to implement.
//! Runs the fizzbuzz up to 15 and then runs interactive calculator on main

mod calculator;
mod fizz_buzz;
mod guessing_game;
mod traffic_light;
mod university;

fn main() {
    fizz_buzz::print_fizz_buzz(15);
    calculator::main();
}
