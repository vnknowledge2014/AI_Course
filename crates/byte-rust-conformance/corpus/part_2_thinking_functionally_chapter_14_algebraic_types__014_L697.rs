#[derive(Debug, Clone, Copy)]
struct Celsius(f64);

#[derive(Debug, Clone, Copy)]
struct Fahrenheit(f64);

impl Celsius {
    fn new(value: f64) -> Self { Celsius(value) }
    fn value(&self) -> f64 { self.0 }
    fn to_fahrenheit(&self) -> Fahrenheit {
        Fahrenheit(self.0 * 9.0 / 5.0 + 32.0)
    }
}

impl Fahrenheit {
    fn new(value: f64) -> Self { Fahrenheit(value) }
    fn value(&self) -> f64 { self.0 }
    fn to_celsius(&self) -> Celsius {
        Celsius((self.0 - 32.0) * 5.0 / 9.0)
    }
}

fn main() {
    let boiling = Celsius::new(100.0);
    let boiling_f = boiling.to_fahrenheit();
    println!("{:.1}°C = {:.1}°F", boiling.value(), boiling_f.value());

    // let wrong = boiling + boiling_f;  // ❌ Cannot add Celsius + Fahrenheit!
}
