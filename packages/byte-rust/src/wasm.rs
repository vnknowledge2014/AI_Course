//! Giao diện WASM — cầu nối giữa interpreter và app.
//!
//! Cố ý dùng C ABI trần thay vì `wasm-bindgen`: không thêm dependency, không
//! sinh mã keo, và module chạy được ở mọi host (trình duyệt, WKWebView của iOS,
//! WebView của Android, Node) mà không cần công cụ đi kèm.
//!
//! Giao thức:
//! ```text
//!   ptr  = br_cap_phat(len)          // xin vùng nhớ cho mã nguồn
//!   ...ghi UTF-8 vào [ptr, ptr+len)
//!   gói = br_chay(ptr, len)          // trả về (ptr<<32 | len) của JSON kết quả
//!   ...đọc JSON ở phần cao/thấp của `gói`
//!   br_giai_phong(ptr_json, len_json)
//!   br_giai_phong(ptr, len)
//! ```

use crate::diag::Severity;
use crate::span::SourceMap;

/// Xin một vùng nhớ để host ghi dữ liệu vào.
///
/// # Safety
/// Host phải trả lại đúng cặp `(ptr, len)` cho [`br_giai_phong`].
#[no_mangle]
pub extern "C" fn br_cap_phat(len: usize) -> *mut u8 {
    let mut v = Vec::<u8>::with_capacity(len);
    let p = v.as_mut_ptr();
    core::mem::forget(v);
    p
}

/// Trả lại vùng nhớ đã xin.
///
/// # Safety
/// `ptr` phải đến từ [`br_cap_phat`] với đúng `len` đó, và chỉ gọi một lần.
#[no_mangle]
pub unsafe extern "C" fn br_giai_phong(ptr: *mut u8, len: usize) {
    if !ptr.is_null() && len > 0 {
        drop(Vec::from_raw_parts(ptr, 0, len));
    }
}

/// Chạy một chương trình, trả về kết quả dạng JSON.
///
/// Giá trị trả về đóng gói con trỏ và độ dài: `(ptr as u64) << 32 | len as u64`.
/// Cách này tránh phải có thêm một hàm để hỏi độ dài, nên host chỉ cần một lời gọi.
///
/// # Safety
/// `[ptr, ptr+len)` phải là UTF-8 hợp lệ và còn sống trong suốt lời gọi.
#[no_mangle]
pub unsafe extern "C" fn br_chay(ptr: *const u8, len: usize) -> u64 {
    let src = match core::str::from_utf8(core::slice::from_raw_parts(ptr, len)) {
        Ok(s) => s,
        Err(_) => {
            return dong_goi(
                r#"{"ok":false,"xuat":"","chan_doan":[{"ma":"BR0000","muc":"loi","thong_diep":"mã nguồn không phải UTF-8 hợp lệ","dong":1,"cot":1,"do_dai":0,"vi_sao":null,"cach_sua":[],"khai_niem":null,"van_ban":""}]}"#
                    .to_string(),
            )
        }
    };
    dong_goi(chay_thanh_json(src))
}

fn dong_goi(s: String) -> u64 {
    let bytes = s.into_bytes();
    let len = bytes.len();
    let mut b = bytes.into_boxed_slice();
    let p = b.as_mut_ptr();
    core::mem::forget(b);
    ((p as u32 as u64) << 32) | (len as u64)
}

/// Chạy và trả kết quả dạng JSON.
///
/// Tách riêng khỏi `br_chay` để test được mà không cần đụng con trỏ thô.
pub fn chay_thanh_json(src: &str) -> String {
    let sm = SourceMap::new(src);
    let (xuat, diags) = crate::interp::chay(src);
    let diags = diags.rut_gon();

    let mut ra = String::with_capacity(512);
    ra.push_str("{\"ok\":");
    ra.push_str(if diags.co_loi() { "false" } else { "true" });
    ra.push_str(",\"xuat\":");
    json_chuoi(&mut ra, &xuat);
    ra.push_str(",\"chan_doan\":[");

    for (i, d) in diags.iter().enumerate() {
        if i > 0 {
            ra.push(',');
        }
        let span = d.span_chinh().unwrap_or_default();
        let lc = sm.line_col(span.start);
        let do_dai = sm.snippet(span).chars().count().max(1);

        ra.push_str("{\"ma\":\"");
        ra.push_str(d.code);
        ra.push_str("\",\"muc\":\"");
        ra.push_str(match d.severity {
            Severity::Loi => "loi",
            Severity::CanhBao => "canh_bao",
        });
        ra.push_str("\",\"thong_diep\":");
        json_chuoi(&mut ra, &d.message);
        ra.push_str(",\"dong\":");
        ra.push_str(&lc.line.to_string());
        ra.push_str(",\"cot\":");
        ra.push_str(&lc.col.to_string());
        ra.push_str(",\"do_dai\":");
        ra.push_str(&do_dai.to_string());

        ra.push_str(",\"vi_sao\":");
        match &d.vi_sao {
            Some(v) => json_chuoi(&mut ra, v),
            None => ra.push_str("null"),
        }

        ra.push_str(",\"cach_sua\":[");
        for (j, s) in d.cach_sua.iter().enumerate() {
            if j > 0 {
                ra.push(',');
            }
            json_chuoi(&mut ra, s);
        }
        ra.push(']');

        ra.push_str(",\"khai_niem\":");
        match d.khai_niem {
            Some(k) => json_chuoi(&mut ra, k),
            None => ra.push_str("null"),
        }

        // Bản kết xuất đầy đủ, để app hiện nguyên khối nếu muốn.
        ra.push_str(",\"van_ban\":");
        json_chuoi(&mut ra, &d.render(&sm));
        ra.push('}');
    }
    ra.push_str("]}");
    ra
}

