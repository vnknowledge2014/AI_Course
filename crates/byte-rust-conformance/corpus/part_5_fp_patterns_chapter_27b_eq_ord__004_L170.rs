// filename: src/sort_by.rs

#[derive(Debug)]
struct Product {
    name: String,
    price: u32,
}

fn main() {
    let mut catalog = vec![
        Product { name: String::from("Laptop"), price: 1000 },
        Product { name: String::from("Mouse"), price: 50 },
        Product { name: String::from("Keyboard"), price: 100 },
    ];

    // Sắp xếp theo giá (Price) - Truyền closure lấy ra key
    catalog.sort_by_key(|p| p.price);
    
    // Đảo ngược (Giảm dần) - dùng Reverse modifier
    use std::cmp::Reverse;
    catalog.sort_by_key(|p| Reverse(p.price));
    
    // Sắp xếp nhiều tiêu chí (Multi-criteria) giống Order.combine!
    // Trả về một Tuple, Rust sẽ so sánh Tuple từ trái qua phải.
    // VD: Giá giảm dần. Nếu trùng giá thì Tên tăng dần.
    catalog.sort_by_key(|p| (Reverse(p.price), p.name.clone()));
}
