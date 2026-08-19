// filename: src/main.rs
fn main() {
    let is_active: bool = true;
    let is_admin = false;

    // bool dùng trong if, match, && || !
    let can_edit = is_active && is_admin;
    println!("Can edit? {}", can_edit);  // false

    // char = 1 Unicode scalar value (4 bytes!)
    let letter = 'A';
    let emoji = '🦀';
    let vietnamese = 'ệ';
    println!("{} {} {} — all chars!", letter, emoji, vietnamese);

    // Output:
    // Can edit? false
    // A 🦀 ệ — all chars!
}
