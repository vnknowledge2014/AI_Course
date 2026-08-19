// filename: src/main.rs
use std::collections::HashMap;

trait Repository<T: Clone> {
    fn get(&self, id: u64) -> Option<T>;
    fn save(&mut self, id: u64, item: T);
    fn delete(&mut self, id: u64) -> bool;
    fn list(&self) -> Vec<T>;
}

// In-memory implementation — hoạt động cho BẤT KỲ type T
struct InMemoryRepo<T> {
    store: HashMap<u64, T>,
}

impl<T> InMemoryRepo<T> {
    fn new() -> Self {
        InMemoryRepo { store: HashMap::new() }
    }
}

impl<T: Clone> Repository<T> for InMemoryRepo<T> {
    fn get(&self, id: u64) -> Option<T> {
        self.store.get(&id).cloned()
    }

    fn save(&mut self, id: u64, item: T) {
        self.store.insert(id, item);
    }

    fn delete(&mut self, id: u64) -> bool {
        self.store.remove(&id).is_some()
    }

    fn list(&self) -> Vec<T> {
        self.store.values().cloned().collect()
    }
}

#[derive(Debug, Clone)]
struct User { name: String, email: String }

#[derive(Debug, Clone)]
struct Product { name: String, price: u32 }

fn main() {
    // Cùng InMemoryRepo — hoạt động cho User VÀ Product
    let mut users: InMemoryRepo<User> = InMemoryRepo::new();
    users.save(1, User { name: "Minh".into(), email: "minh@co.com".into() });
    users.save(2, User { name: "Lan".into(), email: "lan@co.com".into() });

    let mut products: InMemoryRepo<Product> = InMemoryRepo::new();
    products.save(1, Product { name: "Coffee".into(), price: 35_000 });

    println!("Users: {:?}", users.list());
    println!("Products: {:?}", products.list());
    println!("User 1: {:?}", users.get(1));
}
