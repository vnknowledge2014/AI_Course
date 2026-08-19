// filename: src/main.rs
use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];  // immutable

    // Nhiều threads đọc cùng lúc — an toàn!
    // Không ai sửa → không data race → không cần lock
    let handles: Vec<_> = (0..3).map(|i| {
        let data = data.clone();
        thread::spawn(move || {
            let sum: i32 = data.iter().sum();
            println!("Thread {}: sum = {}", i, sum);
        })
    }).collect();

    for h in handles {
        h.join().unwrap();
    }
}
