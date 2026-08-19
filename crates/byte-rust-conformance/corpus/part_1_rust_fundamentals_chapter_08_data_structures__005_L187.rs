// filename: src/main.rs
fn main() {
    // &str — string literal, read-only, sống trong binary
    let greeting: &str = "Hello";

    // String — owned, heap-allocated, mutable
    let mut message = String::from("Hello");
    message.push_str(", Rust!");
    message.push('🦀');
    println!("{}", message);  // Hello, Rust!🦀

    // Chuyển đổi
    let s: String = greeting.to_string();   // &str → String (clone data)
    let r: &str = &s;                        // String → &str (cheap, borrow)
    let also_r: &str = s.as_str();           // tương đương

    println!("s={} r={} also_r={}", s, r, also_r);
}
