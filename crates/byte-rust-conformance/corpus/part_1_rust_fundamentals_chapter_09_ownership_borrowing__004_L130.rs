// filename: src/main.rs

fn print_greeting(msg: String) {  // msg NHẬN ownership
    println!("📩 {}", msg);
} // ← msg ra khỏi scope → drop!

fn main() {
    let greeting = String::from("Hello, Rust!");
    print_greeting(greeting);         // greeting bị MOVE vào function
    // println!("{}", greeting);      // ❌ greeting đã bị move!

    // Giải pháp 1: Clone (copy data)
    let g2 = String::from("Hello again!");
    print_greeting(g2.clone());       // clone truyền vào
    println!("Still mine: {}", g2);   // ✅ g2 vẫn còn

    // Giải pháp 2 (tốt hơn): Borrowing — xem phần 9.3
}
