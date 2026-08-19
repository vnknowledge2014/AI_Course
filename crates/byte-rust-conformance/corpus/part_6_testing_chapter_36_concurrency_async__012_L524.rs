use std::thread;

fn main() {
    let data: Vec<i64> = (1..=1000).collect();
    let chunks: Vec<Vec<i64>> = data.chunks(250).map(|c| c.to_vec()).collect();

    let handles: Vec<_> = chunks.into_iter()
        .map(|chunk| thread::spawn(move || chunk.iter().sum::<i64>()))
        .collect();

    let total: i64 = handles.into_iter().map(|h| h.join().unwrap()).sum();
    println!("Total: {}", total); // 500500
}
