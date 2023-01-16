use std::{io, cmp::Ordering};

use rand::Rng;

fn main() {

    println!("Guess the number!");

    let secrete_number = rand::thread_rng()
        .gen_range(1..=100);

    // println!("Secrete number: {}", secrete_number);

    loop {
        println!("Please input your guess.");
        
        let mut guess = String::new();    
    
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
    
        // let guess = guess.trim().parse::<i32>().expect("Please type a number");
        // let guess: i32 = guess.trim().parse().expect("Please type a number!");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        println!("You guessed: {guess}");
        
        match guess.cmp(&secrete_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
