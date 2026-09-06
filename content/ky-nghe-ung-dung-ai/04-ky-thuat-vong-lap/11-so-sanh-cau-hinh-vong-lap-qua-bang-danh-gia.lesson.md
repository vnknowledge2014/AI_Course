---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.so-sanh-cau-hinh-vong-lap-qua-bang-danh-gia
title: "So sánh nhiều cấu hình loop qua một bảng đánh giá — BA trục đo tách biệt"
summary: "CauHinhVongLap {ten, soBuocToiDa, coPhatHienKhongTienTrien, soLanLapLaiToiDa, soLanThuLaiToiDaTrongBuoc} mo ta MOT lua chon vong lap day du. soSanhNhieuCauHinhVongLap(cacCauHinh, taoBoTacVu) chay CUNG mot bo 3 tac vu (hoi tu 3 buoc, ket dinh lap lai loi, can retry-trong-buoc de tien bo) qua NHIEU cau hinh, tra ve bang {ten, tyLeThanhCong, buocTrungBinhKhiThanhCong, chiPhiTrungBinh}. Bon cau hinh tren CUNG bo tac vu (soBuocToiDa=20): toi-gian (khong co che nao) cho tyLeThanhCong=1/3 (in ra 0.3333333333333333), chiPhiTrungBinh=106/3 (in ra 35.333333333333336); THEM no-progress detection: tyLeThanhCong GIU NGUYEN 0.3333333333333333 nhung chiPhiTrungBinh giam manh con 7; THEM rieng retry-trong-buoc (khong no-progress): tyLeThanhCong TANG len 2/3 (in ra 0.6666666666666666), chiPhiTrungBinh=58/3 (in ra 19.333333333333332); ca hai cung luc (day-du): tyLeThanhCong VAN 0.6666666666666666 (giong retry-trong-buoc rieng) nhung chiPhiTrungBinh THAP NHAT = 8 -- chung minh BA truc do (completion, buoc-den-thanh-cong, cost) doc lap: no-progress detection CHI giam chi phi KHONG doi completion; retry-trong-buoc CHI tang completion; ket hop ca hai cho ca hai loi ich CUNG LUC."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kna.so-sanh-cau-hinh-vong-lap-qua-bang-danh-gia]
requires: [kna.ngan-sach-buoc-chia-se-nhieu-tac-vu]
concepts: [kna.so-sanh-cau-hinh-vong-lap-qua-bang-danh-gia]
gradingMatrix:
  web-chrome: [run, tests, output]
  web-firefox: [run, tests, output]
  macos: [run, tests, output]
  windows: [run, tests, output]
  linux: [run, tests, output]
  android: [run, tests, output]
  ios: [run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Mười bài vừa qua, mỗi lần so sánh hai cách vòng lặp hoạt động LÀ một cặp
`console.log` đọc bằng mắt. Nhưng một đội kỹ sư thật không so sánh HAI
cấu hình một lần — họ so sánh CHỤC cấu hình, LIÊN TỤC. Bài này đóng gói
phép so sánh đó thành MỘT hàm: đưa vào một danh sách cấu hình vòng lặp
(step cap, có/không no-progress, có/không retry-trong-bước), nhận về
MỘT bảng — VÀ chứng minh BA trục đo LOOP không thể gộp thành một con số.
::::

::::explain{#cau_hinh_vong_lap_va_hang_bang}
`CauHinhVongLap` gói bốn tham số: `soBuocToiDa` (step cap, q9.4a bài
`2`), `coPhatHienKhongTienTrien` VÀ `soLanLapLaiToiDa` (no-progress
detection, q9.4a bài `4`), VÀ `soLanThuLaiToiDaTrongBuoc` (retry-trong-
bước, bài `8`). `TacVuVongLapDayDu` gộp CẢ HAI tín hiệu MỘT bước cần —
`chuKy` (cho no-progress) VÀ `chiPhi` (bài `7`) — VÀO một kết quả DUY
NHẤT; `chayMotTacVuVoiCauHinh` áp CẢ BỐN tham số lên MỘT tác vụ:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuHoiTuDayDu(n: number, chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let tongChiPhi = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < cauHinh.soBuocToiDa) {
    const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc);
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", soBuocDaDung: soBuoc, tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", soBuocDaDung: soBuoc, tongChiPhi };
    if (cauHinh.coPhatHienKhongTienTrien) {
      if (kq.chuKy === chuKyTruoc) {
        soLanLapLaiLienTiep++;
      } else {
        soLanLapLaiLienTiep = 1;
        chuKyTruoc = kq.chuKy;
      }
      if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) {
        return { trangThai: "khongTienTrien", soBuocDaDung: soBuoc, tongChiPhi };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocDaDung: soBuoc, tongChiPhi };
}

