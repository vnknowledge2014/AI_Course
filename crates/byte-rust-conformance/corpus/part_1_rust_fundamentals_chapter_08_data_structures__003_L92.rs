// filename: src/main.rs

// Function nhận slice — linh hoạt hơn nhận &Vec
fn average(items: &[f64]) -> f64 {
    let sum: f64 = items.iter().sum();
    sum / items.len() as f64
}

fn main() {
    let scores = vec![8.5, 7.0, 9.0, 6.5, 8.0];

    // Slice của toàn bộ Vec
    println!("All: {:.1}", average(&scores));

    // Slice một phần
    println!("First 3: {:.1}", average(&scores[..3]));
    println!("Last 2: {:.1}", average(&scores[3..]));

    // Array cũng truyền được — vì &[T; N] coerce thành &[T]
    let fixed = [1.0, 2.0, 3.0];
    println!("Fixed avg: {:.1}", average(&fixed));

    // Output:
    // All: 7.8
    // First 3: 8.2
    // Last 2: 7.3
    // Fixed avg: 2.0
}
