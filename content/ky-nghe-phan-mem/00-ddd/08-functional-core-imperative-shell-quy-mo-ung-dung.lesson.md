---
id: ky-nghe-phan-mem.ddd.functional-core-imperative-shell-quy-mo-ung-dung
title: "Functional Core / Imperative Shell ở quy mô ứng dụng — Sandwich Pattern"
summary: "Domain layer = Functional Core (pure) TOÀN BỘ; Application layer = Imperative Shell — orchestrate tuần tự. Sandwich Pattern (cấp MỘT hàm): Read (IO) → Process (Pure) → Write (IO). Anti-pattern: IO rải rác trộn lẫn business logic."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.fc-is-at-scale]
requires: [ddd.layered-architecture]
concepts: [ddd.fc-is-at-scale]
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
Domain thuần, Infrastructure không thuần — bạn đã học Ý này Ở QUY MÔ
một hàm. Áp dụng nó cho CẢ một ỨNG DỤNG trông thế nào?
::::

::::explain{#fc-is-quy-mo-lon}
**Functional Core / Imperative Shell (FC/IS) ở quy mô LỚN**: Domain
layer = Functional Core (pure) TOÀN BỘ. Application layer = Imperative
Shell — orchestrate TUẦN TỰ (đọc dữ liệu → gọi core → ghi dữ liệu),
KHÔNG chứa business rule. Shell GỌI core; core KHÔNG BAO GIỜ gọi ngược
lại shell.

**Sandwich Pattern** (cụ thể hoá FC/IS ở cấp MỘT hàm): Read (IO) →
Process (Pure) → Write (IO) — "bánh sandwich", hai lớp IO bọc quanh MỘT
lớp tính toán thuần ở giữa:

```typescript
type DonHang = { maDonHang: string; tongTien: number };

// Functional Core: THUẦN, chỉ tính toán
function tinhThue(tongTien: number): number {
  return tongTien * 0.1;
}

// Mô phỏng "kho dữ liệu" bằng Map trong bộ nhớ (đóng vai I/O)
const KHO: Map<string, DonHang> = new Map([["DH-01", { maDonHang: "DH-01", tongTien: 100000 }]]);
const thueDaGhi: Map<string, number> = new Map();

function timDonHang(ma: string): DonHang | undefined {
  return KHO.get(ma);
}
function capNhatThue(ma: string, thue: number): void {
  thueDaGhi.set(ma, thue);
}

// Imperative Shell: Read → Process → Write
function xuLyDonHangShell(maDonHang: string): string {
  const dh = timDonHang(maDonHang);          // Read (IO)
  if (dh === undefined) return "không tìm thấy đơn hàng";
  const thue = tinhThue(dh.tongTien);        // Process (Pure)
  capNhatThue(maDonHang, thue);              // Write (IO)
  return `đã tính thuế ${thue}`;
}

console.log(xuLyDonHangShell("DH-01"));
console.log(thueDaGhi.get("DH-01"));
```

```text
đã tính thuế 10000
10000
```

`xuLyDonHangShell` LÀ Imperative Shell — nó orchestrate BA bước theo
đúng thứ tự: TÌM (IO), TÍNH (pure), GHI (IO). Phần TÍNH (`tinhThue`) là
Functional Core — MỘT hàm thuần, test được KHÔNG cần "kho dữ liệu" giả
lập nào, chỉ input→output. Test core KHÔNG CẦN mock.
::::

::::example{#anti-pattern-io-rai-rac}
Đối lập: IO và business logic TRỘN LẪN trong CÙNG một hàm — khó test,
khó đọc:

```typescript title=readonly
type DonHang = { maDonHang: string; tongTien: number };
const KHO: Map<string, DonHang> = new Map([["DH-01", { maDonHang: "DH-01", tongTien: 100000 }]]);
const thueDaGhi: Map<string, number> = new Map();

// Anti-pattern: IO và business logic TRỘN LẪN
function xuLyDonHangXau(maDonHang: string): string {
  const dh = KHO.get(maDonHang);              // IO
  if (dh === undefined) return "không tìm thấy đơn hàng";
  const thue = dh.tongTien * 0.1;             // business logic TRỘN vào giữa IO
  thueDaGhi.set(maDonHang, thue);             // IO
  return `đã tính thuế ${thue}`;
}

console.log(xuLyDonHangXau("DH-01"));
```

```text title=readonly
đã tính thuế 10000
```

CÙNG kết quả! Nhưng `0.1` (tỷ lệ thuế — MỘT quy tắc NGHIỆP VỤ) nằm LẪN
NGAY giữa hai thao tác IO (`KHO.get`, `thueDaGhi.set`) — muốn TEST riêng
"quy tắc tính thuế có đúng không" mà KHÔNG cần dựng cả `KHO`/`thueDaGhi`
giả lập là KHÔNG THỂ, vì phép tính đó KHÔNG hề tách rời được khỏi IO.
::::

::::predict{#doan-shell-khong-tim-thay commitOnce}
```typescript
type DonHang = { maDonHang: string; tongTien: number };
function tinhThue(tongTien: number): number {
  return tongTien * 0.1;
}
const KHO: Map<string, DonHang> = new Map([["DH-01", { maDonHang: "DH-01", tongTien: 100000 }]]);
const thueDaGhi: Map<string, number> = new Map();
function timDonHang(ma: string): DonHang | undefined {
  return KHO.get(ma);
}
function capNhatThue(ma: string, thue: number): void {
  thueDaGhi.set(ma, thue);
}
function xuLyDonHangShell(maDonHang: string): string {
  const dh = timDonHang(maDonHang);
  if (dh === undefined) return "không tìm thấy đơn hàng";
  const thue = tinhThue(dh.tongTien);
  capNhatThue(maDonHang, thue);
  return `đã tính thuế ${thue}`;
}

xuLyDonHangShell("DH-99");
console.log(thueDaGhi.size);
```

`"DH-99"` KHÔNG có trong `KHO`. Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`1` — vì `xuLyDonHangShell` LUÔN ghi thuế vào `thueDaGhi` sau khi tính,
bất kể có tìm thấy đơn hàng hay không
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng hàm CÓ bước ghi thuế (`capNhatThue`)
— quan sát về việc CÓ bước đó tồn tại đúng.

Chỗ lệch: `capNhatThue` CHỈ được gọi SAU `tinhThue`, mà `tinhThue`
CHỈ được gọi SAU khi vượt qua kiểm tra `if (dh === undefined) return
...` — với `"DH-99"`, `timDonHang` trả `undefined`, hàm `return` NGAY
ở dòng đó ("không tìm thấy đơn hàng"), KHÔNG BAO GIỜ chạy tới
`tinhThue` hay `capNhatThue`. `thueDaGhi` giữ nguyên RỖNG (`size` là
`0`).
::
:::

:::opt
Máy báo lỗi lúc chạy — gọi `capNhatThue` với `thue` chưa từng được gán
giá trị (vì `dh` là `undefined`) gây crash
::why
Gần đúng ở việc bạn để ý `dh` CÓ THỂ là `undefined` — quan sát về khả
năng đó đúng, và ĐÂY chính xác là lý do CẦN kiểm tra `if (dh ===
undefined)`.

Chỗ lệch: đoạn code ĐÃ xử lý đúng trường hợp đó — `if (dh === undefined)
return "không tìm thấy đơn hàng";` CHẶN luồng thực thi LẠI NGAY tại
dòng đó bằng `return`, không hề đi tiếp tới dòng `tinhThue(dh.tongTien)`
(nơi mới cần `dh` chắc chắn KHÔNG `undefined`). Không có crash nào xảy
ra — luồng thực thi dừng SỚM, đúng như thiết kế.
::
:::
::::

::::code{#viet_xu_ly_don_hang_shell}
Tự viết `xuLyDonHangShell(maDonHang: string): string` theo khuôn
Sandwich Pattern.

```typescript title=starter
type DonHang = { maDonHang: string; tongTien: number };
function tinhThue(tongTien: number): number {
  return tongTien * 0.1;
}
const KHO: Map<string, DonHang> = new Map([["DH-01", { maDonHang: "DH-01", tongTien: 100000 }]]);
const thueDaGhi: Map<string, number> = new Map();
function timDonHang(ma: string): DonHang | undefined {
  return KHO.get(ma);
}
function capNhatThue(ma: string, thue: number): void {
  thueDaGhi.set(ma, thue);
}

function xuLyDonHangShell(maDonHang: string): string {
  const dh = timDonHang(maDonHang);
  if (dh === undefined) return "không tìm thấy đơn hàng";
  const thue: number = ___;
  ___;
  return `đã tính thuế ${thue}`;
}

console.log(xuLyDonHangShell("DH-01"));
```

```typescript title=solution
type DonHang = { maDonHang: string; tongTien: number };
function tinhThue(tongTien: number): number {
  return tongTien * 0.1;
}
const KHO: Map<string, DonHang> = new Map([["DH-01", { maDonHang: "DH-01", tongTien: 100000 }]]);
const thueDaGhi: Map<string, number> = new Map();
function timDonHang(ma: string): DonHang | undefined {
  return KHO.get(ma);
}
function capNhatThue(ma: string, thue: number): void {
  thueDaGhi.set(ma, thue);
}

function xuLyDonHangShell(maDonHang: string): string {
  const dh = timDonHang(maDonHang);
  if (dh === undefined) return "không tìm thấy đơn hàng";
  const thue: number = tinhThue(dh.tongTien);
  capNhatThue(maDonHang, thue);
  return `đã tính thuế ${thue}`;
}

console.log(xuLyDonHangShell("DH-01"));
```

```typescript title=test
const a = xuLyDonHangShell("DH-01");
if (a !== "đã tính thuế 10000") throw new Error("đơn hàng tìm thấy phải tính đúng thuế và trả về thông điệp khớp");
if (thueDaGhi.get("DH-01") !== 10000) throw new Error("phải GHI đúng giá trị thuế đã tính vào thueDaGhi (100000 * 0.1 = 10000)");

const b = xuLyDonHangShell("DH-99");
if (b !== "không tìm thấy đơn hàng") throw new Error("đơn hàng không tồn tại phải trả về đúng thông điệp");
if (thueDaGhi.has("DH-99")) throw new Error("đơn hàng không tồn tại KHÔNG được ghi thuế nào");
```

:::hints
- kind: attention
  body: "Chỗ trống 1: TÍNH thuế từ dh.tongTien bằng hàm tinhThue đã có sẵn. Chỗ trống 2: GHI kết quả vào bằng capNhatThue(maDonHang, thue) — không cần return gì (chỉ side-effect)."
- kind: strategy
  body: "const thue: number = tinhThue(dh.tongTien); capNhatThue(maDonHang, thue); — đúng khuôn Read → Process → Write, chỗ trống là Process VÀ Write."
- kind: one-line
  body: "___ (thue) = tinhThue(dh.tongTien)\n___ (ghi) = capNhatThue(maDonHang, thue)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "10000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sandwich Pattern: Read (IO) → Process (Pure) → Write (IO). Core (tính
toán) tách rời hoàn toàn khỏi Shell (orchestrate IO) — test core không
cần mock gì cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`xuLyDonHangShell` và các hàm liên quan nằm CHUNG một chỗ ở đây. Trong
một dự án THẬT, chúng nằm ở ĐÂU, và ai được PHÉP thấy chúng?
::::

::::checkpoint{mastery=0.8}
::::
