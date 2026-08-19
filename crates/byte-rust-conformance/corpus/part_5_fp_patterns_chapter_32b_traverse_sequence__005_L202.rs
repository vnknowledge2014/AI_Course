// Tưởng tượng bạn đang gọi API để lấy thông tin của danh sách User IDs.
fn fetch_user(id: u32) -> Result<String, String> {
    if id == 404 {
        Err(format!("User {} not found", id))
    } else if id == 500 {
        Err(format!("Server error on {}", id))
    } else {
        Ok(format!("User {}", id))
    }
}

// YÊU CẦU:
// Hãy viết một hàm `fetch_all_monadic(ids: Vec<u32>) -> Result<Vec<String>, String>`
// dùng `.into_iter().map().collect()` (Fail-fast).
// Test nó với mảng `vec![1, 404, 500]`. Lỗi nào sẽ được trả về?

fn main() {}
