use rand::RngExt;
use std::io;

fn main() {
    let correct_number = rand::rng().random_range(1..=10);
    println!("The correct number is {}", correct_number);
    println!("Hey, Guess a number?");
    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    if correct_number == guess.trim().parse::<i32>().unwrap() {
        println!("You guessed correctly!");
    } else {
        println!("You guessed incorrectly!");
    }

    println!("The correct number was: {} but you guessed: {}", correct_number, guess.trim());

}
