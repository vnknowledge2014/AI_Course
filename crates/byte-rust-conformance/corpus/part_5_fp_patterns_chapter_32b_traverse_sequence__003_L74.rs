// filename: src/rust_traverse.rs

fn parse_even(s: &str) -> Result<i32, String> {
    let n = s.parse::<i32>().map_err(|e| e.to_string())?;
    if n % 2 == 0 {
        Ok(n)
    } else {
        Err(format!("Số {} không phải số chẵn", n))
    }
}

fn main() {
    let inputs = vec!["2", "4", "6"];
    
    // TRAVERSE!
    // Bước 1: .map(parse_even) sinh ra Iterator của Result
    // Bước 2: .collect() lộn ngược Iterator đó thành Result<Vec>
    let result: Result<Vec<i32>, String> = inputs
        .into_iter()
        .map(parse_even)
        .collect();
        
    assert_eq!(result, Ok(vec![2, 4, 6]));

    // Thử với mảng có lỗi
    let bad_inputs = vec!["2", "abc", "5"];
    let bad_result: Result<Vec<i32>, String> = bad_inputs
        .into_iter()
        .map(parse_even)
        .collect();
        
    // Dừng lại ngay lỗi đầu tiên (ParseIntError)
    assert_eq!(bad_result, Err("invalid digit found in string".to_string()));
}