/// Ghi một chuỗi JSON hợp lệ. Tự viết để crate không cần dependency nào.
fn json_chuoi(ra: &mut String, s: &str) {
    ra.push('"');
    for c in s.chars() {
        match c {
            '"' => ra.push_str("\\\""),
            '\\' => ra.push_str("\\\\"),
            '\n' => ra.push_str("\\n"),
            '\r' => ra.push_str("\\r"),
            '\t' => ra.push_str("\\t"),
            c if (c as u32) < 0x20 => {
                ra.push_str(&format!("\\u{:04x}", c as u32));
            }
            // Ký tự tiếng Việt để nguyên: JSON là UTF-8, không cần thoát.
            c => ra.push(c),
        }
    }
    ra.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn chuong_trinh_dung_cho_ok_true() {
        let j = chay_thanh_json(r#"fn main() { println!("chào"); }"#);
        assert!(j.contains("\"ok\":true"), "{j}");
        assert!(j.contains("\"xuat\":\"chào\\n\""), "{j}");
        assert!(j.contains("\"chan_doan\":[]"), "{j}");
    }

    #[test]
    fn loi_co_du_toa_do_va_ba_tang() {
        let j = chay_thanh_json("fn main() { let x = 1 / 0; }");
        assert!(j.contains("\"ok\":false"), "{j}");
        assert!(j.contains("\"ma\":\"BR0532\""), "{j}");
        assert!(j.contains("\"dong\":1"), "{j}");
        assert!(j.contains("\"vi_sao\":\""), "{j}");
        assert!(j.contains("\"cach_sua\":[\""), "{j}");
        assert!(j.contains("\"khai_niem\":\"phép chia\""), "{j}");
    }

    #[test]
    fn json_thoat_dung_va_giu_tieng_viet() {
        let mut s = String::new();
        json_chuoi(&mut s, "dòng\nmới \"trích\" \\ xong");
        assert_eq!(s, r#""dòng\nmới \"trích\" \\ xong""#);
    }

    #[test]
    fn json_luon_phan_tich_duoc_ke_ca_khi_loi_chua_ky_tu_dac_biet() {
        // Thông báo lỗi có chứa dấu nháy và xuống dòng — dễ làm hỏng JSON.
        for src in [
            r#"fn main() { let s = "chưa đóng; }"#,
            "fn main() { let v = vec![1]; v[9]; }",
            "fn main() { khong_co_ham(); }",
            "fn main() { let a = String::from(\"x\"); let b = a; println!(\"{}\", a); }",
        ] {
            let j = chay_thanh_json(src);
            kiem_tra_json_hop_le(&j);
        }
    }

    /// Kiểm tra thô: đếm ngoặc và nháy ngoài chuỗi phải cân.
    fn kiem_tra_json_hop_le(j: &str) {
        let mut trong_chuoi = false;
        let mut thoat = false;
        let mut nhon = 0i32;
        let mut vuong = 0i32;
        for c in j.chars() {
            if thoat {
                thoat = false;
                continue;
            }
            match c {
                '\\' if trong_chuoi => thoat = true,
                '"' => trong_chuoi = !trong_chuoi,
                '{' if !trong_chuoi => nhon += 1,
                '}' if !trong_chuoi => nhon -= 1,
                '[' if !trong_chuoi => vuong += 1,
                ']' if !trong_chuoi => vuong -= 1,
                _ => {}
            }
            assert!(nhon >= 0 && vuong >= 0, "ngoặc âm trong: {j}");
        }
        assert!(!trong_chuoi, "chuỗi chưa đóng trong: {j}");
        assert_eq!(nhon, 0, "ngoặc nhọn lệch: {j}");
        assert_eq!(vuong, 0, "ngoặc vuông lệch: {j}");
    }
}
