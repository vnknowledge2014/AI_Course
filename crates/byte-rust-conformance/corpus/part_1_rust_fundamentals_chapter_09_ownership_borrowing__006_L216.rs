// filename: src/main.rs

fn calculate_length(s: &String) -> usize {
    s.len()  // chỉ đọc — không cần ownership
} // ← s ra khỏi scope nhưng KHÔNG drop — vì chỉ là reference

fn main() {
    let greeting = String::from("Hello, Rust!");

    // &greeting = cho mượn, greeting vẫn là owner
    let len = calculate_length(&greeting);
    println!("\"{}\" has {} chars", greeting, len);  // ✅ greeting vẫn valid!

    // Nhiều shared borrows cùng lúc — OK!
    let r1 = &greeting;
    let r2 = &greeting;
    let r3 = &greeting;
    println!("{} {} {}", r1, r2, r3);  // ✅

    // Output:
    // "Hello, Rust!" has 12 chars
    // Hello, Rust! Hello, Rust! Hello, Rust!
}
