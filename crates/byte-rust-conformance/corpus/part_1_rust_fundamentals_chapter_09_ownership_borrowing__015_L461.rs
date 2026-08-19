// filename: src/main.rs
fn process(data: &[i32]) -> i32 {  // nhận slice, không lấy ownership
    data.iter().sum()
}

fn main() {
    let original = vec![1, 2, 3, 4, 5];

    let sum = process(&original);  // borrow — không cần clone!
    println!("Sum: {}, Original: {:?}", sum, original);
    // Output: Sum: 15, Original: [1, 2, 3, 4, 5]
}
