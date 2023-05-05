use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    // println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess, ser:");

        let mut guess = String::new();

        io::stdin()      // std::io::stdin if we don't import via 'use'
            .read_line(&mut guess)
            .expect("Failed to read the line, ser");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        }; // convert string integer type

        println!("you guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            },
        }
    }
}

// like variables, references are immutable by default,
// hence you need to write &mut guess rather than &guess to make it mutable

// cargo doc --open