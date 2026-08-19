// ❌ BAD: Business logic trộn lẫn rườm rà với IO
fn process_order(order_id: u64) -> Result<String, String> {
    // Đọc database trực tiếp (Hard-dependency)
    // let order = database::find_order(order_id)?;
    
    // Gọi API trực tiếp (Hard-dependency)
    // let receipt = payment_api::charge(order.total)?;
    
    // Gửi email trực tiếp (Hard-dependency)
    // email::send(order.customer_email, receipt)?;
    Ok("done".into())
}
// Test function này kiểu gì? 
// Bạn sẽ phải cần database thật, payment API thật (tốn tiền), email server thật!

fn main() {}
