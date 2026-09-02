---
id: ky-nghe-phan-mem.mau-tai-cau-truc.projections-nhieu-view-tu-mot-event-stream
title: "Projections — NHIỀU view khác nhau từ CÙNG MỘT chuỗi sự kiện"
summary: "Vì sự kiện là nguồn SỰ THẬT DUY NHẤT, có thể \"chiếu\" chúng thành NHIỀU read model KHÁC NHAU CÙNG LÚC — TẤT CẢ đều CHỈ LÀ reduce với hàm gộp KHÁC NHAU trên CÙNG một mảng sự kiện. chieuTongChiTieuTheoKhachHang VÀ chieuSoDonTheoTrangThai là HAI hàm ĐỘC LẬP — không phải hai bảng đồng bộ thủ công, mà HAI PHÉP TÍNH từ cùng dữ liệu gốc."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 23
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.projections-multiple-views]
requires: [mau.event-sourcing-basics]
concepts: [mau.projections-multiple-views]
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
Từ CÙNG một chuỗi sự kiện đơn hàng, cần HAI báo cáo KHÁC NHAU: tổng
chi tiêu THEO KHÁCH, VÀ số đơn THEO TRẠNG THÁI. Cần lưu HAI bảng
riêng KHÔNG?
::::

::::explain{#chieu-nhieu-view}
KHÔNG cần lưu HAI bảng đồng bộ THỦ CÔNG — vì sự kiện LÀ nguồn SỰ THẬT
DUY NHẤT, có thể "CHIẾU" (project) CHÚNG thành **NHIỀU** read model
KHÁC NHAU CÙNG LÚC, MỖI model phục vụ MỘT mục đích. TẤT CẢ đều CHỈ LÀ
`reduce`/vòng lặp với HÀM GỘP KHÁC NHAU trên **CÙNG MỘT** mảng sự
kiện:

```typescript title=readonly
type SuKienDonHang = { khachHang: string; trangThai: string; soTien: number };

function chieuTongChiTieuTheoKhachHang(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.khachHang, (ketQua.get(sk.khachHang) ?? 0) + sk.soTien);
  }
  return ketQua;
}

function chieuSoDonTheoTrangThai(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.trangThai, (ketQua.get(sk.trangThai) ?? 0) + 1);
  }
  return ketQua;
}

const cacSuKien: SuKienDonHang[] = [
  { khachHang: "An", trangThai: "hoan_tat", soTien: 300 },
  { khachHang: "Binh", trangThai: "dang_giao", soTien: 150 },
  { khachHang: "An", trangThai: "hoan_tat", soTien: 200 },
];

console.log(Object.fromEntries(chieuTongChiTieuTheoKhachHang(cacSuKien)));
console.log(Object.fromEntries(chieuSoDonTheoTrangThai(cacSuKien)));
```

```text title=readonly
{"An":500,"Binh":150}
{"hoan_tat":2,"dang_giao":1}
```

HAI hàm ĐỘC LẬP HOÀN TOÀN — KHÔNG gọi lẫn nhau, KHÔNG "đồng bộ" gì
cả — CÙNG đọc `cacSuKien`, MỖI hàm gộp THEO một tiêu chí RIÊNG (theo
`khachHang` vs theo `trangThai`). (`console.log` trên `Map` TRỰC TIẾP
in `{}` — dùng `Object.fromEntries(...)` để xem NỘI DUNG dễ đọc.)
::::

::::example{#ghep-projection-tu-projection}
Một projection có THỂ **GHÉP TỪ** projection KHÁC — KHÔNG cần đọc
LẠI `cacSuKien` TỪ ĐẦU:

```typescript title=readonly
type SuKienDonHang = { khachHang: string; trangThai: string; soTien: number };
function chieuTongChiTieuTheoKhachHang(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.khachHang, (ketQua.get(sk.khachHang) ?? 0) + sk.soTien);
  }
  return ketQua;
}

function chieuKhachHangChiNhieuNhat(cacSuKien: SuKienDonHang[]): string | null {
  const theoKhach = chieuTongChiTieuTheoKhachHang(cacSuKien);
  let tenNhieuNhat: string | null = null;
  let soTienLon = -Infinity;
  for (const [ten, soTien] of theoKhach) {
    if (soTien > soTienLon) {
      soTienLon = soTien;
      tenNhieuNhat = ten;
    }
  }
  return tenNhieuNhat;
}

const cacSuKien: SuKienDonHang[] = [
  { khachHang: "An", trangThai: "hoan_tat", soTien: 300 },
  { khachHang: "Binh", trangThai: "dang_giao", soTien: 150 },
  { khachHang: "An", trangThai: "hoan_tat", soTien: 200 },
];
console.log(chieuKhachHangChiNhieuNhat(cacSuKien));
```

```text title=readonly
An
```

`chieuKhachHangChiNhieuNhat` LÀ MỘT projection MỚI, XÂY TỪ
`chieuTongChiTieuTheoKhachHang` ĐÃ CÓ — thêm MỘT view MỚI hoàn toàn
KHÔNG chạm tới `cacSuKien` hay các projection KHÁC, ĐÚNG kỷ luật
"thêm hàm mới, không sửa gì cũ" đã học Ở bài 14.
::::

::::predict{#doan-cong-don-khong-ghi-de commitOnce}
```typescript
type SuKienDonHang = { khachHang: string; trangThai: string; soTien: number };
function chieuTongChiTieuTheoKhachHang(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.khachHang, (ketQua.get(sk.khachHang) ?? 0) + sk.soTien);
  }
  return ketQua;
}
function chieuSoDonTheoTrangThai(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.trangThai, (ketQua.get(sk.trangThai) ?? 0) + 1);
  }
  return ketQua;
}

const cacSuKien: SuKienDonHang[] = [
  { khachHang: "Chi", trangThai: "moi", soTien: 50 },
  { khachHang: "Chi", trangThai: "hoan_tat", soTien: 100 },
  { khachHang: "Chi", trangThai: "hoan_tat", soTien: 75 },
];
console.log(chieuTongChiTieuTheoKhachHang(cacSuKien).get("Chi"));
console.log(chieuSoDonTheoTrangThai(cacSuKien).get("hoan_tat"));
```

Hai dòng in ra gì? (`"Chi"` xuất hiện BA lần trong sự kiện.)

:::opt{correct}
`225` rồi `2`
:::

:::opt
`75` rồi `1` — vì `Map.set(khoa, giaTri)` GHI ĐÈ giá trị CŨ mỗi lần
gọi VỚI CÙNG một khoá (`"Chi"`), nên chỉ CÓ giá trị của SỰ KIỆN CUỐI
CÙNG (`75`) LÀ CÒN LẠI trong Map
::why
Gần đúng ở việc bạn nhớ ĐÚNG `Map.set` VỚI CÙNG một khoá LUÔN GHI ĐÈ
giá trị TRƯỚC ĐÓ — quan sát ĐÓ, VỀ HÀNH VI `Map.set` TỔNG QUÁT, chính
xác.

Chỗ lệch: đúng LÀ `.set(khoa, giaTri)` GHI ĐÈ — NHƯNG `giaTri` được
TRUYỀN vào Ở ĐÂY **KHÔNG PHẢI** `sk.soTien` TRỰC TIẾP, mà LÀ
`(ketQua.get(sk.khachHang) ?? 0) + sk.soTien` — TỰ ĐỌC giá trị CŨ
(`ketQua.get(...)`) RỒI **CỘNG THÊM** `soTien` MỚI TRƯỚC khi ghi
ĐÈ. Mỗi lần gặp `"Chi"`, giá trị MỚI = giá trị CŨ + tiền MỚI — TÍCH
LŨY, KHÔNG PHẢI thay THẾ. `50 → 50+100=150 → 150+75=225`.
::
:::

:::opt
Máy báo lỗi lúc chạy — `ketQua.get(sk.khachHang)` trả về `number |
undefined` (vì `khachHang` CÓ THỂ chưa từng xuất hiện trong Map),
`?? 0` KHÔNG đủ để "sửa" kiểu `undefined` này, TypeScript/JavaScript
ném lỗi khi CỘNG `undefined + number`
::why
Gần đúng ở việc bạn để ý `ketQua.get(...)` khai kiểu trả về `number |
undefined` — MỘT quan sát ĐÚNG về kiểu trả VỀ của `Map.get`.

Chỗ lệch: `??` (nullish coalescing) **CHÍNH LÀ** toán tử ĐƯỢC THIẾT
KẾ để xử lý ĐÚNG trường hợp NÀY — `x ?? 0` trả về `x` NẾU `x` KHÁC
`null`/`undefined`, NGƯỢC LẠI trả về `0`. Kết quả CỦA `?? 0` LUÔN LÀ
`number` (KHÔNG BAO GIỜ `undefined`) — phép CỘNG SAU ĐÓ hoàn toàn AN
TOÀN. Chạy SẠCH, không lỗi nào cả.
::
:::
::::

::::code{#viet_projections}
Hoàn thiện HAI phép chiếu — `chieuTongChiTieuTheoKhachHang` (cộng
dồn TIỀN) VÀ `chieuSoDonTheoTrangThai` (đếm SỐ ĐƠN).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SuKienDonHang = { khachHang: string; trangThai: string; soTien: number };

function chieuTongChiTieuTheoKhachHang(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.khachHang, ___);
  }
  return ketQua;
}

function chieuSoDonTheoTrangThai(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.trangThai, ___);
  }
  return ketQua;
}

