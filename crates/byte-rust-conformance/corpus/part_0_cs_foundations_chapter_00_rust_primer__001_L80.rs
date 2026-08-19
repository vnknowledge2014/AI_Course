fn main() {
    let x = 5;           // Rust rất thông minh, nó tự suy luận x là kiểu i32 (số nguyên 32-bit)
    let y: f64 = 3.14;   // Bạn cũng có thể ghi rõ kiểu nếu muốn (ở đây là số thực 64-bit)
    let name = "Rust";    // &str — một "lát cắt" chuỗi (string slice)
    let active = true;    // kiểu boolean (đúng/sai)

    println!("x = {}, y = {}, name = {}, active = {}", x, y, name, active);
    // Output: x = 5, y = 3.14, name = Rust, active = true
}
