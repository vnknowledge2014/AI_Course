---
id: ky-nghe-ung-dung-ai.ky-thuat-vong-lap.hinh-dang-vong-lap-co-ban
title: "Hình dạng vòng lặp cơ bản — và vì sao nó có thể không bao giờ dừng"
summary: "TacVuMoPhong{demBuoc; chayMotBuoc(): KetQuaBuoc} mô phỏng MỘT bước tác vụ, trả về trangThai 'xong'/'chua_xong'/'loi'. taoTacVuHoiTuSauNBuoc(n) trả 'chua_xong' đúng n-1 lần đầu rồi 'xong' ở lần thứ n (test: n=3 → soBuocDaChay=3 khi xong). taoTacVuKhongBaoGioXong() LUÔN trả 'chua_xong', không có cơ chế nào khiến nó tự chuyển sang 'xong'. chayVongLapCoBan(tacVu) lặp GỌI chayMotBuoc() cho tới khi 'xong' (trả thanhCong:true) hoặc 'loi' (trả thanhCong:false) — NHƯNG bài này CHƯA có step cap do người thiết kế chọn; vòng lặp chỉ dừng nhờ MỘT giới hạn an toàn cứng GIOI_HAN_AN_TOAN=1000 (phát hiện 'không hội tụ', không phải một tham số harness). Trên taoTacVuKhongBaoGioXong(): chayVongLapCoBan gọi ĐÚNG 1000 lần rồi trả về {thanhCong:false, loi:'khong_hoi_tu_vuot_gioi_han_an_toan'} — xác nhận qua engine thật. Trên bộ 3 tác vụ hỗn hợp (hoiTu(3), hoiTu(1), khongBaoGioXong()): completion rate = 2/3 (in ra 0.6666666666666666) — một tác vụ KHÔNG BAO GIỜ thành công kéo completion rate xuống, không phải vì tool lỗi mà vì bản thân tác vụ không có điều kiện dừng tự nhiên."
locale: vi
track: ky-nghe-ung-dung-ai
module: ky-thuat-vong-lap
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [kna.hinh-dang-vong-lap-co-ban]
requires: [kna.boss-harness-production-grade]
concepts: [kna.hinh-dang-vong-lap-co-ban]
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
T9.3 dạy CHƯƠNG TRÌNH bao quanh MỘT lần gọi tool — retry, fallback, timeout,
circuit breaker. Nhưng một agent thật không gọi tool đúng một lần rồi dừng —
nó LẶP: gọi, nhìn kết quả, gọi tiếp, cho tới khi xong. T9.4 dạy đúng CÁI VÒNG
LẶP đó. Bài đầu tiên dựng hình dạng tối giản nhất của nó — và cho thấy ngay
vì sao hình dạng tối giản đó có thể không bao giờ dừng lại.
::::

::::explain{#buoc_tac_vu_va_tac_vu_mo_phong}
Một "tác vụ" trong quest này KHÔNG gọi model thật — nó LÀ một hàm TypeScript
tất định, đại diện cho MỘT bước công việc mà agent có thể lặp lại nhiều lần.
Mỗi lần gọi `chayMotBuoc()` trả về MỘT trong ba trạng thái: `"xong"` (tác vụ
đã hoàn thành THẬT SỰ), `"chua_xong"` (cần lặp thêm), hoặc `"loi"` (thất bại
dứt khoát, lặp thêm cũng vô ích). `taoTacVuHoiTuSauNBuoc(n)` mô phỏng một tác
vụ cần ĐÚNG `n` bước để hội tụ — nó đóng gói `demBuoc` (đếm `soBuocDaChay`)
qua closure, và LUÔN trả `"chua_xong"` cho tới khi đếm đạt `n`:

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
      if (demBuoc.soBuocDaChay >= n) {
        return { trangThai: "xong", giaTri: "hoan_thanh" };
      }
      return { trangThai: "chua_xong" };
    },
  };
}

