// filename: src/main.rs

fn classify_response(status: u16) -> &'static str {
    match status {
        // @ binding: bắt giá trị vào biến VÀ kiểm tra range
        code @ 200..=299 => {
            println!("  (success code: {})", code);
            "Success"
        }
        code @ 300..=399 => {
            println!("  (redirect code: {})", code);
            "Redirect"
        }
        code @ 400..=499 => {
            println!("  (client error: {})", code);
            "Client Error"
        }
        code @ 500..=599 => {
            println!("  (server error: {})", code);
            "Server Error"
        }
        _ => "Unknown",
    }
}

fn main() {
    for code in [200, 301, 404, 500, 999] {
        println!("{}: {}", code, classify_response(code));
    }
}
