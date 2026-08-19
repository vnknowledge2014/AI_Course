// filename: src/main.rs

// ❌ BAD: Dùng booleans — nhiều trạng thái vô nghĩa
// struct Order {
//     is_paid: bool,
//     is_shipped: bool,
//     is_cancelled: bool,
//     tracking: Option<String>,
// }
// → is_paid=false, is_shipped=true? Giao hàng chưa trả tiền??
// → is_cancelled=true, is_shipped=true? Hủy rồi mà vẫn giao??
// → 2 × 2 × 2 × 2 = 16 states, nhưng chỉ ~5 states hợp lệ!

// ✅ GOOD: Enum — chỉ states hợp lệ mới tồn tại
#[derive(Debug)]
enum Order {
    Draft { items: Vec<String> },
    Confirmed { items: Vec<String>, total: u32 },
    Paid { items: Vec<String>, total: u32, payment_id: String },
    Shipped { tracking: String, payment_id: String },
    Delivered { tracking: String },
    Cancelled { reason: String },
}
// → 6 states — TẤT CẢ hợp lệ! Không thể tạo state vô nghĩa.

// State transitions = pure functions
fn confirm(order: Order) -> Result<Order, String> {
    match order {
        Order::Draft { items } => {
            if items.is_empty() {
                Err("Cannot confirm empty order".to_string())
            } else {
                let total = items.len() as u32 * 50_000; // simplified pricing
                Ok(Order::Confirmed { items, total })
            }
        }
        _ => Err(format!("Cannot confirm order in {:?} state", order)),
    }
}

fn pay(order: Order, payment_id: &str) -> Result<Order, String> {
    match order {
        Order::Confirmed { items, total, .. } => {
            Ok(Order::Paid {
                items, total,
                payment_id: payment_id.to_string(),
            })
        }
        _ => Err(format!("Cannot pay order in {:?} state", order)),
    }
}

fn ship(order: Order, tracking: &str) -> Result<Order, String> {
    match order {
        Order::Paid { payment_id, .. } => {
            Ok(Order::Shipped {
                tracking: tracking.to_string(),
                payment_id,
            })
        }
        _ => Err(format!("Cannot ship order in {:?} state", order)),
    }
}

fn main() {
    let order = Order::Draft {
        items: vec!["Coffee".into(), "Cake".into()],
    };
    println!("1. {:?}", order);

    let order = confirm(order).unwrap();
    println!("2. {:?}", order);

    let order = pay(order, "PAY-001").unwrap();
    println!("3. {:?}", order);

    let order = ship(order, "VN123456").unwrap();
    println!("4. {:?}", order);

    // ❌ Không thể ship Draft — compiler + runtime bảo vệ!
    let draft = Order::Draft { items: vec!["Tea".into()] };
    println!("Ship draft: {:?}", ship(draft, "XX"));
    // Err("Cannot ship order in Draft { items: [\"Tea\"] } state")
}
