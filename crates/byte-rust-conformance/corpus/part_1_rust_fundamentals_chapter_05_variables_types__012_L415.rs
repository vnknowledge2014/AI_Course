// filename: src/main.rs
fn main() {
    // Arithmetic
    println!("10 + 3 = {}", 10 + 3);    // 13
    println!("10 - 3 = {}", 10 - 3);    // 7
    println!("10 * 3 = {}", 10 * 3);    // 30
    println!("10 / 3 = {}", 10 / 3);    // 3 (integer division!)
    println!("10 % 3 = {}", 10 % 3);    // 1 (remainder)
    println!("10.0 / 3.0 = {:.2}", 10.0 / 3.0);  // 3.33 (float division)

    // Comparison — trả bool
    println!("5 == 5: {}", 5 == 5);     // true
    println!("5 != 3: {}", 5 != 3);     // true
    println!("5 > 3: {}", 5 > 3);       // true
    println!("5 <= 5: {}", 5 <= 5);     // true

    // Logical
    println!("true && false: {}", true && false); // false
    println!("true || false: {}", true || false); // true
    println!("!true: {}", !true);                 // false

    // ⚠️ Không thể so sánh khác kiểu!
    // println!("{}", 5_i32 == 5_i64);  // ❌ mismatched types
    println!("{}", 5_i32 == 5);         // ✅ cùng kiểu
}
