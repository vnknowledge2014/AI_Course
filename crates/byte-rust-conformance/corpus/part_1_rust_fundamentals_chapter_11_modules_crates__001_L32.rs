// filename: src/main.rs

// Module = "phòng" trong code
mod kitchen {
    // pub = public — "cửa mở" cho bên ngoài thấy
    pub fn make_coffee(kind: &str) -> String {
        let water = boil_water();  // gọi private function trong cùng module
        format!("{} made with {}", kind, water)
    }

    // Không pub = private — "cửa đóng", chỉ module này dùng
    fn boil_water() -> &'static str {
        "fresh boiled water"
    }
}

mod cashier {
    pub fn calculate_total(items: &[u32]) -> u32 {
        items.iter().sum()
    }

    pub fn format_receipt(total: u32) -> String {
        format!("🧾 Total: {}đ", total)
    }
}

fn main() {
    // Dùng module::function
    let coffee = kitchen::make_coffee("Espresso");
    println!("{}", coffee);

    // kitchen::boil_water();  // ❌ private — không thể gọi từ ngoài!

    let total = cashier::calculate_total(&[35_000, 25_000, 45_000]);
    println!("{}", cashier::format_receipt(total));

    // Output:
    // Espresso made with fresh boiled water
    // 🧾 Total: 105000đ
}
