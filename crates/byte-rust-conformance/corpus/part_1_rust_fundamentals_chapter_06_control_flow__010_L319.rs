// filename: src/main.rs

#[derive(Debug)]
enum Order {
    Pending,
    Processing { items: u32 },
    Shipped { tracking: String },
}

fn describe_order(order: &Order) {
    match order {
        Order::Pending => println!("⏳ Pending"),

        // @ binding: bắt giá trị VÀ kiểm tra điều kiện
        Order::Processing { items: n @ 1..=5 } => {
            println!("🔄 Processing {} items (small order)", n);
        }
        Order::Processing { items: n } => {
            println!("🔄 Processing {} items (large order)", n);
        }

        // Binding toàn bộ inner value
        Order::Shipped { tracking: ref id } => {
            println!("📦 Shipped — tracking: {}", id);
        }
    }
}

fn main() {
    describe_order(&Order::Pending);
    describe_order(&Order::Processing { items: 3 });
    describe_order(&Order::Processing { items: 50 });
    describe_order(&Order::Shipped { tracking: "VN123456".to_string() });

    // Output:
    // ⏳ Pending
    // 🔄 Processing 3 items (small order)
    // 🔄 Processing 50 items (large order)
    // 📦 Shipped — tracking: VN123456
}
