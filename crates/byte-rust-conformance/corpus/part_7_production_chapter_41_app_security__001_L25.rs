fn find_user_bad(email: &str) -> String {
    // ❌ LỖ HỔNG CHẾT NGƯỜI: Nối chuỗi (String Interpolation) trực tiếp vào SQL
    format!("SELECT * FROM users WHERE email = '{}'", email)
}

fn main() {}
