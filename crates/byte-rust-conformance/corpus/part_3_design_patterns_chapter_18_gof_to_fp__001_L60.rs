// filename: src/main.rs

// Strategy = function. Không cần interface/class.
fn calculate_total(prices: &[u32], strategy: impl Fn(u32) -> u32) -> u32 {
    prices.iter().map(|&p| strategy(p)).sum()
}

// "Strategies" = closures hoặc functions
fn no_discount(price: u32) -> u32 { price }
fn vip_discount(price: u32) -> u32 { price * 85 / 100 }

fn make_coupon(percent: u32) -> impl Fn(u32) -> u32 {
    move |price| price * (100 - percent) / 100
}

fn main() {
    let prices = vec![100_000, 200_000, 50_000];

    println!("Regular: {}đ", calculate_total(&prices, no_discount));
    println!("VIP:     {}đ", calculate_total(&prices, vip_discount));
    println!("Coupon:  {}đ", calculate_total(&prices, make_coupon(15)));
    println!("Lambda:  {}đ", calculate_total(&prices, |p| if p > 100_000 { p * 90 / 100 } else { p }));

    // Output:
    // Regular: 350000đ
    // VIP:     297500đ
    // Coupon:  297500đ
    // Lambda:  330000đ
}
