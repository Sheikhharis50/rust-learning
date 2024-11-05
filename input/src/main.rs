use std::io::{self, Write};

fn input_number() -> u32 {
    let mut string: String = String::new();

    loop {
        print!("Please input number: ");
        let _ = io::stdout().flush();

        io::stdin()
            .read_line(&mut string)
            .expect("Failed to read line!");

        let number: u32 = match string.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number!");
                continue;
            }
        };

        return number;
    }
}

fn main() {
    let number = input_number();
    println!("Number: {}", number);
}
