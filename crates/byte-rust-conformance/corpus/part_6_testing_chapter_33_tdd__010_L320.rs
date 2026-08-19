// filename: src/lib.rs

pub fn parse_age(input: &str) -> Result<u32, String> {
    let trimmed = input.trim();
    if trimmed.is_empty() { return Err("Age is required".into()); }
    
    let age: u32 = trimmed.parse().map_err(|_| format!("'{}' is not a number", trimmed))?;
    
    if age < 1 || age > 150 { return Err(format!("Age {} out of range 1-150", age)); }
    Ok(age)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Happy paths
    #[test] fn parse_age_valid() { assert_eq!(parse_age("25"), Ok(25)); }
    #[test] fn parse_age_with_spaces() { assert_eq!(parse_age("  30  "), Ok(30)); }
    
    // Boundary conditions (Kiểm tra biên)
    #[test] fn parse_age_boundary_low() { assert_eq!(parse_age("1"), Ok(1)); }
    #[test] fn parse_age_boundary_high() { assert_eq!(parse_age("150"), Ok(150)); }

    // Error paths (Cố tình phá hoại)
    #[test] fn parse_age_empty() { assert!(parse_age("").is_err()); }
    #[test] fn parse_age_not_number() { assert!(parse_age("abc").is_err()); }
    #[test] fn parse_age_negative() { assert!(parse_age("-5").is_err()); }
    #[test] fn parse_age_zero() { assert!(parse_age("0").is_err()); }
    #[test] fn parse_age_too_high() { assert!(parse_age("151").is_err()); }
    #[test] fn parse_age_float() { assert!(parse_age("25.5").is_err()); }
}

fn main() {}
