// filename: src/main.rs

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Arc = Atomic Reference Count (shared ownership across threads)
    // Mutex = Mutual Exclusion (1 thread access lúc 1 thời điểm)
    let counter = Arc::new(Mutex::new(0));

    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        handles.push(thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
            // Mutex unlock tự động khi `num` ra khỏi scope
        }));
    }

    for h in handles { h.join().unwrap(); }

    println!("Counter: {}", *counter.lock().unwrap()); // 10
}
