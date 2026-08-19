// filename: src/main.rs

#[derive(Debug)]
enum AppError { Validation(String), Database(String) }

fn parse_id(input: &str) -> Result<u64, AppError> {
    input.parse::<u64>()
        // Ép lỗi của hệ thống Parse thành AppError::Validation
        .map_err(|e| AppError::Validation(format!("Invalid ID '{}': {}", input, e)))
}

fn find_user(id: u64) -> Result<String, AppError> {
    if id == 42 { Ok("Minh".into()) }
    else { Err(AppError::Database(format!("User {} not found", id))) }
}

fn main() {
    // Pipeline rẽ Lỗi cực mượt
    let result = parse_id("abc").and_then(find_user);
    
    match result {
        Ok(name) => println!("Hello {}", name),
        Err(AppError::Validation(msg)) => println!("📝 Mời nhập lại cho đúng: {}", msg),
        Err(AppError::Database(msg)) => println!("💾 Báo IT kiểm tra DB: {}", msg),
    }
}
