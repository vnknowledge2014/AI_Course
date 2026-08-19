//! # byte-rust
//!
//! Interpreter cho **tập con Rust dùng trong giảng dạy** của Byte Academy.
//!
//! Xem `docs/decisions/ADR-001-rust-runtime.md` để biết vì sao không dùng `rustc`.
//!
//! Mục tiêu thiết kế, theo thứ tự ưu tiên:
//!
//! 1. **Thông báo lỗi dạy được** — quan trọng hơn cả tính đầy đủ của ngôn ngữ.
//!    Mỗi lỗi nói rõ chuyện gì xảy ra, vì sao Rust không cho, và sửa thế nào.
//! 2. **Chạy được ở mọi nơi** — biên dịch `wasm32-unknown-unknown`, không dependency,
//!    không I/O hệ thống.
//! 3. **Trung thực** — thà báo "chưa hỗ trợ" còn hơn chạy sai. Không bao giờ
//!    trả về "pass" cho code mà ta không thực sự hiểu.

pub mod ast;
pub mod diag;
pub mod interp;
pub mod lexer;
pub mod move_check;
pub mod mut_check;
pub mod parser;
pub mod span;
pub mod tyck;
pub mod value;
pub mod wasm;

pub use diag::{Diagnostic, Diagnostics, Severity};
pub use lexer::{Token, TokKind, TuKhoa};
pub use span::{SourceMap, Span};

/// Kết quả một lần chạy chương trình của người học.
#[derive(Debug, Clone, Default)]
pub struct KetQua {
    /// Những gì chương trình in ra.
    pub xuat: String,
    /// Chẩn đoán đã kết xuất thành text.
    pub chan_doan: String,
    /// Có lỗi chặn việc chạy hay không.
    pub co_loi: bool,
}

/// **Điểm vào duy nhất** của toàn bộ pipeline: phân tích → kiểm → chạy.
///
/// Mọi nơi cần chạy code của người học đều phải đi qua đây — app, bộ đối chiếu
/// `rustc`, test. Trước đây bộ đối chiếu tự dựng lại chuỗi gọi của riêng nó, và
/// khi thêm `mut_check` thì nó lặng lẽ đo một pipeline KHÁC với pipeline mà app
/// chạy. Cổng merge đo nhầm thứ còn tệ hơn không có cổng.
///
/// Thứ tự các pha là cố ý:
/// 1. **parse** — sai cú pháp thì mọi phân tích sau đều vô nghĩa
/// 2. **tyck** — kiểu và tính vét cạn của `match`
/// 3. **move_check** — chuyển quyền sở hữu
/// 4. **mut_check** — tính khả biến
/// 5. **interp** — chỉ chạy khi mọi pha tĩnh đều sạch, đúng như một compiler
pub fn kiem_va_chay(src: &str) -> (String, Diagnostics) {
    let (ct, mut d) = parser::phan_tich(src);
    if d.co_loi() {
        return (String::new(), d.rut_gon());
    }

    tyck::kiem_tra(&ct, &mut d);
    move_check::kiem_tra(&ct, &mut d);
    mut_check::kiem_tra(&ct, &mut d);
    if d.co_loi() || d.co_chua_ho_tro() {
        return (String::new(), d.rut_gon());
    }

    let mut may = interp::MayChay::moi();
    match may.chay(&ct) {
        Ok(()) => (may.xuat, d),
        Err(loi) => {
            let xuat = may.xuat.clone();
            d.push(loi);
            (xuat, d)
        }
    }
}

/// Quét mã nguồn thành token — bước 1 của pipeline.
///
/// Hiện tại đây là toàn bộ những gì đã cài đặt; parser và interpreter đang được
/// xây tiếp. Hàm này đã dùng được để kiểm tra cú pháp mức từ vựng.
pub fn kiem_tra_tu_vung(src: &str) -> KetQua {
    let sm = SourceMap::new(src);
    let (_toks, diags) = lexer::quet(src);
    let diags = diags.rut_gon();
    KetQua {
        xuat: String::new(),
        chan_doan: diags.render(&sm),
        co_loi: diags.co_loi(),
    }
}
