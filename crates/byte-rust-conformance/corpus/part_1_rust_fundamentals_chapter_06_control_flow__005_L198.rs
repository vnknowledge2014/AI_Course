// filename: src/main.rs
fn main() {
    let point = (3, 7);

    match point {
        (0, 0) => println!("Origin"),
        (x, 0) => println!("On x-axis at {}", x),
        (0, y) => println!("On y-axis at {}", y),
        (x, y) => println!("Point ({}, {})", x, y),
    }
    // Output: Point (3, 7)
}
