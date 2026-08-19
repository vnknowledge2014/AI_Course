// filename: src/main.rs

// Core domain — ĐẦU TƯ nhiều nhất! Code cẩn thận, types chặt.
mod pricing {
    use std::fmt;

    #[derive(Debug, Clone)]
    pub struct PricingRule {
        pub name: String,
        pub condition: PriceCondition,
        pub discount: Discount,
    }

    #[derive(Debug, Clone)]
    pub enum PriceCondition {
        MinQuantity(u32),
        CustomerTier(String),
        TimeRange { from: String, to: String },
        Combined(Vec<PriceCondition>),
    }

    #[derive(Debug, Clone)]
    pub enum Discount {
        Percentage(f64),
        FixedAmount(u32),
        BuyXGetY { buy: u32, free: u32 },
    }

    pub fn apply_best_rule(base_price: u32, quantity: u32, rules: &[PricingRule]) -> u32 {
        // Business logic phức tạp — core domain!
        let mut best_price = base_price * quantity;

        for rule in rules {
            let discounted = match &rule.discount {
                Discount::Percentage(pct) => {
                    let total = base_price * quantity;
                    total - (total as f64 * pct / 100.0) as u32
                }
                Discount::FixedAmount(amt) => {
                    (base_price - amt) * quantity
                }
                Discount::BuyXGetY { buy, free } => {
                    let sets = quantity / (buy + free);
                    let remainder = quantity % (buy + free);
                    let paid = sets * buy + remainder.min(*buy);
                    base_price * paid
                }
            };

            if discounted < best_price {
                best_price = discounted;
            }
        }

        best_price
    }
}

// Supporting domain — cần, nhưng logic đơn giản hơn
mod inventory {
    pub fn check_stock(product_id: &str) -> bool {
        // Simplified — trong thực tế query DB
        product_id.starts_with("PROD")
    }
}

// Generic domain — dùng thư viện/SaaS, không build
// mod auth { use jsonwebtoken; }
// mod email { use lettre; }

fn main() {
    use pricing::*;

    let rules = vec![
        PricingRule {
            name: "VIP 20%".into(),
            condition: PriceCondition::CustomerTier("VIP".into()),
            discount: Discount::Percentage(20.0),
        },
        PricingRule {
            name: "Buy 3 Get 1 Free".into(),
            condition: PriceCondition::MinQuantity(4),
            discount: Discount::BuyXGetY { buy: 3, free: 1 },
        },
    ];

    let price = apply_best_rule(100_000, 4, &rules);
    println!("Best price for 4 items: {}đ", price);
    // Buy3Get1Free: pay 3 × 100k = 300k (vs 4 × 80k = 320k)
    // → Best: 300000đ
}
