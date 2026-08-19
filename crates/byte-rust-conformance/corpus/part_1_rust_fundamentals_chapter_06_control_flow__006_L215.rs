// filename: src/main.rs
fn main() {
    // Option
    let items = vec![10, 20, 30];
    let first = items.first();  // Option<&i32>

    match first {
        Some(val) => println!("First item: {}", val),
        None => println!("Empty list!"),
    }

    // Result
    let input = "42abc";
    let parsed: Result<i32, _> = input.parse();

    match parsed {
        Ok(n) => println!("Parsed: {}", n),
        Err(e) => println!("Parse error: {} (input: '{}')", e, input),
    }

    // Output:
    // First item: 10
    // Parse error: invalid digit found in string (input: '42abc')
}
