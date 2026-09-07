//! Cổng merge của `byte-rust`: đối chiếu với `rustc` **thật**.
//!
//! Chạy trên CI native (nơi có toolchain Rust), không chạy trong app.
//!
//! # Vì sao corpus âm mới là cổng
//!
//! Một engine `fn chay() -> Pass` luôn đạt 100% trên corpus dương. Chỉ corpus
//! **âm** mới phân biệt được engine thật với engine giả. Xem ADR-002 §4.
//!
//! # Hai nguồn corpus âm, tính riêng hai con số
//!
//! | Nguồn | Sinh ra sao | Đo được gì |
//! |---|---|---|
//! | **Mutant cơ học** (`mutate.rs`) | Đột biến hạt giống dương theo 10 khuôn mẫu cố định | Hồi quy: những lỗ hổng đã từng vá có mở lại không |
//! | **Corpus âm thủ công** (`corpus_am.rs`) | Đọc mã từng module rồi viết chương trình chạm đúng chỗ suy luận bị bỏ dở | Lỗ hổng THẬT, kể cả những chỗ đột biến cơ học không với tới |
//!
//! Hai con số phải tách bạch. Gộp chung sẽ che mất một sự thật quan trọng:
//! mutant cơ học có thể xanh 100% trong khi engine vẫn nhận oan hàng loạt, đơn
//! giản vì mười khuôn mẫu đột biến không hỏi đúng câu hỏi. Một cổng xanh vì
//! không ai hỏi thì tệ hơn không có cổng — nó phát ra sự tự tin.
//!
//! # Luật phán quyết
//!
//! | `rustc` | `byte-rust` | Kết luận |
//! |---|---|---|
//! | nhận | `Dat` | ✅ đồng ý |
//! | nhận | `ChuaHoTro` | ⚪ thú nhận — chấp nhận được |
//! | nhận | `KhongDat` | ⚠️ **từ chối oan** — theo dõi |
//! | từ chối | `KhongDat` | ✅ đồng ý |
//! | từ chối | `ChuaHoTro` | ⚪ thú nhận — chấp nhận được |
//! | từ chối | `Dat` | ❌ **NHẬN OAN — FAIL MERGE** |
//!
//! # Tham số dòng lệnh
//!
//! - `--chi-mutant` : chỉ chạy mutant cơ học (dùng để xem baseline hồi quy)
//! - `--chi-am`     : chỉ chạy corpus âm thủ công
//! - `--im`         : không in chi tiết từng ca, chỉ in bảng tổng kết
//!
//! Không có tham số = chạy cả hai. Cổng fail nếu **bất kỳ** nguồn nào có nhận
//! oan; đó là điều ADR-002 §4 đòi và không được nới.

mod corpus;
mod corpus_am;
mod mutate;

use byte_rust::diag::KetCuc;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq)]
enum PhanQuyetRustc {
    Nhan,
    TuChoi,
}

struct KetQuaRustc {
    phan_quyet: PhanQuyetRustc,
    ma_loi: Vec<String>,
}

fn chay_rustc(thu_muc: &Path, ten: &str, ma: &str) -> Option<KetQuaRustc> {
    let tep = thu_muc.join(format!("{ten}.rs"));
    std::fs::write(&tep, ma).ok()?;
    let ra = Command::new("rustc")
        .arg("--edition").arg("2021")
        .arg("--crate-type").arg("bin")
        .arg("--emit").arg("metadata") // không cần link, nhanh hơn nhiều
        .arg("-o").arg(thu_muc.join(format!("{ten}.meta")))
        .arg(&tep)
        .output()
        .ok()?;
    let stderr = String::from_utf8_lossy(&ra.stderr);
    let mut ma_loi: Vec<String> = Vec::new();
    for phan in stderr.split("error[") {
        if let Some(dong) = phan.split(']').next() {
            if dong.len() == 5 && dong.starts_with('E') {
                ma_loi.push(dong.to_string());
            }
        }
    }
    ma_loi.sort();
    ma_loi.dedup();
    Some(KetQuaRustc {
        phan_quyet: if ra.status.success() { PhanQuyetRustc::Nhan } else { PhanQuyetRustc::TuChoi },
        ma_loi,
    })
}

