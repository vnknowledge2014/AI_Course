// filename: src/main.rs
fn main() {
    // ✅ Immutable: tạo bản mới, giữ bản cũ
    let prices_v1 = vec![100, 200, 300];

    // Tạo bản mới với giá thay đổi
    let mut prices_v2 = prices_v1.clone();
    prices_v2[1] = 250;

    // Cả hai versions cùng tồn tại!
    println!("V1: {:?}", prices_v1);  // bản gốc
    println!("V2: {:?}", prices_v2);  // bản mới

    // Undo? Dùng lại v1!
    // Thread-safe? Không ai sửa v1 → an toàn!

    // Output:
    // V1: [100, 200, 300]
    // V2: [100, 250, 300]
}
