extern crate rand;

use std::io;
use std::cmp::Ordering;
use rand::Rng;

fn do_guess() -> String {
    println!("please input your guess");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");
    guess
}

fn main() {
    println!("guess the number");

    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("The Secret number is: {}", secret_number);

    loop {
        let guess: u32 = match do_guess().trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("you guessed {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too Small!"),
            Ordering::Greater => println!("Too Large!"),
            Ordering::Equal => { 
                println!("You Win!");
                break;
            }
        }
    }
}
