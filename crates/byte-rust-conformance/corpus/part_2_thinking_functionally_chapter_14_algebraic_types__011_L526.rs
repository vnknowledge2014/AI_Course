// filename: src/main.rs

// Generic enum: "Validation result" — thành công HOẶC danh sách lỗi
#[derive(Debug)]
enum Validated<T> {
    Valid(T),
    Invalid(Vec<String>),
}

impl<T> Validated<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Validated<U> {
        match self {
            Validated::Valid(val) => Validated::Valid(f(val)),
            Validated::Invalid(errors) => Validated::Invalid(errors),
        }
    }

    fn and_then<U, F: FnOnce(T) -> Validated<U>>(self, f: F) -> Validated<U> {
        match self {
            Validated::Valid(val) => f(val),
            Validated::Invalid(errors) => Validated::Invalid(errors),
        }
    }
}

// Validation functions
fn validate_name(name: &str) -> Validated<String> {
    if name.trim().len() >= 2 {
        Validated::Valid(name.trim().to_string())
    } else {
        Validated::Invalid(vec!["Name must be at least 2 characters".into()])
    }
}

fn validate_age(age: i32) -> Validated<u32> {
    if (0..=150).contains(&age) {
        Validated::Valid(age as u32)
    } else {
        Validated::Invalid(vec![format!("Age {} is out of range [0, 150]", age)])
    }
}

fn main() {
    // Valid path
    let name = validate_name("Minh");
    println!("Name: {:?}", name);  // Valid("Minh")

    let mapped = validate_name("Minh").map(|n| n.to_uppercase());
    println!("Upper: {:?}", mapped);  // Valid("MINH")

    // Invalid path
    let bad = validate_name("M");
    println!("Bad: {:?}", bad);  // Invalid(["Name must be at least 2 characters"])

    let bad_age = validate_age(200);
    println!("Bad age: {:?}", bad_age);  // Invalid(["Age 200 is out of range"])
}
