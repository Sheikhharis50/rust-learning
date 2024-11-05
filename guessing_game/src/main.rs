use rand::Rng;
use std::cmp::Ordering;
use std::io::{self, Write};

fn main() {
    println!("Guess the number!");

    // generating random integer.
    let secret_number: u32 = rand::thread_rng().gen_range(1..=100);
    // println!("The secret number is: {secret_number}");

    loop {
        // taking input from user.
        let mut guess: String = String::new();
        print!("Please input your guess: ");
        // after using print we need to flush 
        // newline.
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        // parsing input into integer
        // or else retry.
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        // comparing given integer with random generated
        // number and print messages accordingly.
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
