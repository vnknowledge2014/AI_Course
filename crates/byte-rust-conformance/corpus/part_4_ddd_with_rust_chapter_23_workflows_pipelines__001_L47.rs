// filename: src/main.rs

// Trạm 1: Chuyển chữ thành số
fn parse_amount(input: &str) -> Result<u32, String> {
    input.trim().parse::<u32>()
        .map_err(|_| format!("Invalid amount: '{}'", input))
}

// Trạm 2: Kiểm duyệt số tiền
fn validate_amount(amount: u32) -> Result<u32, String> {
    if amount == 0 { Err("Amount must be > 0".into()) }
    else if amount > 10_000_000 { Err(format!("Amount {} exceeds limit", amount)) }
    else { Ok(amount) }
}

// Trạm 3: Tính thuế (Hàm này không bao giờ lỗi, nên trả về thẳng u32)
fn apply_tax(amount: u32) -> u32 {
    amount + amount * 8 / 100
}

// Trạm 4: In biên lai
fn format_receipt(amount: u32) -> String {
    format!("═══ RECEIPT ═══\n  Total: {}đ\n  Tax included\n═══════════════", amount)
}

fn main() {}
