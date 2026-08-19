// filename: src/main.rs
fn main() {
    let add_one = |x: i32| x + 1;

    // Mỗi dòng đều là β-reduction — thay x vào rồi tính:
    println!("add_one(5) = {}", add_one(5));    // thay x=5 → 6
    println!("add_one(0) = {}", add_one(0));    // thay x=0 → 1
    println!("add_one(99) = {}", add_one(99));  // thay x=99 → 100

    // Output:
    // add_one(5) = 6
    // add_one(0) = 1
    // add_one(99) = 100
}
