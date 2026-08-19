use std::time::Duration;

#[derive(Debug)]
enum Cached<T> {
    Fresh(T),
    Stale(T, Duration),
    Missing,
}

impl<T> Cached<T> {
    fn map<U, F: FnOnce(T) -> U>(self, f: F) -> Cached<U> {
        match self {
            Cached::Fresh(val) => Cached::Fresh(f(val)),
            Cached::Stale(val, age) => Cached::Stale(f(val), age),
            Cached::Missing => Cached::Missing,
        }
    }
}

// GATs version
trait Mappable {
    type Item;
    type Output<U>;
    fn map_items<U, F: FnOnce(Self::Item) -> U>(self, f: F) -> Self::Output<U>;
}

impl<T> Mappable for Cached<T> {
    type Item = T;
    type Output<U> = Cached<U>;

    fn map_items<U, F: FnOnce(T) -> U>(self, f: F) -> Cached<U> {
        self.map(f)
    }
}

fn main() {
    let fresh = Cached::Fresh(42);
    println!("{:?}", fresh.map(|x| x.to_string()));

    let stale = Cached::Stale(100, Duration::from_secs(60));
    println!("{:?}", stale.map(|x| x * 2));

    let missing: Cached<i32> = Cached::Missing;
    println!("{:?}", missing.map(|x| x + 1));
}
