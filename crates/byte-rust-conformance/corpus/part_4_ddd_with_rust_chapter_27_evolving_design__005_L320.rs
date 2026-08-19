// filename: src/main.rs

// ═══ Version evolution ═══

// V1: original
#[derive(Debug)]
enum NotificationV1 {
    Email { to: String, subject: String },
    Sms { phone: String, message: String },
}

// V2: thêm Push + InApp mà không sửa V1 code patterns
#[derive(Debug)]
enum Notification {
    Email { to: String, subject: String, body: String },
    Sms { phone: String, message: String },
    // NEW in V2:
    Push { device_token: String, title: String, badge: u32 },
    InApp { user_id: u64, message: String, action_url: Option<String> },
}

// Handler phải xử lý MỌI variant (exhaustive match)
fn send(notification: &Notification) -> Result<String, String> {
    match notification {
        Notification::Email { to, subject, body } => {
            Ok(format!("📧 Sent email to {} (subject: {})", to, subject))
        }
        Notification::Sms { phone, message } => {
            if phone.len() < 10 { return Err("Invalid phone".into()); }
            Ok(format!("📱 Sent SMS to {}", phone))
        }
        Notification::Push { device_token, title, badge } => {
            Ok(format!("🔔 Push '{}' to device (badge: {})", title, badge))
        }
        Notification::InApp { user_id, message, action_url } => {
            let action = action_url.as_deref().unwrap_or("none");
            Ok(format!("💬 InApp to user #{}: '{}' [action: {}]", user_id, message, action))
        }
    }
}

// Priority: can evolve independently
fn priority(notification: &Notification) -> u32 {
    match notification {
        Notification::Email { .. } => 3,     // low
        Notification::Sms { .. } => 2,       // medium
        Notification::Push { .. } => 1,      // high
        Notification::InApp { .. } => 2,     // medium
    }
}

fn main() {
    let queue = vec![
        Notification::Email {
            to: "minh@co.com".into(),
            subject: "Welcome".into(),
            body: "Hello!".into(),
        },
        Notification::Push {
            device_token: "abc123".into(),
            title: "New order!".into(),
            badge: 1,
        },
        Notification::InApp {
            user_id: 42,
            message: "Your order shipped".into(),
            action_url: Some("/orders/123".into()),
        },
    ];

    // Sort by priority, send
    let mut sorted: Vec<_> = queue.iter().collect();
    sorted.sort_by_key(|n| priority(n));

    for n in sorted {
        match send(n) {
            Ok(msg) => println!("✅ [P{}] {}", priority(n), msg),
            Err(e) => println!("❌ {}", e),
        }
    }
}