const tv = taoTacVuHoiTuSauNBuoc(3);
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
console.log(JSON.stringify(tv.chayMotBuoc()));
```

```text title=readonly
{"trangThai":"chua_xong"}
{"trangThai":"chua_xong"}
{"trangThai":"xong","giaTri":"hoan_thanh"}
```

Hai lần gọi đầu trả `"chua_xong"` (đếm mới đạt `1` VÀ `2`, chưa tới `3`); lần
gọi THỨ BA đếm đạt `3` — ĐÚNG `n` — nên trả `"xong"`. `demBuoc` sống NGOÀI
object trả về nhưng vẫn được cập nhật bên trong `chayMotBuoc()` qua closure,
y hệt cách `TrangThaiTool` hoạt động Ở `ToolMoPhong` của T9.3.
::::

::::example{#vong_lap_khong_step_cap_va_gioi_han_an_toan}
`chayVongLapCoBan(tacVu)` lặp gọi `chayMotBuoc()` cho tới khi nhận `"xong"`
hoặc `"loi"` — nó CHƯA có `soBuocToiDa` do người thiết kế harness CHỌN. Vấn
đề: nếu tác vụ không BAO GIỜ tự báo xong, vòng lặp sẽ chạy MÃI. Vì môi trường
học không thể literally treo vô hạn, bài này thêm một `GIOI_HAN_AN_TOAN`
CỨNG — không phải một tham số của harness, mà một lưới an toàn để PHÁT HIỆN
"không hội tụ" thay vì thật sự treo:

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

type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };
type KetQuaVongLap = KetQua<string, string>;

const GIOI_HAN_AN_TOAN = 1000;

function chayVongLapCoBan(tacVu: TacVuMoPhong): KetQuaVongLap {
  let soBuoc = 0;
  while (soBuoc < GIOI_HAN_AN_TOAN) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { thanhCong: true, giaTri: kq.giaTri ?? "" };
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tac_vu" };
  }
  return { thanhCong: false, loi: "khong_hoi_tu_vuot_gioi_han_an_toan" };
}

const hoiTu = taoTacVuHoiTuSauNBuoc(3);
console.log(JSON.stringify(chayVongLapCoBan(hoiTu)), hoiTu.demBuoc.soBuocDaChay);

const khongBaoGioXong = taoTacVuKhongBaoGioXong();
console.log(JSON.stringify(chayVongLapCoBan(khongBaoGioXong)), khongBaoGioXong.demBuoc.soBuocDaChay);
```

```text title=readonly
{"thanhCong":true,"giaTri":"hoan_thanh"} 3
{"thanhCong":false,"loi":"khong_hoi_tu_vuot_gioi_han_an_toan"} 1000
```

Tác vụ hội tụ sau `3` bước dừng đúng lúc, dùng ĐÚNG `3` lần gọi. Tác vụ KHÔNG
BAO GIỜ xong bị vòng lặp gọi ĐÚNG `1000` lần (khớp `GIOI_HAN_AN_TOAN`) rồi bị
phát hiện — không phải treo mãi, nhưng cũng không phải một điều gì bài này
THIẾT KẾ để dừng đúng lúc: `1000` LÀ một con số tuỳ ý, không liên quan gì tới
bản chất tác vụ. Bài sau biến con số đó thành một THAM SỐ thật sự, do harness
tự chọn theo từng tác vụ.
::::

