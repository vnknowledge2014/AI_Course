// filename: src/main.rs

fn find_first_negative(items: &[i32]) -> Option<i32> {
    for &item in items {
        if item < 0 {
            return Some(item);  // thoát sớm — tìm thấy rồi, không cần duyệt tiếp
        }
    }
    None  // dòng cuối, không có ; → return None
}

fn main() {
    let data = vec![3, 7, -2, 9, -5];
    println!("First negative: {:?}", find_first_negative(&data));
    println!("No negatives: {:?}", find_first_negative(&[1, 2, 3]));
    // Output:
    // First negative: Some(-2)
    // No negatives: None
}
