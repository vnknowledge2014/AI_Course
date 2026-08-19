// filename: src/domain/types.rs

#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);
impl Email {
    pub fn new(email: &str) -> Result<Self, String> {
        let e = email.trim().to_lowercase();
        if !e.contains('@') || e.len() < 5 { return Err("Invalid email".into()); }
        Ok(Email(e))
    }
    pub fn value(&self) -> &str { &self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quantity(u32);
impl Quantity {
    pub fn new(qty: u32) -> Result<Self, String> {
        if qty == 0 || qty > 10_000 { return Err("Quantity 1-10000".into()); }
        Ok(Quantity(qty))
    }
    pub fn value(&self) -> u32 { self.0 }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Money(i64); // Lưu bằng đơn vị nhỏ nhất (cents) để tránh sai số dấu phẩy động
impl Money {
    pub fn new(cents: i64) -> Result<Self, String> {
        if cents < 0 { return Err("Money cannot be negative".into()); }
        Ok(Money(cents))
    }
    pub fn cents(&self) -> i64 { self.0 }
    pub fn add(&self, other: &Money) -> Money { Money(self.0 + other.0) }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, (self.0 % 100).abs())
    }
}

fn main() {}
