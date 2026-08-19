// filename: src/main.rs

#[derive(Debug)]
struct UserInput { name: String, email: String, age: String }
#[derive(Debug)]
struct ValidUser { name: String, email: String, age: u32 }

fn validate_name(name: &str) -> Result<String, String> {
    let trimmed = name.trim();
    if trimmed.len() >= 2 { Ok(trimmed.to_string()) }
    else { Err("Name must be at least 2 characters".into()) }
}

fn validate_email(email: &str) -> Result<String, String> {
    if email.contains('@') { Ok(email.to_lowercase()) }
    else { Err(format!("Invalid email: {}", email)) }
}

fn validate_age(age_str: &str) -> Result<u32, String> {
    let age: u32 = age_str.parse().map_err(|_| format!("Invalid age"))?;
    if (18..=150).contains(&age) { Ok(age) }
    else { Err(format!("Age {} not in range 18-150", age)) }
}

fn main() {}
