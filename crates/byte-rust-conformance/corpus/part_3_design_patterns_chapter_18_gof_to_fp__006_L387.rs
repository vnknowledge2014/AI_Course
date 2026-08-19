// filename: src/main.rs

type Handler = Box<dyn Fn(&str) -> String>;

// "Decorators" = functions wrapping functions
fn with_logging(next: Handler) -> Handler {
    Box::new(move |input| {
        println!("  [LOG] Input: '{}'", input);
        let result = next(input);
        println!("  [LOG] Output: '{}'", result);
        result
    })
}

fn with_timing(next: Handler) -> Handler {
    Box::new(move |input| {
        let start = std::time::Instant::now();
        let result = next(input);
        println!("  [TIME] {}µs", start.elapsed().as_micros());
        result
    })
}

fn with_trimming(next: Handler) -> Handler {
    Box::new(move |input| {
        let trimmed = input.trim();
        next(trimmed)
    })
}

fn main() {
    // Base handler
    let handler: Handler = Box::new(|input: &str| input.to_uppercase());

    // Stack decorators: trim → log → time → uppercase
    let decorated = with_timing(with_logging(with_trimming(handler)));

    println!("Result: {}", decorated("  hello rust  "));
}
