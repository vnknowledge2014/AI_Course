// filename: src/main.rs
fn main() {
    let status_code = 404;

    let message = match status_code {
        200 => "OK",
        301 => "Moved Permanently",
        404 => "Not Found",
        500 => "Internal Server Error",
        _   => "Unknown",  // _ = "mọi trường hợp còn lại"
    };

    println!("{}: {}", status_code, message);
    // Output: 404: Not Found
}
