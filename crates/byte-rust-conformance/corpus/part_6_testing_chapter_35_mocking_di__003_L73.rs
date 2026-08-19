// filename: src/main.rs

// ═══ PORTS (traits) ═══
trait UserRepository {
    fn find_by_id(&self, id: u64) -> Option<User>;
    fn find_by_email(&self, email: &str) -> Option<User>;
    fn save(&mut self, user: &User) -> Result<(), String>;
}

trait EmailService {
    fn send(&self, to: &str, subject: &str, body: &str) -> Result<(), String>;
}

trait PasswordHasher {
    fn hash(&self, password: &str) -> String;
    fn verify(&self, password: &str, hash: &str) -> bool;
}

// ═══ DOMAIN ═══
#[derive(Debug, Clone)]
struct User {
    id: u64,
    email: String,
    name: String,
    password_hash: String,
}

fn main() {}
