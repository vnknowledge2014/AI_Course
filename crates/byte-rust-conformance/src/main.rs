//! Cổng merge của `byte-rust`: đối chiếu với `rustc` **thật**.
//!
//! Chạy trên CI native (nơi có toolchain Rust), không chạy trong app.
//!
//! # Vì sao corpus âm mới là cổng
//!
//! Một engine `fn chay() -> Pass` luôn đạt 100% trên corpus dương. Chỉ corpus
//! **âm** mới phân biệt được engine thật với engine giả. Xem ADR-002 §4.
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

mod corpus;
mod mutate;

use byte_rust::diag::KetCuc;
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
    let (ct, mut d) = byte_rust::parser::phan_tich(ma);
    if !d.co_loi() {
        byte_rust::tyck::kiem_tra(&ct, &mut d);
        byte_rust::move_check::kiem_tra(&ct, &mut d);
        if !d.co_loi() && !d.co_chua_ho_tro() {
            let mut may = byte_rust::interp::MayChay::moi();
            if let Err(loi) = may.chay(&ct) {
                d.push(loi);
            }
        }
    }
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

fn main() {
    // rustc có sẵn không?
    if Command::new("rustc").arg("--version").output().is_err() {
        eprintln!("BỎ QUA: không tìm thấy `rustc`. Bộ đối chiếu chỉ chạy trên CI native.");
        std::process::exit(0);
    }

    let thu_muc = PathBuf::from(std::env::temp_dir()).join("byte-rust-conformance");
    std::fs::create_dir_all(&thu_muc).expect("không tạo được thư mục tạm");

    let mut tk = ThongKe::default();
    let mut theo_kieu: std::collections::BTreeMap<&str, (usize, usize)> = Default::default();

    println!("╔══════════════════════════════════════════════════════════════════");
    println!("║  ĐỐI CHIẾU byte-rust ↔ rustc");
    println!("╚══════════════════════════════════════════════════════════════════\n");

    for hat in corpus::HAT_GIONG {
        let mut cac: Vec<(String, String)> =
            vec![(format!("{}__goc", hat.ten), hat.ma.to_string())];
        for (i, db) in mutate::sinh(hat.ten, hat.ma).into_iter().enumerate() {
            cac.push((format!("{}__{}__{i}", hat.ten, db.kieu), db.ma));
        }

        for (ten, ma) in cac {
            let Some(r) = chay_rustc(&thu_muc, &ten, &ma) else { continue };
            let (kc, _ma_byte) = chay_byte_rust(&ma);
            tk.tong += 1;

            let kieu: &str = ten.split("__").nth(1).unwrap_or("goc");
            let e = theo_kieu.entry(Box::leak(kieu.to_string().into_boxed_str())).or_default();
            e.0 += 1;

            use PhanQuyetRustc::*;
            match (r.phan_quyet, kc) {
                (Nhan, KetCuc::Dat) | (TuChoi, KetCuc::KhongDat) => {
                    tk.dong_y += 1;
                    e.1 += 1;
                }
                (_, KetCuc::ChuaHoTro) => tk.thu_nhan += 1,
                (Nhan, KetCuc::KhongDat) => {
                    tk.tu_choi_oan += 1;
                    println!("  ⚠️  TỪ CHỐI OAN  {ten}");
                    println!("      rustc nhận, byte-rust từ chối\n{}\n", trich(&ma));
                }
                (TuChoi, KetCuc::Dat) => {
                    tk.nhan_oan.push(ten.clone());
                    println!("  ❌ NHẬN OAN     {ten}");
                    println!("      rustc từ chối ({}), byte-rust báo Đạt\n{}\n",
                             if r.ma_loi.is_empty() { "không mã".into() } else { r.ma_loi.join(",") },
                             trich(&ma));
                }
            }
        }
    }

    println!("┌─────────────────────────────────────────────────────────────────");
    println!("│  Tổng chương trình đã đối chiếu : {}", tk.tong);
    println!("│  Đồng ý với rustc               : {} ({:.1}%)", tk.dong_y, pc(tk.dong_y, tk.tong));
    println!("│  Thú nhận ChuaHoTro             : {} ({:.1}%)", tk.thu_nhan, pc(tk.thu_nhan, tk.tong));
    println!("│  Từ chối oan (theo dõi)         : {}", tk.tu_choi_oan);
    println!("│  NHẬN OAN (cổng, phải = 0)      : {}", tk.nhan_oan.len());
    println!("└─────────────────────────────────────────────────────────────────");
    println!("\n  Theo loại đột biến:");
    for (kieu, (tong, dong_y)) in &theo_kieu {
        println!("    {kieu:<32} {dong_y}/{tong} đồng ý");
    }

    if !tk.nhan_oan.is_empty() {
        eprintln!("\n❌ FAIL: {} trường hợp NHẬN OAN.", tk.nhan_oan.len());
        eprintln!("   byte-rust báo Đạt cho code mà rustc từ chối — đúng chế độ hỏng");
        eprintln!("   đã làm engine giả cũ vô dụng. Xem ADR-002 §4.");
        std::process::exit(1);
    }
    println!("\n✅ Không có trường hợp nhận oan nào. Cổng mở.");
}

fn pc(a: usize, b: usize) -> f64 {
    if b == 0 { 0.0 } else { a as f64 * 100.0 / b as f64 }
}

fn trich(ma: &str) -> String {
    ma.trim().lines().map(|l| format!("      │ {l}")).collect::<Vec<_>>().join("\n")
}
