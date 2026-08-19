// filename: src/main.rs

struct Pipeline {
    steps: Vec<Box<dyn Fn(i32) -> i32>>,
}

impl Pipeline {
    fn new() -> Self {
        Pipeline { steps: vec![] }
    }

    fn add_step<F: Fn(i32) -> i32 + 'static>(mut self, step: F) -> Self {
        self.steps.push(Box::new(step));
        self  // return self cho method chaining
    }

    fn execute(&self, input: i32) -> i32 {
        self.steps.iter().fold(input, |acc, step| step(acc))
    }
}

fn main() {
    let pipeline = Pipeline::new()
        .add_step(|x| x + 1)      // +1
        .add_step(|x| x * 2)      // ×2
        .add_step(|x| x - 3);     // -3

    let result = pipeline.execute(5);
    println!("Pipeline(5) = {}", result);
    assert_eq!(result, 9);  // (5+1)*2-3 = 9

    // Reuse pipeline
    println!("Pipeline(10) = {}", pipeline.execute(10));
    assert_eq!(pipeline.execute(10), 19);  // (10+1)*2-3 = 19

    // Output:
    // Pipeline(5) = 9
    // Pipeline(10) = 19
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_pipeline() {
        let p = Pipeline::new();
        assert_eq!(p.execute(42), 42);
    }

    #[test]
    fn test_single_step() {
        let p = Pipeline::new().add_step(|x| x * 10);
        assert_eq!(p.execute(5), 50);
    }

    #[test]
    fn test_multi_steps() {
        let p = Pipeline::new()
            .add_step(|x| x + 1)
            .add_step(|x| x * 2)
            .add_step(|x| x - 3);
        assert_eq!(p.execute(5), 9);
    }
}
