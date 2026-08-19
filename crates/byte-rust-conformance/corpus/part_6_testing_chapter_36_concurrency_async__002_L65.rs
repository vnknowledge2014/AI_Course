// filename: src/main.rs

use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    // `move` chuyển ownership vào thread
    let handle = thread::spawn(move || {
        let sum: i32 = data.iter().sum();
        println!("Sum: {}", sum);
        sum
    });

    // data không còn ở main! (moved)
    // println!("{:?}", data); // ❌ error: value moved

    let result = handle.join().unwrap();
    println!("Result: {}", result);
}
