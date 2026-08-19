//! Corpus hạt giống: các chương trình Rust **hợp lệ**, nằm trong phạm vi
//! `byte-rust` v1 (xem ADR-002 §4.3.2).
//!
//! Mutant được sinh cơ học từ đây. Corpus dương một mình không đo được gì đáng
//! kể — mọi chương trình đúng thì engine nào cũng dễ báo đúng. Giá trị nằm ở
//! chỗ nó là **nguyên liệu** để sinh corpus âm.

pub struct HatGiong {
    pub ten: &'static str,
    pub ma: &'static str,
}

pub const HAT_GIONG: &[HatGiong] = &[
    HatGiong {
        ten: "so_hoc",
        ma: r#"
fn main() {
    let a = 10;
    let b = 3;
    println!("{}", a + b * 2);
}
"#,
    },
    HatGiong {
        ten: "ham_tra_ve",
        ma: r#"
fn cong(a: i64, b: i64) -> i64 {
    a + b
}
fn main() {
    println!("{}", cong(2, 3));
}
"#,
    },
    HatGiong {
        ten: "chuoi_clone",
        ma: r#"
fn dai(s: String) -> usize {
    s.len()
}
fn main() {
    let ten = String::from("Byte");
    let n = dai(ten.clone());
    println!("{} {}", ten, n);
}
"#,
    },
    HatGiong {
        ten: "muon_khong_move",
        ma: r#"
fn dai(s: &String) -> usize {
    s.len()
}
fn main() {
    let ten = String::from("Byte");
    let n = dai(&ten);
    println!("{} {}", ten, n);
}
"#,
    },
    HatGiong {
        ten: "vec_lap",
        ma: r#"
fn main() {
    let mut tong = 0;
    let v = vec![1, 2, 3, 4];
    for x in 0..4 {
        tong += x;
    }
    println!("{} {}", tong, v.len());
}
"#,
    },
    HatGiong {
        ten: "match_option",
        ma: r#"
fn main() {
    let x: Option<i64> = Some(5);
    let y = match x {
        Some(n) => n * 2,
        None => 0,
    };
    println!("{}", y);
}
"#,
    },
    HatGiong {
        ten: "struct_va_impl",
        ma: r#"
struct Diem { x: i64, y: i64 }
impl Diem {
    fn tong(&self) -> i64 { self.x + self.y }
}
fn main() {
    let d = Diem { x: 3, y: 4 };
    println!("{}", d.tong());
}
"#,
    },
    HatGiong {
        ten: "de_quy",
        ma: r#"
fn gt(n: i64) -> i64 {
    if n <= 1 { 1 } else { n * gt(n - 1) }
}
fn main() {
    println!("{}", gt(10));
}
"#,
    },
    HatGiong {
        ten: "enum_tu_dinh_nghia",
        ma: r#"
enum Mau { Do, Xanh }
fn ten(m: Mau) -> i64 {
    match m {
        Mau::Do => 1,
        Mau::Xanh => 2,
    }
}
fn main() {
    println!("{}", ten(Mau::Do));
}
"#,
    },
    HatGiong {
        ten: "move_roi_khong_dung",
        ma: r#"
fn nhan(s: String) -> usize { s.len() }
fn main() {
    let a = String::from("chao");
    let n = nhan(a);
    println!("{}", n);
}
"#,
    },
];
