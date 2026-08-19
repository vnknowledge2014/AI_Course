// filename: src/main.rs

// "Máy lọc" — nhận predicate (function trả bool)
fn filter_items<T, F>(items: &[T], predicate: F) -> Vec<&T>
where
    F: Fn(&T) -> bool,
{
    items.iter().filter(|item| predicate(item)).collect()
}

// "Máy biến đổi" — nhận transform function
fn transform_items<T, U, F>(items: &[T], transform: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    items.iter().map(|item| transform(item)).collect()
}

fn main() {
    let prices = vec![35_000, 25_000, 45_000, 15_000, 55_000];

    // Lọc: chỉ lấy giá >= 30k
    let premium = filter_items(&prices, |&p| p >= 30_000);
    println!("Premium: {:?}", premium);

    // Biến đổi: format thành chuỗi
    let labels = transform_items(&prices, |&p| format!("{}đ", p));
    println!("Labels: {:?}", labels);

    // Output:
    // Premium: [35000, 45000, 55000]
    // Labels: ["35000đ", "25000đ", "45000đ", "15000đ", "55000đ"]
}
