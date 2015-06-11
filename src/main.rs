extern crate rand;

use rand::{thread_rng, Rng};
use std::io;
use std::io::Write;
use std::cmp::Ordering;

fn main() {
    let low  = 1;
    let high = 100;
    let secret = thread_rng().gen_range(low, high + 1);

    println!("There is a secret number between {} and {}", low, high);

    let mut stdout = std::io::stdout();

    loop {
        print!("What do you think the number is? ");
        stdout.flush().ok().expect("Error flushing stdout");

        let mut guess = String::new();
        io::stdin().read_line(&mut guess)
            .ok()
            .expect("That wasn't a string!");

        let guess : i32 = match guess.trim().parse() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match guess.cmp(&secret) {
            Ordering::Less    => println!("Too low!"),
            Ordering::Greater => println!("Too high!"),
            Ordering::Equal   => { println!("You nailed it!"); break },
        }
    }
}
