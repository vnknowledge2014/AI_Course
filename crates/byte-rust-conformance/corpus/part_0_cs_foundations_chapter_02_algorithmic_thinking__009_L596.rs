// a) 
fn sum(items: &[i32]) -> i32 {
    items.iter().sum()
}

// b)
fn print_pairs(items: &[i32]) {
    for i in items {
        for j in items {
            println!("{} {}", i, j);
        }
    }
}

// c)
fn binary_search_stdlib(sorted: &[i32], target: &i32) -> bool {
    sorted.binary_search(target).is_ok()
}

fn main() {}
