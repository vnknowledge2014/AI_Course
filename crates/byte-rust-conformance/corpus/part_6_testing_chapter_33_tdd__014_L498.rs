struct Calculator { value: f64 }

impl Calculator {
    fn new() -> Self { Calculator { value: 0.0 } }
    fn add(&mut self, n: f64) { self.value += n; }
    fn subtract(&mut self, n: f64) { self.value -= n; }
    fn multiply(&mut self, n: f64) { self.value *= n; }
    fn result(&self) -> f64 { self.value }
    fn reset(&mut self) { self.value = 0.0; }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] fn starts_at_zero() { assert_eq!(Calculator::new().result(), 0.0); }
    #[test] fn add_numbers() {
        let mut c = Calculator::new();
        c.add(5.0); c.add(3.0);
        assert_eq!(c.result(), 8.0);
    }
    #[test] fn reset_to_zero() {
        let mut c = Calculator::new();
        c.add(100.0); c.reset();
        assert_eq!(c.result(), 0.0);
    }
}

fn main() {}
