// filename: src/main.rs

// ═══ VALUE OBJECTS ═══

/// Order ID — Không được rỗng, không quá dài
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct OrderId(String);

impl OrderId {
    fn new(id: &str) -> Result<Self, String> {
        let trimmed = id.trim();
        if trimmed.is_empty() { return Err("OrderId cannot be empty".into()); }
        if trimmed.len() > 50 { return Err("OrderId too long".into()); }
        Ok(OrderId(trimmed.into()))
    }
}

/// Customer Email — Phải đúng định dạng cơ bản
#[derive(Debug, Clone, PartialEq)]
struct EmailAddress(String);

impl EmailAddress {
    fn new(email: &str) -> Result<Self, String> {
        let trimmed = email.trim().to_lowercase();
        if !trimmed.contains('@') || trimmed.len() < 5 {
            return Err(format!("Invalid email: {}", email));
        }
        Ok(EmailAddress(trimmed))
    }
}

fn main() {}
