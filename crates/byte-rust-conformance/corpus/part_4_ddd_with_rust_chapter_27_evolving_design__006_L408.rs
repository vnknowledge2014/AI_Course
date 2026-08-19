// filename: src/main.rs

// ═══ Deprecation with type system ═══

mod v1 {
    #[derive(Debug)]
    #[deprecated(since = "2.0", note = "Use v2::Order instead")]
    pub struct Order {
        pub id: u64,
        pub customer: String,
        pub total: u32,
    }
}

mod v2 {
    #[derive(Debug)]
    pub struct Order {
        pub id: u64,
        pub customer: Customer,
        pub lines: Vec<OrderLine>,
        pub status: OrderStatus,
    }

    #[derive(Debug)]
    pub struct Customer { pub name: String, pub email: String }

    #[derive(Debug)]
    pub struct OrderLine { pub product: String, pub price: u32, pub qty: u32 }

    #[derive(Debug)]
    pub enum OrderStatus { Draft, Confirmed, Shipped }
}

// Migration helper
#[allow(deprecated)]
impl From<v1::Order> for v2::Order {
    fn from(old: v1::Order) -> Self {
        v2::Order {
            id: old.id,
            customer: v2::Customer {
                name: old.customer,
                email: "unknown@migration.com".into(),
            },
            lines: vec![v2::OrderLine {
                product: "Migrated item".into(),
                price: old.total,
                qty: 1,
            }],
            status: v2::OrderStatus::Confirmed,
        }
    }
}

fn main() {
    // New code: use v2
    let order = v2::Order {
        id: 1,
        customer: v2::Customer { name: "Minh".into(), email: "minh@co.com".into() },
        lines: vec![
            v2::OrderLine { product: "Coffee".into(), price: 85_000, qty: 2 },
        ],
        status: v2::OrderStatus::Confirmed,
    };
    println!("V2: {:?}\n", order);

    // Migrate old data
    #[allow(deprecated)]
    let old = v1::Order { id: 99, customer: "Legacy".into(), total: 100_000 };
    let migrated: v2::Order = old.into();
    println!("Migrated: {:?}", migrated);
}
