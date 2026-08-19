// filename: src/main.rs

// External API trả format khác
#[derive(Debug)]
struct ExternalUser {
    full_name: String,
    email_address: String,
    is_active: bool,
}

// Domain model — format của chúng ta
#[derive(Debug)]
struct User {
    name: String,
    email: String,
    status: UserStatus,
}

#[derive(Debug)]
enum UserStatus { Active, Inactive }

// Adapter = From implementation
impl From<ExternalUser> for User {
    fn from(ext: ExternalUser) -> Self {
        User {
            name: ext.full_name,
            email: ext.email_address.to_lowercase(),
            status: if ext.is_active { UserStatus::Active } else { UserStatus::Inactive },
        }
    }
}

fn process_user(user: impl Into<User>) {
    let user: User = user.into();
    println!("Processing: {:?}", user);
}

fn main() {
    let external = ExternalUser {
        full_name: "Nguyen Minh".into(),
        email_address: "Minh@Company.COM".into(),
        is_active: true,
    };

    // Tự động convert!
    process_user(external);
    // Processing: User { name: "Nguyen Minh", email: "minh@company.com", status: Active }
}
