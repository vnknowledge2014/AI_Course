/// Quantity — Luôn dương, có giới hạn tối đa
#[derive(Debug, Clone, Copy, PartialEq)]
struct Quantity(u32);

impl Quantity {
    fn new(qty: u32) -> Result<Self, String> {
        if qty == 0 { return Err("Quantity must be > 0".into()); }
        if qty > 10_000 { return Err("Quantity exceeds max 10,000".into()); }
        Ok(Quantity(qty))
    }
    fn value(&self) -> u32 { self.0 }
}

/// Price — Tiền không bao giờ âm
#[derive(Debug, Clone, Copy, PartialEq)]
struct Price(u64);

impl Price {
    fn new(cents: u64) -> Result<Self, String> {
        if cents > 100_000_000 { return Err("Price exceeds max".into()); }
        Ok(Price(cents))
    }
    fn cents(&self) -> u64 { self.0 }
    
    // Tính tổng tiền của một dòng sản phẩm
    fn multiply(&self, qty: Quantity) -> Price {
        Price(self.0 * qty.value() as u64)
    }
}

impl std::fmt::Display for Price {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{:02}đ", self.0 / 100, self.0 % 100)
    }
}

fn main() {}
