// filename: src/main.rs

// #[non_exhaustive] — crate khác PHẢI có _ => arm
#[non_exhaustive]
#[derive(Debug)]
enum ApiError {
    NotFound,
    Unauthorized,
    RateLimit,
}

fn handle(err: &ApiError) -> &str {
    match err {
        ApiError::NotFound => "Not found",
        ApiError::Unauthorized => "Unauthorized",
        ApiError::RateLimit => "Rate limited",
        _ => "Unknown error",  // BẮT BUỘC vì #[non_exhaustive]
    }
}

fn main() {
    println!("{}", handle(&ApiError::NotFound));
}
