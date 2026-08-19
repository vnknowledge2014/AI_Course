// filename: src/main.rs
fn main() {
    // ❌ Mutable: đổi trực tiếp → bản cũ biến mất
    let mut prices = vec![100, 200, 300];
    println!("Before: {:?}", prices);

    prices[1] = 250;  // Giá cũ 200 biến mất hoàn toàn
    println!("After:  {:?}", prices);

    // Nếu muốn undo → không thể!
    // Nếu thread khác đang đọc prices → race condition!

    // Output:
    // Before: [100, 200, 300]
    // After:  [100, 250, 300]
}
