extern crate rand;

use std::io;
use rand::Rng;
use std::cmp::Ordering;
use std::result::Result::Err;

fn main() {
    println!("Guess the number!");
    playing_with_memory();
    let secret_number = random_number();

    loop {
        println!("Please input your guess!");

        let mut guess = String::new();

        io::stdin().read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("You guessed {}", guess);

        let is_guessed = is_guessed(guess, secret_number);

        println!("{}", is_guessed.1);
        if is_guessed.0 {
            break
        }
    }
}

fn random_number() -> u32 {
    rand::thread_rng().gen_range(1, 101)
}

fn is_guessed(user_number: u32, generated_number: u32) -> (bool, String) {
    match user_number.cmp(&generated_number) {
        Ordering::Less => (false, "Too small!".to_string()),
        Ordering::Greater => (false, "Too big!".to_string()),
        Ordering::Equal => (true, "You win".to_string())
    }
}

fn go_though_array() {
    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn playing_with_memory() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    
    println!("s1 {}, s2 {}", s1, s1)
}