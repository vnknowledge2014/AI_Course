// filename: src/main.rs
fn main() {
    let x = 42;           // Rust đoán: i32
    let y = 3.14;         // Rust đoán: f64
    let active = true;    // Rust đoán: bool
    let name = "Rust";    // Rust đoán: &str

    // Rust đoán từ ngữ cảnh
    let numbers: Vec<i32> = vec![1, 2, 3]; // ghi rõ kiểu cho Vec
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    //                ^  _ = "Rust ơi, tự đoán kiểu element đi"
    println!("{:?}", doubled);  // [2, 4, 6]

    // Khi nào PHẢI ghi rõ kiểu?
    // 1. Rust không đủ thông tin để đoán
    let parsed = "42".parse::<i32>().unwrap();  // turbofish ::<i32>
    // let parsed = "42".parse().unwrap();  // ❌ Đoán không ra parse thành gì

    // 2. Muốn kiểu khác mặc định
    let small: i8 = 42;     // muốn i8 thay vì i32
    let big: u64 = 42;      // muốn u64

    println!("parsed={} small={} big={}", parsed, small, big);
}
