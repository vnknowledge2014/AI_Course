// ═══ ERRORS ═══
#[derive(Debug)]
enum OrderError {
    Validation(Vec<String>), // Có thể gom nhiều lỗi validation cùng lúc
    Pricing(String),
    Confirmation(String),
}

fn main() {}
