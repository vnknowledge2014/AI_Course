// filename: src/main.rs

#[derive(Debug)]
struct Inventory {
    items: Vec<String>,
}

impl Inventory {
    fn new() -> Self {
        Inventory { items: vec![] }
    }

    fn add_item(&mut self, name: &str) {
        self.items.push(name.to_string());
    }

    fn remove_item(&mut self, name: &str) -> bool {
        if let Some(pos) = self.items.iter().position(|item| item == name) {
            self.items.remove(pos);
            true
        } else {
            false
        }
    }

    // Trả &str tham chiếu vào self.items
    // Lifetime: kết quả sống ít nhất lâu bằng &self
    fn search(&self, query: &str) -> Vec<&str> {
        self.items.iter()
            .filter(|item| item.to_lowercase().contains(&query.to_lowercase()))
            .map(|item| item.as_str())
            .collect()
    }

    fn summary(&self) -> String {
        format!("{} items: {:?}", self.items.len(), self.items)
    }
}

fn main() {
    let mut inv = Inventory::new();
    inv.add_item("Laptop");
    inv.add_item("Wireless Mouse");
    inv.add_item("Mechanical Keyboard");
    inv.add_item("USB Mouse");

    println!("{}", inv.summary());

    let results = inv.search("mouse");
    println!("Search 'mouse': {:?}", results);

    let removed = inv.remove_item("USB Mouse");
    println!("Removed 'USB Mouse': {}", removed);
    println!("{}", inv.summary());

    // Output:
    // 4 items: ["Laptop", "Wireless Mouse", "Mechanical Keyboard", "USB Mouse"]
    // Search 'mouse': ["Wireless Mouse", "USB Mouse"]
    // Removed 'USB Mouse': true
    // 3 items: ["Laptop", "Wireless Mouse", "Mechanical Keyboard"]
}
