// filename: src/main.rs
fn main() {
    let x: i32 = 42;
    let y: f64 = x as f64;    // i32 → f64: OK, không mất data
    let z: i32 = 3.99_f64 as i32;  // f64 → i32: CẮT phần thập phân!
    println!("x={} y={} z={}", x, y, z);
    // Output: x=42 y=42 z=3  (3.99 → 3, KHÔNG làm tròn!)

    // ⚠️ Cẩn thận: casting có thể mất data
    let big: i32 = 300;
    let small: u8 = big as u8;  // 300 không fit u8 (max 255)!
    println!("300 as u8 = {}", small);
    // Output: 300 as u8 = 44  (300 % 256 = 44, overflow wrap!)

    // Cách an toàn: dùng try_from
    match u8::try_from(300_i32) {
        Ok(val) => println!("OK: {}", val),
        Err(e) => println!("Error: {}", e),
    }
    // Output: Error: out of range integral type conversion attempted
}
