// filename: src/main.rs

fn add_exclamation(s: &mut String) {
    s.push_str("!!!");  // sửa data — cần &mut
}

fn main() {
    let mut message = String::from("Hello");

    add_exclamation(&mut message);
    println!("{}", message);  // Hello!!!

    // ⚠️ Chỉ 1 mutable borrow tại 1 thời điểm!
    // let r1 = &mut message;
    // let r2 = &mut message;  // ❌ cannot borrow as mutable more than once

    // ⚠️ Không thể có &T và &mut T cùng lúc!
    // let r1 = &message;
    // let r2 = &mut message;  // ❌ cannot borrow as mutable — already borrowed as shared
}
