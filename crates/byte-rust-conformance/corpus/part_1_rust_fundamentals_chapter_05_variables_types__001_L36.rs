// filename: src/main.rs
fn main() {
    // "Khắc gỗ" — immutable (mặc định)
    let price = 35_000;
    println!("Coffee: {}đ", price);

    // price = 40_000;  // ❌ error[E0384]: cannot assign twice to immutable variable

    // "Bảng phấn" — mutable (phải nói rõ)
    let mut stock = 100;
    println!("Stock: {}", stock);
    stock -= 1;  // ✅ OK — có mut
    println!("After sale: {}", stock);

    // Output:
    // Coffee: 35000đ
    // Stock: 100
    // After sale: 99
}
