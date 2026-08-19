// filename: src/main.rs

fn parse_int(s: &str) -> Option<i32> {
    s.parse().ok()
}

fn main() {
    let input: Option<&str> = Some("42");

    // Dùng .map() → KẾT QUẢ BỊ NESTED!
    let nested: Option<Option<i32>> = input.map(|s| parse_int(s));
    println!("Nested: {:?}", nested);  // Some(Some(42)) ← 2 lớp Option!

    // Muốn: Option<i32>, không phải Option<Option<i32>>
    // Giải pháp: .and_then() = map + flatten
    let flat: Option<i32> = input.and_then(|s| parse_int(s));
    println!("Flat: {:?}", flat);  // Some(42) ← 1 lớp!
}
