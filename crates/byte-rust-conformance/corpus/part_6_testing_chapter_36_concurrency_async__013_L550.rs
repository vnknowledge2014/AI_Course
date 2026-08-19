use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel();
    let ranges = vec![1..=10, 11..=20, 21..=30];

    for range in ranges {
        let tx = tx.clone();
        thread::spawn(move || {
            for n in range { tx.send(n).unwrap(); }
        });
    }
    drop(tx);

    let sum: i32 = rx.into_iter().sum();
    println!("Sum: {}", sum); // 465
}
