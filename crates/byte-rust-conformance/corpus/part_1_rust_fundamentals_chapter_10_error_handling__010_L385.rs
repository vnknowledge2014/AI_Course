// filename: src/main.rs
use std::fmt;

#[derive(Debug)]
enum OrderError {
    EmptyCart,
    InvalidQuantity(u32),
    OutOfStock { item: String, available: u32 },
    PaymentFailed(String),
}

// Implement Display cho user-friendly messages
impl fmt::Display for OrderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            OrderError::EmptyCart =>
                write!(f, "Cart is empty"),
            OrderError::InvalidQuantity(q) =>
                write!(f, "Invalid quantity: {}", q),
            OrderError::OutOfStock { item, available } =>
                write!(f, "'{}' out of stock (only {} left)", item, available),
            OrderError::PaymentFailed(reason) =>
                write!(f, "Payment failed: {}", reason),
        }
    }
}

// Implement std::error::Error (cho compatibility)
impl std::error::Error for OrderError {}

fn place_order(item: &str, quantity: u32) -> Result<String, OrderError> {
    if item.is_empty() {
        return Err(OrderError::EmptyCart);
    }
    if quantity == 0 || quantity > 100 {
        return Err(OrderError::InvalidQuantity(quantity));
    }

    // Giả lập kiểm tra stock
    let stock = 5;
    if quantity > stock {
        return Err(OrderError::OutOfStock {
            item: item.to_string(),
            available: stock,
        });
    }

    Ok(format!("✅ Ordered {}x {}", quantity, item))
}

fn main() {
    let orders = vec![
        ("Coffee", 2),
        ("", 1),
        ("Tea", 0),
        ("Smoothie", 10),  // only 5 in stock
    ];

    for (item, qty) in orders {
        match place_order(item, qty) {
            Ok(msg) => println!("{}", msg),
            Err(e) => println!("❌ {}", e),
        }
    }

    // Output:
    // ✅ Ordered 2x Coffee
    // ❌ Cart is empty
    // ❌ Invalid quantity: 0
    // ❌ 'Smoothie' out of stock (only 5 left)
}
