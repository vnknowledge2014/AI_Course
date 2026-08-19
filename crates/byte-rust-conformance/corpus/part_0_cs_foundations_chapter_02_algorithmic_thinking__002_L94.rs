// filename: src/main.rs
fn main() {
    let mut numbers: Vec<i32> = Vec::new();

    // Vec bắt đầu với capacity = 0
    println!("Start: len={}, capacity={}", numbers.len(), numbers.capacity());

    for i in 0..20 {
        let old_cap = numbers.capacity();
        numbers.push(i);
        let new_cap = numbers.capacity();

        // Chỉ in khi capacity thay đổi (= resize xảy ra)
        if new_cap != old_cap {
            println!("  Push {}: capacity {} → {} (RESIZE!)", i, old_cap, new_cap);
        }
    }

    println!("Final: len={}, capacity={}", numbers.len(), numbers.capacity());

    // Output (ví dụ):
    // Start: len=0, capacity=0
    //   Push 0: capacity 0 → 4 (RESIZE!)
    //   Push 4: capacity 4 → 8 (RESIZE!)
    //   Push 8: capacity 8 → 16 (RESIZE!)
    //   Push 16: capacity 16 → 32 (RESIZE!)
    // Final: len=20, capacity=32
}
