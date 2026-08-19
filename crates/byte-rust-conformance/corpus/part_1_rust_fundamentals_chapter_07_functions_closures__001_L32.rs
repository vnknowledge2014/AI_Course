// filename: src/main.rs

fn classify(score: u32) -> &'static str {
    // Toàn bộ match là expression → trả thẳng, không cần return
    match score {
        90..=100 => "Excellent",
        70..=89  => "Good",
        50..=69  => "Average",
        _        => "Needs improvement",
    }
    // ↑ không có ; → giá trị này được return
}

fn add(a: i32, b: i32) -> i32 {
    a + b   // expression — trả giá trị
    // a + b;  // ← thêm ; → thành statement → trả () → LỖI type mismatch!
}

fn main() {
    println!("{}", classify(85));  // Good
    println!("{}", add(3, 5));     // 8
}
