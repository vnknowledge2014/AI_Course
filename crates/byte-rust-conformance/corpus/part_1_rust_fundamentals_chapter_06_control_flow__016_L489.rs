// filename: src/main.rs
fn main() {
    let matrix = vec![
        vec![1, 2, 3],
        vec![4, 5, 6],
        vec![7, 8, 9],
    ];

    let target = 5;
    let mut found = false;

    'outer: for (row, cols) in matrix.iter().enumerate() {
        for (col, &val) in cols.iter().enumerate() {
            if val == target {
                println!("Found {} at ({}, {})", target, row, col);
                found = true;
                break 'outer;  // thoát cả 2 vòng lặp!
            }
        }
    }

    if !found {
        println!("{} not found", target);
    }
    // Output: Found 5 at (1, 1)
}
