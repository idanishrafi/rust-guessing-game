use rand::{self};
use std::io;
fn main() {
    println!(
        "This Program has Genrated a number between [1-100].\nTry to Guess it (You Get Five Tries) . "
    );

    let number = rand::random_range(1..=100);

    for i in 1..=5 {
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to Read Input");
        let guess = guess
            .trim()
            .parse::<i32>()
            .expect("Failed to Extract the Number");
        if guess == number {
            println!("You Guessed the Number: {}", number);
            return;
        } else if i < 5 && number < guess {
            println!("Try Lower")
        } else if i < 5 && number > guess {
            println!("Try Higher")
        }
    }
    println!("The Number was: {}", number);
}
