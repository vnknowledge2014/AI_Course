// filename: src/main.rs

use std::thread;

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let chunk_size = 5;

    // Split work across threads
    let mut handles = vec![];

    for chunk in data.chunks(chunk_size) {
        let chunk = chunk.to_vec(); // clone cho mỗi thread
        handles.push(thread::spawn(move || {
            let sum: i32 = chunk.iter().sum();
            println!("  Chunk {:?} → sum={}", chunk, sum);
            sum
        }));
    }

    // Collect results
    let total: i32 = handles.into_iter()
        .map(|h| h.join().unwrap())
        .sum();

    println!("Total: {}", total); // 55
}
