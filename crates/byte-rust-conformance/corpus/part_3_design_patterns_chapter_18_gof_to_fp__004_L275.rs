// filename: src/main.rs

#[derive(Debug)]
enum Shape {
    Circle { radius: f64 },
    Rectangle { w: f64, h: f64 },
    Triangle { base: f64, height: f64 },
}

// "Visitor" = function với match. Thêm "visitor" = thêm function.
fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle { radius } => std::f64::consts::PI * radius * radius,
        Shape::Rectangle { w, h } => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
    }
}

fn svg(shape: &Shape) -> String {
    match shape {
        Shape::Circle { radius } =>
            format!("<circle r=\"{}\" />", radius),
        Shape::Rectangle { w, h } =>
            format!("<rect width=\"{}\" height=\"{}\" />", w, h),
        Shape::Triangle { base, height } =>
            format!("<polygon points=\"0,{} {},{} {},{}\" />", height, base / 2.0, 0, *base, *height),
    }
}

fn description(shape: &Shape) -> String {
    match shape {
        Shape::Circle { radius } => format!("Circle with radius {:.1}", radius),
        Shape::Rectangle { w, h } => format!("Rectangle {:.1}×{:.1}", w, h),
        Shape::Triangle { base, height } => format!("Triangle base={:.1} h={:.1}", base, height),
    }
}

fn main() {
    let shapes = vec![
        Shape::Circle { radius: 5.0 },
        Shape::Rectangle { w: 4.0, h: 6.0 },
        Shape::Triangle { base: 3.0, height: 4.0 },
    ];

    for s in &shapes {
        println!("{} → area={:.2} → {}", description(s), area(s), svg(s));
    }
}
