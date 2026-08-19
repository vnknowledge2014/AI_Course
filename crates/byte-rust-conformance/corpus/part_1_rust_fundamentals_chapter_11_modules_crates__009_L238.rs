// filename: src/payment/card.rs
pub fn process(amount: u32, card_number: &str) -> String {
    let last_four = &card_number[card_number.len()-4..];
    format!("💳 Card ****{}: {}đ", last_four, amount)
}

fn main() {}
