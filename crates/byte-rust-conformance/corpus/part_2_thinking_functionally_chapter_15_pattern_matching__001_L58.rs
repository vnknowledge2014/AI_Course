// filename: src/main.rs
fn main() {
    // 1. let = pattern
    let (x, y, z) = (1, 2, 3);
    let [first, _, last] = [10, 20, 30];
    println!("({}, {}, {}), [{}, {}]", x, y, z, first, last);

    // 2. function params = pattern
    fn distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
        ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
    }
    println!("Distance: {:.2}", distance((0.0, 0.0), (3.0, 4.0)));

    // 3. for = pattern
    let pairs = vec![(1, "one"), (2, "two"), (3, "three")];
    for (num, word) in &pairs {
        println!("{} = {}", num, word);
    }

    // 4. if let = pattern
    let config: Option<u16> = Some(8080);
    if let Some(port) = config {
        println!("Port: {}", port);
    }

    // 5. while let = pattern
    let mut stack = vec![1, 2, 3];
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();
}
