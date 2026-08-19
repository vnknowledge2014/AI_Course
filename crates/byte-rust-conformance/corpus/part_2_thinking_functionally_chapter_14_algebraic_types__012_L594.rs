// filename: src/main.rs

#[derive(Debug, Clone)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { width: f64, height: f64 },
    Triangle { base: f64, height: f64 },
}

impl Shape {
    // Methods cho tất cả variants
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
            Shape::Rectangle { width, height } => width * height,
            Shape::Triangle { base, height } => 0.5 * base * height,
        }
    }

    fn perimeter(&self) -> f64 {
        match self {
            Shape::Circle { radius } => 2.0 * std::f64::consts::PI * radius,
            Shape::Rectangle { width, height } => 2.0 * (width + height),
            Shape::Triangle { base, height } => {
                let hyp = (base * base + height * height).sqrt();
                base + height + hyp
            }
        }
    }

    // Functional transform: scale trả shape MỚI
    fn scale(&self, factor: f64) -> Self {
        match self {
            Shape::Circle { radius } => Shape::Circle { radius: radius * factor },
            Shape::Rectangle { width, height } =>
                Shape::Rectangle { width: width * factor, height: height * factor },
            Shape::Triangle { base, height } =>
                Shape::Triangle { base: base * factor, height: height * factor },
        }
    }

    fn describe(&self) -> String {
        match self {
            Shape::Circle { radius } => format!("⭕ Circle(r={:.1})", radius),
            Shape::Rectangle { width, height } => format!("🟦 Rect({:.1}×{:.1})", width, height),
            Shape::Triangle { base, height } => format!("🔺 Tri({:.1}×{:.1})", base, height),
        }
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { width: 4.0, height: 6.0 },
        Shape::Triangle { base: 3.0, height: 4.0 },
    ];

    println!("{:<25} {:>10} {:>10}", "Shape", "Area", "Perimeter");
    println!("{}", "-".repeat(47));
    for shape in &shapes {
        println!("{:<25} {:>10.2} {:>10.2}", shape.describe(), shape.area(), shape.perimeter());
    }

    // Scale tất cả ×2
    println!("\n📐 After scaling ×2:");
    for shape in &shapes {
        let scaled = shape.scale(2.0);
        println!("  {} → area {:.2}", scaled.describe(), scaled.area());
    }
}
