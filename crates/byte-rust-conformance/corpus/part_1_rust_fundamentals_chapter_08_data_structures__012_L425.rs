// filename: src/main.rs
fn main() {
    let mut names = vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Carol"),
    ];

    // .iter() → &T (shared reference, không sửa, không lấy)
    for name in names.iter() {
        println!("Hello, {}!", name);  // mượn đọc
    }
    println!("names still exists: {:?}", names);  // ✅

    // .iter_mut() → &mut T (mutable reference, sửa được)
    for name in names.iter_mut() {
        *name = name.to_uppercase();
    }
    println!("Uppercased: {:?}", names);

    // .into_iter() → T (lấy ownership, consume collection)
    for name in names.into_iter() {
        println!("Got: {}", name);
    }
    // println!("{:?}", names);  // ❌ names đã bị consumed!
}
