use std::io; // import io library

fn main() {
    println!("Guess the number!");
    println!("Please input your guess.");
    let mut guess = String::new();// mut is for the mutability of a variable
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");
    println!("You guessed: {}", guess)
}
