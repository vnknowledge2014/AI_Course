// filename: src/menu.rs

#[derive(Debug, Clone)]
pub struct MenuItem {
    pub name: String,
    pub price: u32,
}

impl MenuItem {
    pub fn new(name: &str, price: u32) -> Self {
        MenuItem {
            name: name.to_string(),
            price,
        }
    }
}

fn main() {}
