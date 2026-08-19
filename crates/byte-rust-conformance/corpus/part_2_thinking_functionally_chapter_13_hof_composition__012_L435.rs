// filename: src/main.rs

#[derive(Debug, Clone)]
struct Order {
    items: Vec<(String, u32, u32)>, // (name, price, quantity)
}

impl Order {
    fn new(items: Vec<(String, u32, u32)>) -> Self {
        Order { items }
    }

    fn subtotal(&self) -> u32 {
        self.items.iter().map(|(_, p, q)| p * q).sum()
    }

    // HOF: nhận pricing strategy từ bên ngoài
    fn total_with_strategy<F: Fn(u32) -> u32>(&self, pricing: F) -> u32 {
        pricing(self.subtotal())
    }
}

// Pricing strategies — pure functions
fn no_discount(price: u32) -> u32 { price }
fn vip_discount(price: u32) -> u32 { price * 85 / 100 }
fn holiday_sale(price: u32) -> u32 { price * 70 / 100 }

fn make_coupon_discount(percent: u32) -> impl Fn(u32) -> u32 {
    move |price| price * (100 - percent) / 100
}

fn main() {
    let order = Order::new(vec![
        ("Coffee".into(), 35_000, 2),
        ("Cake".into(), 25_000, 1),
    ]);

    println!("Subtotal: {}đ", order.subtotal());
    println!("Regular:  {}đ", order.total_with_strategy(no_discount));
    println!("VIP:      {}đ", order.total_with_strategy(vip_discount));
    println!("Holiday:  {}đ", order.total_with_strategy(holiday_sale));
    println!("Coupon15: {}đ", order.total_with_strategy(make_coupon_discount(15)));

    // Output:
    // Subtotal: 95000đ
    // Regular:  95000đ
    // VIP:      80750đ
    // Holiday:  66500đ
    // Coupon15: 80750đ
}
