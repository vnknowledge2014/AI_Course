// filename: src/main.rs

// ❌ SAI — dùng 4 cái bool
struct OrderBad {
    is_confirmed: bool,
    is_preparing: bool,
    is_ready: bool,
    is_delivered: bool,
}

fn main() {
    // Mỗi bool có 2 giá trị → 2 × 2 × 2 × 2 = 16 trạng thái
    println!("Total states with 4 bools: {}", 2_u32.pow(4));
    // Output: Total states with 4 bools: 16

    // Nhưng chỉ ~4 trạng thái HỢP LÝ:
    // 1. (true,  false, false, false) — mới xác nhận
    // 2. (true,  true,  false, false) — đang pha
    // 3. (true,  true,  true,  false) — sẵn sàng
    // 4. (true,  true,  true,  true)  — đã giao

    // 16 - 4 = 12 trạng thái VÔ NGHĨA! Ví dụ:
    let nonsense = OrderBad {
        is_confirmed: false,
        is_preparing: false,
        is_ready: false,
        is_delivered: true, // 🤯 Giao cho ai khi chưa ai đặt?!
    };

    println!("is_delivered={} but is_confirmed={} → VÔ LÝ!",
        nonsense.is_delivered, nonsense.is_confirmed);
    // Output: is_delivered=true but is_confirmed=false → VÔ LÝ!
}
