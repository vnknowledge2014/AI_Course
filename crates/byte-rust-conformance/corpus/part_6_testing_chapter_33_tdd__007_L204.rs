// filename: src/lib.rs

// ═══ Domain: Money (Phần 1: Khởi tạo) ═══
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Money(i64);

impl Money {
    // Không cho phép khởi tạo tiền âm
    pub fn new(cents: i64) -> Result<Self, String> {
        if cents < 0 { Err("Money cannot be negative".into()) }
        else { Ok(Money(cents)) }
    }

    pub fn zero() -> Self { Money(0) }
    pub fn cents(&self) -> i64 { self.0 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_money_success() {
        assert_eq!(Money::new(500).unwrap().cents(), 500);
    }

    #[test]
    fn reject_negative_money() {
        assert!(Money::new(-100).is_err());
    }
}

fn main() {}
