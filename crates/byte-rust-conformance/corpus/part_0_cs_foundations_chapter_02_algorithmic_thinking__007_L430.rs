// filename: src/main.rs

fn merge_sort(items: &[i32]) -> Vec<i32> {
    // Base case: mảng 0 hoặc 1 phần tử đã sorted
    if items.len() <= 1 {
        return items.to_vec();
    }

    // Divide: chia đôi
    let mid = items.len() / 2;
    let left = merge_sort(&items[..mid]);   // sort nửa trái
    let right = merge_sort(&items[mid..]);  // sort nửa phải

    // Combine: merge 2 nửa đã sorted
    merge(&left, &right)
}

fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let mut i = 0;
    let mut j = 0;

    // So sánh từng cặp, lấy cái nhỏ hơn
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i]);
            i += 1;
        } else {
            result.push(right[j]);
            j += 1;
        }
    }

    // Phần còn lại
    result.extend_from_slice(&left[i..]);
    result.extend_from_slice(&right[j..]);
    result
}

fn main() {
    let data = vec![38, 27, 43, 3, 9, 82, 10];

    println!("Before: {:?}", data);
    let sorted = merge_sort(&data);
    println!("After:  {:?}", sorted);

    assert_eq!(sorted, vec![3, 9, 10, 27, 38, 43, 82]);

    // Output:
    // Before: [38, 27, 43, 3, 9, 82, 10]
    // After:  [3, 9, 10, 27, 38, 43, 82]
}
