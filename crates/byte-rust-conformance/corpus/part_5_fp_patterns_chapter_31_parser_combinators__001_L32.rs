// filename: src/main.rs

// Parser type: ăn &str, trả (parsed_value, remaining_input) hoặc Lỗi
type ParseResult<'a, T> = Result<(T, &'a str), String>;

fn main() {}
