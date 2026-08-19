// filename: src/main.rs

// Newtype: wrap primitive để thêm ý nghĩa + validation
#[derive(Debug, Clone, PartialEq)]
pub struct Email(String);  // private inner → không sửa được trực tiếp

impl Email {
    pub fn new(value: &str) -> Result<Self, String> {
        if value.contains('@') && value.contains('.') {
            Ok(Email(value.to_lowercase()))
        } else {
            Err(format!("Invalid email: {}", value))
        }
    }

    pub fn as_str(&self) -> &str { &self.0 }
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Money(u64);  // cents để tránh floating point

impl Money {
    pub fn from_dong(dong: u64) -> Self { Money(dong) }
    pub fn value(&self) -> u64 { self.0 }

    // Immutable operations — trả giá trị mới
    pub fn add(&self, other: &Money) -> Money {
        Money(self.0 + other.0)
    }

    pub fn multiply(&self, factor: u32) -> Money {
        Money(self.0 * factor as u64)
    }

    pub fn display(&self) -> String {
        format!("{}đ", self.0)
    }
}

fn main() {
    let email = Email::new("Minh@Gmail.Com").unwrap();
    println!("Email: {}, Domain: {}", email.as_str(), email.domain());
    // Email: minh@gmail.com, Domain: gmail.com

    let price = Money::from_dong(35_000);
    let quantity_price = price.multiply(3);
    let total = quantity_price.add(&Money::from_dong(5_000));
    println!("3 × {} + 5000đ = {}", price.display(), total.display());
    // 3 × 35000đ + 5000đ = 110000đ

    // price vẫn là 35000đ — không bị sửa!
    println!("Original price: {}", price.display());
}
