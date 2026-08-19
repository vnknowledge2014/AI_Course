// filename: src/main.rs
use std::collections::HashSet;

// O(n²) — cách "ngây thơ": 2 vòng lặp
fn has_pair_sum_naive(items: &[i32], target: i32) -> bool {
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {
            if items[i] + items[j] == target {
                return true;
            }
        }
    }
    false
}

// O(n) — cách thông minh: HashSet
fn has_pair_sum_hash(items: &[i32], target: i32) -> bool {
    let mut seen = HashSet::new();
    for &item in items {
        let complement = target - item;
        if seen.contains(&complement) {
            return true;
        }
        seen.insert(item);
    }
    false
}

// O(n) — Two pointers: yêu cầu mảng đã sorted
fn has_pair_sum_two_pointers(sorted: &[i32], target: i32) -> bool {
    if sorted.len() < 2 {
        return false;
    }
    let mut left = 0;
    let mut right = sorted.len() - 1;

    while left < right {
        let sum = sorted[left] + sorted[right];
        if sum == target {
            return true;
        } else if sum < target {
            left += 1;      // Tổng nhỏ quá → dịch con trỏ trái sang phải
        } else {
            right -= 1;     // Tổng lớn quá → dịch con trỏ phải sang trái
        }
    }
    false
}

fn main() {
    let data = vec![2, 5, 8, 12, 16, 23, 38];

    // Tìm cặp có tổng = 28: 5 + 23 = 28 ✓
    println!("Pair sum 28 (naive): {}", has_pair_sum_naive(&data, 28));
    println!("Pair sum 28 (hash): {}", has_pair_sum_hash(&data, 28));
    println!("Pair sum 28 (two pointers): {}", has_pair_sum_two_pointers(&data, 28));

    // Tìm cặp có tổng = 20: 8 + 12 = 20 ✓
    println!("Pair sum 20: {}", has_pair_sum_two_pointers(&data, 20));

    // Output:
    // Pair sum 28 (naive): true
    // Pair sum 28 (hash): true
    // Pair sum 28 (two pointers): true
    // Pair sum 20: true
}
