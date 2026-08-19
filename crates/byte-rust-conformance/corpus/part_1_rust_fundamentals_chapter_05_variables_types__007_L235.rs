// filename: src/main.rs
fn main() {
    // Tuple: (kiểu1, kiểu2, kiểu3)
    let order: (&str, u32, bool) = ("Coffee", 35_000, true);

    // Truy cập bằng .0, .1, .2 (index)
    println!("Drink: {}, Price: {}đ, Paid: {}", order.0, order.1, order.2);

    // Destructuring — tách tuple thành biến riêng
    let (drink, price, paid) = order;
    println!("{}: {}đ (paid: {})", drink, price, paid);

    // Tuple 1 phần tử = cần dấu phẩy
    let single = (42,);  // tuple chứa 1 số
    let not_tuple = (42); // chỉ là số 42 trong ngoặc!
    println!("single: {:?}, not_tuple: {}", single, not_tuple);

    // Unit type () = tuple rỗng = "không có gì"
    let nothing: () = ();
    println!("Unit: {:?}", nothing);

    // Output:
    // Drink: Coffee, Price: 35000đ, Paid: true
    // Coffee: 35000đ (paid: true)
    // single: (42,), not_tuple: 42
    // Unit: ()
}
