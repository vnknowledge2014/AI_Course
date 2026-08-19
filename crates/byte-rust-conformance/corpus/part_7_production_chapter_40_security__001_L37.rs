// Mô phỏng cách Argon2 hoạt động (Chỉ dùng để học, Production hãy dùng crate `argon2`)
mod password_security {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::SystemTime;

    // 1. Tạo Salt ngẫu nhiên
    fn generate_salt() -> String {
        let nanos = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos();
        format!("{:x}", nanos)
    }

    // 2. Hàm Hash chậm (Mô phỏng)
    pub fn hash_password(password: &str) -> String {
        let salt = generate_salt();
        let mut hasher = DefaultHasher::new();
        
        // Trộn Password + Salt
        format!("{}:{}", salt, password).hash(&mut hasher);
        let hash_result = format!("{:016x}", hasher.finish());
        
        // Chuỗi lưu vào DB: Thuật toán $ Salt $ Hash
        format!("$argon2_sim${}${}", salt, hash_result)
    }

    // 3. Hàm Verify
    pub fn verify_password(input_password: &str, db_hash: &str) -> bool {
        let parts: Vec<&str> = db_hash.split('$').collect();
        if parts.len() != 4 { return false; }
        
        let salt = parts[2];
        let original_hash = parts[3];
        
        let mut hasher = DefaultHasher::new();
        format!("{}:{}", salt, input_password).hash(&mut hasher);
        let current_hash = format!("{:016x}", hasher.finish());
        
        current_hash == original_hash
    }
}

fn main() {}
