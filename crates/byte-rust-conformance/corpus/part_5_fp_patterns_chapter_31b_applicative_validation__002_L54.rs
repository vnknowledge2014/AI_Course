// filename: src/applicative_zip.rs

// Khai báo một bí danh (alias) cho gọn
type Validated<T, E> = Result<T, Vec<E>>;

// Hàm gộp hai Validated độc lập
fn zip_validated<A, B, E>(
    val_a: Validated<A, E>,
    val_b: Validated<B, E>,
) -> Validated<(A, B), E> {
    match (val_a, val_b) {
        // Cả hai OK -> Gộp thành Tuple
        (Ok(a), Ok(b)) => Ok((a, b)),
        
        // Cả hai LỖI -> Nối 2 mảng lỗi lại với nhau (Thu thập lỗi)
        (Err(mut errs_a), Err(mut errs_b)) => {
            errs_a.append(&mut errs_b);
            Err(errs_a)
        }
        
        // Một trong hai lỗi -> Giữ nguyên lỗi đó
        (Err(errs), Ok(_)) => Err(errs),
        (Ok(_), Err(errs)) => Err(errs),
    }
}

// Giả lập các hàm validation độc lập
fn val_name(name: &str) -> Validated<String, String> {
    if name.len() >= 3 { Ok(name.to_string()) } 
    else { Err(vec!["Name too short".to_string()]) }
}

fn val_age(age: i32) -> Validated<i32, String> {
    if age >= 18 { Ok(age) } 
    else { Err(vec!["Too young".to_string()]) }
}

fn main() {
    let result_ok = zip_validated(val_name("Alice"), val_age(20));
    assert_eq!(result_ok, Ok(("Alice".to_string(), 20)));

    let result_err = zip_validated(val_name("Bo"), val_age(15));
    // THU THẬP ĐƯỢC CẢ 2 LỖI!
    assert_eq!(result_err, Err(vec![
        "Name too short".to_string(), 
        "Too young".to_string()
    ]));
}
