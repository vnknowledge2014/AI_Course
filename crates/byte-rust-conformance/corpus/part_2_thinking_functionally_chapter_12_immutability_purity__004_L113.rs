// filename: src/main.rs

// const: giá trị compile-time, inline vào mọi nơi dùng
const MAX_RETRIES: u32 = 3;
const PI: f64 = 3.14159265358979;

// static: giống const nhưng có địa chỉ bộ nhớ cố định
static APP_VERSION: &str = "1.0.0";

fn main() {
    // let: giá trị runtime, immutable trong scope
    let config_port = std::env::var("PORT")
        .unwrap_or("8080".to_string());

    println!("Max retries: {}", MAX_RETRIES);
    println!("App: {} on port {}", APP_VERSION, config_port);
}
