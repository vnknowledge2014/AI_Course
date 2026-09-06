---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.phat-hien-khong-tien-trien
title: "Phát hiện không tiến triển — dừng SỚM có chủ đích khi lặp lại đúng một lỗi"
summary: "TacVuCoChuKy{demBuoc; chayMotBuoc(): KetQuaBuocCoChuKy} mỗi bước trả THÊM một chuKy (chữ ký hành động/lỗi của bước đó). chayVongLapCoPhatHienKhongTienTrien(tacVu, soBuocToiDa, soLanLapLaiToiDa) đếm soLanLapLaiLienTiep MỖI KHI chuKy hiện tại TRÙNG chuKy bước liền trước; đạt soLanLapLaiToiDa lần LIÊN TIẾP thì trả về trangThai MỚI 'khongTienTrien' (khác cả 'thanhCong'/'loi'/'hetBuoc') -- dừng SỚM có chủ đích, KHÁC dừng sớm SAI ở bài 3 vì đây tác vụ THẬT SỰ không tiến triển, tiếp tục cũng vô ích. Trên taoTacVuKetDinhLapLaiLoi() (LUÔN trả cùng chuKy 'cung_mot_loi', không bao giờ đổi trạng thái), soBuocToiDa=100, soLanLapLaiToiDa=3: phát hiện khongTienTrien Ở bước 3, dùng ĐÚNG 3 lần gọi -- so với KHÔNG có phát hiện (chayVongLapCoCap bài 2 kiểu cũ) trên CÙNG tác vụ: chạy hết TOÀN BỘ 100 bước rồi mới báo hetBuoc, lãng phí 97 bước. Trên taoTacVuHoiTuSauNBuocCoChuKy(4) (chuKy đổi MỖI bước, không lặp lại): phát hiện không tiến triển KHÔNG BAO GIỜ kích hoạt, tác vụ vẫn thanhCong đúng ở bước 4 -- xác nhận cơ chế này CHỈ chặn tác vụ THẬT SỰ kẹt, không ảnh hưởng tác vụ đang tiến triển bình thường."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [kna.phat-hien-khong-tien-trien]
requires: [kna.dieu-kien-dung-som-sai]
concepts: [kna.phat-hien-khong-tien-trien]
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
Ba bài trước đã dạy ba cách một vòng lặp có thể SAI: không dừng (bài `1`),
chạm step cap (bài `2`, không sai nhưng cần đo riêng), dừng SAI quá sớm (bài
`3`). Còn một chế độ NỮA, tinh vi hơn: vòng lặp vẫn "sống" — chưa hết step
cap, không dừng sai — nhưng KHÔNG TIẾN TRIỂN, lặp lại ĐÚNG một hành động
hoặc một lỗi, hết lần này tới lần khác. Bài này dạy cách PHÁT HIỆN đúng điều
đó, VÀ dừng lại — nhưng lần này LÀ dừng ĐÚNG.
::::

