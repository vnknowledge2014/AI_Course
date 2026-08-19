enum AccountEvent {
    Created { id: String },
    Deposited { amount: u32 },
    Withdrawn { amount: u32 },
}

#[derive(Default, Debug)]
struct AccountState {
    balance: u32,
}

// Pure function: (State, Event) -> NewState
fn apply_event(state: AccountState, event: &AccountEvent) -> AccountState {
    match event {
        AccountEvent::Created { .. } => state,
        AccountEvent::Deposited { amount } => AccountState { balance: state.balance + amount },
        AccountEvent::Withdrawn { amount } => AccountState { balance: state.balance - amount },
    }
}

// Lấy danh sách events từ DB và tính toán
fn rebuild_state(events: &[AccountEvent]) -> AccountState {
    events.iter().fold(AccountState::default(), |state, event| apply_event(state, event))
}

fn main() {}
