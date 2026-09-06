---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.rang-buoc-so-buoc-toi-da
title: "Ràng buộc step cap — hetBuoc khác thanhCong VÀ khác loi"
summary: "chayVongLapCoCap(tacVu, soBuocToiDa) tổng quát hoá chayVongLapCoBan (bài 1, GIOI_HAN_AN_TOAN=1000 cứng) thành MỘT tham số tường minh: trả về MỘT trong BA trạng thái phân biệt -- 'thanhCong' (tác vụ tự báo xong), 'loi' (tác vụ báo lỗi dứt khoát), 'hetBuoc' (đạt soBuocToiDa mà CHƯA xong -- KHÁC cả hai trạng thái kia). Trên bộ 6 tác vụ hỗn hợp (ba tác vụ hội tụ ở bước 3/5/4, một tác vụ cần 10 bước, một tác vụ không bao giờ xong, một tác vụ báo lỗi ngay lập tức) với soBuocToiDa=5: kết quả = [thanhCong,thanhCong,thanhCong,hetBuoc,hetBuoc,loi] -- tỷ lệ thành công 3/6 (in ra 0.5), %chạm step cap 2/6 tức 1/3 (in ra 0.3333333333333333) (KHÁC completion rate, vì loi không tính là hetBuoc). Đối chứng đổi soBuocToiDa=10: tác vụ cần-10-bước chuyển từ hetBuoc sang thanhCong (tỷ lệ thành công tăng lên 0.6666666666666666, %chạm step cap giảm còn 0.16666666666666666) -- tác vụ không-bao-giờ-xong VẪN hetBuoc VÀ tác vụ báo-lỗi VẪN loi bất kể cap là bao nhiêu, xác nhận tham số ràng buộc THẬT chứ không phải trang trí."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [kna.rang-buoc-so-buoc-toi-da]
requires: [kna.hinh-dang-vong-lap-co-ban]
concepts: [kna.rang-buoc-so-buoc-toi-da]
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
Bài trước dừng vòng lặp bằng một con số CỨNG (`1000`) — không do harness
thiết kế, chỉ để bài học còn chấm được. Một hệ thống thật cần TỰ CHỌN con số
đó theo TỪNG tác vụ: một tác vụ rẻ có thể đáng `5` bước, một tác vụ phức tạp
có thể đáng `50`. Bài này biến con số Ẩn đó thành một THAM SỐ — VÀ đặt tên
CHÍNH THỨC cho trạng thái "hết bước mà chưa xong", để nó không bị nhầm với
"thất bại vĩnh viễn".
::::

::::explain{#ba_trang_thai_ket_thuc}
`chayVongLapCoCap(tacVu, soBuocToiDa)` giống hệt `chayVongLapCoBan` (bài `1`)
Ở HÌNH DẠNG — vẫn lặp `chayMotBuoc()` cho tới khi có kết quả — nhưng đổi
`GIOI_HAN_AN_TOAN` (hằng số ẩn) thành `soBuocToiDa` (tham số), VÀ đổi kiểu
trả về từ hai biến thể (`thanhCong: true/false`) thành BA biến thể tường
minh: `"thanhCong"`, `"loi"`, VÀ `"hetBuoc"` — một trạng thái MỚI, không hề
LÀ lỗi, chỉ LÀ "chưa đủ thời gian":

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuMoPhong {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuoc;
}

function taoTacVuHoiTuSauNBuoc(n: number): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      if (demBuoc.soBuocDaChay >= n) return { trangThai: "xong", giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong" };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function chayVongLapCoCap(tacVu: TacVuMoPhong, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "" };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

const tv3 = taoTacVuHoiTuSauNBuoc(10);
console.log(JSON.stringify(chayVongLapCoCap(tv3, 5)), tv3.demBuoc.soBuocDaChay);
```

```text title=readonly
{"trangThai":"hetBuoc"} 5
```

Tác vụ cần `10` bước để hội tụ, nhưng `soBuocToiDa` chỉ cho `5` — vòng lặp
dừng ĐÚNG Ở bước `5`, trả về `"hetBuoc"`. Đây KHÔNG phải một lỗi (`"loi"`) —
tác vụ có thể VẪN đang tiến triển đúng hướng, chỉ đơn giản CHƯA đủ thời gian.
Phân biệt này quan trọng: một hệ thống gộp `"hetBuoc"` VÀO `"loi"` sẽ không
bao giờ biết được liệu tăng `soBuocToiDa` có ích hay không.
::::

::::example{#bo_sau_tac_vu_hon_hop}
Chạy `chayVongLapCoCap` trên MỘT bộ `6` tác vụ hỗn hợp — ba tác vụ hội tụ Ở
bước `3`/`5`/`4`, một tác vụ cần `10` bước, một tác vụ KHÔNG BAO GIỜ xong, VÀ
một tác vụ báo lỗi NGAY lập tức — VỚI `soBuocToiDa = 5`:

```typescript title=readonly
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuMoPhong {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuoc;
}

function taoTacVuHoiTuSauNBuoc(n: number): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      if (demBuoc.soBuocDaChay >= n) return { trangThai: "xong", giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong" };
    },
  };
}

