// map(x, |a| g(f(a))) == map(map(x, f), g)
// "Map f rồi map g == Map f∘g một lần"

fn main() {
    let x = Some(5);
    let f = |n: i32| n * 2;
    let g = |n: i32| n + 10;

    // Hai lần map
    let two_maps = x.map(f).map(g);

    // Một lần map (composed)
    let one_map = x.map(|n| g(f(n)));

    assert_eq!(two_maps, one_map);  // ✅ Some(20)
    println!("Law 2: {:?} == {:?}", two_maps, one_map);
}
