// filename: src/main.rs
use std::fs;
use std::io;

fn read_username(path: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(path)?;  // Err? → return Err
    // ↑ Nếu Ok → content = nội dung file
    // ↑ Nếu Err → RETURN Err(io::Error) ngay lập tức!

    Ok(content.lines()
        .next()
        .unwrap_or("")
        .trim()
        .to_string())
}

fn main() {
    match read_username("test_user.txt") {
        Ok(name) => println!("Username: {}", name),
        Err(e) => println!("Error: {}", e),
    }
    // Output (nếu file không tồn tại):
    // Error: No such file or directory (os error 2)
}
