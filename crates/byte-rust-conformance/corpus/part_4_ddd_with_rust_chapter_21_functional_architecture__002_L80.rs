// filename: src/main.rs

// ═══════════════════════════════════════════
// LAYER 1: DOMAIN (innermost) — PURE, no IO
// ═══════════════════════════════════════════
mod domain {
    #[derive(Debug, Clone, PartialEq)]
    pub struct Email(String);

    impl Email {
        pub fn new(value: &str) -> Result<Self, String> {
            if value.contains('@') && value.len() >= 5 {
                Ok(Email(value.to_lowercase()))
            } else {
                Err(format!("Invalid email: {}", value))
            }
        }
        pub fn value(&self) -> &str { &self.0 }
    }

    #[derive(Debug, Clone)]
    pub struct User {
        pub id: u64,
        pub name: String,
        pub email: Email,
    }

    // Pure function — Logic nghiệp vụ kiểm tra tính hợp lệ
    pub fn validate_registration(name: &str, email: &str) -> Result<(String, Email), Vec<String>> {
        let mut errors = vec![];

        if name.trim().len() < 2 {
            errors.push("Name must be at least 2 characters".into());
        }

        let email = match Email::new(email) {
            Ok(e) => Some(e),
            Err(e) => { errors.push(e); None }
        };

        if errors.is_empty() {
            Ok((name.trim().to_string(), email.unwrap()))
        } else {
            Err(errors)
        }
    }
}

fn main() {}
