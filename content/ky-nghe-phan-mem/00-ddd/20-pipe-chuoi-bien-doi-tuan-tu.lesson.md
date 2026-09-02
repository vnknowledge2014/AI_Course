---
id: ky-nghe-phan-mem.ddd.pipe-chuoi-bien-doi-tuan-tu
title: "pipe() — chuỗi biến đổi tuần tự, chạy NGAY"
summary: "pipe(giaTri, f, g, h) nhận giá trị NGAY, chạy tuần tự qua các hàm, trả kết quả cuối. Cài đặt bằng function overloading + reduce bên trong. pipe(sanPhamTho, lamSach, boSung, dinhDangHienThi) — biến đổi MỘT giá trị cụ thể."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 20
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.pipe]
requires: [ddd.why-pipeline]
concepts: [ddd.pipe]
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
T4.2 đã dạy `pipe()` — nối nhiều hàm thành một chuỗi. Viết lại bằng
TypeScript, áp dụng vào domain thật.
::::

::::explain{#pipe-typescript}
`pipe(giaTri, f, g, h)` nhận giá trị NGAY, chạy TUẦN TỰ qua các hàm,
trả về kết quả CUỐI. Cài đặt bằng **function overloading** (vài chữ ký
theo số lượng hàm, TypeScript chọn ĐÚNG chữ ký khớp) + `reduce` (T4.2)
BÊN TRONG:

```typescript
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, f: (a: A) => B, g: (b: B) => C, h: (c: C) => D): D;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}

type SanPhamTho = { ten: string; gia: number };

function lamSach(sp: SanPhamTho): SanPhamTho {
  return { ten: sp.ten.trim(), gia: sp.gia };
}
function boSung(sp: SanPhamTho): SanPhamTho & { coGiamGia: boolean } {
  return { ...sp, coGiamGia: sp.gia > 100000 };
}
function dinhDangHienThi(sp: SanPhamTho & { coGiamGia: boolean }): string {
  return `${sp.ten} - ${sp.gia}đ${sp.coGiamGia ? " (giảm giá)" : ""}`;
}

const ketQua = pipe(
  { ten: "  Ly thuy tinh  ", gia: 150000 },
  lamSach,
  boSung,
  dinhDangHienThi,
);
console.log(ketQua);
```

```text
Ly thuy tinh - 150000đ (giảm giá)
```

BA hàm overload (`function pipe<A,B>(...)`, `<A,B,C>`, `<A,B,C,D>`)
KHÔNG có thân — CHỈ khai chữ ký, để TypeScript BIẾT kiểu trả về CHÍNH
XÁC khi gọi `pipe` với 2, 3, hay 4 tham số. Chữ ký CUỐI (có thân thật,
dùng `unknown`) là implementation THẬT — chạy `reduce` để áp DẦN từng
hàm lên giá trị tích luỹ. Đây LÀ "run now" — `pipe` nhận GIÁ TRỊ CỤ
THỂ (`{ ten: "...", gia: 150000 }`) và BIẾN ĐỔI nó NGAY.
::::

::::example{#pipe-doi-thu-tu}
Thứ tự CÁC HÀM trong `pipe` QUYẾT ĐỊNH kết quả — đổi thứ tự = đổi ý
nghĩa:

```typescript title=readonly
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}

const nhanDoi = (x: number) => x * 2;
const congMuoi = (x: number) => x + 10;

console.log(pipe(5, nhanDoi, congMuoi)); // nhân đôi TRƯỚC, cộng 10 SAU
console.log(pipe(5, congMuoi, nhanDoi)); // cộng 10 TRƯỚC, nhân đôi SAU
```

```text title=readonly
20
30
```

`pipe(5, nhanDoi, congMuoi)`: `5 → nhanDoi → 10 → congMuoi → 20`.
`pipe(5, congMuoi, nhanDoi)`: `5 → congMuoi → 15 → nhanDoi → 30`. CÙNG
hai hàm, THỨ TỰ khác nhau, kết quả khác hẳn — `pipe` đọc TRÁI SANG
PHẢI, đúng thứ tự hàm được LIỆT KÊ.
::::

::::predict{#doan-pipe-ba-buoc commitOnce}
```typescript
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, f: (a: A) => B, g: (b: B) => C, h: (c: C) => D): D;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}

const nhanBa = (x: number) => x * 3;
const truHai = (x: number) => x - 2;
const thanhChuoi = (x: number) => `kết quả: ${x}`;

console.log(pipe(4, nhanBa, truHai, thanhChuoi));
```

Dòng cuối in ra gì?

:::opt{correct}
`kết quả: 10`
:::

:::opt
`kết quả: 6` — vì `pipe` áp cả ba hàm lên GIÁ TRỊ BAN ĐẦU (`4`) một
cách ĐỘC LẬP, rồi chỉ GIỮ LẠI kết quả của hàm CUỐI CÙNG kèm hàm ĐẦU
::why
Gần đúng ở việc bạn nhớ ĐÚNG có BA hàm được áp dụng — quan sát về SỐ
LƯỢNG hàm tham gia đó đúng.

Chỗ lệch: `pipe` áp các hàm THEO CHUỖI, mỗi hàm nhận KẾT QUẢ của hàm
TRƯỚC (không phải giá trị GỐC): `4 → nhanBa → 12 → truHai → 10 →
thanhChuoi → "kết quả: 10"`. `truHai` nhận `12` (KẾT QUẢ của `nhanBa`),
không phải `4` (giá trị GỐC) — nên không có cách nào ra `6` ở bước
giữa của chuỗi này.
::
:::

:::opt
Máy báo lỗi biên dịch — hàm `thanhChuoi` trả về `string`, không khớp
kiểu `D` mà chữ ký `pipe<A,B,C,D>` mong đợi từ MỘT hàm trả `number`
::why
Gần đúng ở việc bạn để ý `thanhChuoi` trả về KIỂU KHÁC (`string`) so
với `nhanBa`/`truHai` (cả hai trả `number`) — quan sát về sự khác kiểu
đó đúng.

Chỗ lệch: chữ ký `pipe<A,B,C,D>(a: A, f: (a:A)=>B, g: (b:B)=>C, h:
(c:C)=>D): D` KHÔNG hề ép `B`, `C`, `D` phải CÙNG kiểu — mỗi tham số
kiểu (`A`, `B`, `C`, `D`) được TypeScript TỰ SUY RA riêng theo TỪNG
hàm truyền vào. `h: (c: C) => D` ở đây là `thanhChuoi: (x: number) =>
string` — hoàn toàn khớp, `D` được suy ra là `string`. Biên dịch sạch.
::
:::
::::

::::code{#viet_pipe}
Tự viết PHẦN THÂN (implementation) của `pipe`.

```typescript title=starter
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return ___;
}

const nhanDoi = (x: number) => x * 2;
const congMuoi = (x: number) => x + 10;
console.log(pipe(5, nhanDoi, congMuoi));
```

```typescript title=solution
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}

const nhanDoi = (x: number) => x * 2;
const congMuoi = (x: number) => x + 10;
console.log(pipe(5, nhanDoi, congMuoi));
```

```typescript title=test
if (pipe(5, nhanDoi, congMuoi) !== 20) throw new Error("pipe(5, nhanDoi, congMuoi) phải ra 20 (5*2=10, 10+10=20)");
if (pipe(5, congMuoi, nhanDoi) !== 30) throw new Error("pipe(5, congMuoi, nhanDoi) phải ra 30 — đảo thứ tự phải đảo kết quả (5+10=15, 15*2=30)");

const chiMotHam = pipe(3, nhanDoi);
if (chiMotHam !== 6) throw new Error("pipe với đúng MỘT hàm vẫn phải hoạt động (3*2=6)");
```

:::hints
- kind: attention
  body: "Dùng reduce (đã học T4.2) trên mảng fns — MỖI bước áp fn lên giá trị tích luỹ acc, bắt đầu từ a."
- kind: strategy
  body: "fns.reduce((acc, fn) => fn(acc), a) — a là giá trị khởi đầu (initial value của reduce), mỗi fn ÁP LÊN acc, KHÔNG PHẢI ngược lại."
- kind: one-line
  body: "return fns.reduce((acc, fn) => fn(acc), a);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
pipe() — nhận giá trị NGAY, chạy tuần tự qua các hàm theo THỨ TỰ liệt
kê. "Run now" — biến đổi MỘT giá trị cụ thể.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`pipe` biến đổi MỘT giá trị NGAY. Nếu muốn DỰNG một CHUỖI biến đổi để
DÙNG LẠI nhiều lần (không phải chỉ một lần), làm sao?
::::

::::checkpoint{mastery=0.8}
::::
