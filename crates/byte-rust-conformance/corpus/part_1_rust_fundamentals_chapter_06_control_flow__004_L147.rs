// filename: src/main.rs

#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        // Tách radius ra từ Circle
        Shape::Circle(r) => std::f64::consts::PI * r * r,

        // Tách width, height ra từ Rectangle
        Shape::Rectangle(w, h) => w * h,

        // Tách named fields từ Triangle
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}

fn describe(shape: &Shape) -> String {
    match shape {
        Shape::Circle(r) => format!("⭕ Circle r={:.1}", r),
        Shape::Rectangle(w, h) => format!("🟦 Rect {}×{}", w, h),
        Shape::Triangle { .. } => "🔺 Triangle".to_string(),
        //                 ^^ bỏ qua fields không cần
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle(5.0),
        Shape::Rectangle(3.0, 4.0),
        Shape::Triangle { base: 6.0, height: 3.0 },
    ];

    for s in &shapes {
        println!("{}: area = {:.2}", describe(s), area(s));
    }
    // Output:
    // ⭕ Circle r=5.0: area = 78.54
    // 🟦 Rect 3×4: area = 12.00
    // 🔺 Triangle: area = 9.00
}
