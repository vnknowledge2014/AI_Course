// filename: src/main.rs

// V1: 3 payment methods
#[derive(Debug)]
enum PaymentMethod {
    Cash,
    Card { last_four: String },
    BankTransfer { bank: String, account: String },
    // V2: thêm E-wallet! 👇
    EWallet { provider: String, phone: String },
}

// Compiler SẼ BÁO LỖI ở mọi `match` thiếu EWallet! ⭐
fn process_payment(method: &PaymentMethod, amount: u32) -> Result<String, String> {
    match method {
        PaymentMethod::Cash => {
            Ok(format!("Received {}đ cash", amount))
        }
        PaymentMethod::Card { last_four } => {
            Ok(format!("Charged {}đ to card ****{}", amount, last_four))
        }
        PaymentMethod::BankTransfer { bank, account } => {
            Ok(format!("Transfer {}đ to {} ({})", amount, bank, account))
        }
        // Bỏ dòng dưới → compiler error: "non-exhaustive patterns"
        PaymentMethod::EWallet { provider, phone } => {
            Ok(format!("Charged {}đ via {} ({})", amount, provider, phone))
        }
    }
}

fn receipt_text(method: &PaymentMethod) -> &str {
    match method {
        PaymentMethod::Cash => "Cash payment",
        PaymentMethod::Card { .. } => "Card payment",
        PaymentMethod::BankTransfer { .. } => "Bank transfer",
        PaymentMethod::EWallet { .. } => "E-wallet payment",
    }
}

fn is_instant(method: &PaymentMethod) -> bool {
    match method {
        PaymentMethod::Cash | PaymentMethod::EWallet { .. } => true,
        PaymentMethod::Card { .. } | PaymentMethod::BankTransfer { .. } => false,
    }
}

fn main() {
    let methods = vec![
        PaymentMethod::Cash,
        PaymentMethod::Card { last_four: "4242".into() },
        PaymentMethod::EWallet { provider: "MoMo".into(), phone: "0901234567".into() },
    ];

    for m in &methods {
        println!("{} — {} — instant: {}",
            receipt_text(m),
            process_payment(m, 100_000).unwrap(),
            is_instant(m));
    }
}
