// filename: src/main.rs

// Product type: mọi field CẦN CÓ MẶT cùng lúc (AND)
#[derive(Debug, Clone)]
struct Customer {
    id: u64,
    name: String,
    email: String,
    is_vip: bool,
}

impl Customer {
    fn new(id: u64, name: &str, email: &str) -> Self {
        Customer {
            id,
            name: name.to_string(),
            email: email.to_string(),
            is_vip: false,
        }
    }

    // Functional update — trả struct mới
    fn promote_to_vip(&self) -> Self {
        Customer { is_vip: true, ..self.clone() }
    }

    fn display(&self) -> String {
        let vip_badge = if self.is_vip { " ⭐" } else { "" };
        format!("[{}] {}{} <{}>", self.id, self.name, vip_badge, self.email)
    }
}

fn main() {
    let customer = Customer::new(1, "Minh", "minh@email.com");
    let vip = customer.promote_to_vip();

    println!("Regular: {}", customer.display());
    println!("VIP:     {}", vip.display());
    // Regular: [1] Minh <minh@email.com>
    // VIP:     [1] Minh ⭐ <minh@email.com>
}
