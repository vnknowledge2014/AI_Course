// filename: src/main.rs
fn main() {
    // Rust tự suy kiểu i32 cho số nguyên
    let age = 25;                // i32
    let price: u64 = 1_500_000;  // _ giúp đọc dễ hơn
    let byte: u8 = 255;
    let hex = 0xFF;              // hex literal = 255
    let binary = 0b1111_0000;    // binary literal = 240
    let octal = 0o77;            // octal literal = 63

    println!("age={} price={} byte={} hex={} binary={} octal={}",
        age, price, byte, hex, binary, octal);
    // Output: age=25 price=1500000 byte=255 hex=255 binary=240 octal=63

    // Integer overflow → panic trong debug, wrap trong release
    // let overflow: u8 = 256;  // ❌ error: literal out of range
}
