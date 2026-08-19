// filename: src/main.rs

// ✅ Code NÓI CÙNG NGÔN NGỮ với domain expert
// Domain expert đọc code này HIỂU ĐƯỢC!

mod ordering {
    #[derive(Debug, Clone)]
    pub struct Customer {
        pub name: String,
        pub email: String,
    }

    #[derive(Debug, Clone)]
    pub struct OrderLine {
        pub product_name: String,
        pub quantity: u32,
        pub unit_price: u32,
    }

    #[derive(Debug)]
    pub enum PaymentMethod {
        CashOnDelivery,
        BankTransfer { account: String },
        CreditCard { last_four: String },
    }

    #[derive(Debug)]
    pub struct Order {
        pub customer: Customer,
        pub lines: Vec<OrderLine>,
        pub payment: PaymentMethod,
    }

    impl Order {
        // "Place an order" — domain language
        pub fn place(customer: Customer, lines: Vec<OrderLine>, payment: PaymentMethod) -> Result<Self, String> {
            if lines.is_empty() {
                return Err("Order must have at least one line item".into());
            }
            Ok(Order { customer, lines, payment })
        }

        // "Calculate order total" — domain language
        pub fn total(&self) -> u32 {
            self.lines.iter().map(|l| l.unit_price * l.quantity).sum()
        }
    }
}

fn main() {
    use ordering::*;

    let customer = Customer { name: "Minh".into(), email: "minh@co.com".into() };
    let lines = vec![
        OrderLine { product_name: "Coffee".into(), quantity: 2, unit_price: 35_000 },
        OrderLine { product_name: "Cake".into(), quantity: 1, unit_price: 25_000 },
    ];

    match Order::place(customer, lines, PaymentMethod::CashOnDelivery) {
        Ok(order) => {
            println!("Order placed for {}", order.customer.name);
            println!("Total: {}đ", order.total());
            println!("Payment: {:?}", order.payment);
        }
        Err(e) => println!("Failed: {}", e),
    }
}
