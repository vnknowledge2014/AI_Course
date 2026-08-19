// filename: src/main.rs

#[derive(Debug)]
struct BatchResult { successes: Vec<String>, failures: Vec<String> }

fn validate_email(email: &str) -> Result<String, String> {
    if email.contains('@') { Ok(email.to_lowercase()) }
    else { Err(format!("Lỗi định dạng: {}", email)) }
}

fn process_batch(emails: &[&str]) -> BatchResult {
    // Dùng .partition() của Rust để chẻ Iterator thành 2 nhánh: Ok và Err
    let (ok, err): (Vec<_>, Vec<_>) = emails.iter()
        .map(|e| validate_email(e))
        .partition(Result::is_ok);

    BatchResult {
        successes: ok.into_iter().map(|r| r.unwrap()).collect(),
        failures: err.into_iter().map(|r| r.unwrap_err()).collect(),
    }
}

fn main() {}
