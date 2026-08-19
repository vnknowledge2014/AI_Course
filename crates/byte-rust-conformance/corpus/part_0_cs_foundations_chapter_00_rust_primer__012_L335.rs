fn main() {
    // Sử dụng macro vec! để khởi tạo nhanh
    let fruits = vec!["🍎", "🍊", "🍇"];

    println!("First: {}", fruits[0]);     // Lấy phần tử đầu tiên
    println!("Length: {}", fruits.len()); // Lấy số lượng phần tử

    // Duyệt qua từng phần tử một cách thanh lịch
    for fruit in &fruits {
        println!("- {}", fruit);
    }
}
