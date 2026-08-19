// filename: src/main.rs

// Sum type: MỘT trong các variants (OR)
// States = variant1 + variant2 + variant3
#[derive(Debug)]
enum PaymentMethod {
    Cash,                           // 1 state
    Card { number: String, cvv: String }, // nhiều states
    Transfer { bank: String, account: String },
    Wallet(String),                 // 1 field
}

fn process_payment(method: &PaymentMethod, amount: u32) -> String {
    match method {
        PaymentMethod::Cash =>
            format!("💵 Cash: {}đ", amount),
        PaymentMethod::Card { number, .. } => {
            let last_four = &number[number.len()-4..];
            format!("💳 Card ****{}: {}đ", last_four, amount)
        }
        PaymentMethod::Transfer { bank, account } =>
            format!("🏦 {} → {}: {}đ", bank, account, amount),
        PaymentMethod::Wallet(provider) =>
            format!("📱 {}: {}đ", provider, amount),
    }
}

fn main() {
    let methods = vec![
        PaymentMethod::Cash,
        PaymentMethod::Card {
            number: "4111222233334444".into(),
            cvv: "123".into(),
        },
        PaymentMethod::Transfer {
            bank: "VCB".into(),
            account: "001234567".into(),
        },
        PaymentMethod::Wallet("MoMo".into()),
    ];

    for method in &methods {
        println!("{}", process_payment(method, 100_000));
    }
    // 💵 Cash: 100000đ
    // 💳 Card ****4444: 100000đ
    // 🏦 VCB → 001234567: 100000đ
    // 📱 MoMo: 100000đ
}
