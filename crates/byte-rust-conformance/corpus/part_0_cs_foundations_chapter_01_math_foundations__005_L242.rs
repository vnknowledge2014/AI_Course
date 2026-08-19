// filename: src/main.rs

// Lời hứa 1: "Cho số nguyên → trả số nguyên. Luôn luôn."
// Đơn giản, rõ ràng, 100% đáng tin.
fn double(x: i32) -> i32 {
    x * 2
}

// Lời hứa 2: "Cho chuỗi → CÓ THỂ trả số, HOẶC không"
// Option = "có thể có, có thể không" — trung thực!
fn parse_number(s: &str) -> Option<i32> {
    s.parse::<i32>().ok()
}

// Lời hứa 3: "Cho hai số → trả kết quả chia HOẶC lỗi"
// Result = "thành công hoặc thất bại — tôi nói rõ cả hai"
fn safe_divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero!".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // Lời hứa 1: luôn luôn trả số
    println!("double(5) = {}", double(5));

    // Lời hứa 2: có thể thành công, có thể không
    // Compiler BẮT BUỘC bạn xử lý CẢ HAI trường hợp
    match parse_number("42") {
        Some(n) => println!("parse_number(\"42\") = {}", n),
        None    => println!("Không parse được!"),
    }
    match parse_number("abc") {
        Some(n) => println!("parse_number(\"abc\") = {}", n),
        None    => println!("\"abc\" không phải số!"),
    }

    // Lời hứa 3: thành công hoặc lỗi — phải xử lý cả hai
    match safe_divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e) => println!("Lỗi: {}", e),
    }
    match safe_divide(10.0, 0.0) {
        Ok(result) => println!("10 / 0 = {:.2}", result),
        Err(e) => println!("Lỗi: {}", e),
    }

    // Output:
    // double(5) = 10
    // parse_number("42") = 42
    // "abc" không phải số!
    // 10 / 3 = 3.33
    // Lỗi: Cannot divide by zero!
}
