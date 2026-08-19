use byte_rust::kiem_tra_tu_vung;

fn main() {
    let vi_du = [
        ("Dấu nháy cong do soạn thảo Word", "let ten = “Byte”;"),
        ("Quên đóng chuỗi",                 "let ten = \"Byte;"),
        ("Viết 10x tưởng là 10 nhân x",     "let a = 10x + 5;"),
        ("Nháy đơn cho nhiều ký tự",        "let s = 'xin chào';"),
        ("Dấu chấm phẩy toàn giác",         "let a = 1；"),
    ];
    for (ten, src) in vi_du {
        println!("┌─ {ten}");
        println!("│  {src}");
        println!("└─────────────────────────────────────────────");
        let kq = kiem_tra_tu_vung(src);
        for d in kq.chan_doan.lines() {
            println!("   {d}");
        }
        println!();
    }
}
