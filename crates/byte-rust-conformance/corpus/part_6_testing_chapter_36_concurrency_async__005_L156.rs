// filename: src/main.rs

use std::sync::{Arc, Mutex};
use std::thread;
use std::collections::HashMap;

fn main() {
    let word_counts = Arc::new(Mutex::new(HashMap::<String, u32>::new()));

    let texts = vec![
        "hello world hello",
        "world foo bar",
        "hello bar baz",
        "foo hello world",
    ];

    let mut handles = vec![];

    for text in texts {
        let counts = Arc::clone(&word_counts);
        let text = text.to_string();
        handles.push(thread::spawn(move || {
            for word in text.split_whitespace() {
                let mut map = counts.lock().unwrap();
                *map.entry(word.to_string()).or_insert(0) += 1;
            }
        }));
    }

    for h in handles { h.join().unwrap(); }

    let counts = word_counts.lock().unwrap();
    let mut sorted: Vec<_> = counts.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));
    println!("Word counts:");
    for (word, count) in sorted {
        println!("  {}: {}", word, count);
    }
}
