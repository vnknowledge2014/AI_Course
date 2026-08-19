// filename: src/main.rs
fn main() {
    let transactions = vec![100, -30, 50, -20, 80];

    // Running balance — scan giữ state qua iterations
    let balances: Vec<i32> = transactions.iter()
        .scan(0_i32, |balance, &tx| {
            *balance += tx;
            Some(*balance)
        })
        .collect();

    println!("Transactions: {:?}", transactions);
    println!("Balances:     {:?}", balances);
    // Transactions: [100, -30, 50, -20, 80]
    // Balances:     [100, 70, 120, 100, 180]
}