function taoTacVuKhongBaoGioXong(): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong" };
    },
  };
}

function taoTacVuLoiNgayLapTuc(): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      return { trangThai: "loi" };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function chayVongLapCoCap(tacVu: TacVuMoPhong, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "" };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

function chayTrenNhieuTacVuCoCap(dsTacVu: TacVuMoPhong[], soBuocToiDa: number): KetQuaVongLapCoCap[] {
  const ketQua: KetQuaVongLapCoCap[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(chayVongLapCoCap(tv, soBuocToiDa));
  }
  return ketQua;
}

function tinhTyLeThanhCong(ketQua: KetQuaVongLapCoCap[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhTyLeHetBuoc(ketQua: KetQuaVongLapCoCap[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}

const dsTacVu6: TacVuMoPhong[] = [
  taoTacVuHoiTuSauNBuoc(3),
  taoTacVuHoiTuSauNBuoc(5),
  taoTacVuHoiTuSauNBuoc(4),
  taoTacVuHoiTuSauNBuoc(10),
  taoTacVuKhongBaoGioXong(),
  taoTacVuLoiNgayLapTuc(),
];
const SO_BUOC_TOI_DA = 5;
const ketQua6 = chayTrenNhieuTacVuCoCap(dsTacVu6, SO_BUOC_TOI_DA);
console.log(JSON.stringify(ketQua6.map((k) => k.trangThai)));
console.log("ty le thanh cong:", tinhTyLeThanhCong(ketQua6));
console.log("ty le het buoc:", tinhTyLeHetBuoc(ketQua6));
```

```text title=readonly
["thanhCong","thanhCong","thanhCong","hetBuoc","hetBuoc","loi"]
ty le thanh cong: 0.5
ty le het buoc: 0.3333333333333333
```

Ba tác vụ đầu hội tụ Ở đúng bước `3`/`5`/`4` — cả ba nằm TRONG `soBuocToiDa =
5`, nên đều `"thanhCong"`. Tác vụ cần `10` bước VÀ tác vụ không bao giờ xong
đều CHẠM `soBuocToiDa` — cả hai LÀ `"hetBuoc"`, KHÔNG phải `"loi"`. Tác vụ
báo lỗi NGAY LÀ `"loi"` — riêng biệt, KHÔNG tính vào `%` chạm step cap. Ba
con số — `0.5` (thành công), `0.3333...` (chạm cap), VÀ phần còn lại LÀ lỗi —
đo BA THỨ khác nhau, không thể gộp làm một.
::::

::::predict{#doan-tang-cap-len-10 commitOnce}
Nếu đổi `SO_BUOC_TOI_DA` từ `5` sang `10` (giữ nguyên `6` tác vụ Ở trên) —
mảng `trangThai` VÀ hai tỷ lệ thay đổi ra sao?

:::opt{correct}
Tác vụ cần `10` bước chuyển từ `"hetBuoc"` sang `"thanhCong"` (giờ đủ `10`
bước để nó tự báo xong) — NĂM tác vụ còn lại giữ nguyên: ba tác vụ hội tụ sớm
vẫn `"thanhCong"`, tác vụ không bao giờ xong VẪN `"hetBuoc"`, tác vụ báo lỗi
NGAY vẫn `"loi"`; tỷ lệ thành công tăng lên `0.6666666666666666`, tỷ lệ chạm
cap giảm còn `0.16666666666666666`
:::
:::opt
Toàn bộ `6` tác vụ đều chuyển thành `"thanhCong"` — tăng `soBuocToiDa` luôn
cho MỌI tác vụ đủ cơ hội xong
::why
Nhầm rằng tăng cap cứu được MỌI loại thất bại — nhưng tác vụ không bao giờ
xong KHÔNG có cơ chế tự chuyển sang `"xong"` bất kể cap lớn cỡ nào, VÀ tác vụ
báo lỗi trả `"loi"` NGAY LẦN GỌI ĐẦU, trước khi phần cap còn lại được dùng
tới.

Chỗ lệch: tăng `soBuocToiDa` chỉ giúp được những tác vụ THẬT SỰ đang tiến
tới `"xong"` nhưng chưa đủ bước — nó không thay đổi được bản chất của một
tác vụ không có điều kiện dừng tự nhiên, VÀ không thay đổi được một tác vụ
báo lỗi dứt khoát.
::
:::
:::opt
Chỉ tác vụ không bao giờ xong đổi từ `"hetBuoc"` sang `"thanhCong"`, vì nó
LÀ tác vụ duy nhất phụ thuộc trực tiếp vào `soBuocToiDa`
::why
Nhầm tác vụ nào THẬT SỰ phụ thuộc vào cap — `taoTacVuKhongBaoGioXong()`
không đọc số bước còn lại để quyết định gì cả, nó LUÔN trả `"chua_xong"`.

Chỗ lệch: tác vụ THAY ĐỔI kết quả khi cap tăng LÀ tác vụ CẦN `10` bước
(`taoTacVuHoiTuSauNBuoc(10)`) — nó tự báo `"xong"` NGAY khi đếm chạm `10`,
bất kể cap LÀ bao nhiêu, miễn cap ĐỦ LỚN để nó tới được đó.
::
:::
::::

::::code{#viet_chay_vong_lap_co_cap}
Hoàn thiện `chayVongLapCoCap` — lặp `while` tối đa `soBuocToiDa` lần, mỗi
lần gọi `tacVu.chayMotBuoc()`; NẾU trạng thái LÀ `"xong"`, trả về
`{ trangThai: "thanhCong", giaTri: ... }` NGAY; nếu LÀ `"loi"`, trả về
`{ trangThai: "loi", loi: "loi_tac_vu" }` NGAY; nếu vòng lặp hết mà chưa
từng trả về, trả `{ trangThai: "hetBuoc" }`. Hoàn thiện
`chayTrenNhieuTacVuCoCap` — với MỖI tác vụ trong `dsTacVu`, gọi
`chayVongLapCoCap(tacVu, soBuocToiDa)`, đẩy kết quả vào mảng trả về.

```typescript title=starter
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuMoPhong {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuoc;
}

function taoTacVuHoiTuSauNBuoc(n: number): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      if (demBuoc.soBuocDaChay >= n) return { trangThai: "xong", giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong" };
    },
  };
}

function taoTacVuKhongBaoGioXong(): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong" };
    },
  };
}

