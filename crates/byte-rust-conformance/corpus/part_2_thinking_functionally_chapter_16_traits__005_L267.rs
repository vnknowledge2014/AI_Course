// filename: src/main.rs

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle { radius: f64 }
struct Rectangle { w: f64, h: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { std::f64::consts::PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.w * self.h }
    fn name(&self) -> &str { "Rectangle" }
}

// Static dispatch: compiler tạo bản riêng cho mỗi type
// Nhanh (inline) nhưng code size lớn nếu nhiều types
fn print_area_static(shape: &impl Shape) {
    println!("{}: {:.2}", shape.name(), shape.area());
}

// Dynamic dispatch: dùng vtable (bảng function pointers) lúc runtime
// Linh hoạt, lưu được trong collections, nhưng có overhead nhỏ
fn print_area_dynamic(shape: &dyn Shape) {
    println!("{}: {:.2}", shape.name(), shape.area());
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { w: 3.0, h: 4.0 };

    // Static dispatch
    print_area_static(&c);
    print_area_static(&r);

    // Dynamic dispatch — cho phép lưu trong VEC!
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 10.0 }),
        Box::new(Rectangle { w: 5.0, h: 8.0 }),
        Box::new(Circle { radius: 3.0 }),
    ];

    println!("\nAll shapes:");
    for shape in &shapes {
        print_area_dynamic(shape.as_ref());
    }

    let total: f64 = shapes.iter().map(|s| s.area()).sum();
    println!("Total area: {:.2}", total);
}
