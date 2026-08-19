// filename: src/main.rs
fn main() {
    {
        let name = String::from("Rust");
        println!("Inside: {}", name);
    } // ← name ra khỏi scope → Rust tự gọi drop() → giải phóng heap memory
    // println!("{}", name);  // ❌ error: not found in this scope

    // Giống như: trong C bạn phải gọi free(name) ở đây
    // Rust làm tự động — KHÔNG BAO GIỜ quên!
    println!("name đã bị drop");
}
