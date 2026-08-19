use std::collections::HashMap;

fn word_frequency(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if !clean.is_empty() {
            *counts.entry(clean).or_insert(0) += 1;
        }
    }
    counts
}

fn main() {
    let text = "Rust is great. Rust is fast. Rust is safe!";
    let freq = word_frequency(text);

    let mut sorted: Vec<_> = freq.iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(a.1));  // sắp theo tần suất giảm dần

    for (word, count) in &sorted {
        println!("{}: {}", word, count);
    }
    // Output:
    // rust: 3
    // is: 3
    // great: 1
    // fast: 1
    // safe: 1
}
