// filename: src/main.rs
fn main() {
    let message = String::from("Hello from closure!");

    // Không move: closure mượn &message
    let borrow_closure = || println!("{}", message);
    borrow_closure();
    println!("Still mine: {}", message);  // ✅ OK

    // Với move: closure SỞ HỮU message
    let move_closure = move || println!("{}", message);
    move_closure();
    // println!("{}", message);  // ❌ message đã bị move

    // Output:
    // Hello from closure!
    // Still mine: Hello from closure!
    // Hello from closure!
}
