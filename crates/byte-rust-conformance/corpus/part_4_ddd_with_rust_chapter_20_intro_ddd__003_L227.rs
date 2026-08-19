// filename: src/main.rs

// Mỗi bounded context = 1 module với types RIÊNG
mod catalog {
    #[derive(Debug, Clone)]
    pub struct Product {
        pub id: String,
        pub name: String,
        pub description: String,
        pub category: String,
    }
}

mod inventory {
    #[derive(Debug, Clone)]
    pub struct StockItem {
        pub product_id: String,
        pub warehouse: String,
        pub quantity: u32,
    }

    impl StockItem {
        pub fn is_available(&self) -> bool { self.quantity > 0 }
    }
}

mod shipping {
    #[derive(Debug, Clone)]
    pub struct Parcel {
        pub product_id: String,
        pub weight_kg: f64,
        pub is_fragile: bool,
    }

    impl Parcel {
        pub fn shipping_cost(&self) -> u32 {
            let base = (self.weight_kg * 30_000.0) as u32;
            if self.is_fragile { base * 150 / 100 } else { base }
        }
    }
}

mod pricing {
    #[derive(Debug, Clone)]
    pub struct PricedItem {
        pub product_id: String,
        pub base_price: u32,
        pub tax_rate: f64,
    }

    impl PricedItem {
        pub fn final_price(&self) -> u32 {
            self.base_price + (self.base_price as f64 * self.tax_rate) as u32
        }
    }
}

fn main() {
    // Mỗi context chỉ biết thông tin CẦN THIẾT
    let product = catalog::Product {
        id: "PROD-001".into(),
        name: "Premium Coffee".into(),
        description: "Single origin, dark roast".into(),
        category: "Beverages".into(),
    };

    let stock = inventory::StockItem {
        product_id: "PROD-001".into(),
        warehouse: "HCM-01".into(),
        quantity: 150,
    };

    let parcel = shipping::Parcel {
        product_id: "PROD-001".into(),
        weight_kg: 0.5,
        is_fragile: false,
    };

    let priced = pricing::PricedItem {
        product_id: "PROD-001".into(),
        base_price: 200_000,
        tax_rate: 0.08,
    };

    println!("📦 {} — {}", product.name, product.category);
    println!("📊 Stock: {} (available: {})", stock.quantity, stock.is_available());
    println!("🚚 Shipping: {}đ", parcel.shipping_cost());
    println!("💰 Price: {}đ (incl. tax)", priced.final_price());
}
