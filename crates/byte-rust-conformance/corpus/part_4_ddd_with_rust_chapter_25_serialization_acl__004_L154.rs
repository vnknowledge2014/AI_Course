// filename: src/main.rs

// ═══ DOMAIN (Hoàn toàn Tinh khiết, Không có tí Serde nào) ═══
mod domain {
    #[derive(Debug, Clone)]
    pub struct Email(String);
    impl Email {
        pub fn new(value: &str) -> Result<Self, String> {
            if value.contains('@') { Ok(Email(value.to_lowercase())) }
            else { Err("Invalid email".into()) }
        }
        pub fn value(&self) -> &str { &self.0 }
    }

    #[derive(Debug, Clone)]
    pub struct Order {
        pub id: u64,
        pub customer_email: Email, // Dùng Value Object xịn
    }
}

fn main() {}