const tv = taoTacVuHoiTuDayDu(3, 2);
console.log(JSON.stringify(chayMotTacVuVoiCauHinh(tv, {
  soBuocToiDa: 10, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0,
})));
```

```text title=readonly
{"trangThai":"thanhCong","giaTri":"hoan_thanh","soBuocDaDung":3,"tongChiPhi":6}
```

`chayMotTacVuVoiCauHinh` chạy MỘT tác vụ QUA MỘT cấu hình cụ thể, trả
về kết quả CỘNG cả `soBuocDaDung` (bao nhiêu bước THẬT SỰ dùng) VÀ
`tongChiPhi` (bài `7`) — tách RIÊNG khỏi `trangThai` (bốn biến thể:
`"thanhCong"`, `"loi"`, `"hetBuoc"`, VÀ `"khongTienTrien"` từ no-progress
detection, q9.4a bài `4`).
::::

::::example{#bon_cau_hinh_tren_cung_mot_bo_tac_vu}
Ngoài tác vụ hội tụ bình thường, một tác vụ `taoTacVuKetDinhDayDu` (kẹt
VĨNH VIỄN, CÙNG `chuKy` mọi bước — q9.4a bài `4`) VÀ một tác vụ
`taoTacVuCanRetryTrongBuoc` (chỉ TIẾN BỘ nếu `soLanThuLaiToiDaTrongBuoc
>= 1`; NGƯỢC LẠI LUÔN `"chua_xong"` VỚI CÙNG `chuKy`, cho phép no-progress
detection cũng bắt được NÓ). `soSanhNhieuCauHinhVongLap` chạy CẢ BỐN
cấu hình trên CÙNG bộ `3` tác vụ đó (`soBuocToiDa = 20`):

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuHoiTuDayDu(n: number, chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKetDinhDayDu(chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuCanRetryTrongBuoc(n: number, chiPhiMoiLuotGoi: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  const demTienBo = { soTienBo: 0 };
  return {
    demBuoc,
    chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      if (soLanThuLaiToiDaTrongBuoc < 1) {
        return { trangThai: "chua_xong", chuKy: "can_retry_trong_buoc", chiPhi: chiPhiMoiLuotGoi };
      }
      demTienBo.soTienBo++;
      const xong = demTienBo.soTienBo >= n;
      const chiPhiBuocNay = chiPhiMoiLuotGoi * 2;
      if (xong) {
        return { trangThai: "xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay, giaTri: "hoan_thanh_qua_retry" };
      }
      return { trangThai: "chua_xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  ten: string;
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let tongChiPhi = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < cauHinh.soBuocToiDa) {
    const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc);
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", soBuocDaDung: soBuoc, tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", soBuocDaDung: soBuoc, tongChiPhi };
    if (cauHinh.coPhatHienKhongTienTrien) {
      if (kq.chuKy === chuKyTruoc) {
        soLanLapLaiLienTiep++;
      } else {
        soLanLapLaiLienTiep = 1;
        chuKyTruoc = kq.chuKy;
      }
      if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) {
        return { trangThai: "khongTienTrien", soBuocDaDung: soBuoc, tongChiPhi };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocDaDung: soBuoc, tongChiPhi };
}

interface HangBangSoSanh {
  ten: string;
  tyLeThanhCong: number;
  buocTrungBinhKhiThanhCong: number;
  chiPhiTrungBinh: number;
}

function chayMotCauHinh(cauHinh: CauHinhVongLap, taoBoTacVu: () => TacVuVongLapDayDu[]): HangBangSoSanh {
  const dsTacVu = taoBoTacVu();
  const ketQua = dsTacVu.map((tv) => chayMotTacVuVoiCauHinh(tv, cauHinh));
  const thanhCong = ketQua.filter((k) => k.trangThai === "thanhCong");
  return {
    ten: cauHinh.ten,
    tyLeThanhCong: ketQua.length === 0 ? 0 : thanhCong.length / ketQua.length,
    buocTrungBinhKhiThanhCong:
      thanhCong.length === 0 ? 0 : thanhCong.reduce((acc, k) => acc + k.soBuocDaDung, 0) / thanhCong.length,
    chiPhiTrungBinh: ketQua.length === 0 ? 0 : ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0) / ketQua.length,
  };
}

function soSanhNhieuCauHinhVongLap(
  cacCauHinh: CauHinhVongLap[],
  taoBoTacVu: () => TacVuVongLapDayDu[],
): HangBangSoSanh[] {
  return cacCauHinh.map((ch) => chayMotCauHinh(ch, taoBoTacVu));
}

function taoBoTacVuHonHop(): TacVuVongLapDayDu[] {
  return [taoTacVuHoiTuDayDu(3, 2), taoTacVuKetDinhDayDu(2), taoTacVuCanRetryTrongBuoc(2, 3)];
}

const bangSoSanh = soSanhNhieuCauHinhVongLap(
  [
    { ten: "toi-gian", soBuocToiDa: 20, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0 },
    { ten: "voi-no-progress", soBuocToiDa: 20, coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0 },
    { ten: "voi-retry-trong-buoc", soBuocToiDa: 20, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 },
    { ten: "day-du", soBuocToiDa: 20, coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 },
  ],
  taoBoTacVuHonHop,
);
for (const hang of bangSoSanh) console.log(JSON.stringify(hang));
```