function taoTacVuLoiNgayLapTuc(): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      return { trangThai: "loi" };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function tinhTyLeThanhCong(ketQua: KetQuaVongLapCoCap[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhTyLeHetBuoc(ketQua: KetQuaVongLapCoCap[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}

function chayVongLapCoCap(tacVu: TacVuMoPhong, soBuocToiDa: number): KetQuaVongLapCoCap {
  ___
}

function chayTrenNhieuTacVuCoCap(dsTacVu: TacVuMoPhong[], soBuocToiDa: number): KetQuaVongLapCoCap[] {
  ___
}

const dsTacVu6: TacVuMoPhong[] = [
  taoTacVuHoiTuSauNBuoc(3),
  taoTacVuHoiTuSauNBuoc(5),
  taoTacVuHoiTuSauNBuoc(4),
  taoTacVuHoiTuSauNBuoc(10),
  taoTacVuKhongBaoGioXong(),
  taoTacVuLoiNgayLapTuc(),
];
const ketQua6 = chayTrenNhieuTacVuCoCap(dsTacVu6, 5);
console.log(JSON.stringify(ketQua6.map((k) => k.trangThai)), tinhTyLeThanhCong(ketQua6), tinhTyLeHetBuoc(ketQua6));
```

```typescript title=solution
type TrangThaiBuoc = "xong" | "chua_xong" | "loi";
type KetQuaBuoc = { trangThai: TrangThaiBuoc; giaTri?: string };
type DemBuoc = { soBuocDaChay: number };

interface TacVuMoPhong {
  demBuoc: DemBuoc;
  chayMotBuoc(): KetQuaBuoc;
}

function taoTacVuHoiTuSauNBuoc(n: number): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      if (demBuoc.soBuocDaChay >= n) return { trangThai: "xong", giaTri: "hoan_thanh" };
      return { trangThai: "chua_xong" };
    },
  };
}

