// filename: src/main.rs
fn main() {
    // Option<T> = Some(T) hoặc None
    let some_number: Option<i32> = Some(42);
    let no_number: Option<i32> = None;

    // KHÔNG thể dùng trực tiếp — phải unwrap
    // let x: i32 = some_number;  // ❌ mismatched types
    // let y: i32 = some_number + 1;  // ❌ cannot add

    // Phải xử lý CẢ HAI trường hợp
    match some_number {
        Some(n) => println!("Got: {}", n),
        None => println!("Nothing!"),
    }

    match no_number {
        Some(n) => println!("Got: {}", n),
        None => println!("Nothing!"),
    }

    // Output:
    // Got: 42
    // Nothing!
}
