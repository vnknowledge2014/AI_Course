---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.boss-vong-lap-don-gian-vs-day-du
title: "BOSS q9.4a — vongLapDonGian vs vongLapDayDu, đo steps-to-success VÀ %chạm step cap"
summary: "vongLapDonGian(tacVu, soBuocToiDa) ráp NGUYÊN VĂN bài 2 (chayVongLapCoCap: step cap, KHÔNG phát hiện không tiến triển). vongLapDayDu(tacVu, soBuocToiDa, soLanLapLaiToiDa) ráp NGUYÊN VĂN bài 4 (step cap + phát hiện không tiến triển qua chuKy). Trên bộ 6 tác vụ hỗn hợp [hoiTu(2), hoiTu(4), ketDinhLapLaiLoi(), hoiTu(3), ketDinhLapLaiLoi(), hoiTu(1)] với soBuocToiDa=50, soLanLapLaiToiDa=3: vongLapDonGian=[thanhCong,thanhCong,hetBuoc,thanhCong,hetBuoc,thanhCong] (completion=0.6666666666666666, %chạm cap=0.3333333333333333, tổng bước dùng=110 -- hai tác vụ kẹt đốt hết 50 bước MỖI tác vụ). vongLapDayDu=[thanhCong,thanhCong,khongTienTrien,thanhCong,khongTienTrien,thanhCong] (completion=0.6666666666666666 GIỐNG HỆT, %chạm cap=0, tổng bước dùng=16 -- hai tác vụ kẹt bị phát hiện ở bước 3 mỗi tác vụ). steps-to-success trung bình (chỉ tính tác vụ THẬT SỰ thanhCong) = 2.5 GIỐNG HỆT ở cả hai harness -- chứng minh no-progress detection giảm 94 bước lãng phí (110→16) MÀ KHÔNG đổi completion rate lẫn tốc độ hội tụ của các tác vụ hội tụ được. Đối chứng soBuocToiDa=2 (nhỏ hơn soLanLapLaiToiDa=3): hai harness cho kết quả GIỐNG HỆT nhau ([thanhCong,hetBuoc,hetBuoc,hetBuoc,hetBuoc,thanhCong], tổng bước=11 cả hai) -- no-progress detection không kịp tích luỹ đủ 3 lần lặp trước khi step cap chặn lại, VÀ hai tác vụ hội tụ chậm hơn (cần 3-4 bước) cũng thất bại vì cap quá nhỏ, không liên quan gì tới no-progress. Đóng q9.4a tại 6/6."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kna.boss-vong-lap-don-gian-vs-day-du]
requires: [kna.vong-lap-la-fold-tren-state]
concepts: [kna.boss-vong-lap-don-gian-vs-day-du]
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

::::byte{trigger=enter mood=happy pose=jump}
Năm bài: hình dạng vòng lặp cơ bản, step cap, tránh dừng sớm sai, phát hiện
không tiến triển, VÀ hình dạng chung của cả bốn thứ đó — fold trên State.
BOSS ráp NGUYÊN VĂN hai cơ chế cốt lõi (step cap VÀ phát hiện không tiến
triển) thành hai harness hoàn chỉnh, đo TRÊN CÙNG một bộ tác vụ hỗn hợp: một
harness ĐƠN GIẢN (chỉ step cap), một harness ĐẦY ĐỦ (step cap CỘNG phát hiện
không tiến triển) — để trả lời câu hỏi mà track T9.4 mở ra: no-progress
detection có thật sự tiết kiệm bước, VÀ có phải đánh đổi bằng completion
rate không?
::::

