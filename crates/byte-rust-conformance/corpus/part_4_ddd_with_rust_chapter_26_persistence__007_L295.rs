// filename: src/main.rs

use std::collections::HashMap;

// ═══ Transaction Abstraction ═══
trait UnitOfWork {
    fn begin(&mut self);
    fn commit(&mut self) -> Result<(), String>;
    fn rollback(&mut self);
}

// ═══ Mock Implementation ═══
#[derive(Clone)]
struct Account { id: u64, name: String, balance: i64 }

struct AccountStore {
    accounts: HashMap<u64, Account>,
    pending: HashMap<u64, Account>, // Dữ liệu nháp
    in_transaction: bool,
}

impl AccountStore {
    fn new() -> Self {
        AccountStore { accounts: HashMap::new(), pending: HashMap::new(), in_transaction: false }
    }
    fn seed(&mut self, account: Account) { self.accounts.insert(account.id, account); }
    
    fn find(&self, id: u64) -> Option<Account> {
        if self.in_transaction {
            if let Some(a) = self.pending.get(&id) { return Some(a.clone()); } // Đọc từ bản nháp
        }
        self.accounts.get(&id).cloned()
    }
    
    fn save(&mut self, account: Account) {
        if self.in_transaction {
            self.pending.insert(account.id, account); // Chỉ ghi nháp
        } else {
            self.accounts.insert(account.id, account);
        }
    }
}

// Implement Transaction logic cho RAM
impl UnitOfWork for AccountStore {
    fn begin(&mut self) {
        self.pending.clear();
        self.in_transaction = true;
    }

    fn commit(&mut self) -> Result<(), String> {
        for (id, account) in self.pending.drain() {
            self.accounts.insert(id, account); // Đổ nháp vào thật
        }
        self.in_transaction = false;
        Ok(())
    }

    fn rollback(&mut self) {
        self.pending.clear(); // Xóa sạch nháp
        self.in_transaction = false;
    }
}

fn main() {}
