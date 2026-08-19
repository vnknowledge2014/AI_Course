// filename: src/main.rs

// Event Storming output → code trực tiếp!
// Sticky notes → types

// 🟧 Domain Events (orange)
#[derive(Debug, Clone)]
enum OrderEvent {
    OrderPlaced { order_id: u64, customer: String, items: Vec<String> },
    OrderConfirmed { order_id: u64 },
    OrderShipped { order_id: u64, tracking: String },
    OrderDelivered { order_id: u64 },
    OrderCancelled { order_id: u64, reason: String },
}

// 🟦 Commands (blue)
#[derive(Debug)]
enum OrderCommand {
    PlaceOrder { customer: String, items: Vec<String> },
    ConfirmOrder { order_id: u64 },
    ShipOrder { order_id: u64, tracking: String },
    CancelOrder { order_id: u64, reason: String },
}

// 🟨 Actor (yellow) — ai trigger command
#[derive(Debug)]
enum Actor {
    Customer(String),
    Staff(String),
    System,
}

// 🟪 Policy (purple) — side-effects từ events
fn apply_policies(event: &OrderEvent) -> Vec<String> {
    match event {
        OrderEvent::OrderPlaced { items, .. } => {
            vec![
                format!("📦 Reserve inventory for {} items", items.len()),
                "📧 Send order confirmation email".into(),
                "📊 Update sales dashboard".into(),
            ]
        }
        OrderEvent::OrderShipped { tracking, .. } => {
            vec![
                format!("📧 Send shipping notification (tracking: {})", tracking),
            ]
        }
        OrderEvent::OrderCancelled { order_id, .. } => {
            vec![
                format!("📦 Release inventory for order #{}", order_id),
                "💰 Process refund".into(),
            ]
        }
        _ => vec![],
    }
}

fn main() {
    // Simulate Event Storming flow
    let events = vec![
        OrderEvent::OrderPlaced {
            order_id: 1,
            customer: "Minh".into(),
            items: vec!["Coffee".into(), "Cake".into()],
        },
        OrderEvent::OrderConfirmed { order_id: 1 },
        OrderEvent::OrderShipped { order_id: 1, tracking: "VN123".into() },
    ];

    for event in &events {
        println!("🟧 Event: {:?}", event);
        let policies = apply_policies(event);
        for p in &policies {
            println!("   🟪 Policy: {}", p);
        }
        println!();
    }
}
