// filename: src/main.rs

fn main() {
    // Setup
    let f = |x: i32| if x > 0 { Some(x * 2) } else { None };
    let g = |x: i32| Some(x + 10);

    // ═══ Law 1: Left Identity ═══
    // return(a).and_then(f) == f(a)
    let a = 5;
    assert_eq!(Some(a).and_then(f), f(a));  // ✅
    println!("Law 1 (Left Identity): {:?} == {:?}", Some(a).and_then(f), f(a));

    // ═══ Law 2: Right Identity ═══
    // m.and_then(return) == m
    let m = Some(42);
    assert_eq!(m.and_then(Some), m);  // ✅
    println!("Law 2 (Right Identity): {:?} == {:?}", m.and_then(Some), m);

    // ═══ Law 3: Associativity ═══
    // m.and_then(f).and_then(g) == m.and_then(|x| f(x).and_then(g))
    let m = Some(5);
    let left = m.and_then(f).and_then(g);
    let right = m.and_then(|x| f(x).and_then(g));
    assert_eq!(left, right);  // ✅
    println!("Law 3 (Associativity): {:?} == {:?}", left, right);
}
