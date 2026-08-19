// Ví dụ kinh điển: Đèn giao thông chỉ có thể là Đỏ, Vàng, hoặc Xanh
#[derive(Debug)] // Dòng này báo cho Rust biết hãy tự động tạo code để có thể in struct/enum này ra màn hình
enum TrafficLight {
    Red,
    Yellow,
    Green,
}

fn main() {
    let light = TrafficLight::Red;
    println!("Light: {:?}", light);
    // Output: Light: Red
}
