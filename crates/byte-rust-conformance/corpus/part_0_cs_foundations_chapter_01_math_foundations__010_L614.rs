// filename: src/main.rs

#[derive(Debug)]
enum BanReason { Spam, Fraud, Harassment }  // 3 lý do

#[derive(Debug)]
enum AuthState {
    Anonymous,                                   // 1 trạng thái
    LoggedIn { user_id: u8, is_admin: bool },    // 256 × 2 = 512
    Banned { reason: BanReason },                // 3
}
// Tổng: 1 + 512 + 3 = 516 trạng thái — TẤT CẢ hợp lệ!

fn greet(state: &AuthState) {
    match state {
        AuthState::Anonymous => {
            println!("👤 Xin chào khách! Hãy đăng nhập.");
        }
        AuthState::LoggedIn { user_id, is_admin } => {
            if *is_admin {
                println!("👑 Chào Admin #{}", user_id);
            } else {
                println!("😊 Chào User #{}", user_id);
            }
        }
        AuthState::Banned { reason } => {
            println!("🚫 Tài khoản bị cấm: {:?}", reason);
        }
    }
}

fn main() {
    println!("Total states: 1 + 512 + 3 = {}", 1 + 512 + 3);

    let users = vec![
        AuthState::Anonymous,
        AuthState::LoggedIn { user_id: 42, is_admin: false },
        AuthState::LoggedIn { user_id: 1, is_admin: true },
        AuthState::Banned { reason: BanReason::Spam },
    ];

    for user in &users {
        greet(user);
    }
    // Output:
    // Total states: 1 + 512 + 3 = 516
    // 👤 Xin chào khách! Hãy đăng nhập.
    // 😊 Chào User #42
    // 👑 Chào Admin #1
    // 🚫 Tài khoản bị cấm: Spam
}
