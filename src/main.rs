use rand::Rng;
use rand::seq::SliceRandom;
use std::cmp::Ordering;
use std::io;

fn main() {
    // Print a "welcome" message
    println!("OK, let's do this guessing game thing.");

    // Generate a random num between 1 and 101 
    // (first inclusive, second exclusive)
    let secret_num = rand::thread_rng().gen_range(1, 101);
    // println!("The nuber I've generated is {}", secret_num);

    // Set the number of guesses
    let guesses = 5;
    for i in 1..guesses {

        // Let's play
        println!("What is your guess?");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line.");
        // Shadow `guess` as an unsigned 32bit int
        let guess: u32 = guess.trim().parse()
            .expect("Please type a number so we can get on with this.");
        
        // println!("You guessed: {}", guess);

        match guess.cmp(&secret_num) {
            Ordering::Less => println!("Too small..."),
            Ordering::Greater => println!("Too big..."),
            Ordering::Equal => {
                println!("You can guess a random number!\nCongrats, I suppose.");
                break;
            }
        }
        // Return an insult and the number of guesses left
        println!("{}", insult());
        println!("You have {} guesses left.", guesses - i);
    }
}

// Pick a random insult and return it
fn insult() -> String {
    let insults = [
        "Did the sun get in your eyes?",
        "Somethingsomething your momma.",
        "Wow. Can't even guess a number.",
        "How many guesses do you have left? Not looking good.",
        "Why is this so hard for you?"
    ];

    insults.choose(&mut rand::thread_rng()).unwrap().to_string()
}
