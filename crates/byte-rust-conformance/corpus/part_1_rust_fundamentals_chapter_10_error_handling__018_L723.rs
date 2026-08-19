// filename: src/main.rs
use std::fmt;

#[derive(Debug)]
enum CalcError {
    DivisionByZero,
    Overflow,
}

impl fmt::Display for CalcError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CalcError::DivisionByZero => write!(f, "Division by zero"),
            CalcError::Overflow => write!(f, "Overflow"),
        }
    }
}

fn safe_add(a: f64, b: f64) -> Result<f64, CalcError> {
    let result = a + b;
    if result.is_infinite() { Err(CalcError::Overflow) } else { Ok(result) }
}

fn safe_sub(a: f64, b: f64) -> Result<f64, CalcError> {
    let result = a - b;
    if result.is_infinite() { Err(CalcError::Overflow) } else { Ok(result) }
}

fn safe_mul(a: f64, b: f64) -> Result<f64, CalcError> {
    let result = a * b;
    if result.is_infinite() { Err(CalcError::Overflow) } else { Ok(result) }
}

fn safe_div(a: f64, b: f64) -> Result<f64, CalcError> {
    if b == 0.0 { Err(CalcError::DivisionByZero) } else { Ok(a / b) }
}

fn main() {
    // (10 + 5) * 3 / 2 = 22.5
    let result = safe_add(10.0, 5.0)
        .and_then(|r| safe_mul(r, 3.0))
        .and_then(|r| safe_div(r, 2.0));
    println!("(10+5)*3/2 = {:?}", result);  // Ok(22.5)

    // (10 + 5) * 3 / 0 → error dừng tại div
    let result = safe_add(10.0, 5.0)
        .and_then(|r| safe_mul(r, 3.0))
        .and_then(|r| safe_div(r, 0.0));
    println!("(10+5)*3/0 = {:?}", result);  // Err(DivisionByZero)
}
