// filename: src/main.rs
use std::fmt;

// Tương đương print_description<T: Display>(item: &T)
fn print_item(item: &impl fmt::Display) {
    println!("Item: {}", item);
}

// Return impl Trait — ẩn concrete type
fn make_greeting(name: &str) -> impl fmt::Display {
    format!("Hello, {}! 👋", name)
}

fn main() {
    print_item(&42);
    print_item(&"hello");
    print_item(&3.14);

    let greeting = make_greeting("Rust");
    println!("{}", greeting);
}
