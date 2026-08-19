fn main() {
    let x = 5;
    // x = 10;  // ❌ Lỗi! Trình biên dịch sẽ ngăn cản bạn thay đổi một biến immutable

    let mut y = 5;
    y = 10;      // ✅ OK — vì bạn đã khai báo `mut`, Rust hiểu rằng biến này có thể thay đổi
    println!("y = {}", y);
    // Output: y = 10
}
