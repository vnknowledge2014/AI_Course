// filename: src/main.rs

fn add(a: i32, b: i32) -> i32 { a + b }
fn multiply(a: i32, b: i32) -> i32 { a * b }

fn main() {
    // Function thường cũng gán vào biến được
    let operation: fn(i32, i32) -> i32 = add;
    println!("add(3, 5) = {}", operation(3, 5));

    let operation = multiply;  // đổi sang multiply
    println!("multiply(3, 5) = {}", operation(3, 5));

    // Mảng functions
    let ops: Vec<fn(i32, i32) -> i32> = vec![add, multiply];
    for op in &ops {
        println!("op(10, 3) = {}", op(10, 3));
    }

    // Output:
    // add(3, 5) = 8
    // multiply(3, 5) = 15
    // op(10, 3) = 13
    // op(10, 3) = 30
}
