// filename: src/main.rs

// ═══ Domain Types (Các Trạng thái tiến hóa của Đơn hàng) ═══

// 1. Vừa mới vào, chưa biết đúng sai
struct UnvalidatedOrder {
    customer_name: String,
    items: Vec<(String, u32)>, // name, qty
}

// 2. Đã được xác thực tên tuổi, số lượng
struct ValidatedOrder {
    customer_name: String,
    items: Vec<(String, u32)>,
}

// 3. Đã được áp giá tiền và tính tổng
struct PricedOrder {
    customer_name: String,
    total: u32,
}

// 4. Đã chốt hạ thành công
struct ConfirmedOrder {
    order_id: String,
    total: u32,
}

fn main() {}
