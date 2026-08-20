//! Vỏ Tauri cho ứng dụng Byte.
//!
//! Toàn bộ giao diện và nội dung nằm ở bản web đã build (`../dist`); vỏ này chỉ
//! làm hai việc: mở cửa sổ, và cho phần web gọi được trình kiểm Rust chạy
//! NATIVE thay vì qua WASM.
//!
//! Vì sao có hai đường chạy Rust: trên web thì `byte-rust` biên dịch sang WASM
//! (476 KB, nạp trong worker); trên desktop và mobile thì gọi thẳng crate này.
//! Cùng một `kiem_va_chay`, nên kết quả chấm bài giống hệt nhau ở cả bảy nền
//! tảng — đó là điều kiện bắt buộc: một bài đạt trên máy Mac mà trượt trên
//! Android thì người học không còn tin vào công cụ nữa.

/// Kiểm rồi chạy một đoạn Rust, trả về (đầu ra, chẩn đoán đã định dạng).
#[tauri::command]
fn chay_rust(nguon: String) -> (String, String, String) {
    let (ra, diags) = byte_rust::kiem_va_chay(&nguon);
    let ket_cuc = format!("{:?}", diags.ket_cuc());
    let bao = diags
        .iter()
        .map(|d| format!("[{}] {}", d.code, d.message))
        .collect::<Vec<_>>()
        .join("\n");
    (ket_cuc, ra, bao)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn chay() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chay_rust])
        .run(tauri::generate_context!())
        .expect("không khởi động được cửa sổ Byte");
}
