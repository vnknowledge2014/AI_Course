// filename: src/main.rs

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: u32,
    category: String,
    in_stock: bool,
}

fn main() {
    let catalog = vec![
        Product { name: "Laptop".into(), price: 25_000_000, category: "Electronics".into(), in_stock: true },
        Product { name: "Mouse".into(), price: 500_000, category: "Electronics".into(), in_stock: true },
        Product { name: "Novel".into(), price: 150_000, category: "Books".into(), in_stock: false },
        Product { name: "Keyboard".into(), price: 1_200_000, category: "Electronics".into(), in_stock: true },
        Product { name: "Textbook".into(), price: 350_000, category: "Books".into(), in_stock: true },
        Product { name: "Monitor".into(), price: 8_000_000, category: "Electronics".into(), in_stock: true },
    ];

    // Closures như building blocks — tái sử dụng!
    let is_available = |p: &&Product| p.in_stock;
    let is_electronics = |p: &&Product| p.category == "Electronics";
    let is_affordable = |max: u32| move |p: &&Product| p.price <= max;
    let to_label = |p: &Product| format!("{} ({}đ)", p.name, p.price);

    // Nối blocks thành pipeline
    let affordable_electronics: Vec<String> = catalog.iter()
        .filter(is_available)
        .filter(is_electronics)
        .filter(is_affordable(2_000_000))
        .map(to_label)
        .collect();

    println!("Affordable electronics:");
    for item in &affordable_electronics {
        println!("  ✅ {}", item);
    }

    // Dùng LẠI blocks cho query khác
    let available_books: Vec<String> = catalog.iter()
        .filter(is_available)
        .filter(|p| p.category == "Books")
        .map(to_label)
        .collect();

    println!("\nAvailable books:");
    for item in &available_books {
        println!("  📚 {}", item);
    }

    // Output:
    // Affordable electronics:
    //   ✅ Mouse (500000đ)
    //   ✅ Keyboard (1200000đ)
    //
    // Available books:
    //   📚 Textbook (350000đ)
}
