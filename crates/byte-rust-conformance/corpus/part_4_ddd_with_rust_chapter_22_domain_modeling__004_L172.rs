// filename: src/main.rs

// ═══════ Entity ID — Value Object đóng vai trò danh tính ═══════
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct CustomerId(u64);

impl CustomerId {
    pub fn new(id: u64) -> Self { CustomerId(id) }
}

// ═══════ Entity ═══════
#[derive(Debug, Clone)]
pub struct Customer {
    id: CustomerId,            // identity — Trường này BẤT BIẾN!
    name: String,
    email: String,
    tier: CustomerTier,
    total_spent: u64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum CustomerTier { Regular, Silver, Gold, Platinum }

impl Customer {
    pub fn new(id: CustomerId, name: &str, email: &str) -> Self {
        Customer {
            id, name: name.into(), email: email.into(),
            tier: CustomerTier::Regular, total_spent: 0,
        }
    }
    
    pub fn id(&self) -> CustomerId { self.id }
}

fn main() {}
