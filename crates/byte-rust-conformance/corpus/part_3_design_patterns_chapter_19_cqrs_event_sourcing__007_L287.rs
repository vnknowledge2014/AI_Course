// filename: src/main.rs

#[derive(Debug)]
enum BankCommand { Withdraw { amount: u64 } }

#[derive(Debug, Clone)]
enum BankEvent { Withdrawn { amount: u64 } }

#[derive(Debug, Clone)]
struct BankState { balance: u64 }

// Trạm Gác (Command Handler): Kiểm duyệt Command, Đẻ ra Event
fn handle_command(state: &BankState, cmd: &BankCommand) -> Result<Vec<BankEvent>, String> {
    match cmd {
        BankCommand::Withdraw { amount } => {
            if *amount > state.balance {
                Err(format!("Không đủ tiền. Có: {}, Cần: {}", state.balance, amount))
            } else {
                Ok(vec![BankEvent::Withdrawn { amount: *amount }])
            }
        }
    }
}

// Ứng dụng Event vào State
fn apply_event(state: BankState, event: &BankEvent) -> BankState {
    match event {
        BankEvent::Withdrawn { amount } => BankState { balance: state.balance - amount },
    }
}

fn main() {}