```text title=readonly
{"ten":"toi-gian","tyLeThanhCong":0.3333333333333333,"buocTrungBinhKhiThanhCong":3,"chiPhiTrungBinh":35.333333333333336}
{"ten":"voi-no-progress","tyLeThanhCong":0.3333333333333333,"buocTrungBinhKhiThanhCong":3,"chiPhiTrungBinh":7}
{"ten":"voi-retry-trong-buoc","tyLeThanhCong":0.6666666666666666,"buocTrungBinhKhiThanhCong":2.5,"chiPhiTrungBinh":19.333333333333332}
{"ten":"day-du","tyLeThanhCong":0.6666666666666666,"buocTrungBinhKhiThanhCong":2.5,"chiPhiTrungBinh":8}
```

Bốn hàng, BA cột — VÀ mỗi cột kể một câu chuyện KHÁC nhau. Thêm
no-progress detection (`"toi-gian"` → `"voi-no-progress"`): `tyLeThanhCong`
KHÔNG đổi (`0.3333333333333333` cả hai), NHƯNG `chiPhiTrungBinh` giảm
MẠNH (`35.333333333333336` → `7`) — tác vụ kẹt bị bắt SỚM Ở bước `3`
thay vì đốt hết `20` bước. Thêm retry-trong-bước (`"toi-gian"` →
`"voi-retry-trong-buoc"`): `tyLeThanhCong` TĂNG (`0.3333333333333333` →
`0.6666666666666666` — tác vụ thứ ba giờ THÀNH CÔNG). Kết hợp CẢ HAI
(`"day-du"`): `tyLeThanhCong` GIỮ nguyên mức CAO của retry-trong-buoc,
NHƯNG `chiPhiTrungBinh` xuống THẤP NHẤT (`8`) — CẢ HAI lợi ích CÙNG LÚC.
::::

