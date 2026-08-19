// filename: src/main.rs

// O(1) — Constant: luôn 1 bước, bất kể data lớn cỡ nào
fn get_first(items: &[i32]) -> Option<&i32> {
    items.first()  // Chỉ lấy phần tử đầu — không cần duyệt
}

// O(n) — Linear: số bước tỉ lệ với data
fn find_max(items: &[i32]) -> Option<i32> {
    if items.is_empty() {
        return None;
    }
    let mut max = items[0];
    for &item in &items[1..] {  // Duyệt từng phần tử — n bước
        if item > max {
            max = item;
        }
    }
    Some(max)
}

// O(n²) — Quadratic: mỗi phần tử so với mọi phần tử khác
fn has_duplicates(items: &[i32]) -> bool {
    for i in 0..items.len() {
        for j in (i + 1)..items.len() {  // Vòng lặp lồng → n × n
            if items[i] == items[j] {
                return true;
            }
        }
    }
    false
}

fn main() {
    let data = vec![3, 7, 1, 9, 4, 7];

    println!("First: {:?}", get_first(&data));         // O(1)
    println!("Max: {:?}", find_max(&data));             // O(n)
    println!("Has duplicates: {}", has_duplicates(&data)); // O(n²)

    // Output:
    // First: Some(3)
    // Max: Some(9)
    // Has duplicates: true
}
