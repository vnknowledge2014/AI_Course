// filename: src/main.rs

// Custom container: "Labeled value"
#[derive(Debug, Clone)]
struct Labeled<T> {
    label: String,
    value: T,
}

impl<T> Labeled<T> {
    fn new(label: &str, value: T) -> Self {
        Labeled { label: label.into(), value }
    }

    // map: transform value, keep label
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Labeled<U> {
        Labeled {
            label: self.label,
            value: f(self.value),
        }
    }
}

fn main() {
    let temp = Labeled::new("Temperature", 36.5_f64);
    println!("{:?}", temp);

    let formatted = temp.map(|t| format!("{:.1}°C", t));
    println!("{:?}", formatted);
    // Labeled { label: "Temperature", value: "36.5°C" }

    // Chain maps
    let score = Labeled::new("Math", 85_u32)
        .map(|s| s as f64 / 100.0)
        .map(|pct| format!("{:.0}%", pct * 100.0));
    println!("{:?}", score);
    // Labeled { label: "Math", value: "85%" }
}
