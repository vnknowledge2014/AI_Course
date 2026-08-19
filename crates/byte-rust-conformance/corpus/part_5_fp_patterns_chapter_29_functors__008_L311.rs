// filename: src/main.rs

// GATs (Generic Associated Types) — since Rust 1.65
trait Mappable {
    type Item;
    type Output<U>;

    fn map_items<U, F: Fn(Self::Item) -> U>(self, f: F) -> Self::Output<U>;
}

impl<T> Mappable for Option<T> {
    type Item = T;
    type Output<U> = Option<U>;

    fn map_items<U, F: Fn(T) -> U>(self, f: F) -> Option<U> {
        self.map(f)
    }
}

impl<T> Mappable for Vec<T> {
    type Item = T;
    type Output<U> = Vec<U>;

    fn map_items<U, F: Fn(T) -> U>(self, f: F) -> Vec<U> {
        self.into_iter().map(f).collect()
    }
}

impl<T, E> Mappable for Result<T, E> {
    type Item = T;
    type Output<U> = Result<U, E>;

    fn map_items<U, F: Fn(T) -> U>(self, f: F) -> Result<U, E> {
        self.map(f)
    }
}

// Bây giờ có thể viết generic function!
fn stringify<C: Mappable<Item = i32>>(container: C) -> C::Output<String> {
    container.map_items(|x| format!("#{}", x))
}

fn main() {
    println!("{:?}", stringify(Some(42)));       // Some("#42")
    println!("{:?}", stringify(vec![1, 2, 3]));  // ["#1", "#2", "#3"]
    println!("{:?}", stringify(Ok::<i32, String>(7)));  // Ok("#7")
}
