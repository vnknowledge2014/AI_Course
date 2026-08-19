// filename: src/main.rs
use std::rc::Rc;

// Persistent linked list dùng Rc (Reference Counting)
#[derive(Debug)]
enum List<T> {
    Nil,
    Cons(T, Rc<List<T>>),
}

impl<T: std::fmt::Debug> List<T> {
    fn new() -> Rc<List<T>> {
        Rc::new(List::Nil)
    }

    // Thêm phần tử vào đầu — O(1)!
    // Không copy list cũ, chỉ tạo node mới trỏ đến list cũ
    fn prepend(value: T, tail: &Rc<List<T>>) -> Rc<List<T>> {
        Rc::new(List::Cons(value, Rc::clone(tail)))
    }

    fn to_vec(list: &Rc<List<T>>) -> Vec<&T> {
        let mut result = vec![];
        let mut current = list.as_ref();
        loop {
            match current {
                List::Nil => break,
                List::Cons(val, next) => {
                    result.push(val);
                    current = next.as_ref();
                }
            }
        }
        result
    }
}

fn main() {
    // Tạo list: 3 → 2 → 1
    let list_v1 = List::prepend(1, &List::new());
    let list_v1 = List::prepend(2, &list_v1);
    let list_v1 = List::prepend(3, &list_v1);

    // Tạo bản mới: thêm 99 vào đầu
    // list_v1 KHÔNG bị thay đổi!
    let list_v2 = List::prepend(99, &list_v1);

    // Tạo bản khác: thêm 42 vào đầu list_v1
    let list_v3 = List::prepend(42, &list_v1);

    println!("V1: {:?}", List::to_vec(&list_v1));
    println!("V2: {:?}", List::to_vec(&list_v2));
    println!("V3: {:?}", List::to_vec(&list_v3));

    // Output:
    // V1: [3, 2, 1]
    // V2: [99, 3, 2, 1]
    // V3: [42, 3, 2, 1]

    // V2 và V3 CHIA SẺ phần đuôi [3, 2, 1] với V1!
    // Không copy — chỉ thêm 1 node mới mỗi bản.
}
