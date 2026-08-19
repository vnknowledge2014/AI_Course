// filename: src/main.rs
fn main() {
    // Range
    for i in 1..=5 {
        print!("{} ", i);
    }
    println!();  // 1 2 3 4 5

    // Vec
    let drinks = vec!["Coffee", "Tea", "Smoothie"];
    for drink in &drinks {
        println!("☕ {}", drink);
    }

    // Enumerate — index + value
    for (i, drink) in drinks.iter().enumerate() {
        println!("#{}: {}", i + 1, drink);
    }

    // Output:
    // 1 2 3 4 5
    // ☕ Coffee
    // ☕ Tea
    // ☕ Smoothie
    // #1: Coffee
    // #2: Tea
    // #3: Smoothie
}
