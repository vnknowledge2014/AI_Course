---
id: ky-nghe-phan-mem.ddd.kien-truc-bon-tang-va-luat-phu-thuoc
title: "Kiến trúc bốn tầng & Dependency Rule — mũi tên chỉ vào trong"
summary: "Ứng dụng chia bốn tầng lồng nhau: Presentation → Application → Domain → Infrastructure. Dependency Rule: tầng TRONG không bao giờ import tầng NGOÀI. Domain = pure; Infrastructure = impure (I/O thật)."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ddd.layered-architecture]
requires: [ddd.event-storming]
concepts: [ddd.layered-architecture]
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
Cụm mới. Bạn có từ vựng (Entity, Value Object, Domain Event) — giờ cần
biết đặt chúng Ở ĐÂU trong một ứng dụng thật.
::::

::::explain{#bon-tang}
Ứng dụng chia BỐN tầng lồng nhau:

```text
Presentation  (UI, HTTP handler — người dùng chạm vào đây)
    ↓
Application   (use case — điều phối, gọi domain + infra)
    ↓
Domain        (Entity, Value Object, business rule — TRÁI TIM)
    ↓
Infrastructure (database, API bên ngoài — chi tiết kỹ thuật)
```

**Dependency Rule**: mũi tên phụ thuộc CHỈ chỉ vào TRONG. Tầng TRONG
KHÔNG BAO GIỜ import tầng NGOÀI — Domain (tầng trong nhất) không được
biết database là gì, HTTP là gì. Tầng NGOÀI phụ thuộc/implement tầng
trong.

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

// Domain layer — THUẦN, không import gì ngoài domain types
type DonHangMoi = { maKhachHang: string; tongTien: number };

function validateDonHangMoi(dh: DonHangMoi): Result<DonHangMoi, string> {
  if (dh.tongTien <= 0) return loi("tổng tiền phải lớn hơn 0");
  return ok(dh);
}

console.log(JSON.stringify(validateDonHangMoi({ maKhachHang: "KH-01", tongTien: 100000 })));
console.log(JSON.stringify(validateDonHangMoi({ maKhachHang: "KH-02", tongTien: 0 })));
```

```text
{"kind":"ok","giaTri":{"maKhachHang":"KH-01","tongTien":100000}}
{"kind":"loi","loi":"tổng tiền phải lớn hơn 0"}
```

`validateDonHangMoi` KHÔNG có dòng nào nhắc tới database, HTTP, hay
`fetch` — nó CHỈ nhận dữ liệu, TRẢ VỀ `Result` (đã học ở T4.5). Domain
= **pure** (chỉ type + hàm thuần, không side-effect). Infrastructure
(lưu vào DB thật, gọi API thật) = **impure** — I/O THẬT, nằm ở TẦNG
NGOÀI, KHÔNG được lẫn vào Domain.
::::

::::example{#dependency-rule-vi-pham}
Đối chiếu ❌ (vi phạm) và ✅ (đúng):

```typescript title=readonly
// ❌ SAI: Domain import trực tiếp Infrastructure
// import { luuVaoDatabase } from "../infra/database";
// function xacNhanDonHang(dh: DonHangMoi) {
//   luuVaoDatabase(dh); // Domain giờ BIẾT database là gì — vi phạm Dependency Rule
// }

// ✅ ĐÚNG: Domain chỉ TÍNH TOÁN, KHÔNG tự lưu
type DonHangDaXacNhan = { maKhachHang: string; tongTien: number; daXacNhan: true };

function xacNhanDonHang(dh: DonHangMoi): DonHangDaXacNhan {
  return { ...dh, daXacNhan: true };
}

console.log(xacNhanDonHang({ maKhachHang: "KH-03", tongTien: 50000 }));
```

```text title=readonly
{"maKhachHang":"KH-03","tongTien":50000,"daXacNhan":true}
```

`xacNhanDonHang` (Domain) chỉ TRẢ VỀ một `DonHangDaXacNhan` MỚI — nó
KHÔNG tự lưu đi đâu cả. VIỆC lưu (I/O thật) là TRÁCH NHIỆM của một tầng
NGOÀI (Application/Infrastructure) — Domain chỉ TÍNH, không LÀM.
::::

::::predict{#doan-validate-am commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type DonHangMoi = { maKhachHang: string; tongTien: number };
function validateDonHangMoi(dh: DonHangMoi): Result<DonHangMoi, string> {
  if (dh.tongTien <= 0) return loi("tổng tiền phải lớn hơn 0");
  return ok(dh);
}

const ketQua = validateDonHangMoi({ maKhachHang: "KH-04", tongTien: -50000 });
console.log(ketQua.kind);
```

Dòng cuối in ra gì (`tongTien` là số ÂM)?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì điều kiện `dh.tongTien <= 0` chỉ chặn giá trị ĐÚNG BẰNG 0,
số âm không khớp điều kiện đó
::why
Gần đúng ở việc bạn đọc ĐÚNG chữ `<= 0` có nhắc tới `0` — quan sát về
việc `0` xuất hiện trong điều kiện đó đúng.

Chỗ lệch: `<=` nghĩa là "NHỎ HƠN HOẶC BẰNG" — `-50000 <= 0` là `true`
(số âm chắc chắn NHỎ HƠN `0`), KHÔNG chỉ khớp riêng `0`. Điều kiện
`dh.tongTien <= 0` chặn CẢ số `0` LẪN mọi số ÂM — `validateDonHangMoi`
trả về `loi(...)`.
::
:::

:::opt
Máy báo lỗi biên dịch — `tongTien: -50000` không hợp lệ vì `number`
trong TypeScript mặc định không cho phép giá trị âm
::why
Gần đúng ở việc bạn nghĩ tới RÀNG BUỘC "không âm" cho tiền — một quy
tắc NGHIỆP VỤ hợp lý (và ĐÚNG LÀ mục đích của `validateDonHangMoi`).

Chỗ lệch: kiểu `number` trong TypeScript KHÔNG có khái niệm "âm/dương"
ở TẦNG KIỂU — nó chấp nhận MỌI số thực, kể cả âm. Ràng buộc "không âm"
là một QUY TẮC NGHIỆP VỤ, kiểm LÚC CHẠY bằng `if`, không phải một giới
hạn của kiểu `number`. `-50000` HOÀN TOÀN hợp lệ về mặt KIỂU.
::
:::
::::

::::code{#viet_validate_don_hang_moi}
Tự viết `validateDonHangMoi(dh: DonHangMoi): Result<DonHangMoi, string>`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type DonHangMoi = { maKhachHang: string; tongTien: number };

function validateDonHangMoi(dh: DonHangMoi): Result<DonHangMoi, string> {
  if (dh.tongTien <= 0) return ___;
  return ___;
}

console.log(JSON.stringify(validateDonHangMoi({ maKhachHang: "KH-01", tongTien: 100000 })));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type DonHangMoi = { maKhachHang: string; tongTien: number };

function validateDonHangMoi(dh: DonHangMoi): Result<DonHangMoi, string> {
  if (dh.tongTien <= 0) return loi("tổng tiền phải lớn hơn 0");
  return ok(dh);
}

console.log(JSON.stringify(validateDonHangMoi({ maKhachHang: "KH-01", tongTien: 100000 })));
```

```typescript title=test
const a = validateDonHangMoi({ maKhachHang: "KH-01", tongTien: 100000 });
if (a.kind !== "ok") throw new Error("tổng tiền dương phải trả về ok");
if (a.kind === "ok" && a.giaTri.maKhachHang !== "KH-01") throw new Error("ok phải giữ nguyên dữ liệu đơn hàng gốc");

const b = validateDonHangMoi({ maKhachHang: "KH-02", tongTien: -50000 });
if (b.kind !== "loi") throw new Error("tổng tiền âm phải trả về loi");

const c = validateDonHangMoi({ maKhachHang: "KH-03", tongTien: 0 });
if (c.kind !== "loi") throw new Error("tổng tiền ĐÚNG BẰNG 0 phải trả về loi (điều kiện dùng <=, không phải <)");

const d = validateDonHangMoi({ maKhachHang: "KH-04", tongTien: 1 });
if (d.kind !== "ok") throw new Error("tổng tiền 1 (vừa lớn hơn 0) phải trả về ok");
```

:::hints
- kind: attention
  body: "Nhánh dh.tongTien <= 0 là LỖI — bọc thông điệp bằng loi(...). Nhánh CÒN LẠI là hợp lệ — bọc chính dh bằng ok(dh), không cần thay đổi gì."
- kind: strategy
  body: 'loi("tổng tiền phải lớn hơn 0") — cho nhánh lỗi. ok(dh) — cho nhánh hợp lệ.'
- kind: one-line
  body: 'return loi("tổng tiền phải lớn hơn 0");\nreturn ok(dh);'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "ok"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn tầng, mũi tên chỉ vào trong. Domain thuần — chỉ tính, không làm.
Infrastructure là nơi I/O thật xảy ra, luôn ở tầng NGOÀI CÙNG.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Domain thuần" và "Infrastructure impure" — bạn đã học ý này Ở QUY MÔ
một hàm (Functional Core / Imperative Shell). Áp dụng nó cho CẢ một
ỨNG DỤNG thì trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
