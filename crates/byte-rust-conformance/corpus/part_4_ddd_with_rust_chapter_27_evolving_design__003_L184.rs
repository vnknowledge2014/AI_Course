// filename: src/main.rs

// ═══ Core trait (stable, không đổi) ═══
trait Greeter {
    fn greet(&self) -> String;
}

// ═══ Extension trait (thêm features mới) ═══
trait GreeterExt: Greeter {
    fn greet_formal(&self) -> String {
        format!("Dear Sir/Madam, {}", self.greet())
    }

    fn greet_casual(&self) -> String {
        format!("Hey! {}", self.greet())
    }
}

// Blanket impl: MỌI Greeter tự động có GreeterExt
impl<T: Greeter> GreeterExt for T {}

struct User { name: String }

impl Greeter for User {
    fn greet(&self) -> String {
        format!("Hello, {}!", self.name)
    }
}

fn main() {
    let user = User { name: "Minh".into() };

    // Old API vẫn hoạt động
    println!("{}", user.greet());

    // New API tự động available
    println!("{}", user.greet_formal());
    println!("{}", user.greet_casual());
}
