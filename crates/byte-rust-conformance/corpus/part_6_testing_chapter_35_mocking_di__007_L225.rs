// filename: src/lib.rs (test section)

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> (MockUserRepo, MockEmailService, MockHasher) {
        (MockUserRepo::new(), MockEmailService::new(), MockHasher)
    }

    #[test]
    fn register_success() {
        let (mut repo, email, hasher) = setup();
        let user = register_user(&mut repo, &email, &hasher, "Minh", "minh@co.com", "Str0ngPass!").unwrap();

        assert_eq!(user.name, "Minh");
        assert_eq!(user.email, "minh@co.com");
        assert_eq!(email.sent_count(), 1); // welcome email sent
    }

    #[test]
    fn register_duplicate_email() {
        let existing = User {
            id: 1, email: "minh@co.com".into(),
            name: "Minh".into(), password_hash: "xxx".into(),
        };
        let (mut repo, email, hasher) = setup();
        
        // Cài sẵn một User vào Mock Database
        let mut repo = repo.with_user(existing);

        let result = register_user(&mut repo, &email, &hasher, "Other", "minh@co.com", "Pass1234!");
        
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("already registered"));
        assert_eq!(email.sent_count(), 0); // Lỗi xảy ra thì không được gửi email!
    }
}

fn main() {}
