// filename: src/main.rs

type Middleware = Box<dyn Fn(&str) -> String>;

fn logger() -> Middleware {
    Box::new(|input| {
        println!("  [LOG] Processing: {}", input);
        input.to_string()
    })
}

fn trimmer() -> Middleware {
    Box::new(|input| input.trim().to_string())
}

fn lowercaser() -> Middleware {
    Box::new(|input| input.to_lowercase())
}

fn validator(min_len: usize) -> Middleware {
    Box::new(move |input| {
        if input.len() >= min_len {
            input.to_string()
        } else {
            format!("[INVALID: too short] {}", input)
        }
    })
}

fn process(input: &str, middlewares: &[Middleware]) -> String {
    middlewares.iter().fold(input.to_string(), |data, mw| mw(&data))
}

fn main() {
    let pipeline = vec![
        logger(),
        trimmer(),
        lowercaser(),
        validator(3),
    ];

    let inputs = vec!["  Hello World  ", "  Hi  ", "  RUST  "];
    for input in inputs {
        println!("Input: '{}'", input);
        let result = process(input, &pipeline);
        println!("Output: '{}'\n", result);
    }
    // Input: '  Hello World  '
    //   [LOG] Processing:   Hello World
    // Output: 'hello world'
    //
    // Input: '  Hi  '
    //   [LOG] Processing:   Hi
    // Output: '[INVALID: too short] hi'
    //
    // Input: '  RUST  '
    //   [LOG] Processing:   RUST
    // Output: 'rust'
}
