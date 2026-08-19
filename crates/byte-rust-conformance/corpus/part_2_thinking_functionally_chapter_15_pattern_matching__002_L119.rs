// filename: src/main.rs

#[derive(Debug)]
enum Currency { VND, USD, EUR }

#[derive(Debug)]
enum Payment {
    Cash(u64, Currency),
    Card { amount: u64, currency: Currency, last_four: String },
    Crypto { amount: f64, token: String },
}

#[derive(Debug)]
enum OrderResult {
    Success(Payment),
    Failed { reason: String, attempted: Payment },
    Cancelled,
}

fn describe(result: &OrderResult) -> String {
    match result {
        // Nested: tách OrderResult → Payment → Currency
        OrderResult::Success(Payment::Cash(amount, Currency::VND)) =>
            format!("✅ Cash {}đ", amount),

        OrderResult::Success(Payment::Cash(amount, currency)) =>
            format!("✅ Cash {:?} {}", currency, amount),

        OrderResult::Success(Payment::Card { amount, last_four, .. }) =>
            format!("✅ Card ****{}: {}đ", last_four, amount),

        OrderResult::Success(Payment::Crypto { token, amount }) =>
            format!("✅ {} {:.4}", token, amount),

        OrderResult::Failed { reason, attempted } =>
            format!("❌ Failed: {} (tried {:?})", reason, attempted),

        OrderResult::Cancelled =>
            "🚫 Cancelled".to_string(),
    }
}

fn main() {
    let results = vec![
        OrderResult::Success(Payment::Cash(500_000, Currency::VND)),
        OrderResult::Success(Payment::Card {
            amount: 1_200_000,
            currency: Currency::VND,
            last_four: "4444".into(),
        }),
        OrderResult::Success(Payment::Crypto { amount: 0.0025, token: "BTC".into() }),
        OrderResult::Failed {
            reason: "Insufficient funds".into(),
            attempted: Payment::Cash(999_999, Currency::USD),
        },
        OrderResult::Cancelled,
    ];

    for r in &results {
        println!("{}", describe(r));
    }
}
