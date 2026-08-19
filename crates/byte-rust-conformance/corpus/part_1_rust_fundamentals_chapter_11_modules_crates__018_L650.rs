// filename: src/convert.rs
pub fn celsius_to_fahrenheit(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

pub fn km_to_miles(km: f64) -> f64 {
    km * 0.621371
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_boiling() {
        assert!((celsius_to_fahrenheit(100.0) - 212.0).abs() < 0.01);
    }

    #[test]
    fn test_marathon() {
        assert!((km_to_miles(42.195) - 26.219).abs() < 0.01);
    }
}

fn main() {}
