// map(x, |a| a) == x
// "Map với identity function → không đổi"

fn main() {
    let x = Some(42);
    assert_eq!(x.map(|a| a), x);  // ✅

    let v = vec![1, 2, 3];
    let same: Vec<_> = v.iter().map(|&a| a).collect();
    assert_eq!(same, vec![1, 2, 3]);  // ✅
}
