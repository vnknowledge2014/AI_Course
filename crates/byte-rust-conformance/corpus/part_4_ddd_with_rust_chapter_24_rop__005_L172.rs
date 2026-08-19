// filename: src/main.rs

/// Validated: Hoặc là Hợp lệ Valid(T), hoặc là chứa Một Rổ Lỗi Invalid(Vec<Lỗi>)
#[derive(Debug, Clone)]
enum Validated<T> {
    Valid(T),
    Invalid(Vec<String>), // Luôn là một mảng!
}

fn main() {}
