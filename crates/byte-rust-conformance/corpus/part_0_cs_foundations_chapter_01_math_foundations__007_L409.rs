// filename: src/main.rs

// Sum type: enum = chọn MỘT = phép CỘNG
#[derive(Debug)]
enum Payment {
    Cash(u64),                        // HOẶC
    Card { number: String },          // HOẶC
    EWallet { provider: String },
}

fn describe_payment(payment: &Payment) {
    match payment {
        Payment::Cash(amount) => {
            println!("💵 Cash: {}đ", amount);
        }
        Payment::Card { number } => {
            // Chỉ hiện 4 số cuối
            println!("💳 Card: ****{}", &number[number.len()-4..]);
        }
        Payment::EWallet { provider } => {
            println!("📱 E-Wallet: {}", provider);
        }
    }
}

fn main() {
    describe_payment(&Payment::Cash(50_000));
    describe_payment(&Payment::Card { number: "4111111111111234".to_string() });
    describe_payment(&Payment::EWallet { provider: "MoMo".to_string() });
    // Output:
    // 💵 Cash: 50000đ
    // 💳 Card: ****1234
    // 📱 E-Wallet: MoMo
}
