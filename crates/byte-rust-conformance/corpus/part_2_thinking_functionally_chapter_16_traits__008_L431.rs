// filename: src/main.rs

#[derive(Debug)]
struct Email(String);

impl From<&str> for Email {
    fn from(s: &str) -> Self {
        Email(s.to_lowercase())
    }
}

impl From<String> for Email {
    fn from(s: String) -> Self {
        Email(s.to_lowercase())
    }
}

fn send_notification(to: impl Into<Email>, message: &str) {
    let email: Email = to.into();  // auto convert
    println!("📧 To {:?}: {}", email, message);
}

fn main() {
    // Cả &str và String đều convert được!
    send_notification("ADMIN@Company.Com", "Server restarted");
    send_notification(String::from("User@Domain.Org"), "Welcome!");
}
