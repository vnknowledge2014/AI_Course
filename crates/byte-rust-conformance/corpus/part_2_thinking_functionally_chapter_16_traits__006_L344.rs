// filename: src/main.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct ProductId(u64);

#[derive(Debug, Clone, PartialEq)]
struct Product {
    id: ProductId,
    name: String,
    price: u32,
}

fn main() {
    let a = Product { id: ProductId(1), name: "Coffee".into(), price: 35_000 };
    let b = a.clone();  // Clone

    println!("{:?}", a);        // Debug
    println!("Equal: {}", a == b);  // PartialEq

    // Hash cho phép dùng trong HashSet/HashMap
    use std::collections::HashSet;
    let mut ids = HashSet::new();
    ids.insert(ProductId(1));
    ids.insert(ProductId(2));
    ids.insert(ProductId(1));  // duplicate → bỏ qua
    println!("Unique IDs: {}", ids.len());  // 2
}
