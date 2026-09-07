//! Vỏ Tauri cho ứng dụng Byte.
//!
//! Toàn bộ giao diện và nội dung nằm ở bản web đã build (`../dist`); vỏ này chỉ
//! làm hai việc: mở cửa sổ, và cho phần web gọi được trình kiểm Rust chạy
//! NATIVE thay vì qua WASM.
//!
//! Vì sao có hai đường chạy Rust: trên web thì `byte-rust` biên dịch sang WASM
//! (476 KB, nạp trong worker); trên desktop và mobile thì gọi thẳng crate này
//! qua command `chay_rust`. Cả hai cùng đi qua MỘT hàm kết xuất
//! (`wasm::chay_thanh_json`), nên kết quả chấm bài giống hệt nhau ở mọi nền
//! tảng — đó là điều kiện bắt buộc: một bài đạt trên máy Mac mà trượt trên
//! Android thì người học không còn tin vào công cụ nữa.

/// Kiểm rồi chạy một đoạn Rust, trả về kết quả dưới dạng chuỗi JSON.
///
/// Trả về ĐÚNG chuỗi JSON mà bản WASM xuất qua `br_chay` (cùng một hàm
/// [`byte_rust::wasm::chay_thanh_json`] — một hàm, một định dạng), vì hai lý do
/// gắn liền nhau:
///
/// 1. **Khớp kết quả chấm theo cấu trúc.** Đường native và đường WASM không tự
///    diễn dịch `Diagnostics` mỗi nơi một kiểu nữa; phía TS dùng chung một hàm
///    `ketQuaTuThoRust` để đổi JSON này thành `KetQuaChay`, nên một bài ĐẠT hay
///    TRƯỢT không thể khác nhau giữa desktop và trình duyệt.
/// 2. **Không làm mất chẩn đoán ba tầng.** Bản cũ trả tuple `(kết cục, đầu ra,
///    diags-thành-text)` — text thì người đọc được nhưng app không dựng lại
///    được `dong`/`cot`/`vi_sao`/`cach_sua`, tức người học trên desktop sẽ mất
///    đúng thứ giàu nhất của bộ chấm.
#[tauri::command]
fn chay_rust(nguon: String) -> String {
    byte_rust::wasm::chay_thanh_json(&nguon)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn chay() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![chay_rust])
        .run(tauri::generate_context!())
        .expect("không khởi động được cửa sổ Byte");
}

#[cfg(test)]
mod tests {
    use super::*;

    // Command trả chuỗi của `chay_thanh_json` — hàm ấy đã có test riêng trong
    // crates/byte-rust/src/wasm.rs. Test ở đây khóa HỢP ĐỒNG mà phía TS dựa
    // vào: command trả ĐÚNG chuỗi JSON ấy, không bọc thêm, không cắt bớt.
    #[test]
    fn chay_rust_tra_dung_chuoi_json_cua_duong_wasm() {
        let nguon = r#"fn main() { println!("chào"); }"#;
        assert_eq!(chay_rust(nguon.into()), byte_rust::wasm::chay_thanh_json(nguon));
    }

    #[test]
    fn chay_rust_giu_nguyen_chan_doan_ba_tang_khi_loi() {
        let j = chay_rust("fn main() { let x = 1 / 0; }".into());
        assert!(j.contains("\"ok\":false"), "{j}");
        assert!(j.contains("\"ma\":\"BR0532\""), "{j}");
        assert!(j.contains("\"vi_sao\":\""), "{j}");
        assert!(j.contains("\"cach_sua\":[\""), "{j}");
    }
}
