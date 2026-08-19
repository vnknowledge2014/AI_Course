struct Pipeline<T> {
    steps: Vec<Box<dyn Fn(T) -> T>>,
}

impl<T> Pipeline<T> {
    fn new() -> Self { Pipeline { steps: vec![] } }

    fn add_step<F: Fn(T) -> T + 'static>(mut self, f: F) -> Self {
        self.steps.push(Box::new(f));
        self
    }

    fn execute(&self, input: T) -> T {
        self.steps.iter().fold(input, |acc, step| step(acc))
    }
}

fn main() {
    // String pipeline
    let text_pipe = Pipeline::new()
        .add_step(|s: String| s.trim().to_string())
        .add_step(|s| s.to_lowercase())
        .add_step(|s| s.replace("rust", "Rust 🦀"));

    println!("{}", text_pipe.execute("  HELLO RUST WORLD  ".into()));
    // hello Rust 🦀 world

    // Math pipeline
    let math_pipe = Pipeline::new()
        .add_step(|x: i32| x + 10)
        .add_step(|x| x * 2)
        .add_step(|x| x - 5);

    println!("{}", math_pipe.execute(5));  // (5+10)*2 - 5 = 25
}
