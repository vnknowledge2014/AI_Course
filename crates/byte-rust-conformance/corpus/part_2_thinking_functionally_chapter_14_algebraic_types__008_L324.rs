// filename: src/main.rs

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Percentage(f64);

impl Percentage {
    fn new(value: f64) -> Result<Self, String> {
        if (0.0..=100.0).contains(&value) {
            Ok(Percentage(value))
        } else {
            Err(format!("{}% out of range [0, 100]", value))
        }
    }

    fn value(&self) -> f64 { self.0 }
    fn as_multiplier(&self) -> f64 { self.0 / 100.0 }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Money(u64);  // VNĐ, đơn vị đồng

impl Money {
    fn new(amount: u64) -> Self { Money(amount) }
    fn value(&self) -> u64 { self.0 }

    fn apply_discount(&self, discount: &Percentage) -> Self {
        let reduction = (self.0 as f64 * discount.as_multiplier()) as u64;
        Money(self.0 - reduction)
    }

    fn add_tax(&self, rate: &Percentage) -> Self {
        let tax = (self.0 as f64 * rate.as_multiplier()) as u64;
        Money(self.0 + tax)
    }
}

impl std::fmt::Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}đ", self.0)
    }
}

fn main() {
    let price = Money::new(500_000);
    let discount = Percentage::new(20.0).unwrap();
    let tax = Percentage::new(8.0).unwrap();

    let after_discount = price.apply_discount(&discount);
    let final_price = after_discount.add_tax(&tax);

    println!("Original:  {}", price);
    println!("Discount:  {} ({:.0}% off)", after_discount, discount.value());
    println!("Final:     {} (+{:.0}% tax)", final_price, tax.value());
    // Original:  500000đ
    // Discount:  400000đ (20% off)
    // Final:     432000đ (+8% tax)

    // ❌ Invalid percentage
    println!("{:?}", Percentage::new(150.0));  // Err("150% out of range")
}
