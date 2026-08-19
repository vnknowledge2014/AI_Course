// ✅ Whitelist: Chỉ cho phép chữ cái, số, và dấu gạch dưới. Giới hạn độ dài 3-30.
fn validate_username(input: &str) -> Result<String, String> {
    let trimmed = input.trim();
    if trimmed.len() < 3 || trimmed.len() > 30 {
        return Err("Username phải từ 3-30 ký tự".into());
    }
    
    // Nếu có 1 ký tự nào KHÔNG nằm trong danh sách trắng, lập tức reject!
    if !trimmed.chars().all(|c| c.is_alphanumeric() || c == '_') {
        return Err("Username chỉ được chứa chữ, số, và dấu _".into());
    }
    
    Ok(trimmed.to_string())
}

fn main() {}
