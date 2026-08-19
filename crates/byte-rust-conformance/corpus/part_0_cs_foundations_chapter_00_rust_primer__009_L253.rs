#[derive(Debug)]
enum Season { Spring, Summer, Fall, Winter }

fn describe(season: &Season) -> &str {
    match season {
        Season::Spring => "Hoa nở 🌸",
        Season::Summer => "Nắng nóng ☀️",
        Season::Fall   => "Lá rụng 🍂",
        Season::Winter => "Lạnh giá ❄️",
    }
    // Nếu bạn vô tình quên mất mùa Đông, trình biên dịch sẽ chặn đứng bạn ngay lập tức!
}

fn main() {
    let now = Season::Summer;
    println!("{}: {}", format!("{:?}", now), describe(&now));
    // Output: Summer: Nắng nóng ☀️
}
