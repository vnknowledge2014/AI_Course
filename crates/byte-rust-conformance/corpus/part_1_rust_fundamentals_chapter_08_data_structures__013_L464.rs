// filename: src/main.rs

#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
    in_stock: bool,
}

fn main() {
    let catalog = vec![
        Product { name: "Laptop".into(), price: 25_000_000, in_stock: true },
        Product { name: "Mouse".into(), price: 500_000, in_stock: true },
        Product { name: "Keyboard".into(), price: 1_200_000, in_stock: false },
        Product { name: "Monitor".into(), price: 8_000_000, in_stock: true },
        Product { name: "Webcam".into(), price: 900_000, in_stock: false },
        Product { name: "Headset".into(), price: 2_500_000, in_stock: true },
    ];

    // Pipeline: lọc in-stock → giá > 1M → sắp xếp → format
    let mut available_premium: Vec<String> = catalog.iter()
        .filter(|p| p.in_stock)                       // chỉ còn hàng
        .filter(|p| p.price > 1_000_000)              // giá > 1M
        .map(|p| format!("{}: {}đ", p.name, p.price)) // format
        .collect();
    available_premium.sort();

    println!("🏷️ Premium products in stock:");
    for item in &available_premium {
        println!("  {}", item);
    }

    // Stats
    let total_value: u32 = catalog.iter()
        .filter(|p| p.in_stock)
        .map(|p| p.price)
        .sum();

    let count = catalog.iter().filter(|p| p.in_stock).count();
    let avg = total_value / count as u32;

    println!("\n📊 In-stock: {} items, total {}đ, avg {}đ", count, total_value, avg);

    // Output:
    // 🏷️ Premium products in stock:
    //   Headset: 2500000đ
    //   Laptop: 25000000đ
    //   Monitor: 8000000đ
    //
    // 📊 In-stock: 4 items, total 36000000đ, avg 9000000đ
}
