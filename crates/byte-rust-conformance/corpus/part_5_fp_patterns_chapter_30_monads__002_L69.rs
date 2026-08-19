// filename: src/main.rs

use std::collections::HashMap;

fn find_user(id: u64) -> Option<String> {
    let users: HashMap<u64, &str> = [(1, "Minh"), (2, "Lan")].into();
    users.get(&id).map(|s| s.to_string())
}

fn find_email(name: &str) -> Option<String> {
    match name {
        "Minh" => Some("minh@co.com".into()),
        "Lan" => Some("lan@co.com".into()),
        _ => None,
    }
}

fn find_domain(email: &str) -> Option<String> {
    email.split('@').nth(1).map(String::from)
}

fn main() {
    // Chaining với and_then — mỗi step có thể fail (None)
    let domain = find_user(1)
        .and_then(|name| find_email(&name))
        .and_then(|email| find_domain(&email));
    println!("User 1 domain: {:?}", domain);  // Some("co.com")

    // Fail ở giữa → tất cả sau skip
    let domain = find_user(99)                  // None!
        .and_then(|name| find_email(&name))     // skipped
        .and_then(|email| find_domain(&email)); // skipped
    println!("User 99 domain: {:?}", domain);  // None

    // Mix map và and_then
    let greeting = find_user(2)
        .and_then(|name| find_email(&name))    // Option<String>
        .map(|email| format!("Hello! Your email: {}", email));  // transform
    println!("Greeting: {:?}", greeting);
}
