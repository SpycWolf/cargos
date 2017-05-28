extern create rand;

use std::io;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rang().gen_range(1, 101);

    println!("The secret number is: {}, secret_number");

    println!("Please input your geuss.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    println!("Your geussed: {}", guess);
}
