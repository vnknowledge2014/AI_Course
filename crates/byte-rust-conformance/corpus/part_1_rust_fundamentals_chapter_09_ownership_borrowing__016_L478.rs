// filename: src/main.rs
fn create_greeting(name: &str) -> String {
    format!("Hello, {}!", name)  // tạo String mới → return ownership
}

fn main() {
    let greeting = create_greeting("Rust");
    println!("{}", greeting);  // caller sở hữu String
    // Output: Hello, Rust!
}
