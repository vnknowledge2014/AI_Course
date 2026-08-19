// filename: src/main.rs
fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // sum bằng fold
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("Sum: {}", sum);  // 15

    // product bằng fold
    let product = numbers.iter().fold(1, |acc, &x| acc * x);
    println!("Product: {}", product);  // 120

    // max bằng fold
    let max = numbers.iter().fold(i32::MIN, |acc, &x| if x > acc { x } else { acc });
    println!("Max: {}", max);  // 5

    // Build string bằng fold
    let csv = numbers.iter().fold(String::new(), |acc, &x| {
        if acc.is_empty() {
            x.to_string()
        } else {
            format!("{},{}", acc, x)
        }
    });
    println!("CSV: {}", csv);  // 1,2,3,4,5
}
