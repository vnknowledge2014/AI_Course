use byte_rust::interp::chay;
use byte_rust::span::SourceMap;

fn main() {
    let cac_bai = [
        ("Chạy đúng — đệ quy", r#"
fn giai_thua(n: i64) -> i64 {
    if n <= 1 { 1 } else { n * giai_thua(n - 1) }
}
fn main() {
    for i in 1..=5 {
        println!("{}! = {}", i, giai_thua(i));
    }
}"#),
        ("Bài học ownership", r#"
fn main() {
    let ten = String::from("Byte");
    let khac = ten;
    println!("{}", ten);
}"#),
        ("Vòng lặp vô hạn — KHÔNG treo máy", r#"
fn main() {
    let mut i = 0;
    while true { i += 1; }
}"#),
        ("Cộng chuỗi với số", r#"
fn main() {
    let s = String::from("tuoi: ") + 25;
}"#),
    ];

    for (ten, src) in cac_bai {
        println!("╔══ {ten}");
        for l in src.trim().lines() { println!("║  {l}"); }
        println!("╚═══════════════════════════════════════════════");
        let (xuat, d) = chay(src);
        if !xuat.is_empty() {
            for l in xuat.lines() { println!("   │ {l}"); }
        }
        if d.co_loi() {
            for l in d.render(&SourceMap::new(src)).lines() { println!("   {l}"); }
        }
        println!();
    }
}
