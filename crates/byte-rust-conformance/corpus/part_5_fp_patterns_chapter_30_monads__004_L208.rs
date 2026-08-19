// filename: src/main.rs

fn main() {
    // .map() = Functor: transform each element
    let nums = vec![1, 2, 3];
    let doubled: Vec<i32> = nums.iter().map(|x| x * 2).collect();
    println!("map: {:?}", doubled);  // [2, 4, 6]

    // .flat_map() = Monad: each element → multiple elements
    let expanded: Vec<i32> = nums.iter()
        .flat_map(|&x| vec![x, x * 10, x * 100])
        .collect();
    println!("flat_map: {:?}", expanded);  // [1, 10, 100, 2, 20, 200, 3, 30, 300]

    // Practical: generate combinations
    let colors = vec!["Red", "Blue"];
    let sizes = vec!["S", "M", "L"];

    // Cartesian product (List Monad!)
    let combos: Vec<String> = colors.iter()
        .flat_map(|&color| {
            sizes.iter().map(move |&size| format!("{}-{}", color, size))
        })
        .collect();
    println!("Combos: {:?}", combos);
    // ["Red-S", "Red-M", "Red-L", "Blue-S", "Blue-M", "Blue-L"]

    // flat_map with filtering
    fn divisors(n: u32) -> Vec<u32> {
        (1..=n).filter(|&d| n % d == 0).collect()
    }

    let all_divisors: Vec<(u32, u32)> = vec![6, 10, 15].iter()
        .flat_map(|&n| divisors(n).into_iter().map(move |d| (n, d)))
        .collect();
    println!("Divisors: {:?}", all_divisors);
}
