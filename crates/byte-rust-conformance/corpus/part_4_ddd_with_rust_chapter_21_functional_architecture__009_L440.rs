// filename: src/main.rs

mod domain {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Money(u32);

    impl Money {
        pub fn new(amount: u32) -> Self { Money(amount) }
        pub fn value(&self) -> u32 { self.0 }

        pub fn apply_discount(&self, percent: u32) -> Result<Self, String> {
            if percent > 100 { return Err("Discount > 100%".into()); }
            Ok(Money(self.0 * (100 - percent) / 100))
        }

        pub fn add_tax(&self, rate_percent: u32) -> Self {
            Money(self.0 + self.0 * rate_percent / 100)
        }
    }

    // Domain rule: Pure function → Mời viết Test thỏa thích!
    pub fn calculate_total(items: &[(u32, u32)], discount: u32, tax: u32) -> Result<Money, String> {
        let subtotal: u32 = items.iter().map(|(price, qty)| price * qty).sum();
        let after_discount = Money::new(subtotal).apply_discount(discount)?;
        Ok(after_discount.add_tax(tax))
    }
}

// VIẾT TEST NGAY BÊN DƯỚI, CHẠY KHÔNG CẦN CÀI ĐẶT GÌ!
#[cfg(test)]
mod domain_tests {
    use super::domain::*;

    #[test]
    fn test_discount_10_percent() {
        let price = Money::new(100_000);
        assert_eq!(price.apply_discount(10).unwrap().value(), 90_000); // Ngọt sớt!
    }

    #[test]
    fn test_discount_qua_ho() {
        let price = Money::new(100_000);
        assert!(price.apply_discount(150).is_err()); // Sale 150% là công ty phá sản
    }

    #[test]
    fn test_tinh_tong_don_hang() {
        let items = vec![(50_000, 2), (30_000, 1)]; // = 130k
        let total = calculate_total(&items, 10, 8).unwrap();
        
        // Toán học lớp 5: 130,000 * 90% = 117,000. Lấy 117,000 + 8% VAT = 126,360
        assert_eq!(total.value(), 126_360);
    }
}

fn main() {}