::::predict{#doan-soBuocToiDa-qua-nho commitOnce}
Nếu chạy CHỈ cấu hình `"voi-retry-trong-buoc"` NHƯNG đổi `soBuocToiDa`
từ `20` xuống `1` (giữ NGUYÊN mọi tham số khác VÀ bộ `3` tác vụ) —
`tyLeThanhCong` đổi ra sao?

:::opt{correct}
Giảm xuống `0` — VỚI `soBuocToiDa = 1`, MỌI tác vụ chỉ được thử ĐÚNG một
bước NGOÀI: tác vụ hội tụ (cần `3` bước) VÀ tác vụ kẹt vĩnh viễn đều
`"hetBuoc"` NGAY; tác vụ retry-trong-bước (cần `2` bước TIẾN BỘ, dù MỖI
bước Ở đây LUÔN thành công NHỜ retry) cũng KHÔNG đủ MỘT bước để đạt
`n = 2` — retry-trong-bước chỉ cứu được tác vụ KHI vòng lặp NGOÀI có ĐỦ
số bước để nó tích luỹ tiến bộ, KHÔNG thay thế được việc thiếu ngân
sách bước
:::
:::opt
Vẫn LÀ `0.6666666666666666` — retry-trong-bước ĐÃ chứng minh cứu được
tác vụ thứ ba, nên nó THÀNH CÔNG bất kể `soBuocToiDa` LÀ bao nhiêu
::why
Nhầm rằng MỘT cơ chế (retry-trong-bước) có thể thay thế HOÀN TOÀN cho
việc CÓ ĐỦ ngân sách bước — nhưng tác vụ `taoTacVuCanRetryTrongBuoc(2,
3)` CẦN đúng `2` bước NGOÀI để tích luỹ đủ tiến bộ (`n = 2`), VÀ
`soBuocToiDa = 1` không đủ CHO DÙ mỗi bước ĐỀU thành công.

Chỗ lệch: `demTienBo.soTienBo` chỉ tăng `1` MỖI bước NGOÀI (dù retry-
trong-bước LUÔN cứu được lỗi tạm thời) — với `soBuocToiDa = 1`, vòng
lặp CHỈ được gọi `chayMotBuoc` đúng MỘT lần, `demTienBo.soTienBo` dừng
Ở `1`, chưa đạt `n = 2`.
::
:::
:::opt
Tác vụ HỘI TỤ (cần `3` bước) vẫn thành công vì nó KHÔNG phụ thuộc
retry-trong-bước, chỉ hai tác vụ CÒN LẠI mới thất bại
::why
Nhầm rằng "không phụ thuộc retry-trong-bước" nghĩa LÀ "không phụ thuộc
step cap" — nhưng `taoTacVuHoiTuDayDu(3, 2)` VẪN cần đúng `3` bước NGOÀI
để `trangThai` chuyển thành `"xong"`, VÀ `soBuocToiDa = 1` KHÔNG đủ cho
`3` bước đó, bất kể tác vụ có liên quan gì tới retry-trong-bước hay
không.

Chỗ lệch: `soBuocToiDa` LÀ một RÀNG BUỘC áp dụng CHUNG cho MỌI tác vụ
trong `chayMotTacVuVoiCauHinh` — không có tác vụ nào "miễn nhiễm" với
step cap, kể cả tác vụ hội tụ đơn giản nhất.
::
:::
::::

::::code{#viet_so_sanh_cau_hinh_vong_lap}
Hoàn thiện `chayMotTacVuVoiCauHinh` — lặp `while` tối đa
`cauHinh.soBuocToiDa` bước, mỗi lần gọi `tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc)`
VÀ CỘNG `kq.chiPhi` vào `tongChiPhi`; xử lý `"xong"`/`"loi"` NGAY như
các bài trước; NẾU `cauHinh.coPhatHienKhongTienTrien`, so `kq.chuKy` VỚI
`chuKyTruoc` để đếm `soLanLapLaiLienTiep` (bài `4`), trả `"khongTienTrien"`
khi đạt `cauHinh.soLanLapLaiToiDa`; hết vòng lặp thì trả `"hetBuoc"`.
Hoàn thiện `chayMotCauHinh` — chạy `taoBoTacVu()` QUA `chayMotTacVuVoiCauHinh`
cho TỪNG tác vụ, tính `tyLeThanhCong`, `buocTrungBinhKhiThanhCong` (chỉ
trên tác vụ THÀNH CÔNG), VÀ `chiPhiTrungBinh` (trên MỌI tác vụ).

```typescript title=starter
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuHoiTuDayDu(n: number, chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKetDinhDayDu(chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuCanRetryTrongBuoc(n: number, chiPhiMoiLuotGoi: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  const demTienBo = { soTienBo: 0 };
  return {
    demBuoc,
    chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      if (soLanThuLaiToiDaTrongBuoc < 1) {
        return { trangThai: "chua_xong", chuKy: "can_retry_trong_buoc", chiPhi: chiPhiMoiLuotGoi };
      }
      demTienBo.soTienBo++;
      const xong = demTienBo.soTienBo >= n;
      const chiPhiBuocNay = chiPhiMoiLuotGoi * 2;
      if (xong) {
        return { trangThai: "xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay, giaTri: "hoan_thanh_qua_retry" };
      }
      return { trangThai: "chua_xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  ten: string;
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

interface HangBangSoSanh {
  ten: string;
  tyLeThanhCong: number;
  buocTrungBinhKhiThanhCong: number;
  chiPhiTrungBinh: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  ___
}

function chayMotCauHinh(cauHinh: CauHinhVongLap, taoBoTacVu: () => TacVuVongLapDayDu[]): HangBangSoSanh {
  ___
}

function soSanhNhieuCauHinhVongLap(
  cacCauHinh: CauHinhVongLap[],
  taoBoTacVu: () => TacVuVongLapDayDu[],
): HangBangSoSanh[] {
  return cacCauHinh.map((ch) => chayMotCauHinh(ch, taoBoTacVu));
}

function taoBoTacVuHonHop(): TacVuVongLapDayDu[] {
  return [taoTacVuHoiTuDayDu(3, 2), taoTacVuKetDinhDayDu(2), taoTacVuCanRetryTrongBuoc(2, 3)];
}

const bangSoSanh = soSanhNhieuCauHinhVongLap(
  [
    { ten: "toi-gian", soBuocToiDa: 20, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0 },
    { ten: "voi-no-progress", soBuocToiDa: 20, coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0 },
    { ten: "voi-retry-trong-buoc", soBuocToiDa: 20, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 },
    { ten: "day-du", soBuocToiDa: 20, coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 },
  ],
  taoBoTacVuHonHop,
);
for (const hang of bangSoSanh) console.log(JSON.stringify(hang));
```

```typescript title=solution
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuocDayDu = { trangThai: TrangThaiBuoc; chuKy: string; chiPhi: number; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuVongLapDayDu {
  demBuoc: DemBuoc;
  chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu;
}

function taoTacVuHoiTuDayDu(n: number, chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      if (xong) return { trangThai: "xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc, giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong", chuKy: `buoc_${demBuoc.soBuocDaChay}`, chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuKetDinhDayDu(chiPhiMoiBuoc: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(_soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", chiPhi: chiPhiMoiBuoc };
    },
  };
}

function taoTacVuCanRetryTrongBuoc(n: number, chiPhiMoiLuotGoi: number): TacVuVongLapDayDu {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  const demTienBo = { soTienBo: 0 };
  return {
    demBuoc,
    chayMotBuoc(soLanThuLaiToiDaTrongBuoc: number): KetQuaBuocDayDu {
      demBuoc.soBuocDaChay++;
      if (soLanThuLaiToiDaTrongBuoc < 1) {
        return { trangThai: "chua_xong", chuKy: "can_retry_trong_buoc", chiPhi: chiPhiMoiLuotGoi };
      }
      demTienBo.soTienBo++;
      const xong = demTienBo.soTienBo >= n;
      const chiPhiBuocNay = chiPhiMoiLuotGoi * 2;
      if (xong) {
        return { trangThai: "xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay, giaTri: "hoan_thanh_qua_retry" };
      }
      return { trangThai: "chua_xong", chuKy: `tien_bo_${demTienBo.soTienBo}`, chiPhi: chiPhiBuocNay };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "loi"; loi: string; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "hetBuoc"; soBuocDaDung: number; tongChiPhi: number }
  | { trangThai: "khongTienTrien"; soBuocDaDung: number; tongChiPhi: number };

interface CauHinhVongLap {
  ten: string;
  soBuocToiDa: number;
  coPhatHienKhongTienTrien: boolean;
  soLanLapLaiToiDa: number;
  soLanThuLaiToiDaTrongBuoc: number;
}

interface HangBangSoSanh {
  ten: string;
  tyLeThanhCong: number;
  buocTrungBinhKhiThanhCong: number;
  chiPhiTrungBinh: number;
}

function chayMotTacVuVoiCauHinh(tacVu: TacVuVongLapDayDu, cauHinh: CauHinhVongLap): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let tongChiPhi = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < cauHinh.soBuocToiDa) {
    const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc);
    soBuoc++;
    tongChiPhi += kq.chiPhi;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "", soBuocDaDung: soBuoc, tongChiPhi };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu", soBuocDaDung: soBuoc, tongChiPhi };
    if (cauHinh.coPhatHienKhongTienTrien) {
      if (kq.chuKy === chuKyTruoc) {
        soLanLapLaiLienTiep++;
      } else {
        soLanLapLaiLienTiep = 1;
        chuKyTruoc = kq.chuKy;
      }
      if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) {
        return { trangThai: "khongTienTrien", soBuocDaDung: soBuoc, tongChiPhi };
      }
    }
  }
  return { trangThai: "hetBuoc", soBuocDaDung: soBuoc, tongChiPhi };
}

function chayMotCauHinh(cauHinh: CauHinhVongLap, taoBoTacVu: () => TacVuVongLapDayDu[]): HangBangSoSanh {
  const dsTacVu = taoBoTacVu();
  const ketQua = dsTacVu.map((tv) => chayMotTacVuVoiCauHinh(tv, cauHinh));
  const thanhCong = ketQua.filter((k) => k.trangThai === "thanhCong");
  return {
    ten: cauHinh.ten,
    tyLeThanhCong: ketQua.length === 0 ? 0 : thanhCong.length / ketQua.length,
    buocTrungBinhKhiThanhCong:
      thanhCong.length === 0 ? 0 : thanhCong.reduce((acc, k) => acc + k.soBuocDaDung, 0) / thanhCong.length,
    chiPhiTrungBinh: ketQua.length === 0 ? 0 : ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0) / ketQua.length,
  };
}

function soSanhNhieuCauHinhVongLap(
  cacCauHinh: CauHinhVongLap[],
  taoBoTacVu: () => TacVuVongLapDayDu[],
): HangBangSoSanh[] {
  return cacCauHinh.map((ch) => chayMotCauHinh(ch, taoBoTacVu));
}

function taoBoTacVuHonHop(): TacVuVongLapDayDu[] {
  return [taoTacVuHoiTuDayDu(3, 2), taoTacVuKetDinhDayDu(2), taoTacVuCanRetryTrongBuoc(2, 3)];
}

const bangSoSanh = soSanhNhieuCauHinhVongLap(
  [
    { ten: "toi-gian", soBuocToiDa: 20, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0 },
    { ten: "voi-no-progress", soBuocToiDa: 20, coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 0 },
    { ten: "voi-retry-trong-buoc", soBuocToiDa: 20, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 },
    { ten: "day-du", soBuocToiDa: 20, coPhatHienKhongTienTrien: true, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 },
  ],
  taoBoTacVuHonHop,
);
for (const hang of bangSoSanh) console.log(JSON.stringify(hang));
```

```typescript title=test
if (bangSoSanh.length !== 4) throw new Error("soSanhNhieuCauHinhVongLap phai tra ve dung 4 phan tu (dung so cau hinh)");

const [toiGian, voiNoProgress, voiRetry, dayDu] = bangSoSanh;

if (Math.abs(toiGian!.tyLeThanhCong - 1 / 3) > 1e-9) throw new Error("toi-gian: tyLeThanhCong phai la 1/3 (chi tac vu hoi tu thanh cong)");
if (toiGian!.buocTrungBinhKhiThanhCong !== 3) throw new Error("toi-gian: buocTrungBinhKhiThanhCong phai la 3");
if (Math.abs(toiGian!.chiPhiTrungBinh - 106 / 3) > 1e-9) throw new Error("toi-gian: chiPhiTrungBinh phai la 106/3");

if (Math.abs(voiNoProgress!.tyLeThanhCong - 1 / 3) > 1e-9) throw new Error("voi-no-progress: tyLeThanhCong phai GIONG HET toi-gian (1/3) -- no-progress khong doi completion rate");
if (voiNoProgress!.buocTrungBinhKhiThanhCong !== 3) throw new Error("voi-no-progress: buocTrungBinhKhiThanhCong phai GIONG HET toi-gian (3)");
if (Math.abs(voiNoProgress!.chiPhiTrungBinh - 7) > 1e-9) throw new Error("voi-no-progress: chiPhiTrungBinh phai la 7 -- giam RAT nhieu so voi toi-gian (106/3)");

if (Math.abs(voiRetry!.tyLeThanhCong - 2 / 3) > 1e-9) throw new Error("voi-retry-trong-buoc: tyLeThanhCong phai la 2/3 -- retry-trong-buoc cuu them mot tac vu");
if (Math.abs(voiRetry!.buocTrungBinhKhiThanhCong - 2.5) > 1e-9) throw new Error("voi-retry-trong-buoc: buocTrungBinhKhiThanhCong phai la 2.5");
if (Math.abs(voiRetry!.chiPhiTrungBinh - 58 / 3) > 1e-9) throw new Error("voi-retry-trong-buoc: chiPhiTrungBinh phai la 58/3");

if (Math.abs(dayDu!.tyLeThanhCong - 2 / 3) > 1e-9) throw new Error("day-du: tyLeThanhCong phai GIONG HET voi-retry-trong-buoc (2/3)");
if (Math.abs(dayDu!.buocTrungBinhKhiThanhCong - 2.5) > 1e-9) throw new Error("day-du: buocTrungBinhKhiThanhCong phai GIONG HET voi-retry-trong-buoc (2.5)");
if (Math.abs(dayDu!.chiPhiTrungBinh - 8) > 1e-9) throw new Error("day-du: chiPhiTrungBinh phai la 8 -- THAP NHAT trong bon cau hinh, ket hop ca hai loi ich");

const bangMotCauHinh = soSanhNhieuCauHinhVongLap(
  [{ ten: "rieng", soBuocToiDa: 1, coPhatHienKhongTienTrien: false, soLanLapLaiToiDa: 3, soLanThuLaiToiDaTrongBuoc: 1 }],
  taoBoTacVuHonHop,
);
if (bangMotCauHinh.length !== 1) throw new Error("doi so luong cau hinh trong danh sach phai doi do dai mang tra ve");
if (bangMotCauHinh[0]!.tyLeThanhCong !== 0) throw new Error("soBuocToiDa=1 qua nho: KHONG tac vu nao kip thanh cong, tyLeThanhCong phai la 0");
if (Math.abs(bangMotCauHinh[0]!.chiPhiTrungBinh - 10 / 3) > 1e-9) throw new Error("soBuocToiDa=1: chiPhiTrungBinh phai la 10/3 (2+2+6, chia 3)");

const ketQuaRieng = chayMotTacVuVoiCauHinh(taoTacVuKetDinhDayDu(5), {
  ten: "x",
  soBuocToiDa: 100,
  coPhatHienKhongTienTrien: true,
  soLanLapLaiToiDa: 4,
  soLanThuLaiToiDaTrongBuoc: 0,
});
if (ketQuaRieng.trangThai !== "khongTienTrien") throw new Error("tac vu ket dinh voi phat hien BAT phai la khongTienTrien");
if (ketQuaRieng.soBuocDaDung !== 4) throw new Error("phat hien phai kich hoat DUNG o buoc khop soLanLapLaiToiDa (4), khong chay het 100 buoc");
if (ketQuaRieng.tongChiPhi !== 20) throw new Error("4 buoc x chi phi 5 moi buoc phai cho tongChiPhi = 20");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayMotTacVuVoiCauHinh): mot vong while nhu bai 4/7, dung cauHinh.soBuocToiDa lam gioi han, goi tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc), cong kq.chiPhi vao tongChiPhi; xu ly xong/loi truoc; SAU DO, CHI khi cauHinh.coPhatHienKhongTienTrien, dem soLanLapLaiLienTiep dua tren kq.chuKy nhu bai 4, tra khongTienTrien khi du nguong cauHinh.soLanLapLaiToiDa. Cho hai (chayMotCauHinh): goi taoBoTacVu() de lay MOT bo tac vu MOI, .map qua chayMotTacVuVoiCauHinh cho tung tac vu, roi tinh ba con so: tyLeThanhCong (tren MOI ket qua), buocTrungBinhKhiThanhCong (CHI tren ket qua thanhCong), chiPhiTrungBinh (tren MOI ket qua)."
- kind: strategy
  body: "Cho dau: let soBuoc = 0; let tongChiPhi = 0; let chuKyTruoc: string | null = null; let soLanLapLaiLienTiep = 0; while (soBuoc < cauHinh.soBuocToiDa) { const kq = tacVu.chayMotBuoc(cauHinh.soLanThuLaiToiDaTrongBuoc); soBuoc++; tongChiPhi += kq.chiPhi; if (kq.trangThai === \"xong\") return { trangThai: \"thanhCong\", giaTri: kq.giaTri ?? \"\", soBuocDaDung: soBuoc, tongChiPhi }; if (kq.trangThai === \"loi\") return { trangThai: \"loi\", loi: \"loi_tac_vu\", soBuocDaDung: soBuoc, tongChiPhi }; if (cauHinh.coPhatHienKhongTienTrien) { if (kq.chuKy === chuKyTruoc) { soLanLapLaiLienTiep++; } else { soLanLapLaiLienTiep = 1; chuKyTruoc = kq.chuKy; } if (soLanLapLaiLienTiep >= cauHinh.soLanLapLaiToiDa) { return { trangThai: \"khongTienTrien\", soBuocDaDung: soBuoc, tongChiPhi }; } } } return { trangThai: \"hetBuoc\", soBuocDaDung: soBuoc, tongChiPhi }; Cho hai: const dsTacVu = taoBoTacVu(); const ketQua = dsTacVu.map((tv) => chayMotTacVuVoiCauHinh(tv, cauHinh)); const thanhCong = ketQua.filter((k) => k.trangThai === \"thanhCong\"); return { ten: cauHinh.ten, tyLeThanhCong: ketQua.length === 0 ? 0 : thanhCong.length / ketQua.length, buocTrungBinhKhiThanhCong: thanhCong.length === 0 ? 0 : thanhCong.reduce((acc, k) => acc + k.soBuocDaDung, 0) / thanhCong.length, chiPhiTrungBinh: ketQua.length === 0 ? 0 : ketQua.reduce((acc, k) => acc + k.tongChiPhi, 0) / ketQua.length };"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "{\"ten\":\"day-du\",\"tyLeThanhCong\":0.6666666666666666,\"buocTrungBinhKhiThanhCong\":2.5,\"chiPhiTrungBinh\":8}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn cấu hình, BA cột số — mỗi cột kể một câu chuyện KHÁC nhau về CÙNG
một sự thay đổi cấu hình. Track NÀY còn đúng MỘT bài: ráp NGUYÊN VĂN
MỌI cơ chế đã học Ở CẢ q9.4a VÀ q9.4b thành MỘT vòng lặp production-
grade, chạy trên N tác vụ VỚI một TỔNG ngân sách bước hữu hạn.
::::

::::reflect{#nghi-lai}
Bảng bốn hàng Ở bài này không chứng minh "cấu hình `day-du` LÀ tốt
nhất" theo một nghĩa MƠ HỒ — nó chứng minh CHÍNH XÁC hai cơ chế đóng
góp Ở HAI chỗ khác nhau, không thể gộp thành một con số duy nhất mà
không mất thông tin: no-progress detection CHỈ giảm chi phí (cột thứ
ba), hoàn toàn KHÔNG chạm tới completion rate (cột đầu); retry-trong-
bước CHỈ tăng completion rate, không hề làm tác vụ hội tụ nhanh hơn
(buocTrungBinhKhiThanhCong không đổi giữa `"toi-gian"` VÀ giả sử NẾU
thêm retry mà KHÔNG thêm no-progress). Đây chính LÀ lý do MASTERPLAN
định nghĩa LOOP bằng BA trục tách biệt: một kỹ sư chỉ nhìn MỘT con số
(ví dụ completion rate) sẽ KHÔNG BAO GIỜ biết được liệu THÊM một cơ chế
có đáng hay không, VÌ SAO nó đáng, VÀ nó đáng Ở KHÍA CẠNH nào.
::::

::::checkpoint{mastery=0.9}
::::
