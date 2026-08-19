// ✅ Object-safe trait: CÓ THỂ dùng dyn
trait Drawable {
    fn draw(&self) -> String;  // OK: &self, trả String
    fn area(&self) -> f64;     // OK: &self, trả f64
}

// ❌ NOT object-safe: KHÔNG dùng dyn được
trait Clonable {
    fn clone_self(&self) -> Self;  // ❌ trả Self (unsized)
}

trait Generic {
    fn process<T>(&self, item: T);  // ❌ generic method
}

fn main() {}
