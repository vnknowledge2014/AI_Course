// filename: src/main.rs
fn main() {
    let sentences = vec![
        "Rust is fast",
        "FP is elegant",
        "DDD is powerful",
    ];

    // map → Vec<Vec<&str>> (nested)
    // flat_map → Vec<&str> (flattened)
    let all_words: Vec<&str> = sentences.iter()
        .flat_map(|s| s.split_whitespace())
        .collect();

    println!("All words: {:?}", all_words);
    // ["Rust", "is", "fast", "FP", "is", "elegant", "DDD", "is", "powerful"]

    // Unique words
    let unique: std::collections::HashSet<&str> = all_words.iter().cloned().collect();
    println!("Unique: {:?}", unique);
}
