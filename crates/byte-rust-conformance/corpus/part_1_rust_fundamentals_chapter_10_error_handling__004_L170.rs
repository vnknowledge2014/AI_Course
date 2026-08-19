// filename: src/main.rs
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}

fn main() {
    // map — biến đổi Ok value
    let result = divide(10.0, 3.0).map(|v| format!("{:.2}", v));
    println!("{:?}", result);  // Ok("3.33")

    // unwrap_or — default cho Err
    let safe = divide(10.0, 0.0).unwrap_or(0.0);
    println!("Safe: {}", safe);  // 0.0

    // and_then — chain Result-returning operations
    let chained = divide(100.0, 5.0)
        .and_then(|r| divide(r, 2.0))  // 100/5 = 20, rồi 20/2 = 10
        .map(|v| format!("Final: {:.1}", v));
    println!("{:?}", chained);  // Ok("Final: 10.0")

    // Chain thất bại — dừng ở lỗi đầu tiên
    let failed = divide(100.0, 0.0)
        .and_then(|r| divide(r, 2.0))  // KHÔNG chạy
        .map(|v| format!("Final: {:.1}", v));  // KHÔNG chạy
    println!("{:?}", failed);  // Err("Division by zero")
}
