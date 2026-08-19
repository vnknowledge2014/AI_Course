// filename: src/main.rs
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum ShopEvent {
    ProductAdded { name: String, qty: u32 },
    ProductSold { name: String, revenue: u32 },
}

// Góc nhìn của TRƯỞNG KHO (Chỉ quan tâm Hàng hóa)
#[derive(Debug, Default)]
struct InventoryView {
    stock: HashMap<String, u32>,
}
impl InventoryView {
    fn apply(&mut self, event: &ShopEvent) {
        match event {
            ShopEvent::ProductAdded { name, qty } => *self.stock.entry(name.clone()).or_insert(0) += qty,
            ShopEvent::ProductSold { name, .. } => *self.stock.entry(name.clone()).or_insert(0) -= 1,
        }
    }
}

// Góc nhìn của KẾ TOÁN (Chỉ quan tâm Tiền)
#[derive(Debug, Default)]
struct RevenueView {
    total: u32,
}
impl RevenueView {
    fn apply(&mut self, event: &ShopEvent) {
        if let ShopEvent::ProductSold { revenue, .. } = event {
            self.total += revenue;
        }
    }
}

fn main() {}
