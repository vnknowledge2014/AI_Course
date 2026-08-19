// filename: src/main.rs

// V1: original struct
#[derive(Debug, Clone)]
struct UserProfile {
    name: String,
    email: String,
    // V2: thêm fields MỚI với defaults
    avatar_url: Option<String>,      // None = chưa set
    bio: Option<String>,             // None = chưa set
    notification_prefs: NotificationPrefs,
}

#[derive(Debug, Clone)]
struct NotificationPrefs {
    email_enabled: bool,
    push_enabled: bool,
    sms_enabled: bool,
}

impl Default for NotificationPrefs {
    fn default() -> Self {
        NotificationPrefs {
            email_enabled: true,
            push_enabled: true,
            sms_enabled: false,
        }
    }
}

impl UserProfile {
    // Constructor giữ backward-compatibility
    fn new(name: &str, email: &str) -> Self {
        UserProfile {
            name: name.into(),
            email: email.into(),
            avatar_url: None,
            bio: None,
            notification_prefs: NotificationPrefs::default(),
        }
    }

    // Builder methods cho new fields
    fn with_avatar(mut self, url: &str) -> Self {
        self.avatar_url = Some(url.into());
        self
    }

    fn with_bio(mut self, bio: &str) -> Self {
        self.bio = Some(bio.into());
        self
    }

    fn with_notifications(mut self, prefs: NotificationPrefs) -> Self {
        self.notification_prefs = prefs;
        self
    }
}

fn main() {
    // Old code vẫn hoạt động!
    let user = UserProfile::new("Minh", "minh@co.com");
    println!("V1 style: {:?}\n", user);

    // New code dùng thêm features
    let user = UserProfile::new("Lan", "lan@co.com")
        .with_avatar("https://example.com/avatar.jpg")
        .with_bio("Rust enthusiast");
    println!("V2 style: {:?}", user);
}
