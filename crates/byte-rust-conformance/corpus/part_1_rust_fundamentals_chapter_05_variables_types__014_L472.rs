// filename: src/main.rs

// const: biết lúc compile, PHẢI ghi kiểu, naming = SCREAMING_SNAKE_CASE
const MAX_ORDERS: u32 = 1_000;
const TAX_RATE: f64 = 0.08;

// static: giống const nhưng có địa chỉ bộ nhớ cố định
// Hiếm dùng — chỉ khi cần reference lâu dài
static APP_NAME: &str = "Cafe System";

fn main() {
    println!("{}: max {} orders, tax {}%",
        APP_NAME, MAX_ORDERS, TAX_RATE * 100.0);

    let price = 35_000;
    let total = price as f64 * (1.0 + TAX_RATE);
    println!("Price: {}đ → Total with tax: {:.0}đ", price, total);

    // Output:
    // Cafe System: max 1000 orders, tax 8%
    // Price: 35000đ → Total with tax: 37800đ
}
