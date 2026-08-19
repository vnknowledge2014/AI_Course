// filename: src/rust_sequence.rs

fn main() {
    // 1. Mảng toàn Ok
    let oks: Vec<Result<i32, &str>> = vec![Ok(1), Ok(2), Ok(3)];
    
    // SEQUENCE: Ép kiểu đích là Result<Vec<i32>, &str>
    // collect() tự động gom Ok lại, trả về Ok(Vec)
    let sequenced_oks: Result<Vec<i32>, &str> = oks.into_iter().collect();
    assert_eq!(sequenced_oks, Ok(vec![1, 2, 3]));

    // 2. Mảng có chứa Err
    let errs: Vec<Result<i32, &str>> = vec![Ok(1), Err("Lỗi ở đây!"), Ok(3)];
    
    // SEQUENCE fail-fast: Nó sẽ dừng ngay khi gặp Err đầu tiên!
    let sequenced_errs: Result<Vec<i32>, &str> = errs.into_iter().collect();
    assert_eq!(sequenced_errs, Err("Lỗi ở đây!"));
    
    println!("Sequence OK ✅");
}
