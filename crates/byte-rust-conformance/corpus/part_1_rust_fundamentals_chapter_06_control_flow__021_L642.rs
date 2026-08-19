// filename: src/main.rs
fn main() {
    let result: Vec<String> = (1..=30)
        .map(|n| match (n % 3, n % 5) {
            (0, 0) => "FizzBuzz".to_string(),
            (0, _) => "Fizz".to_string(),
            (_, 0) => "Buzz".to_string(),
            _      => n.to_string(),
        })
        .collect();

    // In 10 items mỗi dòng
    for chunk in result.chunks(10) {
        println!("{}", chunk.join(", "));
    }
    // Output:
    // 1, 2, Fizz, 4, Buzz, Fizz, 7, 8, Fizz, Buzz
    // 11, Fizz, 13, 14, FizzBuzz, 16, 17, Fizz, 19, Buzz
    // Fizz, 22, 23, Fizz, Buzz, 26, Fizz, 28, 29, FizzBuzz
}
