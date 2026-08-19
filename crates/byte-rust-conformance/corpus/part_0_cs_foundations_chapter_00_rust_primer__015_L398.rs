fn find_even(numbers: &[i32]) -> Option<i32> {
    // Thử tìm số chẵn đầu tiên
    for &n in numbers {
        if n % 2 == 0 {
            return Some(n);  // Nếu thấy, gói nó trong hộp `Some()`
        }
    }
    None  // Nếu tìm hoài không thấy, trả về hộp rỗng `None`
}

fn main() {
    match find_even(&[1, 3, 4, 7]) {
        Some(n) => println!("Found even: {}", n),
        None    => println!("No even number"),
    }
}
