// filename: src/main.rs

// Sự kiện (Luôn dùng Thì Quá Khứ)
#[derive(Debug, Clone)]
enum AccountEvent {
    Opened { id: u64, initial_balance: u64 },
    Deposited { amount: u64 },
    Withdrawn { amount: u64 },
}

fn main() {}
