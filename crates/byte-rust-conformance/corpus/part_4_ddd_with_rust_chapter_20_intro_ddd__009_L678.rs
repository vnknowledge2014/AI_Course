#[derive(Debug, Clone)]
enum BookstoreEvent {
    BookListed { isbn: String, title: String, price: u32 },
    BookPurchased { isbn: String, buyer: String, quantity: u32 },
    BookReviewed { isbn: String, reviewer: String, stars: u32 },
    StockDepleted { isbn: String },
    RefundIssued { order_id: u64, amount: u32 },
}

#[derive(Debug)]
enum BookstoreCommand {
    ListBook { isbn: String, title: String, price: u32 },
    PurchaseBook { isbn: String, buyer: String, quantity: u32 },
    ReviewBook { isbn: String, reviewer: String, stars: u32 },
}

fn apply_policies(event: &BookstoreEvent) -> Vec<String> {
    match event {
        BookstoreEvent::BookPurchased { isbn, buyer, .. } => vec![
            format!("📧 Send purchase confirmation to {}", buyer),
            format!("📦 Update inventory for {}", isbn),
            "📊 Update sales analytics".into(),
        ],
        BookstoreEvent::StockDepleted { isbn } => vec![
            format!("⚠️ Notify supplier to restock {}", isbn),
            format!("🔴 Mark {} as 'Out of Stock' on website", isbn),
        ],
        BookstoreEvent::BookReviewed { isbn, stars, .. } => vec![
            format!("⭐ Update average rating for {}", isbn),
            if *stars <= 2 { format!("🔔 Alert team: low review for {}", isbn) } else { String::new() },
        ].into_iter().filter(|s| !s.is_empty()).collect(),
        _ => vec![],
    }
}

fn main() {
    let events = vec![
        BookstoreEvent::BookPurchased {
            isbn: "978-0-13-468599-1".into(),
            buyer: "Minh".into(), quantity: 1,
        },
        BookstoreEvent::StockDepleted { isbn: "978-0-13-468599-1".into() },
        BookstoreEvent::BookReviewed {
            isbn: "978-0-13-468599-1".into(),
            reviewer: "Lan".into(), stars: 5,
        },
    ];

    for event in &events {
        println!("🟧 {:?}", event);
        for p in apply_policies(event) {
            println!("   🟪 {}", p);
        }
        println!();
    }
}