::::explain{#chu_ky_hanh_dong}
Một `TacVuCoChuKy` mỗi bước trả về THÊM một `chuKy` — một "chữ ký" đại diện
cho HÀNH ĐỘNG hoặc LỖI của bước đó (Ở đây LÀ một chuỗi). `taoTacVuKetDinhLapLaiLoi`
mô phỏng đúng một tác vụ KẸT: mọi bước đều trả về CÙNG một `chuKy` — nghĩa LÀ
agent đang lặp lại ĐÚNG một hành động, không hề đổi cách tiếp cận:

```typescript title=readonly
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocCoChuKy = { trangThai: "xong" | "chua_xong" | "loi"; chuKy: string; giaTri: string };

interface TacVuCoChuKy {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChuKy;
}

function taoTacVuKetDinhLapLaiLoi(): TacVuCoChuKy {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChuKy {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", giaTri: "chua_co_gia_tri" };
    },
  };
}

const tv = taoTacVuKetDinhLapLaiLoi();
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
```

```text title=readonly
{"trangThai":"chua_xong","chuKy":"cung_mot_loi","giaTri":"chua_co_gia_tri"}
{"trangThai":"chua_xong","chuKy":"cung_mot_loi","giaTri":"chua_co_gia_tri"}
{"trangThai":"chua_xong","chuKy":"cung_mot_loi","giaTri":"chua_co_gia_tri"}
```

`trangThai` LUÔN LÀ `"chua_xong"` — KHÔNG phải `"loi"` (dứt khoát, bài `2`
đã xử lý), cũng KHÔNG BAO GIỜ `"xong"`. Tác vụ này "sống" mãi theo nghĩa step
cap, nhưng `chuKy` không hề đổi — đúng dấu hiệu của một agent đang lặp lại
CÙNG một hành động vô ích.
::::

::::example{#phat_hien_va_tiet_kiem_buoc}
`chayVongLapCoPhatHienKhongTienTrien` đếm `soLanLapLaiLienTiep` — số lần
LIÊN TIẾP `chuKy` hiện tại TRÙNG `chuKy` của bước NGAY TRƯỚC. Đạt đủ
`soLanLapLaiToiDa` lần thì dừng lại VỚI trạng thái MỚI — `"khongTienTrien"`
— tách biệt khỏi `"hetBuoc"`:

```typescript title=readonly
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocCoChuKy = { trangThai: "xong" | "chua_xong" | "loi"; chuKy: string; giaTri: string };

interface TacVuCoChuKy {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChuKy;
}

function taoTacVuKetDinhLapLaiLoi(): TacVuCoChuKy {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChuKy {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", giaTri: "chua_co_gia_tri" };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

// Kieu cu (bai 2): KHONG phat hien khong tien trien.
function chayVongLapCoCap(tacVu: TacVuCoChuKy, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" }
  | { trangThai: "khongTienTrien" };

function chayVongLapCoPhatHienKhongTienTrien(
  tacVu: TacVuCoChuKy,
  soBuocToiDa: number,
  soLanLapLaiToiDa: number,
): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
    if (kq.chuKy === chuKyTruoc) {
      soLanLapLaiLienTiep++;
    } else {
      soLanLapLaiLienTiep = 1;
      chuKyTruoc = kq.chuKy;
    }
    if (soLanLapLaiLienTiep >= soLanLapLaiToiDa) {
      return { trangThai: "khongTienTrien" };
    }
  }
  return { trangThai: "hetBuoc" };
}

const tvKhong = taoTacVuKetDinhLapLaiLoi();
console.log("khong phat hien:", JSON.stringify(chayVongLapCoCap(tvKhong, 100)), tvKhong.demBuoc.soBuocDaChay);

const tvCo = taoTacVuKetDinhLapLaiLoi();
console.log("co phat hien:   ", JSON.stringify(chayVongLapCoPhatHienKhongTienTrien(tvCo, 100, 3)), tvCo.demBuoc.soBuocDaChay);
```

```text title=readonly
khong phat hien: {"trangThai":"hetBuoc"} 100
co phat hien:    {"trangThai":"khongTienTrien"} 3
```

CÙNG một tác vụ kẹt, CÙNG `soBuocToiDa = 100` — không phát hiện chạy HẾT
`100` bước rồi mới báo `"hetBuoc"` (lãng phí toàn bộ ngân sách). Có phát
hiện dừng NGAY Ở bước `3` (khớp `soLanLapLaiToiDa`), báo `"khongTienTrien"`
— tiết kiệm `97` bước cho một tác vụ KHÁC có thể dùng số bước đó tốt hơn.
::::

::::predict{#doan-tac-vu-hoi-tu-co-chu-ky commitOnce}
Chạy `chayVongLapCoPhatHienKhongTienTrien` (ngưỡng `soLanLapLaiToiDa = 3`)
trên MỘT tác vụ hội tụ THẬT (mỗi bước có `chuKy` KHÁC nhau, ví dụ
`"buoc_1"`, `"buoc_2"`, ... cho tới khi `"xong"` Ở bước `4`) — cơ chế phát
hiện không tiến triển có bao giờ kích hoạt SAI, chặn nhầm tác vụ này không?

:::opt{correct}
Không — `chuKy` đổi Ở MỖI bước nên `soLanLapLaiLienTiep` bị reset về `1`
liên tục, KHÔNG BAO GIỜ đạt tới `soLanLapLaiToiDa (3)`; tác vụ vẫn báo
`"thanhCong"` đúng Ở bước `4`, y hệt như không có cơ chế phát hiện nào
:::
:::opt
Có — sau đúng `3` bước bất kỳ, cơ chế LUÔN kích hoạt `"khongTienTrien"`, bất
kể `chuKy` có đổi hay không
::why
Nhầm điều kiện kích hoạt LÀ "đã chạy đủ `3` bước" — nhưng điều kiện THẬT LÀ
"CÙNG một `chuKy` xuất hiện LIÊN TIẾP đủ `3` lần". Hai điều kiện này khác
nhau hoàn toàn.

Chỗ lệch: mỗi lần `kq.chuKy !== chuKyTruoc`, `soLanLapLaiLienTiep` bị RESET
về `1` (không phải tăng dần) — với một tác vụ mà `chuKy` LUÔN khác bước
trước, biến đếm đó không bao giờ vượt quá `1`.
::
:::
:::opt
Có thể có hoặc không, tuỳ vào việc `soBuocToiDa` được đặt LÀ bao nhiêu
::why
Nhầm rằng `soBuocToiDa` (step cap tổng) ảnh hưởng tới việc phát hiện không
tiến triển CÓ kích hoạt hay không — nhưng hai tham số này ĐỘC LẬP: một cái
đếm TỔNG số bước, một cái đếm số lần LẶP LẠI LIÊN TIẾP của MỘT chữ ký.

Chỗ lệch: miễn `chuKy` tiếp tục đổi Ở MỌI bước, `soLanLapLaiLienTiep` không
bao giờ vượt `1` — bất kể `soBuocToiDa` LÀ `4`, `100`, hay bất kỳ số nào lớn
hơn số bước tác vụ cần để hội tụ.
::
:::
::::

::::code{#viet_phat_hien_khong_tien_trien}
Hoàn thiện `chayVongLapCoPhatHienKhongTienTrien` — lặp `while` tối đa
`soBuocToiDa` lần; NẾU `trangThai` LÀ `"xong"` hoặc `"loi"`, trả về NGAY như
bài `2`; NGƯỢC LẠI, so `chuKy` hiện tại VỚI `chuKyTruoc` — TRÙNG thì tăng
`soLanLapLaiLienTiep`, KHÁC thì reset `soLanLapLaiLienTiep` VỀ `1` VÀ cập
nhật `chuKyTruoc`; nếu `soLanLapLaiLienTiep` đạt `soLanLapLaiToiDa`, trả về
`{ trangThai: "khongTienTrien" }` NGAY; hết vòng lặp thì trả `{ trangThai:
"hetBuoc" }`. Hoàn thiện `chayTrenNhieuTacVuCoPhatHien` — với MỖI tác vụ
trong `dsTacVu`, gọi hàm trên, đẩy kết quả vào mảng trả về.

```typescript title=starter
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocCoChuKy = { trangThai: "xong" | "chua_xong" | "loi"; chuKy: string; giaTri: string };

interface TacVuCoChuKy {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChuKy;
}

function taoTacVuKetDinhLapLaiLoi(): TacVuCoChuKy {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChuKy {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", giaTri: "chua_co_gia_tri" };
    },
  };
}

function taoTacVuHoiTuSauNBuocCoChuKy(n: number): TacVuCoChuKy {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChuKy {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        chuKy: `buoc_${demBuoc.soBuocDaChay}`,
        giaTri: xong ? "hoan_thanh_that_su" : "chua_co_gia_tri",
      };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" }
  | { trangThai: "khongTienTrien" };

function chayVongLapCoPhatHienKhongTienTrien(
  tacVu: TacVuCoChuKy,
  soBuocToiDa: number,
  soLanLapLaiToiDa: number,
): KetQuaVongLapDayDu {
  ___
}

function chayTrenNhieuTacVuCoPhatHien(
  dsTacVu: TacVuCoChuKy[],
  soBuocToiDa: number,
  soLanLapLaiToiDa: number,
): KetQuaVongLapDayDu[] {
  ___
}

const dsTacVu3: TacVuCoChuKy[] = [
  taoTacVuKetDinhLapLaiLoi(),
  taoTacVuHoiTuSauNBuocCoChuKy(4),
  taoTacVuKetDinhLapLaiLoi(),
];
const ketQua3 = chayTrenNhieuTacVuCoPhatHien(dsTacVu3, 100, 3);
console.log(JSON.stringify(ketQua3.map((k) => k.trangThai)));
```

```typescript title=solution
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocCoChuKy = { trangThai: "xong" | "chua_xong" | "loi"; chuKy: string; giaTri: string };

interface TacVuCoChuKy {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChuKy;
}

function taoTacVuKetDinhLapLaiLoi(): TacVuCoChuKy {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChuKy {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong", chuKy: "cung_mot_loi", giaTri: "chua_co_gia_tri" };
    },
  };
}

function taoTacVuHoiTuSauNBuocCoChuKy(n: number): TacVuCoChuKy {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuocCoChuKy {
      demBuoc.soBuocDaChay++;
      const xong = demBuoc.soBuocDaChay >= n;
      return {
        trangThai: xong ? "xong" : "chua_xong",
        chuKy: `buoc_${demBuoc.soBuocDaChay}`,
        giaTri: xong ? "hoan_thanh_that_su" : "chua_co_gia_tri",
      };
    },
  };
}

type KetQuaVongLapDayDu =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" }
  | { trangThai: "khongTienTrien" };

function chayVongLapCoPhatHienKhongTienTrien(
  tacVu: TacVuCoChuKy,
  soBuocToiDa: number,
  soLanLapLaiToiDa: number,
): KetQuaVongLapDayDu {
  let soBuoc = 0;
  let chuKyTruoc: string | null = null;
  let soLanLapLaiLienTiep = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
    if (kq.chuKy === chuKyTruoc) {
      soLanLapLaiLienTiep++;
    } else {
      soLanLapLaiLienTiep = 1;
      chuKyTruoc = kq.chuKy;
    }
    if (soLanLapLaiLienTiep >= soLanLapLaiToiDa) {
      return { trangThai: "khongTienTrien" };
    }
  }
  return { trangThai: "hetBuoc" };
}

function chayTrenNhieuTacVuCoPhatHien(
  dsTacVu: TacVuCoChuKy[],
  soBuocToiDa: number,
  soLanLapLaiToiDa: number,
): KetQuaVongLapDayDu[] {
  const ketQua: KetQuaVongLapDayDu[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(chayVongLapCoPhatHienKhongTienTrien(tv, soBuocToiDa, soLanLapLaiToiDa));
  }
  return ketQua;
}

const dsTacVu3: TacVuCoChuKy[] = [
  taoTacVuKetDinhLapLaiLoi(),
  taoTacVuHoiTuSauNBuocCoChuKy(4),
  taoTacVuKetDinhLapLaiLoi(),
];
const ketQua3 = chayTrenNhieuTacVuCoPhatHien(dsTacVu3, 100, 3);
console.log(JSON.stringify(ketQua3.map((k) => k.trangThai)));
```

```typescript title=test
if (ketQua3.length !== 3) throw new Error("chayTrenNhieuTacVuCoPhatHien phai tra ve mang dung 3 phan tu");
if (JSON.stringify(ketQua3.map((k) => k.trangThai)) !== JSON.stringify(["khongTienTrien", "thanhCong", "khongTienTrien"])) {
  throw new Error("hai tac vu ket dinh phai la khongTienTrien, tac vu hoi tu that phai la thanhCong");
}

const tvKet = taoTacVuKetDinhLapLaiLoi();
const kqKet = chayVongLapCoPhatHienKhongTienTrien(tvKet, 100, 3);
if (kqKet.trangThai !== "khongTienTrien") throw new Error("tac vu ket dinh lap lai loi phai duoc phat hien khongTienTrien");
if (tvKet.demBuoc.soBuocDaChay !== 3) throw new Error("phat hien phai kich hoat DUNG o buoc 3 (khop soLanLapLaiToiDa), KHONG chay het 100 buoc");

const tvKet5 = taoTacVuKetDinhLapLaiLoi();
const kqKet5 = chayVongLapCoPhatHienKhongTienTrien(tvKet5, 100, 5);
if (tvKet5.demBuoc.soBuocDaChay !== 5) throw new Error("doi soLanLapLaiToiDa tu 3 sang 5 phai doi diem kich hoat thanh buoc 5 -- tham so phai duoc dung that");

const tvHoiTu = taoTacVuHoiTuSauNBuocCoChuKy(4);
const kqHoiTu = chayVongLapCoPhatHienKhongTienTrien(tvHoiTu, 100, 3);
if (kqHoiTu.trangThai !== "thanhCong") throw new Error("tac vu hoi tu that (chuKy doi moi buoc) KHONG duoc bi phat hien nham la khongTienTrien");
if (tvHoiTu.demBuoc.soBuocDaChay !== 4) throw new Error("tac vu hoi tu that phai chay DUNG 4 buoc, phat hien khong tien trien khong duoc lam no dung som hon");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayVongLapCoPhatHienKhongTienTrien): mot vong while nhu bai 2, nhung SAU khi kiem xong/loi, so sanh kq.chuKy voi chuKyTruoc -- TRUNG thi tang soLanLapLaiLienTiep, KHAC thi dat lai soLanLapLaiLienTiep = 1 VA cap nhat chuKyTruoc = kq.chuKy; neu soLanLapLaiLienTiep >= soLanLapLaiToiDa thi tra ve { trangThai: \"khongTienTrien\" } NGAY. Cho hai (chayTrenNhieuTacVuCoPhatHien): mot vong for-of qua dsTacVu, goi ham tren voi CA BA tham so, day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: let soBuoc = 0; let chuKyTruoc: string | null = null; let soLanLapLaiLienTiep = 0; while (soBuoc < soBuocToiDa) { const kq = tacVu.chayMotBuoc(); soBuoc++; if (kq.trangThai === \"xong\") return { trangThai: \"thanhCong\", giaTri: kq.giaTri }; if (kq.trangThai === \"loi\") return { trangThai: \"loi\", loi: \"loi_tac_vu\" }; if (kq.chuKy === chuKyTruoc) { soLanLapLaiLienTiep++; } else { soLanLapLaiLienTiep = 1; chuKyTruoc = kq.chuKy; } if (soLanLapLaiLienTiep >= soLanLapLaiToiDa) { return { trangThai: \"khongTienTrien\" }; } } return { trangThai: \"hetBuoc\" }; Cho hai: const ketQua: KetQuaVongLapDayDu[] = []; for (const tv of dsTacVu) { ketQua.push(chayVongLapCoPhatHienKhongTienTrien(tv, soBuocToiDa, soLanLapLaiToiDa)); } return ketQua;"
- kind: one-line
  body: "Sao chep dung hai khoi o phan Strategy vao dung vi tri tuong ung."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "[\"khongTienTrien\",\"thanhCong\",\"khongTienTrien\"]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`3` bước thay vì `100` — VÀ tác vụ hội tụ thật hoàn toàn KHÔNG bị đụng tới.
Bốn cơ chế đã có: hình dạng vòng lặp cơ bản, step cap, tránh dừng sớm sai,
VÀ giờ LÀ phát hiện không tiến triển. Bài sau lùi lại một bước, nhìn từ góc
độ hàm học: cả bốn cơ chế NÀY thật ra LÀ MỘT hình dạng duy nhất, được lập
trình hàm gọi tên LÀ "fold trên State".
::::

::::reflect{#nghi-lai}
Phát hiện không tiến triển KHÁC hẳn ba cơ chế trước Ở một điểm tinh tế: nó
KHÔNG nhìn vào MỘT bước riêng lẻ (như `"xong"`/`"loi"` Ở bài `1`-`2`, hay
`tinHieuOn` Ở bài `3`) — nó nhìn vào MỐI QUAN HỆ giữa các bước LIÊN TIẾP
nhau, cụ thể LÀ "chữ ký này có GIỐNG chữ ký ngay trước không". Đây LÀ lần
đầu tiên vòng lặp trong quest này cần NHỚ một mẩu thông tin XUYÊN QUA nhiều
bước (`chuKyTruoc`, `soLanLapLaiLienTiep`) — không chỉ đếm số bước như bài
`1`-`2`. VÀ khác VỚI dừng sớm SAI Ở bài `3` (dừng dựa trên tín hiệu không
liên quan tới trạng thái thật), dừng Ở đây LÀ dừng ĐÚNG: khi CHÍNH tác vụ
LẶP LẠI đúng một hành động không đổi, tiếp tục lặp thêm KHÔNG hề đưa nó tới
gần `"xong"` hơn — dừng sớm LÀ quyết định ĐÚNG, không phải một lỗi.
::::

::::checkpoint{mastery=0.88}
::::
