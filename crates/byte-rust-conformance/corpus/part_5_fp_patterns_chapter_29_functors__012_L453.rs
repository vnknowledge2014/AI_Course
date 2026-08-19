#[derive(Debug, Clone, PartialEq)]
struct Pair<T>(T, T);

impl<T> Pair<T> {
    fn map<U, F: Fn(&T) -> U>(&self, f: F) -> Pair<U> {
        Pair(f(&self.0), f(&self.1))
    }
}

fn main() {
    let p = Pair(3, 7);
    println!("{:?}", p.map(|x| x * 2));  // Pair(6, 14)
    println!("{:?}", p.map(|x| x.to_string()));  // Pair("3", "7")

    // Law 1: Identity
    assert_eq!(p.map(|x| *x), Pair(3, 7));  // ✅

    // Law 2: Composition
    let f = |x: &i32| x * 2;
    let g = |x: &i32| x + 10;
    assert_eq!(p.map(f).map(g), p.map(|x| g(&f(x))));  // ✅
}
