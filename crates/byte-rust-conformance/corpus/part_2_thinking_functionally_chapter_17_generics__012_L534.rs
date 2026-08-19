// filename: src/main.rs

// T: 'a = "T sống ít nhất bằng 'a"
struct Wrapper<'a, T: 'a> {
    data: &'a T,
}

impl<'a, T: 'a + std::fmt::Display> Wrapper<'a, T> {
    fn show(&self) {
        println!("Wrapped: {}", self.data);
    }
}

fn main() {
    let value = 42;
    let w = Wrapper { data: &value };
    w.show();  // Wrapped: 42
}
