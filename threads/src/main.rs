use std::thread;
use std::time::Duration;
use std::sync::mpsc;

fn main() {
    println!("Hello, world!");
    let x = 1;

    // multiple producer singl consumer
    let (tx, rx) = mpsc::channel();
    
    // we create one more producer
    let tx1 = mpsc::Sender::clone(&tx);
    
    // these are 1:1 threads
    // move keyword signals to thread that we are moving
    // variable into the scope
    let t = thread::spawn(move || {
        for i in 1..10 {
            println!("sleeeeeping {}", x + i);

            // sending value to channel with unwrap failing with panic if it fails
            tx.send(i).unwrap();
            thread::sleep(Duration::from_secs(1))
        };
    });

    let t2 = thread::spawn(move || {
        for i in 1..10 {
            // sending value to channel with unwrap failing with panic if it fails
            tx1.send(i * 100).unwrap();
            thread::sleep(Duration::from_secs(1))
        };
    });

    

    let r = thread::spawn(move || {
        for received in rx {
            println!("Got: {}", received);
        }
    });

    t.join().unwrap();

}
