// filename: src/main.rs
fn main() {
    // Mutable: mất history
    let mut balance = 1_000_000;
    balance -= 200_000;  // mua gì đó — history biến mất
    balance += 50_000;   // nhận lương — không biết balance cũ

    // Immutable: giữ tất cả versions
    let history = vec![
        1_000_000,                    // initial
        1_000_000 - 200_000,          // purchase
        1_000_000 - 200_000 + 50_000, // salary
    ];

    println!("Current: {}", history.last().unwrap());
    println!("History: {:?}", history);
    // Undo? Dùng history[history.len() - 2]
    println!("Undo: {}", history[history.len() - 2]);
}
