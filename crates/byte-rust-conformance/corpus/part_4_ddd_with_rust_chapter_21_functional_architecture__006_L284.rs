// filename: src/main.rs

// ═══════ PORTS (Traits) ═══════
// Domain/Application định nghĩa → Infrastructure cắm vào
trait OrderRepository {
    fn save(&mut self, order: &Order) -> Result<(), String>;
}

trait PaymentGateway {
    fn charge(&self, amount: u32) -> Result<String, String>;
}

// ═══════ DOMAIN ═══════
#[derive(Debug, Clone)]
struct Order {
    id: u64,
    customer: String,
    total: u32,
}

// ═══════ APPLICATION ═══════
fn place_order(
    customer: &str,
    total: u32,
    repo: &mut dyn OrderRepository,    // Cắm Port Database
    payment: &dyn PaymentGateway,      // Cắm Port Payment
) -> Result<Order, String> {
    
    // Cà thẻ trước
    payment.charge(total)?;

    // Lưu DB sau
    let order = Order { id: 1, customer: customer.into(), total };
    repo.save(&order)?;

    Ok(order)
}

fn main() {}
