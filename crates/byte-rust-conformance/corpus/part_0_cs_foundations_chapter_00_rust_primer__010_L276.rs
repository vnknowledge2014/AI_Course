#[derive(Debug)]
enum Shape {
    Circle(f64),
    Rectangle { width: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        // Bóc tách biến 'radius' ra khỏi khối Circle
        Shape::Circle(radius) => std::f64::consts::PI * radius * radius,
        
        // Bóc tách 'width' và 'height' ra khỏi khối Rectangle
        Shape::Rectangle { width, height } => width * height,
    }
}

fn main() {
    let c = Shape::Circle(5.0);
    let r = Shape::Rectangle { width: 3.0, height: 4.0 };

    println!("Circle area: {:.2}", area(&c));
    println!("Rect area: {:.2}", area(&r));
}
