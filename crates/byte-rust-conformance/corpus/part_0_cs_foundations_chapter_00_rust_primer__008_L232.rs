#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let p = Point { x: 1.0, y: 2.5 };
    println!("Point: ({}, {})", p.x, p.y);
    // Output: Point: (1, 2.5)
}
