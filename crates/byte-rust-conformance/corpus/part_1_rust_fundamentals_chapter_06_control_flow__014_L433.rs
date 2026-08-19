// filename: src/main.rs
fn main() {
    let mut stack = vec![1, 2, 3, 4, 5];

    // Pop cho đến khi hết
    while let Some(top) = stack.pop() {
        println!("Popped: {}", top);
    }
    println!("Stack empty!");

    // Output:
    // Popped: 5
    // Popped: 4
    // Popped: 3
    // Popped: 2
    // Popped: 1
    // Stack empty!
}
