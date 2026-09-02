---
id: ky-nghe-phan-mem.ddd.tap-quan-sat-giua-pipeline
title: "tap() — quan sát MỘT bước giữa pipeline, không đổi giá trị"
summary: "tap<T>(f: (x:T) => void): (x:T) => T — nhận một hàm side-effect, TRẢ VỀ giá trị NGUYÊN VẸN không đổi — chèn được vào GIỮA một pipe()/flow() để \"nhìn trộm\" giá trị mà không phá vỡ chuỗi biến đổi."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 22
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.tap]
requires: [ddd.flow]
concepts: [ddd.tap]
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
`pipe`/`flow` ghép các hàm BIẾN ĐỔI giá trị. Nếu chỉ muốn NHÌN giá trị
tại một bước — để debug — mà KHÔNG đổi gì cả, thì sao?
::::

::::explain{#tap}
`tap<T>(f: (x: T) => void): (x: T) => T` — nhận một hàm SIDE-EFFECT
(thường là `console.log` để debug), TRẢ VỀ giá trị NGUYÊN VẸN không
đổi. Chèn được vào GIỮA một `pipe()`/`flow()` để "nhìn trộm" giá trị
TẠI MỘT bước mà KHÔNG phá vỡ chuỗi biến đổi:

```typescript
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, f: (a: A) => B, g: (b: B) => C, h: (c: C) => D): D;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}

function tap<T>(f: (x: T) => void): (x: T) => T {
  return (x) => {
    f(x);
    return x;
  };
}

const nhanDoi = (x: number) => x * 2;
const congMuoi = (x: number) => x + 10;

const ketQua = pipe(
  5,
  nhanDoi,
  tap((x) => console.log(`sau nhân đôi: ${x}`)),
  congMuoi,
);
console.log(ketQua);
```

```text
sau nhân đôi: 10
20
```

`tap((x) => console.log(...))` chèn NGAY GIỮA `nhanDoi` và `congMuoi`
— dòng log `"sau nhân đôi: 10"` xuất hiện, NHƯNG kết quả CUỐI CÙNG
(`20`) VẪN đúng NHƯ THỂ `tap` chưa từng ở đó. `tap` KHÔNG "chiếm chỗ"
của một bước biến đổi — nó chỉ CHEN vào QUAN SÁT rồi TRẢ nguyên giá
trị lại.
::::

::::example{#tap-khong-doi-ket-qua}
Đối lập `map`/`flow` (ĐỔI giá trị): `tap` CHỈ quan sát, không đổi gì —
kết quả CUỐI CÙNG GIỐNG HỆT dù CÓ hay KHÔNG có `tap`:

```typescript title=readonly
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, f: (a: A) => B, g: (b: B) => C, h: (c: C) => D): D;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}
function tap<T>(f: (x: T) => void): (x: T) => T {
  return (x) => {
    f(x);
    return x;
  };
}
const nhanDoi = (x: number) => x * 2;
const congMuoi = (x: number) => x + 10;

const khongCoTap = pipe(5, nhanDoi, congMuoi);
const coTap = pipe(5, nhanDoi, tap((x) => console.log(`giá trị giữa: ${x}`)), congMuoi);

console.log(khongCoTap);
console.log(coTap);
console.log(khongCoTap === coTap);
```

```text title=readonly
giá trị giữa: 10
20
20
true
```

`khongCoTap` và `coTap` là HAI pipeline GIỐNG HỆT nhau về Ý NGHĨA (chỉ
khác `tap` có mặt hay không) — CẢ HAI ra `20`, `khongCoTap === coTap`
là `true`. `tap` là công cụ DEBUG AN TOÀN — thêm vào để QUAN SÁT, gỡ
bỏ đi mà KHÔNG lo phá vỡ logic pipeline.
::::

::::predict{#doan-tap-doi-gia-tri commitOnce}
```typescript
function tap<T>(f: (x: T) => void): (x: T) => T {
  return (x) => {
    f(x);
    return x;
  };
}

const ghiLog: number[] = [];
const tapGhi = tap<number>((x) => ghiLog.push(x * 100)); // f LÀM một việc "lạ" -- nhân 100 rồi ghi log

const ketQua = tapGhi(7);
console.log(ketQua);
console.log(ghiLog);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`7` rồi `[700]`
:::

:::opt
`700` rồi `[700]` — vì `f` đã TÍNH `x * 100`, và `tap` TRẢ VỀ đúng kết
quả CỦA `f` (không phải giá trị GỐC `x`)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `f` (`(x) => ghiLog.push(x * 100)`) THẬT
SỰ tính `x * 100` VÀ đẩy vào `ghiLog` — quan sát về việc `f` LÀM gì đó
đúng.

Chỗ lệch: `tap` LUÔN `return x` (giá trị GỐC, tham số ĐẦU VÀO của HÀM
`tap` trả về) — KHÔNG BAO GIỜ trả kết quả của `f` (dù `f` tính gì bên
trong, kể cả `push` trả về ĐỘ DÀI mảng mới — ở đây `f` có kiểu `(x: T)
=> void`, KHÔNG hề định nghĩa để TRẢ giá trị nào cả). `tapGhi(7)`
LUÔN trả về `7` — nguyên vẹn, bất kể `f` bên trong làm gì.
::
:::

:::opt
Máy báo lỗi biên dịch — `f` có kiểu `(x: T) => void` (không trả gì)
nhưng thân hàm lại có `ghiLog.push(...)`, một biểu thức CÓ giá trị trả
về (`number`, độ dài mảng mới)
::why
Gần đúng ở việc bạn để ý `ghiLog.push(x * 100)` LÀ một biểu thức CÓ
giá trị (Array.push trả về độ dài mảng MỚI, kiểu `number`) — quan sát
về việc CÓ giá trị trả đó đúng.

Chỗ lệch: TypeScript cho phép một hàm khai `void` có THÂN chứa một
biểu thức TRẢ giá trị (miễn KHÔNG dùng từ khoá `return` với giá trị
đó) — giá trị `push` trả về đơn giản bị BỎ QUA, không gây lỗi kiểu.
`(x) => ghiLog.push(x * 100)` là một arrow function HỢP LỆ kiểu
`(x: number) => void` (thân biểu thức, giá trị trả về bị bỏ qua ngầm).
::
:::
::::

::::code{#viet_tap}
Tự viết PHẦN THÂN (implementation) của `tap`.

```typescript title=starter
function tap<T>(f: (x: T) => void): (x: T) => T {
  return ___;
}

const ghi: number[] = [];
const tapDem = tap<number>((x) => ghi.push(x));
console.log(tapDem(5));
console.log(ghi);
```

```typescript title=solution
function tap<T>(f: (x: T) => void): (x: T) => T {
  return (x) => {
    f(x);
    return x;
  };
}

const ghi: number[] = [];
const tapDem = tap<number>((x) => ghi.push(x));
console.log(tapDem(5));
console.log(ghi);
```

```typescript title=test
const nhatKy: string[] = [];
const tapGhiLog = tap<string>((s) => nhatKy.push(`đã thấy: ${s}`));

const ketQua1 = tapGhiLog("xin chao");
if (ketQua1 !== "xin chao") throw new Error("tap phải trả về ĐÚNG giá trị đầu vào, không đổi gì");
const soLuongSauLan1 = nhatKy.length;
if (soLuongSauLan1 !== 1) throw new Error("hàm f truyền vào phải được GỌI (side-effect phải xảy ra)");
if (nhatKy[0] !== "đã thấy: xin chao") throw new Error("hàm f phải nhận ĐÚNG giá trị x truyền vào");

const ketQua2 = tapGhiLog("lan hai");
if (ketQua2 !== "lan hai") throw new Error("gọi tap NHIỀU lần vẫn phải trả đúng giá trị mỗi lần");
const soLuongSauLan2 = nhatKy.length;
if (soLuongSauLan2 !== 2) throw new Error("side-effect phải TÍCH LUỸ qua nhiều lần gọi, không reset");
```

:::hints
- kind: attention
  body: "tap trả về MỘT HÀM MỚI nhận x: gọi f(x) để LÀM side-effect, RỒI return x (giá trị GỐC, không phải kết quả của f)."
- kind: strategy
  body: "(x) => { f(x); return x; } — hai dòng trong thân hàm mới: gọi f, rồi trả x nguyên vẹn."
- kind: one-line
  body: "return (x) => { f(x); return x; };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
tap(): chèn quan sát vào giữa pipeline mà KHÔNG đổi giá trị — công cụ
debug an toàn, thêm vào hay gỡ bỏ đều không ảnh hưởng logic.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có `pipe`, `flow`, `tap` — công cụ GHÉP hàm THUẦN. Nhưng bước nào
đó trong workflow CÓ THỂ LỖI thì sao — `pipe` bình thường xử lý được
không?
::::

::::checkpoint{mastery=0.8}
::::
