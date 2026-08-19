// filename: src/main.rs

// Smart constructor: validate + construct — đảm bảo type luôn valid
#[derive(Debug, Clone)]
struct Password(String);

impl Password {
    fn new(value: &str) -> Result<Self, Vec<String>> {
        let mut errors = vec![];

        if value.len() < 8 {
            errors.push("Must be at least 8 characters".into());
        }
        if !value.chars().any(|c| c.is_uppercase()) {
            errors.push("Must contain uppercase letter".into());
        }
        if !value.chars().any(|c| c.is_ascii_digit()) {
            errors.push("Must contain digit".into());
        }
        if !value.chars().any(|c| "!@#$%^&*".contains(c)) {
            errors.push("Must contain special character".into());
        }

        if errors.is_empty() {
            Ok(Password(value.to_string()))
        } else {
            Err(errors)
        }
    }

    fn value(&self) -> &str { &self.0 }
}

fn main() {
    let attempts = vec!["short", "NoDigitsHere!", "NoSpecial1A", "V@lid_Pass1"];

    for attempt in attempts {
        match Password::new(attempt) {
            Ok(pwd) => println!("✅ '{}' → valid password", pwd.value()),
            Err(errors) => {
                println!("❌ '{}':", attempt);
                for e in &errors { println!("   - {}", e); }
            }
        }
    }
}
