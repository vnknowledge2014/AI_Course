fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Cannot divide by zero".to_string())  // Trả về lỗi được bọc trong Err
    } else {
        Ok(a / b)  // Trả về kết quả thành công được bọc trong Ok
    }
}

fn main() {
    match divide(10.0, 3.0) {
        Ok(result) => println!("10 / 3 = {:.2}", result),
        Err(e)     => println!("Error: {}", e),
    }
}
