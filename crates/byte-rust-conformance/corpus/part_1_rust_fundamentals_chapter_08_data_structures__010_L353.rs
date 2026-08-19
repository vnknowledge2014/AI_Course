// filename: src/main.rs
use std::collections::HashSet;

fn main() {
    let mut languages: HashSet<&str> = HashSet::new();
    languages.insert("Rust");
    languages.insert("Go");
    languages.insert("Zig");
    languages.insert("Rust");  // trùng → bị bỏ qua

    println!("Languages: {:?}", languages);
    println!("Contains Rust: {}", languages.contains("Rust"));
    println!("Size: {}", languages.len());  // 3, không phải 4

    // Set operations
    let systems: HashSet<&str> = ["Rust", "C", "Zig"].iter().cloned().collect();
    let modern: HashSet<&str> = ["Rust", "Go", "Zig"].iter().cloned().collect();

    // Giao (intersection): có trong CẢ HAI
    let both: HashSet<&&str> = systems.intersection(&modern).collect();
    println!("Both systems & modern: {:?}", both);  // {"Rust", "Zig"}

    // Hợp (union): có trong ÍT NHẤT MỘT
    let any: HashSet<&&str> = systems.union(&modern).collect();
    println!("Either: {:?}", any);  // {"Rust", "C", "Zig", "Go"}

    // Hiệu (difference): có trong A nhưng không trong B
    let only_systems: HashSet<&&str> = systems.difference(&modern).collect();
    println!("Only systems: {:?}", only_systems);  // {"C"}
}
