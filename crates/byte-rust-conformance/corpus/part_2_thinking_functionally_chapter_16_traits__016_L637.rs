trait Drawable {
    fn draw(&self) -> String;
    fn bounding_box(&self) -> (f64, f64) { (0.0, 0.0) }
}

struct Circle { x: f64, y: f64, radius: f64 }
struct Rect { x: f64, y: f64, w: f64, h: f64 }

impl Drawable for Circle {
    fn draw(&self) -> String { format!("⭕ at ({:.0},{:.0}) r={:.0}", self.x, self.y, self.radius) }
    fn bounding_box(&self) -> (f64, f64) { (self.radius * 2.0, self.radius * 2.0) }
}

impl Drawable for Rect {
    fn draw(&self) -> String { format!("🟦 at ({:.0},{:.0}) {:.0}×{:.0}", self.x, self.y, self.w, self.h) }
    fn bounding_box(&self) -> (f64, f64) { (self.w, self.h) }
}

fn main() {
    let canvas: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { x: 10.0, y: 20.0, radius: 5.0 }),
        Box::new(Rect { x: 0.0, y: 0.0, w: 100.0, h: 50.0 }),
        Box::new(Circle { x: 50.0, y: 50.0, radius: 15.0 }),
    ];
    for shape in &canvas {
        let (w, h) = shape.bounding_box();
        println!("{} [bbox: {:.0}×{:.0}]", shape.draw(), w, h);
    }
}
