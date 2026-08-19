// filename: src/main.rs
fn main() {
    let s = "  Hello, Rust Programming! ";

    // Kiểm tra
    println!("Length (bytes): {}", s.len());       // 28
    println!("Starts with '  H': {}", s.starts_with("  H"));
    println!("Contains 'Rust': {}", s.contains("Rust"));

    // Biến đổi (trả String mới)
    println!("Trimmed: '{}'", s.trim());           // 'Hello, Rust Programming!'
    println!("Upper: {}", s.to_uppercase());
    println!("Replace: {}", s.replace("Rust", "FP"));

    // Split
    let words: Vec<&str> = s.trim().split_whitespace().collect();
    println!("Words: {:?}", words);  // ["Hello,", "Rust", "Programming!"]

    let parts: Vec<&str> = "a,b,c,d".split(',').collect();
    println!("CSV: {:?}", parts);  // ["a", "b", "c", "d"]

    // Join
    let joined = words.join(" → ");
    println!("Joined: {}", joined);  // Hello, → Rust → Programming!

    // ⚠️ String là UTF-8! Indexing theo BYTE, không phải character
    let vietnamese = "Việt Nam";
    println!("Bytes: {}", vietnamese.len());  // 10 (ệ = 3 bytes)
    println!("Chars: {}", vietnamese.chars().count());  // 8
    // vietnamese[0]  // ❌ Không thể index String!
    // Phải dùng .chars().nth(0)
    println!("First char: {:?}", vietnamese.chars().next());  // Some('V')
}
