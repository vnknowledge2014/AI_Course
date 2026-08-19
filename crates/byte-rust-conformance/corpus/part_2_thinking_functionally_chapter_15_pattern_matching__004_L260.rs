// filename: src/main.rs

#[derive(Debug)]
struct Order {
    total: u32,
    items: u32,
    is_vip: bool,
}

fn shipping_fee(order: &Order) -> u32 {
    match order {
        // Free shipping: VIP hoặc order > 500k
        Order { is_vip: true, .. } => 0,
        Order { total, .. } if *total >= 500_000 => 0,

        // Reduced: 3+ items
        Order { items, total, .. } if *items >= 3 && *total >= 200_000 => 15_000,

        // Standard
        Order { total, .. } if *total >= 100_000 => 30_000,

        // Small orders
        _ => 50_000,
    }
}

fn main() {
    let orders = vec![
        Order { total: 600_000, items: 2, is_vip: false },
        Order { total: 300_000, items: 5, is_vip: false },
        Order { total: 150_000, items: 1, is_vip: false },
        Order { total: 50_000, items: 1, is_vip: false },
        Order { total: 50_000, items: 1, is_vip: true },
    ];

    for o in &orders {
        println!("{:?} → shipping: {}đ", o, shipping_fee(o));
    }
}
