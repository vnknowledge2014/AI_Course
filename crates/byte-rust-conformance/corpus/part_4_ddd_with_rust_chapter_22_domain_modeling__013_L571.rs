#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct AccountNumber(String);
impl AccountNumber {
    fn new(value: &str) -> Result<Self, String> {
        if value.len() == 10 && value.chars().all(|c| c.is_ascii_digit()) {
            Ok(AccountNumber(value.into()))
        } else { Err("Must be exactly 10 digits".into()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct PositiveAmount(f64);
impl PositiveAmount {
    fn new(value: f64) -> Result<Self, String> {
        if value > 0.0 { Ok(PositiveAmount(value)) }
        else { Err("Must be positive".into()) }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Currency { VND, USD, EUR }

fn main() {}
