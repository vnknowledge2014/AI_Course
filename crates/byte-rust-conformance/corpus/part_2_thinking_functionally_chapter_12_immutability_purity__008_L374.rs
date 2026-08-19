// filename: src/main.rs

// Pure function: test SIÊU ĐƠN GIẢN
fn calculate_tax(amount: u32, rate: f64) -> u32 {
    (amount as f64 * rate) as u32
}

fn apply_discount(price: u32, code: &str) -> u32 {
    match code {
        "VIP20" => price * 80 / 100,
        "SAVE10" => price * 90 / 100,
        _ => price,
    }
}

fn main() {
    println!("Tax: {}", calculate_tax(100_000, 0.08));
    println!("VIP: {}", apply_discount(100_000, "VIP20"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tax() {
        assert_eq!(calculate_tax(100_000, 0.08), 8_000);
        assert_eq!(calculate_tax(0, 0.08), 0);
    }

    #[test]
    fn test_discount_vip() {
        assert_eq!(apply_discount(100_000, "VIP20"), 80_000);
    }

    #[test]
    fn test_discount_unknown_code() {
        assert_eq!(apply_discount(100_000, "FAKE"), 100_000);
    }
}
