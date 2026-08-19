// filename: src/main.rs

// Trả về tuple — cách Rust trả nhiều giá trị
fn min_max(items: &[i32]) -> (i32, i32) {
    let mut min = items[0];
    let mut max = items[0];
    for &item in &items[1..] {
        if item < min { min = item; }
        if item > max { max = item; }
    }
    (min, max)
}

fn main() {
    let data = vec![3, 7, 1, 9, 4];
    let (min, max) = min_max(&data);
    println!("Min: {}, Max: {}", min, max);
    // Output: Min: 1, Max: 9
}
