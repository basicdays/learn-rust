#![allow(dead_code, unused_variables)]
use rand::Rng;
use std::{cmp::Ordering, io};

struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 || value > 100 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess = Guess::new(match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        });

        // or with the `::<>` turbofish syntax style
        // let guess = match guess.trim().parse::<u32>() {
        //     Ok(num) => num,
        //     Err(_) => continue,
        // };

        println!("You guessed: {}", guess.value());

        match guess.value().cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}