function taoTacVuKhongBaoGioXong(): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      return { trangThai: "chua_xong" };
    },
  };
}

function taoTacVuLoiNgayLapTuc(): TacVuMoPhong {
  const demBuoc: DemBuoc = { soBuocDaChay: 0 };
  return {
    demBuoc,
    chayMotBuoc(): KetQuaBuoc {
      demBuoc.soBuocDaChay++;
      return { trangThai: "loi" };
    },
  };
}

type KetQuaVongLapCoCap =
  | { trangThai: "thanhCong"; giaTri: string }
  | { trangThai: "loi"; loi: string }
  | { trangThai: "hetBuoc" };

function tinhTyLeThanhCong(ketQua: KetQuaVongLapCoCap[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "thanhCong").length / ketQua.length;
}

function tinhTyLeHetBuoc(ketQua: KetQuaVongLapCoCap[]): number {
  if (ketQua.length === 0) return 0;
  return ketQua.filter((k) => k.trangThai === "hetBuoc").length / ketQua.length;
}

function chayVongLapCoCap(tacVu: TacVuMoPhong, soBuocToiDa: number): KetQuaVongLapCoCap {
  let soBuoc = 0;
  while (soBuoc < soBuocToiDa) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { trangThai: "thanhCong", giaTri: kq.giaTri ?? "" };
    if (kq.trangThai === "loi") return { trangThai: "loi", loi: "loi_tac_vu" };
  }
  return { trangThai: "hetBuoc" };
}

function chayTrenNhieuTacVuCoCap(dsTacVu: TacVuMoPhong[], soBuocToiDa: number): KetQuaVongLapCoCap[] {
  const ketQua: KetQuaVongLapCoCap[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(chayVongLapCoCap(tv, soBuocToiDa));
  }
  return ketQua;
}

const dsTacVu6: TacVuMoPhong[] = [
  taoTacVuHoiTuSauNBuoc(3),
  taoTacVuHoiTuSauNBuoc(5),
  taoTacVuHoiTuSauNBuoc(4),
  taoTacVuHoiTuSauNBuoc(10),
  taoTacVuKhongBaoGioXong(),
  taoTacVuLoiNgayLapTuc(),
];
const ketQua6 = chayTrenNhieuTacVuCoCap(dsTacVu6, 5);
console.log(JSON.stringify(ketQua6.map((k) => k.trangThai)), tinhTyLeThanhCong(ketQua6), tinhTyLeHetBuoc(ketQua6));
```

```typescript title=test
if (ketQua6.length !== 6) throw new Error("chayTrenNhieuTacVuCoCap phai tra ve mang dung 6 phan tu");
if (JSON.stringify(ketQua6.map((k) => k.trangThai)) !== JSON.stringify(["thanhCong", "thanhCong", "thanhCong", "hetBuoc", "hetBuoc", "loi"])) {
  throw new Error("ket qua 6 tac vu voi soBuocToiDa=5 phai la [thanhCong,thanhCong,thanhCong,hetBuoc,hetBuoc,loi]");
}
if (Math.abs(tinhTyLeThanhCong(ketQua6) - 0.5) > 1e-9) throw new Error("ty le thanh cong tren 6 tac vu phai la 0.5");
if (Math.abs(tinhTyLeHetBuoc(ketQua6) - 1 / 3) > 1e-9) throw new Error("ty le het buoc tren 6 tac vu phai la 1/3");

