use std::fs;
use std::env;
use std::thread;
use std::time::Duration;
// Constants can be declared in any scope, 
//including the global scope, which makes them useful for values that many parts of code need to know about.
const GOGA_JE_CAR:u32 = 1;
fn main() {
    // variables are immutable by default
    let mut x = 5;
    x = GOGA_JE_CAR;
    //variables can be shadowed, and also we can change the type
    //and keep the name
    let x = 2;
    let guess: u32 = "42".parse().expect("not a number baby!");
    let heart_eyed_cat = '😻';
    println!("Hello, world! {}", heart_eyed_cat);

    let contents = fs::read_to_string("variables.iml").expect("Something went wrong reading the file");


    println!("With text:\n{}", contents);

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {}", i);
            thread::sleep(Duration::from_millis(1000))
        }
    });

    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {}", i + 100);
            thread::sleep(Duration::from_millis(1000))
        }
    });


}
