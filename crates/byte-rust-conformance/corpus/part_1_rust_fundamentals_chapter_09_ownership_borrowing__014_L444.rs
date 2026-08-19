// filename: src/main.rs
fn process(data: Vec<i32>) -> i32 {
    data.iter().sum()  // data bị consumed sau function
}

fn main() {
    let original = vec![1, 2, 3, 4, 5];

    let sum = process(original.clone());  // clone → original vẫn còn
    println!("Sum: {}, Original: {:?}", sum, original);
    // Output: Sum: 15, Original: [1, 2, 3, 4, 5]
}
