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
