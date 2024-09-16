// Prints the classic fizzbuzz sequence to stdout.
//
// @input max_num: u32 - the maximum number to print fizzbuzz up to
//
// Prints "Fizz" for numbers divisible by 3
// Prints "Buzz" for numbers divisible by 5,
// Prints "FizzBuzz" for numbers divisible by both.
// Prints the number itself if it is not divisible by 3 or 5.
pub fn print_fizz_buzz(max_num: u32) {
    for i in 0..max_num {
        match (i % 3 == 0, i % 5 == 0) {
            (false, false) => println!("{i}"),
            (true, false) => println!("Fizz"),
            (false, true) => println!("Buzz"),
            (true, true) => println!("FizBuzz"),
        };
    }
}
