// filename: src/main.rs

// ═══════════════════════════════════════════
// CORE: Pure functions — dễ test, không I/O
// ═══════════════════════════════════════════

#[derive(Debug, Clone)]
struct Order {
    items: Vec<(String, u32)>,
}

// Pure: chỉ tính toán
fn calculate_subtotal(order: &Order) -> u32 {
    order.items.iter().map(|(_, price)| price).sum()
}

// Pure: chỉ tính toán
fn apply_tax(subtotal: u32, rate: f64) -> u32 {
    subtotal + (subtotal as f64 * rate) as u32
}

// Pure: chỉ tạo string
fn format_order_receipt(order: &Order, total: u32) -> String {
    let mut lines = vec!["🧾 Receipt".to_string()];
    for (name, price) in &order.items {
        lines.push(format!("  {} — {}đ", name, price));
    }
    lines.push(format!("  ─────────"));
    lines.push(format!("  Total: {}đ", total));
    lines.join("\n")
}

// ═══════════════════════════════════════════
// SHELL: I/O, side-effects — main()
// ═══════════════════════════════════════════

fn main() {
    // Shell: tạo data (trong production: đọc từ DB/API)
    let order = Order {
        items: vec![
            ("Coffee".into(), 35_000),
            ("Cake".into(), 25_000),
            ("Juice".into(), 30_000),
        ],
    };

    // Core: pure computations
    let subtotal = calculate_subtotal(&order);
    let total = apply_tax(subtotal, 0.08);
    let receipt = format_order_receipt(&order, total);

    // Shell: I/O output
    println!("{}", receipt);
}

// Tests: chỉ test CORE — không cần mock I/O!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_subtotal() {
        let order = Order {
            items: vec![("A".into(), 10_000), ("B".into(), 20_000)],
        };
        assert_eq!(calculate_subtotal(&order), 30_000);
    }

    #[test]
    fn test_tax() {
        assert_eq!(apply_tax(100_000, 0.1), 110_000);
    }

    #[test]
    fn test_receipt_format() {
        let order = Order { items: vec![("Tea".into(), 25_000)] };
        let receipt = format_order_receipt(&order, 27_000);
        assert!(receipt.contains("Tea"));
        assert!(receipt.contains("27000đ"));
    }
}
