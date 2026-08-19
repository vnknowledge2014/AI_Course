fn calculate_discount(subtotal: u64, is_vip: bool) -> u32 {
    let volume_discount = match subtotal {
        s if s > 1_000_000 => 10,
        s if s > 500_000 => 5,
        _ => 0,
    };
    let vip_bonus = if is_vip { 5 } else { 0 };
    volume_discount + vip_bonus
}

// Tests
#[test]
fn small_order_no_discount() { assert_eq!(calculate_discount(300_000, false), 0); }
#[test]
fn medium_order_5_percent() { assert_eq!(calculate_discount(700_000, false), 5); }
#[test]
fn large_order_10_percent() { assert_eq!(calculate_discount(2_000_000, false), 10); }
#[test]
fn vip_gets_extra() { assert_eq!(calculate_discount(700_000, true), 10); }

fn main() {}
