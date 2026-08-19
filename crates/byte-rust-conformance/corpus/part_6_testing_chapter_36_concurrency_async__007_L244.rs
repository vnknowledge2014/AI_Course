// filename: src/main.rs

use std::sync::mpsc;
use std::thread;

fn main() {
    // Stage 1: Generate → Stage 2: Process → Stage 3: Collect
    let (tx1, rx1) = mpsc::channel::<i32>();
    let (tx2, rx2) = mpsc::channel::<String>();

    // Stage 1: Generate numbers
    thread::spawn(move || {
        for i in 1..=10 { tx1.send(i).unwrap(); }
    });

    // Stage 2: double + format
    thread::spawn(move || {
        for n in rx1 {
            tx2.send(format!("{} → {}", n, n * 2)).unwrap();
        }
    });

    // Stage 3: Collect
    let results: Vec<String> = rx2.into_iter().collect();
    println!("Pipeline results:");
    for r in &results { println!("  {}", r); }
}
