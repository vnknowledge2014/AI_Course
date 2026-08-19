// filename: src/main.rs

#[derive(Debug, Clone)]
enum LoginState {
    Anonymous,
    EnteringCredentials { username: String },
    Authenticating { username: String, password: String },
    Authenticated { user_id: u64, username: String },
    Locked { username: String, attempts: u32 },
}

#[derive(Debug)]
enum LoginAction {
    StartLogin(String),
    SubmitPassword(String),
    AuthSuccess(u64),
    AuthFailed,
    Reset,
}

fn transition(state: LoginState, action: LoginAction) -> LoginState {
    match (state, action) {
        // Anonymous → nhập username
        (LoginState::Anonymous, LoginAction::StartLogin(username)) =>
            LoginState::EnteringCredentials { username },

        // Nhập xong → submit password
        (LoginState::EnteringCredentials { username }, LoginAction::SubmitPassword(password)) =>
            LoginState::Authenticating { username, password },

        // Auth thành công
        (LoginState::Authenticating { username, .. }, LoginAction::AuthSuccess(user_id)) =>
            LoginState::Authenticated { user_id, username },

        // Auth thất bại → quay lại (hoặc lock)
        (LoginState::Authenticating { username, .. }, LoginAction::AuthFailed) =>
            LoginState::EnteringCredentials { username },

        // Reset từ bất kỳ state nào
        (_, LoginAction::Reset) => LoginState::Anonymous,

        // Mọi transition khác: giữ nguyên state (hoặc báo lỗi)
        (state, action) => {
            println!("  ⚠️ Invalid: {:?} in {:?}", action, state);
            state
        }
    }
}

fn main() {
    let mut state = LoginState::Anonymous;
    println!("State: {:?}", state);

    let actions = vec![
        LoginAction::StartLogin("minh".into()),
        LoginAction::SubmitPassword("pass123".into()),
        LoginAction::AuthSuccess(42),
    ];

    for action in actions {
        println!("Action: {:?}", action);
        state = transition(state, action);
        println!("State: {:?}\n", state);
    }
}
