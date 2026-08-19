// filename: src/main.rs
fn main() {
    let name: Option<&str> = Some("Rust");
    let empty: Option<&str> = None;

    // unwrap_or — giá trị mặc định nếu None
    println!("{}", name.unwrap_or("Unknown"));   // Rust
    println!("{}", empty.unwrap_or("Unknown"));  // Unknown

    // map — biến đổi giá trị bên trong Some
    let upper = name.map(|n| n.to_uppercase());
    println!("{:?}", upper);  // Some("RUST")
    let upper_empty = empty.map(|n| n.to_uppercase());
    println!("{:?}", upper_empty);  // None — map bỏ qua None

    // and_then (flatMap) — chain operations trả Option
    let parsed: Option<i32> = Some("42").and_then(|s| s.parse().ok());
    println!("Parsed: {:?}", parsed);  // Some(42)
    let failed: Option<i32> = Some("abc").and_then(|s| s.parse().ok());
    println!("Failed: {:?}", failed);  // None

    // filter — giữ Some nếu thỏa điều kiện
    let big = Some(100).filter(|&n| n > 50);
    let small = Some(10).filter(|&n| n > 50);
    println!("big={:?}, small={:?}", big, small);  // Some(100), None

    // is_some / is_none
    println!("name.is_some()={}, empty.is_none()={}", name.is_some(), empty.is_none());
}
