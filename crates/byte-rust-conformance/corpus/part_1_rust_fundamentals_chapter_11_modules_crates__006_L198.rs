// filename: src/payment.rs

pub fn format_receipt(total: u32) -> String {
    let tax = (total as f64 * 0.08) as u32;
    format!("🧾 Subtotal: {}đ\n   Tax (8%): {}đ\n   Total: {}đ",
        total, tax, total + tax)
}

fn main() {}
