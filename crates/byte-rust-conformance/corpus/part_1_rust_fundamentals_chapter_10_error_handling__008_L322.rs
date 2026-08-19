// filename: src/main.rs

fn first_even(numbers: &[i32]) -> Option<i32> {
    let first = numbers.first()?;  // None → return None
    if first % 2 == 0 {
        Some(*first)
    } else {
        None
    }
}

fn main() {
    println!("{:?}", first_even(&[4, 1, 3]));   // Some(4)
    println!("{:?}", first_even(&[3, 1, 4]));   // None (3 is odd)
    println!("{:?}", first_even(&[]));           // None (empty)
}