::::predict{#doan-tang-gioi-han-an-toan commitOnce}
Nếu đổi `GIOI_HAN_AN_TOAN` từ `1000` sang `2000` (KHÔNG đổi gì khác) rồi chạy
`chayVongLapCoBan` trên MỘT `taoTacVuKhongBaoGioXong()` MỚI — kết quả VÀ số
bước cuối cùng thay đổi ra sao?

:::opt{correct}
Kết quả VẪN thất bại y hệt (`"khong_hoi_tu_vuot_gioi_han_an_toan"`) — vì tác
vụ KHÔNG BAO GIỜ trả `"xong"` bất kể giới hạn LÀ bao nhiêu; chỉ số bước trước
khi bị phát hiện tăng từ `1000` lên `2000` — tốn thêm bước, không đổi kết cục
:::
:::opt
Kết quả sẽ THÀNH CÔNG — giới hạn lớn hơn cho tác vụ nhiều cơ hội hơn để tự
khỏi
::why
Nhầm rằng thêm bước sẽ giúp một tác vụ KHÔNG có cơ chế thay đổi trạng thái tự
"khỏi" — nhưng `taoTacVuKhongBaoGioXong()` luôn trả `"chua_xong"` bất kể được
gọi bao nhiêu lần, không hề có logic nào khiến nó chuyển sang `"xong"`.

Chỗ lệch: `chayMotBuoc()` của tác vụ này CHỈ có một dòng —
`return { trangThai: "chua_xong" };` — không đọc `demBuoc.soBuocDaChay` để
quyết định bất cứ điều gì, nên gọi thêm bao nhiêu lần cũng ra cùng một kết
quả.
::
:::
:::opt
Không đổi gì cả — số bước vẫn LÀ `1000`, vì `GIOI_HAN_AN_TOAN` chỉ LÀ một
hằng số trang trí không thực sự ràng buộc vòng lặp
::why
Nhầm rằng một hằng số dùng làm điều kiện `while` không THẬT SỰ điều khiển
hành vi — nhưng `while (soBuoc < GIOI_HAN_AN_TOAN)` đọc THẲNG giá trị đó mỗi
vòng lặp.

Chỗ lệch: đổi `GIOI_HAN_AN_TOAN` LÀ đổi trực tiếp điều kiện dừng của vòng
`while` — số bước trước khi phát hiện "không hội tụ" tăng ĐÚNG theo giá trị
mới, từ `1000` lên `2000`.
::
:::
::::

::::code{#viet_chay_vong_lap_co_ban}
Hoàn thiện `chayVongLapCoBan` — lặp `while` tối đa `GIOI_HAN_AN_TOAN` lần,
mỗi lần gọi `tacVu.chayMotBuoc()`; NẾU trạng thái LÀ `"xong"`, trả về
`{ thanhCong: true, giaTri: ... }` NGAY; nếu LÀ `"loi"`, trả về
`{ thanhCong: false, loi: "loi_tac_vu" }` NGAY; nếu vòng lặp hết mà chưa từng
trả về, trả `{ thanhCong: false, loi: "khong_hoi_tu_vuot_gioi_han_an_toan" }`.
Hoàn thiện `chayTrenNhieuTacVu` — với MỖI tác vụ trong `dsTacVu`, gọi
`chayVongLapCoBan` trên nó, đẩy kết quả vào mảng trả về.

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

type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };
type KetQuaVongLap = KetQua<string, string>;

const GIOI_HAN_AN_TOAN = 1000;

function tinhTyLeHoanThanh(ketQua: KetQuaVongLap[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function chayVongLapCoBan(tacVu: TacVuMoPhong): KetQuaVongLap {
  ___
}

function chayTrenNhieuTacVu(dsTacVu: TacVuMoPhong[]): KetQuaVongLap[] {
  ___
}

const dsTacVu3: TacVuMoPhong[] = [
  taoTacVuHoiTuSauNBuoc(3),
  taoTacVuHoiTuSauNBuoc(1),
  taoTacVuKhongBaoGioXong(),
];
const ketQua3 = chayTrenNhieuTacVu(dsTacVu3);
console.log(JSON.stringify(ketQua3.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua3));
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

type KetQua<T, E> =
  | { thanhCong: true; giaTri: T }
  | { thanhCong: false; loi: E };
type KetQuaVongLap = KetQua<string, string>;

const GIOI_HAN_AN_TOAN = 1000;

function tinhTyLeHoanThanh(ketQua: KetQuaVongLap[]): number {
  if (ketQua.length === 0) return 0;
  const soThanhCong = ketQua.filter((k) => k.thanhCong).length;
  return soThanhCong / ketQua.length;
}

function chayVongLapCoBan(tacVu: TacVuMoPhong): KetQuaVongLap {
  let soBuoc = 0;
  while (soBuoc < GIOI_HAN_AN_TOAN) {
    const kq = tacVu.chayMotBuoc();
    soBuoc++;
    if (kq.trangThai === "xong") return { thanhCong: true, giaTri: kq.giaTri ?? "" };
    if (kq.trangThai === "loi") return { thanhCong: false, loi: "loi_tac_vu" };
  }
  return { thanhCong: false, loi: "khong_hoi_tu_vuot_gioi_han_an_toan" };
}

function chayTrenNhieuTacVu(dsTacVu: TacVuMoPhong[]): KetQuaVongLap[] {
  const ketQua: KetQuaVongLap[] = [];
  for (const tv of dsTacVu) {
    ketQua.push(chayVongLapCoBan(tv));
  }
  return ketQua;
}

const dsTacVu3: TacVuMoPhong[] = [
  taoTacVuHoiTuSauNBuoc(3),
  taoTacVuHoiTuSauNBuoc(1),
  taoTacVuKhongBaoGioXong(),
];
const ketQua3 = chayTrenNhieuTacVu(dsTacVu3);
console.log(JSON.stringify(ketQua3.map((k) => k.thanhCong)), tinhTyLeHoanThanh(ketQua3));
```

```typescript title=test
if (ketQua3.length !== 3) throw new Error("chayTrenNhieuTacVu phai tra ve mang dung 3 phan tu");
if (JSON.stringify(ketQua3.map((k) => k.thanhCong)) !== JSON.stringify([true, true, false])) {
  throw new Error("hai tac vu hoi tu phai thanh cong, tac vu khong bao gio xong phai that bai");
}
if (Math.abs(tinhTyLeHoanThanh(ketQua3) - 2 / 3) > 1e-9) throw new Error("completion rate tren 3 tac vu phai la 2/3");

const tvRieng = taoTacVuHoiTuSauNBuoc(5);
const kqRieng = chayVongLapCoBan(tvRieng);
if (!kqRieng.thanhCong) throw new Error("tac vu hoi tu sau 5 buoc phai THANH CONG");
if (tvRieng.demBuoc.soBuocDaChay !== 5) throw new Error("chayVongLapCoBan phai dung lai NGAY khi tac vu bao xong, dung 5 buoc");

const tvKhongXong = taoTacVuKhongBaoGioXong();
const kqKhongXong = chayVongLapCoBan(tvKhongXong);
if (kqKhongXong.thanhCong !== false) throw new Error("tac vu khong bao gio xong phai THAT BAI qua gioi han an toan");
if (kqKhongXong.thanhCong === false && kqKhongXong.loi !== "khong_hoi_tu_vuot_gioi_han_an_toan") {
  throw new Error("loi phai la khong_hoi_tu_vuot_gioi_han_an_toan");
}
if (tvKhongXong.demBuoc.soBuocDaChay !== GIOI_HAN_AN_TOAN) throw new Error("phai chay DUNG GIOI_HAN_AN_TOAN buoc truoc khi bo cuoc");

const ketQua1 = chayTrenNhieuTacVu([taoTacVuHoiTuSauNBuoc(2)]);
if (ketQua1.length !== 1) throw new Error("doi so danh sach tac vu phai doi do dai mang tra ve -- tham so phai duoc dung that");
```

:::hints
- kind: attention
  body: "Hai cho trong. Cho dau (chayVongLapCoBan): mot vong while chay toi da GIOI_HAN_AN_TOAN lan, moi lan goi tacVu.chayMotBuoc() luu vao mot bien; neu trangThai la 'xong' thi return { thanhCong: true, giaTri: ... } NGAY; neu la 'loi' thi return { thanhCong: false, loi: 'loi_tac_vu' } NGAY; sau khi vong while ket thuc (het gioi han), return { thanhCong: false, loi: 'khong_hoi_tu_vuot_gioi_han_an_toan' }. Cho hai (chayTrenNhieuTacVu): mot vong for-of chay qua tung phan tu cua dsTacVu, goi chayVongLapCoBan tren no, day ket qua vao mot mang, cuoi cung return mang do."
- kind: strategy
  body: "Cho dau: let soBuoc = 0; while (soBuoc < GIOI_HAN_AN_TOAN) { const kq = tacVu.chayMotBuoc(); soBuoc++; if (kq.trangThai === \"xong\") return { thanhCong: true, giaTri: kq.giaTri ?? \"\" }; if (kq.trangThai === \"loi\") return { thanhCong: false, loi: \"loi_tac_vu\" }; } return { thanhCong: false, loi: \"khong_hoi_tu_vuot_gioi_han_an_toan\" }; Cho hai: const ketQua: KetQuaVongLap[] = []; for (const tv of dsTacVu) { ketQua.push(chayVongLapCoBan(tv)); } return ketQua;"
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
  expect: "[true,true,false] 0.6666666666666666"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai tác vụ hội tụ, một tác vụ không bao giờ xong — completion rate `2/3`.
Nhưng chú ý: bài này KHÔNG hề "sửa" được tác vụ không hội tụ, nó chỉ PHÁT
HIỆN được vấn đề sau đúng `1000` bước — một con số KHÔNG do harness thiết
kế, do chính bài học gán cứng để giữ an toàn. Bài sau biến con số đó thành
một THAM SỐ thật — `soBuocToiDa` — và cho phép harness quyết định TRẠNG THÁI
"hết bước" khác hẳn "thành công" hay "lỗi".
::::

::::reflect{#nghi-lai}
Vòng lặp cơ bản Ở bài này giải quyết được câu chuyện "một bước, gọi lại, lặp
tới khi xong" — hình dạng tối thiểu của MỌI agent loop. Nhưng nó lộ ra ngay
một chế độ hỏng mà T9.3 (chỉ MỘT lần gọi tool) không bao giờ gặp phải: một
vòng lặp có thể KHÔNG BAO GIỜ dừng, nếu tác vụ bên trong không có điều kiện
tự nhiên nào để báo "xong". `GIOI_HAN_AN_TOAN` không phải LÀ giải pháp — nó
chỉ LÀ lưới an toàn để bài học này còn chấm được. Câu hỏi thật ("bao nhiêu
bước LÀ đủ cho tác vụ NÀY, và ai quyết định con số đó") LÀ chủ đề của bài
sau.
::::

::::checkpoint{mastery=0.8}
::::