const tvCanMuoi = taoTacVuHoiTuSauNBuoc(10);
const kqCap5 = chayVongLapCoCap(tvCanMuoi, 5);
if (kqCap5.trangThai !== "hetBuoc") throw new Error("tac vu can 10 buoc voi cap 5 phai la hetBuoc, KHONG phai loi hay thanhCong");
if (tvCanMuoi.demBuoc.soBuocDaChay !== 5) throw new Error("chayVongLapCoCap phai dung DUNG soBuocToiDa lan khi chua xong");

const tvCanMuoi2 = taoTacVuHoiTuSauNBuoc(10);
const kqCap10 = chayVongLapCoCap(tvCanMuoi2, 10);
if (kqCap10.trangThai !== "thanhCong") throw new Error("tac vu can 10 buoc voi cap 10 phai THANH CONG");

const tvLoi = taoTacVuLoiNgayLapTuc();
const kqLoi = chayVongLapCoCap(tvLoi, 5);
if (kqLoi.trangThai !== "loi") throw new Error("tac vu bao loi ngay phai tra trangThai loi, KHONG phai hetBuoc");
if (tvLoi.demBuoc.soBuocDaChay !== 1) throw new Error("tac vu bao loi ngay phai dung lai o LAN GOI DAU, khong chay het cap");

const ketQua2 = chayTrenNhieuTacVuCoCap([taoTacVuHoiTuSauNBuoc(1)], 5);
if (ketQua2.length !== 1) throw new Error("doi so danh sach tac vu phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayVongLapCoCap): giong het chayVongLapCoBan bai 1 nhung dung soBuocToiDa (tham so) THAY VI GIOI_HAN_AN_TOAN (hang so), va tra ve { trangThai: ... } (ba bien the) THAY VI { thanhCong: ... } (hai bien the). Cho hai (chayTrenNhieuTacVuCoCap): mot vong for-of qua dsTacVu, goi chayVongLapCoCap(tv, soBuocToiDa) tren moi phan tu, day ket qua vao mot mang."
- kind: strategy
  body: "Cho dau: let soBuoc = 0; while (soBuoc < soBuocToiDa) { const kq = tacVu.chayMotBuoc(); soBuoc++; if (kq.trangThai === \"xong\") return { trangThai: \"thanhCong\", giaTri: kq.giaTri ?? \"\" }; if (kq.trangThai === \"loi\") return { trangThai: \"loi\", loi: \"loi_tac_vu\" }; } return { trangThai: \"hetBuoc\" }; Cho hai: const ketQua: KetQuaVongLapCoCap[] = []; for (const tv of dsTacVu) { ketQua.push(chayVongLapCoCap(tv, soBuocToiDa)); } return ketQua;"
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
  expect: "[\"thanhCong\",\"thanhCong\",\"thanhCong\",\"hetBuoc\",\"hetBuoc\",\"loi\"] 0.5 0.3333333333333333"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`soBuocToiDa` thay cho một con số bí ẩn — VÀ `"hetBuoc"` giờ LÀ một trạng
thái riêng, không lẫn với `"loi"`. Nhưng chú ý: MỌI tác vụ Ở bài này dừng
đúng lúc dựa trên tín hiệu THẬT (`"xong"`/`"loi"` do chính tác vụ báo). Bài
sau lật ngược vấn đề: điều gì xảy ra nếu vòng lặp dừng dựa trên một tín hiệu
KHÔNG đáng tin — dừng "thành công" trước khi tác vụ THẬT SỰ xong?
::::

::::reflect{#nghi-lai}
Bài này không thêm một CƠ CHẾ mới — nó chỉ LÀM RÕ một con số đã tồn tại ẩn
trong bài trước (`GIOI_HAN_AN_TOAN`), biến nó thành `soBuocToiDa`, VÀ tách
"hết bước" ra khỏi "thất bại" thành một trạng thái riêng. Sự tách biệt đó
quan trọng hơn vẻ ngoài của nó: nếu `"hetBuoc"` bị gộp chung VÀO `"loi"`,
một kỹ sư nhìn vào log sẽ không bao giờ biết được liệu tăng ngân sách bước
có ích hay không — với `"loi"` thì KHÔNG (dứt khoát), với `"hetBuoc"` thì
CÓ THỂ (nếu tác vụ thật sự đang tiến triển). Đo lường đúng bắt đầu từ việc
đặt tên đúng cho từng chế độ dừng.
::::

::::checkpoint{mastery=0.82}
::::
