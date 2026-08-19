// filename: src/main.rs

// ═══ LỚP 1: DOMAIN (Trái tim thuần khiết, không gọi thư viện ngoài) ═══
mod domain {
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: u64,
        pub name: String,
        pub price: u32,
        pub stock: u32,
    }

    #[derive(Debug)]
    pub enum OrderError {
        OutOfStock(String),
        InvalidQuantity,
    }

    // Pure domain logic — no dependencies!
    pub fn can_fulfill(product: &Product, qty: u32) -> Result<(), OrderError> {
        if qty == 0 { return Err(OrderError::InvalidQuantity); }
        if product.stock < qty {
            Err(OrderError::OutOfStock(format!("{}: have {}, need {}", product.name, product.stock, qty)))
        } else {
            Ok(())
        }
    }

    pub fn calculate_total(price: u32, qty: u32, discount_pct: u32) -> u32 {
        let subtotal = price * qty;
        subtotal - subtotal * discount_pct / 100
    }
}

// ═══ LỚP 2: PORTS (Bộ giao thức mà Core yêu cầu thế giới ngoài phải tuân thủ) ═══
mod ports {
    use super::domain::*;

    pub trait ProductRepo {
        fn find(&self, id: u64) -> Option<Product>;
        fn update_stock(&mut self, id: u64, new_stock: u32) -> Result<(), String>;
    }

    pub trait PaymentGateway {
        fn charge(&self, amount: u32, description: &str) -> Result<String, String>;
    }
}

fn main() {}
