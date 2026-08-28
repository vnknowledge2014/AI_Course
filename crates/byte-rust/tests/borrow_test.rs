//! Test hồi quy cho `borrow_check.rs`.
//!
//! Ba ca đầu là ba lỗ NHẬN OAN cuối cùng của cổng đối chiếu `rustc` — mã mà
//! `rustc` từ chối còn `byte-rust` từng báo Đạt. Chúng ở đây để không tụt lại.
//!
//! Nhưng nửa dưới mới là nửa quan trọng hơn. Một bộ kiểm mượn viết ẩu thì dễ
//! làm cổng xanh bằng cách từ chối mọi thứ, và **từ chối oan còn tệ hơn nhận
//! oan** với người học: họ viết đúng mà máy nói sai, không có cách nào biết vì
//! sao. Nên mỗi luật đều có ca ĐỐI CHỨNG — mã hợp lệ mà `rustc` chấp nhận, và
//! `byte-rust` phải im lặng.

use byte_rust::interp::chay;
use byte_rust::span::SourceMap;

fn ma_loi(src: &str) -> String {
    let (_out, d) = chay(src);
    assert!(d.co_loi(), "mong đợi CÓ lỗi:\n{src}");
    // Buộc kết thúc lần mượn `d` TRƯỚC khi khối đóng.
    //
    // Viết thẳng chuỗi `.iter()...` làm biểu thức cuối khối thì chính `rustc`
    // báo E0597 — đúng cái lỗi mà tệp test này đang kiểm. Ghi lại vì nó là một
    // lời nhắc tốt: luật mượn không tha ai, kể cả người đang viết bộ kiểm mượn.
    let ma = d
        .iter()
        .find(|x| x.severity == byte_rust::Severity::Loi)
        .unwrap()
        .code
        .to_string();
    ma
}

fn khong_loi(src: &str) {
    let (_out, d) = chay(src);
    assert!(
        !d.co_loi(),
        "mã này hợp lệ với rustc, không được báo lỗi:\n{src}\n---\n{}",
        d.render(&SourceMap::new(src))
    );
}

// ── E0499: hai `&mut` cùng sống ─────────────────────────────────────────────

#[test]
fn hai_muon_mut_cung_song_la_loi() {
    assert_eq!(
        ma_loi(
            r#"fn main() {
    let mut v = vec![1];
    let a = &mut v;
    let b = &mut v;
    a.push(2);
    b.push(3);
}"#
        ),
        "BR0500"
    );
}

#[test]
fn hai_muon_mut_khong_chong_nhau_thi_khong_sao() {
    // NLL: lần mượn thứ nhất chết ngay sau lần dùng cuối của nó, nên lần mượn
    // thứ hai hoàn toàn hợp lệ. `rustc` cho qua — ta cũng phải cho qua.
    khong_loi(
        r#"fn main() {
    let mut v = vec![1];
    let a = &mut v;
    a.push(2);
    let b = &mut v;
    b.push(3);
}"#,
    );
}

// ── E0502: đọc biến khi đang bị mượn khả biến ───────────────────────────────

#[test]
fn doc_bien_dang_bi_muon_mut_la_loi() {
    assert_eq!(
        ma_loi(
            r#"fn main() {
    let mut v = vec![1, 2, 3];
    let r = &mut v;
    let n = v.len();
    r.push(4);
    println!("{}", n);
}"#
        ),
        "BR0501"
    );
}

#[test]
fn doc_bien_sau_khi_muon_da_chet_thi_khong_sao() {
    khong_loi(
        r#"fn main() {
    let mut v = vec![1, 2, 3];
    let r = &mut v;
    r.push(4);
    let n = v.len();
    println!("{}", n);
}"#,
    );
}

// ── E0597: tham chiếu sống lâu hơn thứ nó trỏ tới ───────────────────────────

#[test]
fn tham_chieu_song_lau_hon_la_loi() {
    assert_eq!(
        ma_loi(
            r#"fn main() {
    let r;
    {
        let x = 5;
        r = &x;
    }
    println!("{}", r);
}"#
        ),
        "BR0502"
    );
}

#[test]
fn tham_chieu_khong_dung_sau_khoi_thi_khong_sao() {
    // `r` không được dùng sau khi khối đóng, nên không có gì treo lơ lửng.
    khong_loi(
        r#"fn main() {
    let r;
    {
        let x = 5;
        r = &x;
        println!("{}", r);
    }
    println!("xong");
}"#,
    );
}

// ── Ranh giới: có nhánh thì module phải BỎ CUỘC, không đoán ─────────────────

#[test]
fn co_re_nhanh_thi_im_lang() {
    // Hai `&mut` nhưng có `if` chen giữa: thứ tự trong AST thôi không còn nói
    // được thứ tự thi hành. Bỏ cuộc là đúng — đoán ở đây là đoán bừa.
    khong_loi(
        r#"fn main() {
    let mut v = vec![1];
    let a = &mut v;
    a.push(2);
    if v.len() > 0 {
        let b = &mut v;
        b.push(3);
    }
}"#,
    );
}
