// filename: src/main.rs
fn main() {
    let maybe_name: Option<&str> = Some("Minh");

    // Cách dài: match
    match maybe_name {
        Some(name) => println!("Hello, {}!", name),
        None => {} // không làm gì
    }

    // Cách gọn: if let — chỉ xử lý Some, bỏ qua None
    if let Some(name) = maybe_name {
        println!("Welcome, {}!", name);
    }

    // if let với else
    let config: Option<u32> = None;
    let port = if let Some(p) = config {
        p
    } else {
        8080 // giá trị mặc định
    };
    println!("Port: {}", port);

    // Output:
    // Hello, Minh!
    // Welcome, Minh!
    // Port: 8080
}
