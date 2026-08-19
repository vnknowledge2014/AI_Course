// filename: src/main.rs
fn main() {
    let tax_rate = 0.08;  // biến bên ngoài

    // Closure "bắt" (capture) tax_rate từ environment
    let calculate_total = |price: f64| -> f64 {
        price * (1.0 + tax_rate)  // dùng tax_rate — không phải tham số!
    };

    println!("35000đ + tax = {:.0}đ", calculate_total(35_000.0));
    println!("25000đ + tax = {:.0}đ", calculate_total(25_000.0));

    // Output:
    // 35000đ + tax = 37800đ
    // 25000đ + tax = 27000đ
}
