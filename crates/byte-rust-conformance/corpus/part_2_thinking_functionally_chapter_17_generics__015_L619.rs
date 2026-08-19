fn first_and_last<T: Clone>(items: &[T]) -> Option<(T, T)> {
    let first = items.first()?.clone();
    let last = items.last()?.clone();
    Some((first, last))
}

fn main() {
    println!("{:?}", first_and_last(&[1, 2, 3, 4, 5]));  // Some((1, 5))
    println!("{:?}", first_and_last(&["a", "b", "c"]));   // Some(("a", "c"))
    println!("{:?}", first_and_last::<i32>(&[]));          // None
}
