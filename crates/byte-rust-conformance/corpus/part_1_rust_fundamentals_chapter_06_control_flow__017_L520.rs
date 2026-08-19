// filename: src/main.rs
fn main() {
    let orders = vec![35_000, 25_000, 45_000, 40_000, 30_000];

    // ❌ Imperative: mutable variable + loop
    let mut total_imp = 0;
    for &price in &orders {
        if price >= 35_000 {
            total_imp += price;
        }
    }

    // ✅ FP: iterator chain — không cần mut
    let total_fp: u32 = orders.iter()
        .filter(|&&p| p >= 35_000)  // chỉ lấy ≥ 35k
        .sum();                      // tính tổng

    assert_eq!(total_imp, total_fp);
    println!("Premium orders total: {}đ", total_fp);
    // Output: Premium orders total: 120000đ

    // Nhiều bước hơn
    let report: Vec<String> = orders.iter()
        .enumerate()
        .map(|(i, &p)| format!("#{}: {}đ", i + 1, p))
        .collect();
    println!("Orders: {:?}", report);
    // Output: Orders: ["#1: 35000đ", "#2: 25000đ", "#3: 45000đ", "#4: 40000đ", "#5: 30000đ"]
}
