// filename: src/main.rs
fn main() {
    // Product: struct { bool, bool } = 2 × 2 = 4 states
    // (true,true), (true,false), (false,true), (false,false)

    // Sum: enum { A(bool), B(bool) } = 2 + 2 = 4 states
    // A(true), A(false), B(true), B(false)

    // Option<bool> = None + Some(bool) = 1 + 2 = 3 states
    // None, Some(true), Some(false)

    // Result<bool, u8> = Ok(bool) + Err(u8) = 2 + 256 = 258 states

    println!("Product (bool, bool): {} states", 2 * 2);
    println!("Sum A(bool)|B(bool): {} states", 2 + 2);
    println!("Option<bool>: {} states", 1 + 2);
    println!("Result<bool, u8>: {} states", 2 + 256);
}
