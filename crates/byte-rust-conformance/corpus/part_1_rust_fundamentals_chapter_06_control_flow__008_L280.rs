// filename: src/main.rs
fn main() {
    let day = "Sat";

    let day_type = match day {
        "Mon" | "Tue" | "Wed" | "Thu" | "Fri" => "Weekday",
        "Sat" | "Sun" => "Weekend",
        _ => "Invalid",
    };

    println!("{}: {}", day, day_type);
    // Output: Sat: Weekend
}
