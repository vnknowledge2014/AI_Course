// filename: src/main.rs
use std::fmt;

// Hai type parameters
fn print_pair<A: fmt::Display, B: fmt::Debug>(first: A, second: B) {
    println!("({}, {:?})", first, second);
}

// Where clause cho dễ đọc
fn convert_and_display<T, U>(input: T) -> String
where
    T: Into<U>,
    U: fmt::Display,
{
    let converted: U = input.into();
    format!("{}", converted)
}

fn main() {
    print_pair(42, vec![1, 2, 3]);       // (42, [1, 2, 3])
    print_pair("hello", (true, 3.14));   // (hello, (true, 3.14))
}
