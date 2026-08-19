// filename: src/main.rs

fn binary_search(sorted: &[i32], target: i32) -> Option<usize> {
    let mut low = 0;
    let mut high = sorted.len();

    while low < high {
        let mid = low + (high - low) / 2;  // tránh overflow

        if sorted[mid] == target {
            return Some(mid);               // Tìm thấy!
        } else if sorted[mid] < target {
            low = mid + 1;                  // Bỏ nửa trái
        } else {
            high = mid;                     // Bỏ nửa phải
        }
    }

    None  // Không tìm thấy
}

// Phiên bản recursive (để thấy rõ divide & conquer)
fn binary_search_recursive(sorted: &[i32], target: i32, low: usize, high: usize) -> Option<usize> {
    if low >= high {
        return None;  // Base case: không còn gì để tìm
    }

    let mid = low + (high - low) / 2;

    if sorted[mid] == target {
        Some(mid)
    } else if sorted[mid] < target {
        binary_search_recursive(sorted, target, mid + 1, high) // Tìm nửa phải
    } else {
        binary_search_recursive(sorted, target, low, mid) // Tìm nửa trái
    }
}

fn main() {
    let data = vec![2, 5, 8, 12, 16, 23, 38, 56, 72, 91];

    // Tìm thấy
    println!("Find 23: {:?}", binary_search(&data, 23));
    println!("Find 23 (recursive): {:?}",
        binary_search_recursive(&data, 23, 0, data.len()));

    // Không tìm thấy
    println!("Find 42: {:?}", binary_search(&data, 42));

    // So sánh: 10 phần tử
    // Linear search: tối đa 10 bước
    // Binary search: tối đa log2(10) ≈ 4 bước
    println!("\n10 items: linear max 10 steps, binary max {} steps",
        (10_f64).log2().ceil() as u32);

    // 1 triệu phần tử
    // Linear: tối đa 1,000,000 bước
    // Binary: tối đa 20 bước!
    println!("1M items: linear max 1000000 steps, binary max {} steps",
        (1_000_000_f64).log2().ceil() as u32);

    // Output:
    // Find 23: Some(5)
    // Find 23 (recursive): Some(5)
    // Find 42: None
    //
    // 10 items: linear max 10 steps, binary max 4 steps
    // 1M items: linear max 1000000 steps, binary max 20 steps
}