::::explain{#hai_harness_tu_bai_2_va_bai_4}
`vongLapDonGian` LÀ đúng `chayVongLapCoCap` của bài `2`: step cap, KHÔNG có
phát hiện không tiến triển. `vongLapDayDu` LÀ đúng
`chayVongLapCoPhatHienKhongTienTrien` của bài `4`: step cap CỘNG phát hiện
`soLanLapLaiToiDa` lần lặp lại LIÊN TIẾP cùng `chuKy`. Cả hai dùng CHUNG một
giao diện tác vụ — `TacVuCoChuKy` (bài `4`) — để so sánh công bằng trên
CHÍNH CÙNG một bộ tác vụ:

```typescript title=readonly
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocCoChuKy = { trangThai: "xong" | "chua_xong" | "loi"; chuKy: string; giaTri: string };

interface TacVuCoChuKy {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChuKy;
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

type KetQuaVongLapDonGian =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

type KetQuaVongLapDayDu = KetQuaVongLapDonGian | { trangThai: "khongTienTrien" };

function vongLapDonGian(tacVu: TacVuCoChuKy, soBuocToiDa: number): KetQuaVongLapDonGian {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

function vongLapDayDu(tacVu: TacVuCoChuKy, soBuocToiDa: number, soLanLapLaiToiDa: number): KetQuaVongLapDayDu {
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

const tvHoiTu = taoTacVuHoiTuSauNBuocCoChuKy(3);
console.log(JSON.stringify(vongLapDonGian(tvHoiTu, 50)));

const tvKet = taoTacVuKetDinhLapLaiLoi();
console.log(JSON.stringify(vongLapDayDu(tvKet, 50, 3)), tvKet.demBuoc.soBuocDaChay);
```

```text title=readonly
{"trangThai":"thanhCong","giaTri":"hoan_thanh_that_su"}
{"trangThai":"khongTienTrien"} 3
```

Tác vụ hội tụ thật báo `"thanhCong"` bình thường Ở CẢ HAI harness (chưa thấy
Ở đây vì `vongLapDonGian` không hề đọc `chuKy`). Tác vụ kẹt bị `vongLapDayDu`
phát hiện Ở bước `3` — TRƯỚC KHI chạm `soBuocToiDa = 50` rất xa.
::::

::::example{#so_sanh_tren_bo_hon_hop}
Một bộ `6` tác vụ HỖN HỢP: BA tác vụ hội tụ Ở bước `2`/`4`/`3`/`1` (bốn tác
vụ hội tụ Ở bốn vị trí khác nhau trong danh sách), HAI tác vụ KẸT (lặp lại
đúng một `chuKy` mãi mãi). Chạy `vongLapDonGian` VÀ `vongLapDayDu` trên
CÙNG bộ đó, `soBuocToiDa = 50`, `soLanLapLaiToiDa = 3`:

```typescript title=readonly
// (đầy đủ hai harness + tac vu + bo tac vu hon hop — xem khối `solution` Ở
// bước ::::code bên dưới để đọc TOÀN VĂN)

function taoBoTacVuHonHop(): TacVuCoChuKy[] {
  return [
    taoTacVuHoiTuSauNBuocCoChuKy(2),
    taoTacVuHoiTuSauNBuocCoChuKy(4),
    taoTacVuKetDinhLapLaiLoi(),
    taoTacVuHoiTuSauNBuocCoChuKy(3),
    taoTacVuKetDinhLapLaiLoi(),
    taoTacVuHoiTuSauNBuocCoChuKy(1),
  ];
}

const dsDonGian = taoBoTacVuHonHop();
const ketQuaDonGian = dsDonGian.map((tv) => vongLapDonGian(tv, 50));

const dsDayDu = taoBoTacVuHonHop();
const ketQuaDayDu = dsDayDu.map((tv) => vongLapDayDu(tv, 50, 3));

console.log("don gian:", JSON.stringify(ketQuaDonGian.map((k) => k.trangThai)));
console.log("day du:  ", JSON.stringify(ketQuaDayDu.map((k) => k.trangThai)));
```

```text title=readonly
don gian: ["thanhCong","thanhCong","hetBuoc","thanhCong","hetBuoc","thanhCong"]
day du:   ["thanhCong","thanhCong","khongTienTrien","thanhCong","khongTienTrien","thanhCong"]
```

BỐN tác vụ hội tụ thật đều `"thanhCong"` Ở CẢ HAI harness — GIỐNG HỆT nhau,
Ở ĐÚNG vị trí. HAI tác vụ kẹt khác NHAU về NHÃN — `"hetBuoc"` (đơn giản, đốt
hết `50` bước) so VỚI `"khongTienTrien"` (đầy đủ, phát hiện Ở bước `3`) —
nhưng KHÔNG khác nhau về việc CÓ thành công hay không: cả hai đều KHÔNG
`"thanhCong"`, đúng như bản chất — hai tác vụ đó THẬT SỰ không bao giờ xong.
::::

::::predict{#doan-completion-rate-va-tong-buoc commitOnce}
So sánh `tinhTyLeThanhCong` (completion rate) VÀ tổng số bước THẬT SỰ đã
dùng (cộng `demBuoc.soBuocDaChay` của cả `6` tác vụ) giữa `vongLapDonGian`
VÀ `vongLapDayDu` Ở bộ tác vụ trên — hai con số NÀY khác nhau như thế nào?

:::opt{correct}
Completion rate GIỐNG HỆT nhau Ở cả hai (`0.6666666666666666` — đúng `4/6`
tác vụ hội tụ thật) — NHƯNG tổng số bước khác XA nhau: `vongLapDonGian` dùng
`110` bước (hai tác vụ kẹt đốt hết `50` bước MỖI tác vụ = `100`, cộng `10`
bước của bốn tác vụ hội tụ), `vongLapDayDu` chỉ dùng `16` bước (hai tác vụ
kẹt dừng Ở bước `3` MỖI tác vụ = `6`, cộng CÙNG `10` bước của bốn tác vụ hội
tụ) — tiết kiệm `94` bước MÀ KHÔNG đổi completion rate
:::
:::opt
Completion rate của `vongLapDayDu` cao hơn — phát hiện không tiến triển
"cứu" thêm được vài tác vụ mà `vongLapDonGian` bỏ lỡ
::why
Nhầm rằng no-progress detection LÀ một cơ chế PHỤC HỒI (như retry/fallback
Ở T9.3) — nhưng nó CHỈ dừng SỚM một tác vụ đã CHẮC CHẮN không hội tụ, không
hề làm tác vụ đó THÀNH CÔNG.

Chỗ lệch: hai tác vụ kẹt VẪN không `"thanhCong"` Ở `vongLapDayDu` — chúng
chỉ đổi NHÃN thất bại (`"khongTienTrien"` thay vì `"hetBuoc"`), completion
rate (đếm riêng biến thể `"thanhCong"`) không hề đổi.
::
:::
:::opt
Tổng số bước GIỐNG HỆT nhau Ở cả hai — no-progress detection chỉ đổi NHÃN
kết quả, không đổi số lần `chayMotBuoc()` THẬT SỰ được gọi
::why
Nhầm rằng đổi NHÃN kết quả (`"hetBuoc"` → `"khongTienTrien"`) không kèm
theo việc DỪNG SỚM số lần gọi thật — nhưng chính việc PHÁT HIỆN sớm ĐÓ mới
LÀ lý do `vongLapDayDu` trả về `{ trangThai: "khongTienTrien" }` Ở bước `3`
thay vì tiếp tục gọi `chayMotBuoc()` cho tới bước `50`.

Chỗ lệch: `vongLapDayDu` `return` NGAY khi `soLanLapLaiLienTiep` đạt
`soLanLapLaiToiDa` — nó KHÔNG chạy tiếp cho hết `soBuocToiDa` như
`vongLapDonGian` vẫn làm, nên số lần gọi THẬT SỰ (`demBuoc.soBuocDaChay`)
khác nhau rất xa giữa hai tác vụ kẹt Ở hai harness.
::
:::
::::

::::code{#viet_hai_harness_vong_lap}
Hoàn thiện `vongLapDonGian` — ĐÚNG hình dạng bài `2` (step cap, KHÔNG phát
hiện không tiến triển) NHƯNG dùng `TacVuCoChuKy` (bỏ qua trường `chuKy`).
Hoàn thiện `vongLapDayDu` — ĐÚNG hình dạng bài `4` (step cap CỘNG phát hiện
`soLanLapLaiToiDa` lần lặp lại liên tiếp cùng `chuKy`).

```typescript title=starter
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocCoChuKy = { trangThai: "xong" | "chua_xong" | "loi"; chuKy: string; giaTri: string };

interface TacVuCoChuKy {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChuKy;
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

type KetQuaVongLapDonGian =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

type KetQuaVongLapDayDu = KetQuaVongLapDonGian | { trangThai: "khongTienTrien" };

function tinhTyLeThanhCong(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhTyLeChamCap(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}

function tinhSoBuocTrungBinhKhiThanhCong(dsTacVu: TacVuCoChuKy[], ketQua: KetQuaVongLapDayDu[]): number {
  const viTriThanhCong = ketQua.map((k, i) => ({ k, i })).filter((x) => x.k.trangThai === "thanhCong");
  if (viTriThanhCong.length === 0) return 0;
  const tong = viTriThanhCong.reduce((acc, x) => acc + dsTacVu[x.i]!.demBuoc.soBuocDaChay, 0);
  return tong / viTriThanhCong.length;
}

function tongSoBuocDaDung(dsTacVu: TacVuCoChuKy[]): number {
  return dsTacVu.reduce((acc, tv) => acc + tv.demBuoc.soBuocDaChay, 0);
}

function vongLapDonGian(tacVu: TacVuCoChuKy, soBuocToiDa: number): KetQuaVongLapDonGian {
  ___
}

function vongLapDayDu(tacVu: TacVuCoChuKy, soBuocToiDa: number, soLanLapLaiToiDa: number): KetQuaVongLapDayDu {
  ___
}

function taoBoTacVuHonHop(): TacVuCoChuKy[] {
  return [
    taoTacVuHoiTuSauNBuocCoChuKy(2),
    taoTacVuHoiTuSauNBuocCoChuKy(4),
    taoTacVuKetDinhLapLaiLoi(),
    taoTacVuHoiTuSauNBuocCoChuKy(3),
    taoTacVuKetDinhLapLaiLoi(),
    taoTacVuHoiTuSauNBuocCoChuKy(1),
  ];
}

const dsDonGian = taoBoTacVuHonHop();
const ketQuaDonGian = dsDonGian.map((tv) => vongLapDonGian(tv, 50));

const dsDayDu = taoBoTacVuHonHop();
const ketQuaDayDu = dsDayDu.map((tv) => vongLapDayDu(tv, 50, 3));

console.log(
  JSON.stringify(ketQuaDonGian.map((k) => k.trangThai)),
  JSON.stringify(ketQuaDayDu.map((k) => k.trangThai)),
  tinhTyLeThanhCong(ketQuaDonGian),
  tinhTyLeThanhCong(ketQuaDayDu),
  tinhTyLeChamCap(ketQuaDonGian),
  tinhTyLeChamCap(ketQuaDayDu),
  tongSoBuocDaDung(dsDonGian),
  tongSoBuocDaDung(dsDayDu),
);
```

```typescript title=solution
type DemBuoc = { soBuocDaChay: number };
type KetQuaBuocCoChuKy = { trangThai: "xong" | "chua_xong" | "loi"; chuKy: string; giaTri: string };

interface TacVuCoChuKy {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuocCoChuKy;
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

type KetQuaVongLapDonGian =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

type KetQuaVongLapDayDu = KetQuaVongLapDonGian | { trangThai: "khongTienTrien" };

function tinhTyLeThanhCong(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhTyLeChamCap(ketQua: KetQuaVongLapDayDu[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}

function tinhSoBuocTrungBinhKhiThanhCong(dsTacVu: TacVuCoChuKy[], ketQua: KetQuaVongLapDayDu[]): number {
  const viTriThanhCong = ketQua.map((k, i) => ({ k, i })).filter((x) => x.k.trangThai === "thanhCong");
  if (viTriThanhCong.length === 0) return 0;
  const tong = viTriThanhCong.reduce((acc, x) => acc + dsTacVu[x.i]!.demBuoc.soBuocDaChay, 0);
  return tong / viTriThanhCong.length;
}

function tongSoBuocDaDung(dsTacVu: TacVuCoChuKy[]): number {
  return dsTacVu.reduce((acc, tv) => acc + tv.demBuoc.soBuocDaChay, 0);
}

function vongLapDonGian(tacVu: TacVuCoChuKy, soBuocToiDa: number): KetQuaVongLapDonGian {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

function vongLapDayDu(tacVu: TacVuCoChuKy, soBuocToiDa: number, soLanLapLaiToiDa: number): KetQuaVongLapDayDu {
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

function taoBoTacVuHonHop(): TacVuCoChuKy[] {
  return [
    taoTacVuHoiTuSauNBuocCoChuKy(2),
    taoTacVuHoiTuSauNBuocCoChuKy(4),
    taoTacVuKetDinhLapLaiLoi(),
    taoTacVuHoiTuSauNBuocCoChuKy(3),
    taoTacVuKetDinhLapLaiLoi(),
    taoTacVuHoiTuSauNBuocCoChuKy(1),
  ];
}

const dsDonGian = taoBoTacVuHonHop();
const ketQuaDonGian = dsDonGian.map((tv) => vongLapDonGian(tv, 50));

const dsDayDu = taoBoTacVuHonHop();
const ketQuaDayDu = dsDayDu.map((tv) => vongLapDayDu(tv, 50, 3));

console.log(
  JSON.stringify(ketQuaDonGian.map((k) => k.trangThai)),
  JSON.stringify(ketQuaDayDu.map((k) => k.trangThai)),
  tinhTyLeThanhCong(ketQuaDonGian),
  tinhTyLeThanhCong(ketQuaDayDu),
  tinhTyLeChamCap(ketQuaDonGian),
  tinhTyLeChamCap(ketQuaDayDu),
  tongSoBuocDaDung(dsDonGian),
  tongSoBuocDaDung(dsDayDu),
);
```

```typescript title=test
if (ketQuaDonGian.length !== 6) throw new Error("vongLapDonGian phai duoc goi tren dung 6 tac vu");
if (ketQuaDayDu.length !== 6) throw new Error("vongLapDayDu phai duoc goi tren dung 6 tac vu");

if (JSON.stringify(ketQuaDonGian.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "thanhCong", "hetBuoc", "thanhCong", "hetBuoc", "thanhCong"])) {
  throw new Error("vongLapDonGian tren bo hon hop phai la [thanhCong,thanhCong,hetBuoc,thanhCong,hetBuoc,thanhCong]");
}
if (JSON.stringify(ketQuaDayDu.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "thanhCong", "khongTienTrien", "thanhCong", "khongTienTrien", "thanhCong"])) {
  throw new Error("vongLapDayDu tren bo hon hop phai la [thanhCong,thanhCong,khongTienTrien,thanhCong,khongTienTrien,thanhCong]");
}

if (Math.abs(tinhTyLeThanhCong(ketQuaDonGian) - 2 / 3) > 1e-9) throw new Error("completion rate cua vongLapDonGian phai la 2/3");
if (Math.abs(tinhTyLeThanhCong(ketQuaDayDu) - 2 / 3) > 1e-9) throw new Error("completion rate cua vongLapDayDu phai GIONG HET vongLapDonGian, la 2/3");

if (Math.abs(tinhTyLeChamCap(ketQuaDonGian) - 1 / 3) > 1e-9) throw new Error("ty le cham step cap cua vongLapDonGian phai la 1/3");
if (tinhTyLeChamCap(ketQuaDayDu) !== 0) throw new Error("ty le cham step cap cua vongLapDayDu phai la 0 -- hai tac vu ket duoc bat qua khongTienTrien, KHONG phai hetBuoc");

if (tongSoBuocDaDung(dsDonGian) !== 110) throw new Error("tong so buoc THAT SU dung cua vongLapDonGian phai la 110 (hai tac vu ket dot het 50 buoc moi tac vu)");
if (tongSoBuocDaDung(dsDayDu) !== 16) throw new Error("tong so buoc THAT SU dung cua vongLapDayDu phai la 16 (hai tac vu ket bi phat hien o buoc 3 moi tac vu)");

const trungBinhDonGian = tinhSoBuocTrungBinhKhiThanhCong(dsDonGian, ketQuaDonGian);
const trungBinhDayDu = tinhSoBuocTrungBinhKhiThanhCong(dsDayDu, ketQuaDayDu);
if (Math.abs(trungBinhDonGian - 2.5) > 1e-9) throw new Error("steps-to-success trung binh cua vongLapDonGian phai la 2.5");
if (Math.abs(trungBinhDayDu - 2.5) > 1e-9) throw new Error("steps-to-success trung binh cua vongLapDayDu phai GIONG HET vongLapDonGian, la 2.5 -- no-progress detection khong lam cham tac vu hoi tu that");

const tvRieng = taoTacVuKetDinhLapLaiLoi();
const kqRieng = vongLapDayDu(tvRieng, 50, 3);
if (kqRieng.trangThai !== "khongTienTrien") throw new Error("mot tac vu ket rieng le phai duoc phat hien khongTienTrien qua vongLapDayDu");
if (tvRieng.demBuoc.soBuocDaChay !== 3) throw new Error("vongLapDayDu phai phat hien NGAY o buoc 3, khong dot het 50 buoc nhu vongLapDonGian");
```

:::hints
- kind: attention
  body: "Hai cho trong. vongLapDonGian: DUNG HET het hinh dang bai 2 (chi step cap) nhung kieu TacVuCoChuKy -- bo qua truong chuKy hoan toan. vongLapDayDu: DUNG HET hinh dang bai 4 (step cap CONG dem soLanLapLaiLienTiep dua tren chuKy, tra ve khongTienTrien khi du nguong)."
- kind: strategy
  body: "vongLapDonGian: let soBuoc = 0; while (soBuoc < soBuocToiDa) { const kq = tacVu.chayMotBuoc(); soBuoc++; if (kq.trangThai === \"xong\") return { trangThai: \"thanhCong\", giaTri: kq.giaTri }; if (kq.trangThai === \"loi\") return { trangThai: \"loi\", loi: \"loi_tac_vu\" }; } return { trangThai: \"hetBuoc\" }; vongLapDayDu: them chuKyTruoc/soLanLapLaiLienTiep, so sanh kq.chuKy moi vong, tra khongTienTrien khi soLanLapLaiLienTiep >= soLanLapLaiToiDa (xem bai 4 de doi chieu tung dong)."
- kind: one-line
  body: "Sao chep dung hai ham tu bai 2 (vongLapDonGian doi ten tu chayVongLapCoCap) va bai 4 (vongLapDayDu doi ten tu chayVongLapCoPhatHienKhongTienTrien), GIU NGUYEN toan bo logic ben trong."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 10000
- tier: output
  match: contains
  expect: "0.6666666666666666 0.6666666666666666 0.3333333333333333 0 110 16"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Completion rate GIỐNG HỆT (`0.6666666666666666` cả hai), steps-to-success
trung bình GIỐNG HỆT (`2.5` cả hai) — nhưng tổng số bước THẬT SỰ dùng khác
XA nhau: `110` so VỚI `16`. Quest `q9.4a` — "Kỹ thuật Vòng lặp: hình dạng,
step cap, dừng sớm, không tiến triển" — khép lại tại `6/6`. Track `T9.4` giờ
có bằng chứng bằng số cho đúng luận điểm mở đầu: no-progress detection
KHÔNG phải một cách "cứu" thêm completion rate — nó LÀ một cách NGỪNG lãng
phí ngân sách bước vào những tác vụ CHẮC CHẮN sẽ không bao giờ tự xong.
::::

::::reflect{#nghi-lai}
Con số `110` so VỚI `16` không đến từ việc `vongLapDayDu` "giỏi hơn" Ở việc
LÀM tác vụ xong — cả hai harness đều để đúng hai tác vụ kẹt THẤT BẠI, đúng
bản chất của chúng. Khác biệt DUY NHẤT nằm Ở TỐC ĐỘ nhận ra một tác vụ đã
kẹt: `vongLapDonGian` phải đợi tới tận `soBuocToiDa` mới biết, `vongLapDayDu`
biết NGAY sau `soLanLapLaiToiDa` lần lặp lại liên tiếp — sớm hơn RẤT NHIỀU
khi `soBuocToiDa` được đặt lớn (như trường hợp thực tế, nơi step cap thường
phải đủ RỘNG để không chặn nhầm những tác vụ CHẬM nhưng THẬT SỰ đang tiến
triển). Đối chứng `soBuocToiDa = 2` (nhỏ hơn `soLanLapLaiToiDa`) cho thấy
giới hạn của lợi ích này: khi step cap TỰ NÓ đã quá nhỏ, no-progress
detection không kịp tích luỹ đủ bằng chứng trước khi cap chặn lại — hai
harness trở nên GIỐNG HỆT nhau, VÀ những tác vụ hội tụ chậm hơn cũng thất
bại, không liên quan gì tới việc có phát hiện không tiến triển hay không.
Bài học: mỗi cơ chế phòng thủ giải quyết đúng MỘT lớp vấn đề — no-progress
detection tiết kiệm ngân sách cho tác vụ KẸT, nhưng không hề thay thế được
việc CHỌN một step cap đủ lớn cho tác vụ THẬT SỰ cần nhiều bước.
::::

::::checkpoint{mastery=0.92}
::::
