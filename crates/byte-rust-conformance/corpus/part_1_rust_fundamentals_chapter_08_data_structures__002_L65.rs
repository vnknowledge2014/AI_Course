// filename: src/main.rs
fn main() {
    // Length = số phần tử hiện tại
    // Capacity = kích thước bộ nhớ đã cấp (>= length)
    let mut v = Vec::with_capacity(10);  // pre-allocate 10 slots
    println!("len={}, capacity={}", v.len(), v.capacity());
    // len=0, capacity=10

    for i in 0..10 {
        v.push(i);
    }
    println!("len={}, capacity={}", v.len(), v.capacity());
    // len=10, capacity=10 — vừa khít, không resize

    v.push(10);  // phải resize!
    println!("len={}, capacity={}", v.len(), v.capacity());
    // len=11, capacity=20 — doubled

    // 💡 Nếu biết trước số lượng, dùng with_capacity → tránh resize
}
