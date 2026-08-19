// filename: src/main.rs

#[derive(Debug)]
struct RawInput { customer: String, coupon: Option<String>, items: u32 }

#[derive(Debug)]
struct FulfilledOrder { customer: String, subtotal: u32, total: u32 }

// Trạm 1
fn validate_input(input: RawInput) -> Result<RawInput, String> {
    if input.customer.trim().len() < 2 { Err("Name required".into()) }
    else { Ok(input) }
}

// Trạm 2 (Vừa trả về Giỏ hàng, vừa trả thêm Phần trăm Giảm giá)
fn apply_coupon(input: RawInput) -> Result<(RawInput, u32), String> {
    let discount = match input.coupon.as_deref() {
        Some("SAVE10") => 10,
        Some(code) => return Err(format!("Unknown coupon: {}", code)),
        None => 0,
    };
    Ok((input, discount))
}

// Trạm 3 (Nhận dữ liệu gộp từ Trạm 2)
fn calculate_totals(input: RawInput, discount_pct: u32) -> FulfilledOrder {
    let subtotal = input.items * 50_000;
    let total = subtotal * (100 - discount_pct) / 100;

    FulfilledOrder { customer: input.customer, subtotal, total }
}

// GHÉP NỐI TOÀN BỘ ĐƯỜNG ỐNG DÂY CHUYỀN
fn process_order(input: RawInput) -> Result<FulfilledOrder, String> {
    validate_input(input)
        .and_then(apply_coupon)
        .map(|(input, discount)| calculate_totals(input, discount)) // Tách tuple ra xử lý
}

fn main() {
    let order = RawInput { customer: "Minh".into(), coupon: Some("SAVE10".into()), items: 4 };

    match process_order(order) {
        Ok(fulfilled) => println!("✅ Thành công! Phải trả: {}đ", fulfilled.total),
        Err(e) => println!("❌ {}", e),
    }
}
