fn main() {
    let mut names = vec!["Alice", "Bob", "Carol"];

    // Cách 1: Dùng first trước khi push
    let first = &names[0];
    println!("First: {}", first);  // dùng xong → borrow kết thúc (NLL)
    names.push("Dave");            // ✅ OK
    println!("All: {:?}", names);

    // Cách 2: Clone giá trị ra
    let first_owned = names[0].to_string();
    names.push("Eve");
    println!("First: {}, All: {:?}", first_owned, names);
}
