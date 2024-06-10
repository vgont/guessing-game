use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    println!("Guessing Game!");

    let secret_number = rand::thread_rng().gen_range(1..=100); //generates a random u32 between 1 and 100

    loop {
        let mut guess = String::new();
        println!("Input your guess.");

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input: {}", guess);
                continue; //-> restarts the loop
            }
        };

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        };
    }
}
