// filename: src/applicative_traverse.rs

// Bí danh Applicative Validation
type Validated<T, E> = Result<T, Vec<E>>;

// Hàm zip quen thuộc từ Ch31b
fn zip_validated<A, B, E>(a: Validated<A, E>, b: Validated<B, E>) -> Validated<(A, B), E> {
    match (a, b) {
        (Ok(a), Ok(b)) => Ok((a, b)),
        (Err(mut ea), Err(mut eb)) => { ea.append(&mut eb); Err(ea) },
        (Err(e), _) | (_, Err(e)) => Err(e),
    }
}

// ── APPLICATIVE TRAVERSE ──
// Nhận vào một Iterator, và một hàm sinh ra Validated.
// Trả về Validated của Vec.
fn traverse_validated<I, T, U, E, F>(iter: I, f: F) -> Validated<Vec<U>, E>
where
    I: IntoIterator<Item = T>,
    F: Fn(T) -> Validated<U, E>,
{
    // Bắt đầu với một mảng rỗng thành công Ok(vec![])
    let init: Validated<Vec<U>, E> = Ok(Vec::new());

    // Fold qua Iterator
    iter.into_iter().fold(init, |acc_val, item| {
        // Áp dụng hàm f lên item
        let item_val = f(item);
        
        // Zip kết quả (Mảng tích lũy, Giá trị hiện tại)
        // Sau khi zip thành công, đẩy giá trị mới vào mảng
        zip_validated(acc_val, item_val).map(|(mut vec, new_item)| {
            vec.push(new_item);
            vec
        })
    })
}

// Hàm test
fn validate_even(n: i32) -> Validated<i32, String> {
    if n % 2 == 0 { Ok(n) } 
    else { Err(vec![format!("{} is odd", n)]) }
}

fn main() {
    let numbers = vec![2, 3, 4, 5, 6];
    
    // Thực hiện Applicative Traverse
    let result = traverse_validated(numbers, validate_even);
    
    // NÓ THU THẬP ĐƯỢC CẢ 2 LỖI!
    println!("{:#?}", result);
    // Err([
    //     "3 is odd",
    //     "5 is odd",
    // ])
}
