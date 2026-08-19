// filename: src/main.rs
fn main() {
    let mut data = vec![1, 2, 3];

    let first = &data[0];  // shared borrow bắt đầu
    println!("First: {}", first);  // dùng shared borrow LẦN CUỐI ở đây

    // Compiler biết: first không dùng nữa sau dòng trên
    // → shared borrow KẾT THÚC (NLL)
    // → mutable borrow OK!
    data.push(4);  // ✅ mutable borrow — OK vì first đã "hết hạn"
    println!("Data: {:?}", data);
    // Output:
    // First: 1
    // Data: [1, 2, 3, 4]
}
