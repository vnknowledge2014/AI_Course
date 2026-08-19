// filename: src/main.rs

mod shapes {
    pub fn circle_area(radius: f64) -> f64 {
        std::f64::consts::PI * radius * radius
    }

    pub fn rect_area(width: f64, height: f64) -> f64 {
        width * height
    }
}

// use = import tên vào scope hiện tại
use shapes::circle_area;
use shapes::rect_area;

// Hoặc gom:
// use shapes::{circle_area, rect_area};

// Hoặc tất cả (cẩn thận — dễ name clash):
// use shapes::*;

fn main() {
    // Không cần shapes:: prefix nữa
    println!("Circle: {:.2}", circle_area(5.0));
    println!("Rect: {:.2}", rect_area(3.0, 4.0));
}
