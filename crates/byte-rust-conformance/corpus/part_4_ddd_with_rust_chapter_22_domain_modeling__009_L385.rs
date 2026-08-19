// filename: src/main.rs
use std::marker::PhantomData;

// Các Tag (Zero-size, biến mất khi ứng dụng chạy)
struct Draft;
struct Confirmed;
struct Shipped;

// Order giờ mang theo cái Tag (State) 
struct Order<State> {
    id: u64,
    items: Vec<String>,
    _state: PhantomData<State>, // Dán tag vào đây
}

// KHỐI 1: Chỉ Đơn Nháp mới có các hàm này!
impl Order<Draft> {
    fn new(id: u64) -> Self {
        Order { id, items: vec![], _state: PhantomData }
    }

    fn add_item(mut self, name: &str) -> Self {
        self.items.push(name.into());
        self
    }

    // Phép màu: Chuyển Tag từ Draft -> Confirmed
    fn confirm(self) -> Result<Order<Confirmed>, String> {
        if self.items.is_empty() { return Err("Trống không!".into()); }
        
        Ok(Order {
            id: self.id, items: self.items,
            _state: PhantomData, // Đeo tag mới!
        })
    }
}

// KHỐI 2: Chỉ Đơn Đã Xác Nhận mới được Giao hàng!
impl Order<Confirmed> {
    // Chuyển Tag từ Confirmed -> Shipped
    fn ship(self) -> Order<Shipped> {
        Order {
            id: self.id, items: self.items,
            _state: PhantomData,
        }
    }
}

fn main() {}
