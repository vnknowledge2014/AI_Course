// filename: src/main.rs
use std::fmt;

// ═══════ Value Objects ═══════

/// Email — validated, normalized, immutable
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String); // Trường này là private! Không có chữ `pub`

impl Email {
    /// Smart constructor: validate + normalize
    pub fn new(value: &str) -> Result<Self, String> {
        let trimmed = value.trim().to_lowercase();
        
        if !trimmed.contains('@') {
            return Err(format!("Email missing @: '{}'", value));
        }
        
        let parts: Vec<&str> = trimmed.split('@').collect();
        if parts.len() != 2 || parts[0].is_empty() || parts[1].len() < 3 {
            return Err(format!("Invalid email format: '{}'", value));
        }
        
        Ok(Email(trimmed))
    }

    pub fn value(&self) -> &str { &self.0 }
    
    pub fn domain(&self) -> &str {
        self.0.split('@').nth(1).unwrap_or("")
    }
}

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn main() {}
