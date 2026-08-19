// filename: src/main.rs
use std::fmt;

// Trait của chúng ta
trait Loggable {
    fn log(&self);
}

// Blanket impl: MỌI type có Display → tự động có Loggable
impl<T: fmt::Display> Loggable for T {
    fn log(&self) {
        println!("[LOG] {}", self);
    }
}

fn main() {
    42.log();                // i32 impl Display → có Loggable
    "hello".log();           // &str impl Display → có Loggable
    3.14.log();              // f64 impl Display → có Loggable
    String::from("Rust").log(); // String impl Display → có Loggable
}
