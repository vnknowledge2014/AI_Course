// filename: src/main.rs

// ❌ Mutable flags — khó track, dễ quên
// let mut is_paid = false;
// let mut is_shipped = false;
// let mut is_cancelled = false;

// ✅ Enum — mỗi state rõ ràng, immutable transition
#[derive(Debug, Clone)]
enum OrderStatus {
    Pending,
    Paid { amount: u32 },
    Shipped { tracking: String },
    Delivered,
    Cancelled { reason: String },
}

// Pure function: transition trả state MỚI
fn process_payment(status: &OrderStatus, amount: u32) -> Result<OrderStatus, String> {
    match status {
        OrderStatus::Pending => Ok(OrderStatus::Paid { amount }),
        _ => Err(format!("Cannot pay order in {:?} state", status)),
    }
}

fn ship_order(status: &OrderStatus, tracking: &str) -> Result<OrderStatus, String> {
    match status {
        OrderStatus::Paid { .. } => Ok(OrderStatus::Shipped {
            tracking: tracking.to_string(),
        }),
        _ => Err(format!("Cannot ship order in {:?} state", status)),
    }
}

fn main() {
    let status = OrderStatus::Pending;
    println!("1. {:?}", status);

    let status = process_payment(&status, 50_000).unwrap();
    println!("2. {:?}", status);

    let status = ship_order(&status, "VN123456").unwrap();
    println!("3. {:?}", status);

    // Thử ship lại → error (đúng!)
    let error = ship_order(&status, "XX").unwrap_err();
    println!("4. Error: {}", error);

    // Output:
    // 1. Pending
    // 2. Paid { amount: 50000 }
    // 3. Shipped { tracking: "VN123456" }
    // 4. Error: Cannot ship order in Shipped { tracking: "VN123456" } state
}
