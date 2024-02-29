use std::io;
use rand::Rng; //using the rand crate added in cargo.toml
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");

    //additions after building project with rand crate
    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The Secret Number is {secret_number}");
    loop { // to start an infinite loop
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read the line");

        //unsigned 32-bit integer that is both trimmed and parsed, and is expecting an int value
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num, //moves to matching the guess
            Err(_) => continue, //invalidates input since it is not an integer, continues the loop
        };


        println!("You guessed: {guess}");

        //matching the Guess to the Secret Number
        match guess.cmp(&secret_number){
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Big!"),
            Ordering::Equal => {
                println!("You Win!");
                break; //to break an infinite loop
            }
        }
    }
}
