struct FloatRange {
    current: f64,
    end: f64,
    step: f64,
}

impl FloatRange {
    fn new(start: f64, end: f64, step: f64) -> Self {
        FloatRange { current: start, end, step }
    }
}

impl Iterator for FloatRange {
    type Item = f64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current >= self.end {
            None
        } else {
            let value = self.current;
            self.current += self.step;
            Some(value)
        }
    }
}

fn main() {
    let values: Vec<f64> = FloatRange::new(0.0, 1.0, 0.2).collect();
    println!("{:.1?}", values);  // [0.0, 0.2, 0.4, 0.6, 0.8]
}
