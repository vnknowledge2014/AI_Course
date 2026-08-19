// filename: src/lib.rs

mod domain {
    // ═══ FUNCTIONAL CORE — pure, no IO ═══
    pub fn validate_discount(price: u32, discount_pct: u32) -> Result<u32, String> {
        if discount_pct > 50 { return Err("Max discount is 50%".into()); }
        Ok(price * (100 - discount_pct) / 100)
    }

    pub fn tier_from_total_spent(total: u64) -> &'static str {
        match total {
            0..=999_999 => "Bronze",
            1_000_000..=4_999_999 => "Silver",
            5_000_000..=19_999_999 => "Gold",
            _ => "Platinum",
        }
    }

    pub fn shipping_cost(weight_grams: u32, zone: &str) -> u32 {
        let base = match zone {
            "local" => 15_000,
            "domestic" => 30_000,
            "international" => 150_000,
            _ => 50_000,
        };
        let weight_surcharge = (weight_grams / 500) * 5_000;
        base + weight_surcharge
    }
}

fn main() {}