const cacSuKien1: SuKienDonHang[] = [{ khachHang: "An", trangThai: "hoan_tat", soTien: 100 }];
assertEqual(chieuTongChiTieuTheoKhachHang(cacSuKien1).get("An"), 100, "mot don hang");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type SuKienDonHang = { khachHang: string; trangThai: string; soTien: number };

function chieuTongChiTieuTheoKhachHang(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.khachHang, (ketQua.get(sk.khachHang) ?? 0) + sk.soTien);
  }
  return ketQua;
}

function chieuSoDonTheoTrangThai(cacSuKien: SuKienDonHang[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    ketQua.set(sk.trangThai, (ketQua.get(sk.trangThai) ?? 0) + 1);
  }
  return ketQua;
}

const cacSuKien1: SuKienDonHang[] = [{ khachHang: "An", trangThai: "hoan_tat", soTien: 100 }];
assertEqual(chieuTongChiTieuTheoKhachHang(cacSuKien1).get("An"), 100, "mot don hang");
```

```typescript title=test
const cacSuKien2: SuKienDonHang[] = [
  { khachHang: "An", trangThai: "hoan_tat", soTien: 300 },
  { khachHang: "Binh", trangThai: "dang_giao", soTien: 150 },
  { khachHang: "An", trangThai: "hoan_tat", soTien: 200 },
];
assertEqual(chieuTongChiTieuTheoKhachHang(cacSuKien2).get("An"), 500, "cong don nhieu don cua an");
assertEqual(chieuTongChiTieuTheoKhachHang(cacSuKien2).get("Binh"), 150, "binh chi mot don");
assertEqual(chieuSoDonTheoTrangThai(cacSuKien2).get("hoan_tat"), 2, "hai don hoan tat");
assertEqual(chieuSoDonTheoTrangThai(cacSuKien2).get("dang_giao"), 1, "mot don dang giao");
assertEqual(chieuTongChiTieuTheoKhachHang([]).get("An"), undefined, "khong su kien nao -- khong co khach nao trong map");
```

:::hints
- kind: attention
  body: "chieuTongChiTieuTheoKhachHang: đọc giá trị CŨ (mặc định 0 nếu chưa có), CỘNG THÊM sk.soTien. chieuSoDonTheoTrangThai: y hệt, nhưng CỘNG THÊM 1 (đếm)."
- kind: strategy
  body: "(ketQua.get(sk.khachHang) ?? 0) + sk.soTien : (ketQua.get(sk.trangThai) ?? 0) + 1"
- kind: one-line
  body: '___ (theoKhachHang) = (ketQua.get(sk.khachHang) ?? 0) + sk.soTien\n___ (theoTrangThai) = (ketQua.get(sk.trangThai) ?? 0) + 1'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Projections: nhiều view, MỘT nguồn sự thật. Bài BOSS: ghép CQRS + ES
+ Projection thành một hệ nhỏ hoàn chỉnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ghép TOÀN BỘ track: WRITE model (bài 21, validate + sinh sự kiện) +
lưu TOÀN BỘ sự kiện (bài 22) + NHIỀU projection (bài NÀY) — bạn hình
dung được MỘT hệ THỰC (kho hàng nhỏ) dùng CẢ BA kỹ thuật CÙNG lúc
chưa?
::::

::::checkpoint{mastery=0.8}
::::
