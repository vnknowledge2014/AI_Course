// filename: src/main.rs
use std::fmt;

// 1. Các lỗi Cấp Thấp (Validation)
#[derive(Debug, Clone)]
enum ValidationError {
    FieldRequired(String),
    FieldTooShort { field: String, min: usize, actual: usize },
    OutOfRange { field: String, min: String, max: String, actual: String },
}

// 2. Các lỗi Cấp Cao (Domain / Business)
#[derive(Debug)]
enum DomainError {
    // Bao bọc các lỗi Cấp thấp vào bên trong
    Validation(Vec<ValidationError>),
    BusinessRule(String),
    NotFound { entity: String, id: String },
}

fn main() {}
