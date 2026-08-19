// filename: src/main.rs

// Loại 1: Hàm không bao giờ lỗi (Pure transform)
// Đi thẳng làn trên
fn double(x: i32) -> i32 { x * 2 }
// 👉 Dùng công tắc: .map()

// Loại 2: Hàm có thể Lỗi (Nguy hiểm)
// Đang ở làn trên, có thể rớt xuống làn dưới
fn validate_positive(x: i32) -> Result<i32, String> {
    if x > 0 { Ok(x) } else { Err(format!("{} is not positive", x)) }
}
// 👉 Dùng công tắc: .and_then()

// Loại 3: Hàm sửa Lỗi
// Đi thẳng làn dưới (Nhặt rác và đóng gói lại)
fn add_context(err: String) -> String {
    format!("[Validation] {}", err)
}
// 👉 Dùng công tắc: .map_err()

fn main() {}
