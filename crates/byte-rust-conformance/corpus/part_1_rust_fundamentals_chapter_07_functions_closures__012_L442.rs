// filename: src/main.rs

fn compose<A, B, C, F, G>(f: F, g: G) -> impl Fn(A) -> C
where
    F: Fn(A) -> B,
    G: Fn(B) -> C,
{
    move |x| g(f(x))
}

fn main() {
    let add_one = |x: i32| x + 1;
    let double = |x: i32| x * 2;
    let to_string = |x: i32| format!("Result: {}", x);

    // Compose: add_one → double
    let add_then_double = compose(add_one, double);
    println!("{}", add_then_double(5));  // (5+1)*2 = 12

    // Compose tiếp: add_one → double → to_string
    let pipeline = compose(add_then_double, to_string);
    println!("{}", pipeline(5));  // "Result: 12"

    // Output:
    // 12
    // Result: 12
}
