use std::io;
use rand::Rng; //using the rand crate added in cargo.toml

fn main() {
    println!("Guess the number!");

    //additions after building project with rand crate
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number is {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read the line");

    println!("You guessed: {guess}");
}
