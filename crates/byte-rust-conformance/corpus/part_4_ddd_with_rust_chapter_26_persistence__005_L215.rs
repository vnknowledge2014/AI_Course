// filename: src/main.rs

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Order {
    id: u64, customer: String, status: OrderStatus,
}
#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Confirmed, Shipped }

// ═══ COMMANDS (Ghi) — thay đổi State ═══
trait OrderCommands {
    fn save(&mut self, order: &Order) -> Result<(), String>;
    fn update_status(&mut self, id: u64, status: OrderStatus) -> Result<(), String>;
}

// ═══ QUERIES (Đọc) — Đọc Data, không có side effects ═══
trait OrderQueries {
    fn find_by_id(&self, id: u64) -> Option<Order>;
    fn find_by_customer(&self, customer: &str) -> Vec<Order>;
    fn count(&self) -> usize;
}

// Implementation
struct OrderStore { orders: HashMap<u64, Order> }
impl OrderStore { fn new() -> Self { OrderStore { orders: HashMap::new() } } }

impl OrderCommands for OrderStore {
    fn save(&mut self, order: &Order) -> Result<(), String> {
        self.orders.insert(order.id, order.clone()); Ok(())
    }
    fn update_status(&mut self, id: u64, status: OrderStatus) -> Result<(), String> {
        let order = self.orders.get_mut(&id).ok_or("Not found")?;
        order.status = status; Ok(())
    }
}

impl OrderQueries for OrderStore {
    fn find_by_id(&self, id: u64) -> Option<Order> { self.orders.get(&id).cloned() }
    fn find_by_customer(&self, c: &str) -> Vec<Order> { 
        self.orders.values().filter(|o| o.customer == c).cloned().collect() 
    }
    fn count(&self) -> usize { self.orders.len() }
}

fn main() {}
