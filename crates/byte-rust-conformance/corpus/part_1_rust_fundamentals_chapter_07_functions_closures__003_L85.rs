// filename: src/main.rs

fn stats(items: &[f64]) -> (f64, f64, f64) {
    let sum: f64 = items.iter().sum();
    let count = items.len() as f64;
    let avg = sum / count;

    let min = items.iter().cloned().reduce(f64::min).unwrap();
    let max = items.iter().cloned().reduce(f64::max).unwrap();

    (avg, min, max)  // trả tuple 3 phần tử
}

fn main() {
    let prices = vec![35.0, 25.0, 45.0, 40.0, 30.0];
    let (avg, min, max) = stats(&prices);  // destructure
    println!("Avg: {:.1}, Min: {:.1}, Max: {:.1}", avg, min, max);
    // Output: Avg: 35.0, Min: 25.0, Max: 45.0
}
