// filename: src/main.rs
fn main() {
    // Tạo Vec
    let mut drinks = vec!["Coffee", "Tea", "Smoothie"]; // macro vec!
    let empty: Vec<i32> = Vec::new();                    // constructor
    let zeros = vec![0; 5];                              // [0, 0, 0, 0, 0]

    println!("Drinks: {:?}", drinks);
    println!("Empty: {:?}", empty);
    println!("Zeros: {:?}", zeros);

    // Thêm / Xóa
    drinks.push("Juice");         // thêm cuối — O(1) amortized
    drinks.push("Milk");
    let last = drinks.pop();      // xóa cuối — O(1)
    println!("Popped: {:?}", last);  // Some("Milk")
    println!("Now: {:?}", drinks);   // ["Coffee", "Tea", "Smoothie", "Juice"]

    // Insert / Remove theo index — O(n) vì phải dịch phần tử
    drinks.insert(1, "Matcha");   // chèn tại vị trí 1
    drinks.remove(3);             // xóa vị trí 3 (Smoothie)
    println!("After: {:?}", drinks); // ["Coffee", "Matcha", "Tea", "Juice"]

    // Output:
    // Drinks: ["Coffee", "Tea", "Smoothie"]
    // Empty: []
    // Zeros: [0, 0, 0, 0, 0]
    // Popped: Some("Milk")
    // Now: ["Coffee", "Tea", "Smoothie", "Juice"]
    // After: ["Coffee", "Matcha", "Tea", "Juice"]
}
