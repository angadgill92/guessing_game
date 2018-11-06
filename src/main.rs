extern crate rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let _secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Please enter your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
}
