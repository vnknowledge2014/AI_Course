// filename: src/main.rs

// ✅ ĐÚNG — liệt kê chính xác các trạng thái hợp lệ
#[derive(Debug)]
enum OrderStatus {
    Confirmed,
    Preparing,
    Ready,
    Delivered,
}

#[derive(Debug)]
struct Order {
    id: u32,
    item: String,
    status: OrderStatus,
}

fn advance(order: &mut Order) {
    order.status = match order.status {
        OrderStatus::Confirmed => {
            println!("  ☕ Start preparing #{}", order.id);
            OrderStatus::Preparing
        }
        OrderStatus::Preparing => {
            println!("  ✅ Order #{} ready!", order.id);
            OrderStatus::Ready
        }
        OrderStatus::Ready => {
            println!("  🎉 Customer picked up #{}", order.id);
            OrderStatus::Delivered
        }
        OrderStatus::Delivered => {
            println!("  📌 Order #{} already done!", order.id);
            OrderStatus::Delivered
        }
    };
}

fn main() {
    let mut order = Order {
        id: 1,
        item: "Cà phê sữa đá".to_string(),
        status: OrderStatus::Confirmed,
    };

    // Chỉ 4 trạng thái — TẤT CẢ hợp lệ!
    println!("📝 New order: {:?}", order.status);

    advance(&mut order);  // Confirmed → Preparing
    advance(&mut order);  // Preparing → Ready
    advance(&mut order);  // Ready → Delivered

    // Output:
    // 📝 New order: Confirmed
    //   ☕ Start preparing #1
    //   ✅ Order #1 ready!
    //   🎉 Customer picked up #1
}
