//! Sinh corpus **âm** bằng đột biến cơ học.
//!
//! Đây mới là phần đo được điều quan trọng. Chạy engine trên toàn code đúng chỉ
//! chứng minh "code đúng thì báo đúng" — điều mà cả một hàm `return true` cũng
//! làm được. Chế độ hỏng nguy hiểm là **false accept**: báo Đạt cho code mà
//! `rustc` từ chối. Muốn đo nó thì phải có code sai, và phải biết chắc nó sai.
//!
//! Nên: lấy code đúng, làm hỏng nó theo cách máy móc, rồi hỏi `rustc` thật xem
//! có hỏng không. `rustc` là trọng tài, không phải ta.

#[derive(Debug, Clone)]
pub struct DotBien {
    pub hat_giong: String,
    pub kieu: &'static str,
    pub ma: String,
}

/// Mọi phép đột biến áp dụng được cho một chương trình.
pub fn sinh(ten_hat: &str, ma: &str) -> Vec<DotBien> {
    let mut ra = Vec::new();
    let mut them = |kieu: &'static str, m: String| {
        if m != ma {
            ra.push(DotBien { hat_giong: ten_hat.to_string(), kieu, ma: m });
        }
    };

    // 1. Xoá `.clone()` — biến mượn-an-toàn thành move.
    if ma.contains(".clone()") {
        them("xoa_clone", ma.replacen(".clone()", "", 1));
    }

    // 2. Xoá `&` khi truyền tham số — biến mượn thành move.
    if let Some(i) = ma.find("(&") {
        let mut m = ma.to_string();
        m.replace_range(i + 1..i + 2, "");
        them("xoa_muon", m);
    }

    // 3. Xoá `&` trong khai báo kiểu tham số.
    if ma.contains(": &String") {
        them("xoa_muon_kieu", ma.replacen(": &String", ": String", 1));
    }

    // 4. Dùng lại biến sau khi đã move — chèn ngay trước `}` cuối của main.
    if let Some(ten_bien) = tim_bien_khong_copy(ma) {
        if let Some(i) = ma.rfind('}') {
            if let Some(j) = ma[..i].rfind('}') {
                let mut m = ma.to_string();
                m.insert_str(j, &format!("    println!(\"{{}}\", {ten_bien});\n"));
                them("dung_sau_move", m);
            }
        }
    }

    // 5. Xoá một nhánh `match` — phá tính vét cạn.
    if let Some(i) = ma.find("        None => 0,\n") {
        let mut m = ma.to_string();
        m.replace_range(i..i + "        None => 0,\n".len(), "");
        them("xoa_nhanh_match", m);
    }
    if let Some(i) = ma.find("        Mau::Xanh => 2,\n") {
        let mut m = ma.to_string();
        m.replace_range(i..i + "        Mau::Xanh => 2,\n".len(), "");
        them("xoa_nhanh_match", m);
    }

    // 6. Đổi kiểu trả về thành kiểu không khớp.
    if ma.contains("-> i64 {") {
        them("sai_kieu_tra_ve", ma.replacen("-> i64 {", "-> String {", 1));
    }
    if ma.contains("-> usize {") {
        them("sai_kieu_tra_ve", ma.replacen("-> usize {", "-> String {", 1));
    }

    // 7. Gán lại biến không `mut`.
    if let Some(ten_bien) = tim_bien_khong_mut(ma) {
        if let Some(i) = ma.rfind('}') {
            if let Some(j) = ma[..i].rfind('}') {
                let mut m = ma.to_string();
                m.insert_str(j, &format!("    {ten_bien} = Default::default();\n"));
                them("gan_lai_khong_mut", m);
            }
        }
    }

    // 8. Gọi hàm chưa định nghĩa.
    if let Some(i) = ma.rfind('}') {
        if let Some(j) = ma[..i].rfind('}') {
            let mut m = ma.to_string();
            m.insert_str(j, "    ham_khong_ton_tai();\n");
            them("goi_ham_khong_co", m);
        }
    }

    // 9. Dùng biến chưa khai báo.
    if let Some(i) = ma.rfind('}') {
        if let Some(j) = ma[..i].rfind('}') {
            let mut m = ma.to_string();
            m.insert_str(j, "    println!(\"{}\", bien_chua_khai_bao);\n");
            them("bien_chua_khai_bao", m);
        }
    }

    // 10. Bọc phép move vào trong `if` — ca mà kiểm-lúc-chạy sẽ bỏ lọt.
    if let Some(ten_bien) = tim_bien_khong_copy(ma) {
        if let Some(i) = ma.rfind('}') {
            if let Some(j) = ma[..i].rfind('}') {
                let mut m = ma.to_string();
                m.insert_str(
                    j,
                    &format!(
                        "    if false {{ let _tam = {ten_bien}; }}\n    println!(\"{{}}\", {ten_bien});\n"
                    ),
                );
                them("move_trong_nhanh_khong_chay", m);
            }
        }
    }

    ra
}

/// Tìm một biến nhiều khả năng không-`Copy` (khai báo từ `String::from`).
fn tim_bien_khong_copy(ma: &str) -> Option<String> {
    for dong in ma.lines() {
        let d = dong.trim();
        if let Some(sau) = d.strip_prefix("let ") {
            if d.contains("String::from") || d.contains("vec![") {
                let ten = sau.split([':', '=', ' ']).next()?.trim();
                if !ten.is_empty() && ten != "mut" {
                    return Some(ten.to_string());
                }
            }
        }
    }
    None
}

fn tim_bien_khong_mut(ma: &str) -> Option<String> {
    for dong in ma.lines() {
        let d = dong.trim();
        if let Some(sau) = d.strip_prefix("let ") {
            if !sau.starts_with("mut ") && d.contains('=') {
                let ten = sau.split([':', '=', ' ']).next()?.trim();
                if !ten.is_empty() {
                    return Some(ten.to_string());
                }
            }
        }
    }
    None
}
