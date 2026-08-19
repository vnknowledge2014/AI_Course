// filename: src/main.rs

#[derive(Debug, Clone)]
struct Pair<T> {
    first: T,
    second: T,
}

impl<T> Pair<T> {
    fn new(first: T, second: T) -> Self {
        Pair { first, second }
    }
}

// Methods chỉ cho Pair<T> khi T: PartialOrd
impl<T: PartialOrd> Pair<T> {
    fn max(&self) -> &T {
        if self.first >= self.second { &self.first } else { &self.second }
    }
}

// Methods chỉ cho Pair<T> khi T: Display
impl<T: std::fmt::Display> Pair<T> {
    fn display(&self) -> String {
        format!("({}, {})", self.first, self.second)
    }
}

fn main() {
    let numbers = Pair::new(10, 20);
    println!("Max: {}", numbers.max());      // 20
    println!("Display: {}", numbers.display()); // (10, 20)

    let names = Pair::new("Alice", "Bob");
    println!("Max: {}", names.max());        // Bob
}
