// ❌ Verbose: match lồng match
fn read_username_verbose(path: &str) -> Result<String, String> {
    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => return Err(format!("Read error: {}", e)),
    };
    let first_line = match content.lines().next() {
        Some(line) => line,
        None => return Err("File is empty".to_string()),
    };
    Ok(first_line.trim().to_string())
}

fn main() {}
