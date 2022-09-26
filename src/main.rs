extern crate rand;

use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number in 6 times! the number range is 1 to 100!");

    let secret_number = rand::thread_rng().gen_range(1, 101);
    let mut guess_number = 0;

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => print!("Too small!"),
            Ordering::Greater => print!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }

        guess_number += 1;
        println!("  (you left {} times)\n", 6 - guess_number);

        if guess_number == 6 {
            println!("You lose!");
            println!("the answer is {}\n", secret_number);
            break;
        }
    }
}
