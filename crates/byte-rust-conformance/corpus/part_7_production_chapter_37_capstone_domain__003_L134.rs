/// Product Code — Định dạng khắt khe
#[derive(Debug, Clone, PartialEq)]
enum ProductCode {
    Widget(String),  // "W" + 4 digits
    Gizmo(String),   // "G" + 3 digits
}

impl ProductCode {
    fn new(code: &str) -> Result<Self, String> {
        match code.chars().next() {
            Some('W') if code.len() == 5 && code[1..].chars().all(|c| c.is_ascii_digit()) =>
                Ok(ProductCode::Widget(code.into())),
            Some('G') if code.len() == 4 && code[1..].chars().all(|c| c.is_ascii_digit()) =>
                Ok(ProductCode::Gizmo(code.into())),
            _ => Err(format!("Invalid product code: {}", code)),
        }
    }
}

fn main() {}
