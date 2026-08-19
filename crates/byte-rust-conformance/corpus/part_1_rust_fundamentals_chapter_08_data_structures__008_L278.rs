// filename: src/main.rs
use std::collections::HashMap;

fn main() {
    // Tạo
    let mut menu: HashMap<&str, u32> = HashMap::new();
    menu.insert("coffee", 35_000);
    menu.insert("tea", 25_000);
    menu.insert("smoothie", 45_000);

    // Lookup — O(1) average
    println!("Coffee: {:?}", menu.get("coffee"));   // Some(35000)
    println!("Juice: {:?}", menu.get("juice"));      // None

    // Sửa
    menu.insert("coffee", 40_000);  // overwrite
    println!("Coffee (new): {}", menu["coffee"]);  // 40000

    // Xóa
    let removed = menu.remove("tea");
    println!("Removed: {:?}", removed);  // Some(25000)

    // Duyệt
    for (drink, price) in &menu {
        println!("  {}: {}đ", drink, price);
    }

    println!("Total items: {}", menu.len());
}
