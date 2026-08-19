// filename: src/main.rs

fn merge_sort_strings(items: &[String]) -> Vec<String> {
    if items.len() <= 1 {
        return items.to_vec();
    }
    let mid = items.len() / 2;
    let left = merge_sort_strings(&items[..mid]);
    let right = merge_sort_strings(&items[mid..]);
    merge_strings(&left, &right)
}

fn merge_strings(left: &[String], right: &[String]) -> Vec<String> {
    let mut result = Vec::with_capacity(left.len() + right.len());
    let (mut i, mut j) = (0, 0);

    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            result.push(left[i].clone());
            i += 1;
        } else {
            result.push(right[j].clone());
            j += 1;
        }
    }
    for item in &left[i..] { result.push(item.clone()); }
    for item in &right[j..] { result.push(item.clone()); }
    result
}

fn main() {
    let names: Vec<String> = vec!["Rust", "Go", "Zig", "Roc", "Elm"]
        .into_iter().map(String::from).collect();

    println!("Before: {:?}", names);
    let sorted = merge_sort_strings(&names);
    println!("After:  {:?}", sorted);
    // Output:
    // Before: ["Rust", "Go", "Zig", "Roc", "Elm"]
    // After:  ["Elm", "Go", "Roc", "Rust", "Zig"]
}
