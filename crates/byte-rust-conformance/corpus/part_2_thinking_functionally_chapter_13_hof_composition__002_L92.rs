// filename: src/main.rs

type Filter = Box<dyn Fn(&i32) -> bool>;

fn build_filters(min: i32, max: i32) -> Vec<Filter> {
    vec![
        Box::new(move |x| *x >= min),      // >= min
        Box::new(move |x| *x <= max),      // <= max
        Box::new(|x| *x % 2 == 0),         // chẵn
    ]
}

fn apply_filters(data: &[i32], filters: &[Filter]) -> Vec<i32> {
    data.iter()
        .filter(|x| filters.iter().all(|f| f(x)))
        .copied()
        .collect()
}

fn main() {
    let data = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let filters = build_filters(3, 10);

    let result = apply_filters(&data, &filters);
    println!("3 <= x <= 10, even: {:?}", result);
    // Output: 3 <= x <= 10, even: [4, 6, 8, 10]
}
