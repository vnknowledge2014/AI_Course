// filename: src/main.rs

type ValidationResult = Result<(), String>;

trait Validator<T> {
    fn validate(&self, value: &T) -> ValidationResult;
}

// Concrete validators
struct NotEmpty;
impl Validator<String> for NotEmpty {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.trim().is_empty() { Err("Must not be empty".into()) } else { Ok(()) }
    }
}

struct MinLength(usize);
impl Validator<String> for MinLength {
    fn validate(&self, value: &String) -> ValidationResult {
        if value.len() < self.0 { Err(format!("Min length: {}", self.0)) } else { Ok(()) }
    }
}

struct InRange { min: i64, max: i64 }
impl Validator<i64> for InRange {
    fn validate(&self, value: &i64) -> ValidationResult {
        if *value >= self.min && *value <= self.max { Ok(()) }
        else { Err(format!("Must be in [{}, {}]", self.min, self.max)) }
    }
}

// Generic validation runner
fn validate_all<T>(value: &T, validators: &[&dyn Validator<T>]) -> Vec<String> {
    validators.iter()
        .filter_map(|v| v.validate(value).err())
        .collect()
}

fn main() {
    let name = String::from("Mi");
    let name_errors = validate_all(&name, &[&NotEmpty, &MinLength(3)]);
    println!("Name '{}': {:?}", name, name_errors);
    // Name 'Mi': ["Min length: 3"]

    let age: i64 = 200;
    let age_errors = validate_all(&age, &[&InRange { min: 0, max: 150 }]);
    println!("Age {}: {:?}", age, age_errors);
    // Age 200: ["Must be in [0, 150]"]

    let valid_name = String::from("Minh Nguyen");
    let no_errors = validate_all(&valid_name, &[&NotEmpty, &MinLength(3)]);
    println!("Name '{}': {:?}", valid_name, no_errors);
    // Name 'Minh Nguyen': []
}
