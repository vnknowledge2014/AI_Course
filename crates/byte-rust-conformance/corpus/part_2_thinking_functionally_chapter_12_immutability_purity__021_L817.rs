fn main() {
    let items = vec!["apple", "banana", "cherry"];

    // Pipeline: transform mà không mutate
    let mut extended = items.clone();
    extended.push("date");
    extended.sort();

    let result: String = extended.iter()
        .filter(|i| i.len() > 5)
        .cloned()
        .collect::<Vec<_>>()
        .join(", ");

    println!("{}", result);            // banana, cherry
    println!("Original: {:?}", items); // vẫn nguyên
}
