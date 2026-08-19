// filename: src/main.rs
use std::fmt;

trait Describable {
    fn describe(&self) -> String;
}

// Trait bound: T phải implement Describable
fn print_description<T: Describable>(item: &T) {
    println!("→ {}", item.describe());
}

// Multiple bounds: T phải implement CẢ Describable VÀ Clone
fn clone_and_describe<T: Describable + Clone>(item: &T) -> String {
    let cloned = item.clone();
    cloned.describe()
}

// Where clause — dễ đọc hơn khi nhiều bounds
fn process<T>(item: &T) -> String
where
    T: Describable + Clone + fmt::Debug,
{
    format!("{:?} — {}", item, item.describe())
}

#[derive(Debug, Clone)]
struct Product {
    name: String,
    price: u32,
}

impl Describable for Product {
    fn describe(&self) -> String {
        format!("{}: {}đ", self.name, self.price)
    }
}

fn main() {
    let laptop = Product { name: "Laptop".into(), price: 25_000_000 };
    print_description(&laptop);
    println!("{}", clone_and_describe(&laptop));
    println!("{}", process(&laptop));
}
