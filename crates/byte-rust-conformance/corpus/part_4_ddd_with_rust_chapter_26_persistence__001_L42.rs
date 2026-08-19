// filename: src/main.rs

// ═══ LỚP 1: DOMAIN (Pure, No IO) ═══
mod domain {
    #[derive(Debug, Clone, PartialEq)]
    pub struct ProductId(pub u64);

    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: ProductId,
        pub name: String,
        pub price: u32,
        pub stock: u32,
    }

    // Các nghiệp vụ cốt lõi
    impl Product {
        pub fn restock(&self, amount: u32) -> Self {
            Product { stock: self.stock + amount, ..self.clone() }
        }

        pub fn reserve(&self, qty: u32) -> Result<Self, String> {
            if qty > self.stock {
                Err(format!("Insufficient stock: have {}, need {}", self.stock, qty))
            } else {
                Ok(Product { stock: self.stock - qty, ..self.clone() })
            }
        }
    }
}

fn main() {}
