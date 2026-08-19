// filename: src/main.rs
fn main() {
    let temperature = 35;

    let description = match temperature {
        t if t < 0   => "Đóng băng 🥶",
        t if t < 15  => "Lạnh 🧥",
        t if t < 25  => "Mát mẻ 😊",
        t if t < 35  => "Nóng ☀️",
        _            => "Rất nóng 🔥",
    };

    println!("{}°C: {}", temperature, description);
    // Output: 35°C: Rất nóng 🔥
}
