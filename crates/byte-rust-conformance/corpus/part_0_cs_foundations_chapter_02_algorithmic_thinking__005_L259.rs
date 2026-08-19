// filename: src/main.rs

// Tính tổng: cách FP (không cần mut)
fn sum_recursive(items: &[i32]) -> i32 {
    match items {
        [] => 0,                              // mảng rỗng → tổng = 0
        [first, rest @ ..] => first + sum_recursive(rest), // head + sum(tail)
    }
}

// Tính tổng: cách idiomatic Rust (dùng iterator — cũng không cần mut!)
fn sum_iter(items: &[i32]) -> i32 {
    items.iter().sum()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5];

    assert_eq!(sum_recursive(&data), 15);
    assert_eq!(sum_iter(&data), 15);

    println!("Sum (recursive): {}", sum_recursive(&data));
    println!("Sum (iterator): {}", sum_iter(&data));
    // Output:
    // Sum (recursive): 15
    // Sum (iterator): 15
}
