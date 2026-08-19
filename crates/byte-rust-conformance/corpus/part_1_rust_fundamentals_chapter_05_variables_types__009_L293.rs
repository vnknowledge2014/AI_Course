// filename: src/main.rs
fn main() {
    // Array: [kiểu; kích_thước]
    let days: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];
    let zeros = [0; 5];  // [0, 0, 0, 0, 0] — lặp giá trị

    println!("Days: {:?}", days);
    println!("First day: {}", days[0]);
    println!("Length: {}", days.len());
    println!("Zeros: {:?}", zeros);

    // ⚠️ Array có kích thước cố định — CỐ ĐỊNH
    // let mut arr = [1, 2, 3];
    // arr.push(4);  // ❌ Không có push! Dùng Vec nếu cần grow

    // Slices — "mượn" một phần array
    let weekdays = &days[0..5]; // Mon-Fri
    let weekend = &days[5..7];  // Sat-Sun
    println!("Weekdays: {:?}", weekdays);
    println!("Weekend: {:?}", weekend);

    // Output:
    // Days: ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"]
    // First day: Mon
    // Length: 7
    // Zeros: [0, 0, 0, 0, 0]
    // Weekdays: ["Mon", "Tue", "Wed", "Thu", "Fri"]
    // Weekend: ["Sat", "Sun"]
}
