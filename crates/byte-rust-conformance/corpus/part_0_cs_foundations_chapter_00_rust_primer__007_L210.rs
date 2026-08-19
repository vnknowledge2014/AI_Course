#[derive(Debug)]
enum Shape {
    Circle(f64),                           // Hình tròn lưu giữ bán kính
    Rectangle { width: f64, height: f64 }, // Hình chữ nhật cần cả chiều rộng và cao
}

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle { width: 3.0, height: 4.0 };
    
    println!("{:?}", c);  // Output: Circle(5.0)
    println!("{:?}", r);  // Output: Rectangle { width: 3.0, height: 4.0 }
}
