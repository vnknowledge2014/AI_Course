// filename: src/main.rs

// ═══════ Complete domain model ═══════

// --- Value Objects ---
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProductId(String);

#[derive(Debug, Clone, PartialEq)]
struct ProductName(String);
impl ProductName {
    fn new(name: &str) -> Result<Self, String> {
        let trimmed = name.trim();
        if trimmed.len() < 2 || trimmed.len() > 100 {
            Err("Product name: 2-100 chars".into())
        } else {
            Ok(ProductName(trimmed.into()))
        }
    }
    fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
struct Price(u32);
impl Price {
    fn new(amount: u32) -> Result<Self, String> {
        if amount == 0 { Err("Price must be > 0".into()) }
        else { Ok(Price(amount)) }
    }
    fn value(&self) -> u32 { self.0 }
}

// --- Entity ---
#[derive(Debug, Clone)]
struct Product {
    id: ProductId,
    name: ProductName,
    price: Price,
    stock: u32,
}

impl Product {
    // Thu thập tất cả lỗi cùng lúc thay vì Fail Fast
    fn new(id: &str, name: &str, price: u32) -> Result<Self, Vec<String>> {
        let mut errors = vec![];
        let name = ProductName::new(name).map_err(|e| errors.push(e)).ok();
        let price = Price::new(price).map_err(|e| errors.push(e)).ok();

        if errors.is_empty() {
            Ok(Product {
                id: ProductId(id.into()),
                name: name.unwrap(),
                price: price.unwrap(),
                stock: 0,
            })
        } else {
            Err(errors)
        }
    }

    fn restock(&self, amount: u32) -> Self {
        Product { stock: self.stock + amount, ..self.clone() }
    }

    fn reserve(&self, quantity: u32) -> Result<Self, String> {
        if quantity > self.stock {
            Err(format!("Insufficient stock: have {}, need {}", self.stock, quantity))
        } else {
            Ok(Product { stock: self.stock - quantity, ..self.clone() })
        }
    }
}

fn main() {}
