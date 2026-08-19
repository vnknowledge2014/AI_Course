// filename: src/main.rs

// --- Các chi tiết bên trong ---
#[derive(Debug, Clone, PartialEq)]
struct OrderId(u64);

#[derive(Debug, Clone)]
struct OrderLine {
    product_name: String,
    unit_price: u32,
    quantity: u32,
}

impl OrderLine {
    fn subtotal(&self) -> u32 { self.unit_price * self.quantity }
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Confirmed, Paid, Shipped, Delivered }

#[derive(Debug)]
enum OrderError {
    EmptyOrder,
    MaxItemsExceeded,
    InvalidTransition(String),
}

fn main() {}