fn chay_byte_rust(ma: &str) -> (KetCuc, Vec<String>) {
    // Gọi ĐÚNG pipeline mà app chạy, không dựng lại chuỗi gọi của riêng mình.
    // Bản trước tự gọi tyck + move_check + interp, nên khi thêm mut_check nó
    // lặng lẽ đo một pipeline khác — cổng merge đo nhầm còn tệ hơn không có cổng.
    let (_xuat, d) = byte_rust::kiem_va_chay(ma);
    let ma_loi: Vec<String> = d.iter().map(|x| x.code.to_string()).collect();
    (d.ket_cuc(), ma_loi)
}

#[derive(Default)]
struct ThongKe {
    tong: usize,
    dong_y: usize,
    thu_nhan: usize,
    tu_choi_oan: usize,
    nhan_oan: Vec<String>,
}

impl ThongKe {
    fn in_bang(&self, tieu_de: &str) {
        println!("┌─────────────────────────────────────────────────────────────────");
        println!("│  {tieu_de}");
        println!("├─────────────────────────────────────────────────────────────────");
        println!("│  Tổng chương trình đã đối chiếu : {}", self.tong);
        println!("│  Đồng ý với rustc               : {} ({:.1}%)", self.dong_y, pc(self.dong_y, self.tong));
        println!("│  Thú nhận ChuaHoTro             : {} ({:.1}%)", self.thu_nhan, pc(self.thu_nhan, self.tong));
        println!("│  Từ chối oan (theo dõi)         : {}", self.tu_choi_oan);
        println!("│  NHẬN OAN (cổng, phải = 0)      : {} ({:.1}%)",
                 self.nhan_oan.len(), pc(self.nhan_oan.len(), self.tong));
        println!("└─────────────────────────────────────────────────────────────────");
    }
}

/// Ghi nhận một cặp (phán quyết rustc, kết cục byte-rust) vào thống kê.
fn cham(
    tk: &mut ThongKe,
    ten: &str,
    ma: &str,
    r: &KetQuaRustc,
    kc: KetCuc,
    im: bool,
) -> bool {
    tk.tong += 1;
    use PhanQuyetRustc::*;
    match (r.phan_quyet, kc) {
        (Nhan, KetCuc::Dat) | (TuChoi, KetCuc::KhongDat) => {
            tk.dong_y += 1;
            true
        }
        (_, KetCuc::ChuaHoTro) => {
            tk.thu_nhan += 1;
            false
        }
        (Nhan, KetCuc::KhongDat) => {
            tk.tu_choi_oan += 1;
            if !im {
                println!("  ⚠️  TỪ CHỐI OAN  {ten}");
                println!("      rustc nhận, byte-rust từ chối\n{}\n", trich(ma));
            }
            false
        }
        (TuChoi, KetCuc::Dat) => {
            tk.nhan_oan.push(ten.to_string());
            if !im {
                println!("  ❌ NHẬN OAN     {ten}");
                println!("      rustc từ chối ({}), byte-rust báo Đạt\n{}\n",
                         if r.ma_loi.is_empty() { "không mã E".into() } else { r.ma_loi.join(",") },
                         trich(ma));
            }
            false
        }
    }
}

