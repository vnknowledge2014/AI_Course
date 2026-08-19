// Domain expert: "Order goes through Draft → Confirmed → Shipped"
// Code phản ánh 1:1
enum OrderState {
    Draft,
    Confirmed { confirmed_at: String },
    Shipped { tracking: String },
}

// Domain expert: "Chỉ Confirmed order mới ship được"
fn ship(order: OrderState, tracking: &str) -> Result<OrderState, String> {
    match order {
        OrderState::Confirmed { .. } => Ok(OrderState::Shipped {
            tracking: tracking.to_string(),
        }),
        _ => Err("Only confirmed orders can be shipped".into()),
    }
}

fn main() {}
