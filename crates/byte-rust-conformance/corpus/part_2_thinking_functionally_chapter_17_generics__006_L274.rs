// filename: src/main.rs

fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

fn main() {
    let x = add(1_i32, 2);    // compiler tạo: fn add_i32(a: i32, b: i32) -> i32
    let y = add(1.0_f64, 2.0); // compiler tạo: fn add_f64(a: f64, b: f64) -> f64
    println!("{} {}", x, y);
}

// Sau compilation, code GIỐNG NHƯ bạn viết tay từng function!
// Không có vtable, không có boxing, không có runtime dispatch.
// → ZERO COST
