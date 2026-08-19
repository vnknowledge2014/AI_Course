// filename: src/equality.rs

// Derive tự động tạo code kiểm tra từng field một từ trên xuống dưới.
#[derive(PartialEq, Eq, Debug)]
struct User {
    id: u32,
    name: String,
}

fn main() {
    let u1 = User { id: 1, name: String::from("Alice") };
    let u2 = User { id: 1, name: String::from("Alice") };
    let u3 = User { id: 2, name: String::from("Bob") };

    // So sánh cấu trúc (Structural Equality) hoạt động ngay lập tức!
    assert!(u1 == u2);
    assert!(u1 != u3);
    
    // Lưu ý về f32:
    #[derive(PartialEq)] // KHÔNG THỂ derive Eq vì có field f32!
    struct Point {
        x: f32,
        y: f32,
    }
    
    let p1 = Point { x: f32::NAN, y: 1.0 };
    assert!(p1 != p1); // Tính phản xạ bị phá vỡ!
}
