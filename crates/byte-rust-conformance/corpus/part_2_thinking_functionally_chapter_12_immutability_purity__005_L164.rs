// filename: src/main.rs

// Struct "frozen" — mọi fields private, chỉ có getters
#[derive(Debug, Clone)]
pub struct UserProfile {
    name: String,      // private!
    email: String,     // private!
    age: u32,          // private!
}

// Builder — nơi duy nhất tạo được UserProfile
pub struct UserProfileBuilder {
    name: Option<String>,
    email: Option<String>,
    age: Option<u32>,
}

impl UserProfileBuilder {
    pub fn new() -> Self {
        UserProfileBuilder { name: None, email: None, age: None }
    }

    pub fn name(mut self, name: &str) -> Self {
        self.name = Some(name.to_string());
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = Some(email.to_string());
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = Some(age);
        self
    }

    pub fn build(self) -> Result<UserProfile, String> {
        Ok(UserProfile {
            name: self.name.ok_or("name is required")?,
            email: self.email.ok_or("email is required")?,
            age: self.age.ok_or("age is required")?,
        })
    }
}

// UserProfile chỉ có getters — KHÔNG có setters!
impl UserProfile {
    pub fn name(&self) -> &str { &self.name }
    pub fn email(&self) -> &str { &self.email }
    pub fn age(&self) -> u32 { self.age }

    // "Update" = tạo bản mới (functional update)
    pub fn with_email(&self, new_email: &str) -> Self {
        UserProfile {
            name: self.name.clone(),
            email: new_email.to_string(),
            age: self.age,
        }
    }
}

fn main() {
    let user = UserProfileBuilder::new()
        .name("Minh")
        .email("minh@email.com")
        .age(25)
        .build()
        .unwrap();

    println!("User: {} ({})", user.name(), user.email());

    // user.name = "Other".to_string();  // ❌ private — không sửa được!

    // "Update" tạo bản mới, bản cũ không đổi
    let updated = user.with_email("new@email.com");
    println!("Original: {}", user.email());   // minh@email.com
    println!("Updated: {}", updated.email()); // new@email.com
}
