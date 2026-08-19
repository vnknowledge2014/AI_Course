// filename: src/lib.rs

pub fn fizzbuzz(n: u32) -> String {
    match (n % 3, n % 5) {
        (0, 0) => "FizzBuzz".into(),
        (0, _) => "Fizz".into(),
        (_, 0) => "Buzz".into(),
        _ => n.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fizzbuzz_table() {
        // (Đầu vào, Kết quả mong đợi)
        let cases = vec![
            (1, "1"),
            (3, "Fizz"),
            (5, "Buzz"),
            (15, "FizzBuzz"),
            (30, "FizzBuzz"),
            (7, "7"),
            (10, "Buzz"),
        ];

        // Lặp qua mảng và test
        for (input, expected) in cases {
            assert_eq!(fizzbuzz(input), expected, "fizzbuzz({}) failed", input);
        }
    }
}

fn main() {}
