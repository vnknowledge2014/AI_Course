// filename: src/main.rs
fn main() {
    // Shadowing: OK — đổi kiểu từ &str → usize
    let input = "hello";
    let input = input.len();  // Bây giờ là usize = 5
    println!("Length: {}", input);

    // mut: KHÔNG OK — không đổi được kiểu
    // let mut value = "hello";
    // value = 42;  // ❌ error: mismatched types

    // Output: Length: 5
}
