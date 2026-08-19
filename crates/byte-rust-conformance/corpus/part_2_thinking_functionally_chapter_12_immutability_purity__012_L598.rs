// filename: src/main.rs
fn main() {
    let prices = vec![35_000, 25_000, 45_000, 40_000];

    // ❌ Imperative: mutate in-place
    // let mut discounted = prices.clone();
    // for p in &mut discounted { *p = *p * 90 / 100; }

    // ✅ Functional: transform → new collection
    let discounted: Vec<u32> = prices.iter()
        .map(|&p| p * 90 / 100)  // 10% off
        .collect();

    println!("Original: {:?}", prices);       // không đổi
    println!("Discounted: {:?}", discounted);  // mới
}