fn main() {
    // rustc có sẵn không?
    if Command::new("rustc").arg("--version").output().is_err() {
        eprintln!("BỎ QUA: không tìm thấy `rustc`. Bộ đối chiếu chỉ chạy trên CI native.");
        std::process::exit(0);
    }

    let tham_so: Vec<String> = std::env::args().skip(1).collect();
    let im = tham_so.iter().any(|a| a == "--im");
    let chi_mutant = tham_so.iter().any(|a| a == "--chi-mutant");
    let chi_am = tham_so.iter().any(|a| a == "--chi-am");
    let chay_mutant = !chi_am;
    let chay_am = !chi_mutant;

    let thu_muc = PathBuf::from(std::env::temp_dir()).join("byte-rust-conformance");
    std::fs::create_dir_all(&thu_muc).expect("không tạo được thư mục tạm");

    println!("╔══════════════════════════════════════════════════════════════════");
    println!("║  ĐỐI CHIẾU byte-rust ↔ rustc");
    println!("╚══════════════════════════════════════════════════════════════════");

    // ── Nguồn 1: mutant cơ học sinh từ corpus dương ─────────────────────────
    let mut tk_mutant = ThongKe::default();
    let mut theo_kieu: BTreeMap<String, (usize, usize)> = BTreeMap::new();

    if chay_mutant {
        println!("\n▎A. MUTANT CƠ HỌC (mutate.rs — đột biến hạt giống dương)\n");
        for hat in corpus::HAT_GIONG {
            let mut cac: Vec<(String, String)> =
                vec![(format!("{}__goc", hat.ten), hat.ma.to_string())];
            for (i, db) in mutate::sinh(hat.ma).into_iter().enumerate() {
                cac.push((format!("{}__{}__{i}", hat.ten, db.kieu), db.ma));
            }

            for (ten, ma) in cac {
                let Some(r) = chay_rustc(&thu_muc, &ten, &ma) else { continue };
                let (kc, _) = chay_byte_rust(&ma);

                let kieu = ten.split("__").nth(1).unwrap_or("goc").to_string();
                let dong_y = cham(&mut tk_mutant, &ten, &ma, &r, kc, im);
                let e = theo_kieu.entry(kieu).or_default();
                e.0 += 1;
                if dong_y {
                    e.1 += 1;
                }
            }
        }
        tk_mutant.in_bang("A. MUTANT CƠ HỌC");
        println!("\n  Theo loại đột biến:");
        for (kieu, (tong, dong_y)) in &theo_kieu {
            println!("    {kieu:<32} {dong_y}/{tong} đồng ý");
        }
    }

    // ── Nguồn 2: corpus âm thủ công ─────────────────────────────────────────
    let mut tk_am = ThongKe::default();
    let mut theo_nhom: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
    let mut theo_ma_loi: BTreeMap<String, usize> = BTreeMap::new();
    let mut theo_module: BTreeMap<&str, usize> = BTreeMap::new();

    if chay_am {
        println!("\n▎B. CORPUS ÂM THỦ CÔNG (corpus_am.rs — viết theo lỗ hổng đọc được trong mã)\n");
        for hat in corpus_am::CORPUS_AM {
            let ten = format!("am__{}", hat.ten);
            let Some(r) = chay_rustc(&thu_muc, &ten, hat.ma) else { continue };

            // Một ca trong corpus âm mà rustc lại NHẬN là ca ghi sai — phải la
            // lên, vì nó làm hỏng ý nghĩa của cả tệp.
            if r.phan_quyet == PhanQuyetRustc::Nhan {
                println!("  ‼️  GHI SAI       {ten}: rustc NHẬN chương trình này.");
                println!("      Ca trong corpus âm phải bị rustc từ chối. Sửa hoặc bỏ ca.\n");
            }

            let (kc, _) = chay_byte_rust(hat.ma);
            let dong_y = cham(&mut tk_am, &ten, hat.ma, &r, kc, im);

            let e = theo_nhom.entry(hat.nhom).or_default();
            e.0 += 1;
            if dong_y {
                e.1 += 1;
            } else if kc == KetCuc::Dat {
                *theo_ma_loi.entry(hat.ma_loi.to_string()).or_default() += 1;
                *theo_module.entry(mo_dun_chinh(hat.lo_hong)).or_default() += 1;
            }
        }
        tk_am.in_bang("B. CORPUS ÂM THỦ CÔNG");

        println!("\n  Theo nhóm lỗ hổng:");
        for (ma_nhom, ten_nhom) in corpus_am::NHOM {
            let (tong, dong_y) = theo_nhom.get(ma_nhom).copied().unwrap_or((0, 0));
            if tong == 0 {
                continue;
            }
            println!("    {:<38} {}/{} đồng ý  ({} nhận oan)",
                     ten_nhom, dong_y, tong, tong - dong_y);
        }

        if !theo_ma_loi.is_empty() {
            println!("\n  Mã lỗi rustc bị bỏ lọt (nhận oan), theo tần suất:");
            let mut v: Vec<_> = theo_ma_loi.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            for (ma_loi, n) in v {
                println!("    {ma_loi:<34} {n} ca");
            }
        }

        if !theo_module.is_empty() {
            println!("\n  Module chứa lỗ hổng (theo số ca nhận oan):");
            let mut v: Vec<_> = theo_module.iter().collect();
            v.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
            for (mo_dun, n) in v {
                println!("    {mo_dun:<34} {n} ca");
            }
        }
    }

    // ── Phán quyết cổng ─────────────────────────────────────────────────────
    println!("\n════════════════════════════════════════════════════════════════════");
    println!("  PHÁN QUYẾT CỔNG");
    println!("════════════════════════════════════════════════════════════════════");
    if chay_mutant {
        println!("  A. Mutant cơ học      — nhận oan: {:>3}/{}", tk_mutant.nhan_oan.len(), tk_mutant.tong);
    }
    if chay_am {
        println!("  B. Corpus âm thủ công — nhận oan: {:>3}/{}", tk_am.nhan_oan.len(), tk_am.tong);
    }
    let tong_nhan_oan = tk_mutant.nhan_oan.len() + tk_am.nhan_oan.len();

    if tong_nhan_oan > 0 {
        eprintln!("\n❌ FAIL: {tong_nhan_oan} trường hợp NHẬN OAN.");
        eprintln!("   byte-rust báo Đạt cho code mà rustc từ chối — đúng chế độ hỏng");
        eprintln!("   đã làm engine giả cũ vô dụng. Xem ADR-002 §4.");
        if chay_mutant && chay_am && tk_mutant.nhan_oan.is_empty() && !tk_am.nhan_oan.is_empty() {
            eprintln!();
            eprintln!("   Lưu ý: nguồn A xanh, nguồn B đỏ. Nghĩa là mười khuôn mẫu đột biến");
            eprintln!("   cơ học KHÔNG với tới các lỗ hổng này — không phải chúng không tồn");
            eprintln!("   tại. Đừng đọc nguồn A một mình như bằng chứng engine đã đúng.");
        }
        eprintln!("\n   Báo cáo chi tiết + đề xuất vá: docs/generated/rust-conformance.md");
        eprintln!("   ĐỪNG vá vừa khít từng ca ở corpus_am.rs — phải vá ở gốc.");
        std::process::exit(1);
    }
    println!("\n✅ Không có trường hợp nhận oan nào. Cổng mở.");
}

fn pc(a: usize, b: usize) -> f64 {
    if b == 0 { 0.0 } else { a as f64 * 100.0 / b as f64 }
}

/// Lấy tên module đứng đầu trong mô tả lỗ hổng (`"tyck.rs (…) + interp.rs"` -> `"tyck.rs"`).
fn mo_dun_chinh(lo_hong: &str) -> &str {
    for m in ["move_check.rs", "tyck.rs", "interp.rs", "value.rs", "parser.rs", "ast.rs"] {
        if lo_hong.starts_with(m) {
            return m;
        }
    }
    "khac"
}

fn trich(ma: &str) -> String {
    ma.trim().lines().map(|l| format!("      │ {l}")).collect::<Vec<_>>().join("\n")
}
