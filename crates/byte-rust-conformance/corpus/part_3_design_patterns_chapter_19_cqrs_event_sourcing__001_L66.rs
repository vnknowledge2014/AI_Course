// filename: src/main.rs

// ═══════════════════════════════════════
// COMMANDS — Write side (Yêu cầu làm gì đó)
// ═══════════════════════════════════════
#[derive(Debug)]
enum OrderCommand {
    Create { customer: String },
    AddItem { name: String, price: u32, quantity: u32 },
    Submit,
}

// ═══════════════════════════════════════
// WRITE MODEL — Xử lý Commands
// ═══════════════════════════════════════
#[derive(Debug, Clone)]
struct Order {
    customer: String,
    items: Vec<(String, u32, u32)>, // name, price, qty
    status: OrderStatus,
}

#[derive(Debug, Clone, PartialEq)]
enum OrderStatus { Draft, Submitted }

impl Order {
    fn new(customer: &str) -> Self {
        Order { customer: customer.to_string(), items: vec![], status: OrderStatus::Draft }
    }

    // Nhận một lệnh, và trả về một Order MỚI (Functional Update)
    fn handle(self, cmd: OrderCommand) -> Result<Self, String> {
        match cmd {
            OrderCommand::Create { customer } => Ok(Order::new(&customer)),
            OrderCommand::AddItem { name, price, quantity } => {
                if self.status != OrderStatus::Draft {
                    return Err("Chỉ được thêm món khi đơn hàng đang nháp".into());
                }
                let mut items = self.items;
                items.push((name, price, quantity));
                Ok(Order { items, ..self })
            }
            OrderCommand::Submit => {
                if self.items.is_empty() {
                    return Err("Không thể chốt đơn rỗng".into());
                }
                Ok(Order { status: OrderStatus::Submitted, ..self })
            }
        }
    }
}

fn main() {}
