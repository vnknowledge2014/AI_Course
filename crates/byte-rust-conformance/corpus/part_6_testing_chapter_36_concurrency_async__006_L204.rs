// filename: src/main.rs

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    // Producer 1
    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 1..=3 {
            tx1.send(format!("[P1] Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Producer 2
    let tx2 = tx.clone();
    thread::spawn(move || {
        for i in 1..=3 {
            tx2.send(format!("[P2] Message {}", i)).unwrap();
            thread::sleep(Duration::from_millis(150));
        }
    });

    drop(tx); // Drop original sender so rx knows when all done

    // Consumer
    for received in rx {
        println!("Got: {}", received);
    }
    println!("All producers done!");
}
