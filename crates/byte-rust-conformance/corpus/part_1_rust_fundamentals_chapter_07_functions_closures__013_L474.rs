// filename: src/main.rs

#[derive(Debug)]
struct Order {
    customer: String,
    amount: u32,
    is_paid: bool,
}

fn main() {
    let orders = vec![
        Order { customer: "Minh".into(), amount: 35_000, is_paid: true },
        Order { customer: "Lan".into(), amount: 55_000, is_paid: false },
        Order { customer: "Hùng".into(), amount: 25_000, is_paid: true },
        Order { customer: "Mai".into(), amount: 45_000, is_paid: true },
        Order { customer: "Dũng".into(), amount: 15_000, is_paid: false },
    ];

    // Pipeline: lọc đã thanh toán → lấy amount → tính tổng
    let total_paid: u32 = orders.iter()
        .filter(|o| o.is_paid)               // HOF: filter nhận closure
        .map(|o| o.amount)                    // HOF: map nhận closure
        .sum();                               // HOF: fold/reduce

    // Pipeline: tìm top spenders (paid, >= 30k)
    let top_customers: Vec<&str> = orders.iter()
        .filter(|o| o.is_paid && o.amount >= 30_000)
        .map(|o| o.customer.as_str())
        .collect();

    println!("Total paid: {}đ", total_paid);
    println!("Top customers: {:?}", top_customers);

    // Output:
    // Total paid: 105000đ
    // Top customers: ["Minh", "Mai"]
}
