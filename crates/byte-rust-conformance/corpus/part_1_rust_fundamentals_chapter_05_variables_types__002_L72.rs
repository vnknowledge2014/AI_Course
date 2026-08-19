// filename: src/main.rs
fn main() {
    let price = "35000";             // &str
    println!("String: {}", price);

    let price = price.parse::<u32>().unwrap();  // Bây giờ là u32!
    println!("Number: {}", price);

    let price = price + 5_000;       // Tính toán với u32
    println!("After markup: {}đ", price);

    // Output:
    // String: 35000
    // Number: 35000
    // After markup: 40000đ
}
