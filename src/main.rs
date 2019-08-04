use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("OK, let's do this guessing game thing.");
    let secret_num = rand::thread_rng().gen_range(1, 101);
    println!("What is your guess?");
    let mut guess = String::new();
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line.");
    println!("You guessed: {}", guess);
}
