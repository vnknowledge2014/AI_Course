// filename: src/main.rs

mod cafe {
    pub struct Menu {
        pub name: String,       // pub: ai cũng thấy
        pub(crate) price: u32,  // pub(crate): chỉ trong crate này
        secret_recipe: String,  // private: chỉ module cafe
    }

    impl Menu {
        pub fn new(name: &str, price: u32) -> Self {
            Menu {
                name: name.to_string(),
                price,
                secret_recipe: "secret!".to_string(),
            }
        }

        // Private method — chỉ dùng trong module
        fn get_recipe(&self) -> &str {
            &self.secret_recipe
        }

        // Public method — dùng private fields bên trong
        pub fn describe(&self) -> String {
            format!("{}: {}đ (recipe: {})", self.name, self.price, self.get_recipe())
        }
    }
}

fn main() {
    let item = cafe::Menu::new("Latte", 45_000);

    // ✅ pub fields OK
    println!("Name: {}", item.name);
    println!("Price: {}", item.price);  // pub(crate) OK — cùng crate

    // ❌ Private field
    // println!("{}", item.secret_recipe);  // error: field is private
    // item.get_recipe();  // error: method is private

    // ✅ Public method truy cập private data
    println!("{}", item.describe());
}
