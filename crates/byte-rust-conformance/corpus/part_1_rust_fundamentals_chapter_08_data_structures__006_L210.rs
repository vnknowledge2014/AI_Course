// filename: src/main.rs

// ✅ Nhận &str — chấp nhận cả String lẫn &str
fn greet(name: &str) -> String {
    format!("Hello, {}! 👋", name)  // trả String mới
}

fn main() {
    // Truyền &str
    println!("{}", greet("Rust"));

    // Truyền String (auto-deref thành &str)
    let name = String::from("World");
    println!("{}", greet(&name));

    // Output:
    // Hello, Rust! 👋
    // Hello, World! 👋
}
