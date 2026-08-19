// filename: src/main.rs
fn main() {
    let data: Vec<Option<i32>> = vec![Some(1), None, Some(3), Some(4), None];

    // Duyệt + destructure lồng nhau
    let valid: Vec<i32> = data.iter()
        .filter_map(|item| *item)  // bỏ None, unwrap Some
        .collect();

    println!("Valid: {:?}", valid);
    // Output: Valid: [1, 3, 4]

    // Match trên nested structure
    let nested = Some(Some(42));
    match nested {
        Some(Some(n)) => println!("Got: {}", n),
        Some(None) => println!("Outer Some, inner None"),
        None => println!("Nothing"),
    }
    // Output: Got: 42
}
