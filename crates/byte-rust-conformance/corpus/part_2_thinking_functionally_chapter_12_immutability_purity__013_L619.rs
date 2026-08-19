// filename: src/main.rs
fn main() {
    let transactions = vec![100, -50, 200, -30, 150];

    // ❌ Mutable accumulator
    // let mut balance = 0;
    // for &t in &transactions { balance += t; }

    // ✅ Fold — no mutation
    let balance: i32 = transactions.iter().sum();
    println!("Balance: {}", balance);  // 370

    // Running balance (scan)
    let running: Vec<i32> = transactions.iter()
        .scan(0, |acc, &x| {
            *acc += x;
            Some(*acc)
        })
        .collect();
    println!("Running: {:?}", running);  // [100, 50, 250, 220, 370]
}
