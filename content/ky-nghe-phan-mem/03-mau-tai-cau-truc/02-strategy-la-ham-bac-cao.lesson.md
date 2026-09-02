---
id: ky-nghe-phan-mem.mau-tai-cau-truc.strategy-la-ham-bac-cao
title: "Strategy = Higher-Order Function — đổi thuật toán bằng cách TRUYỀN hàm"
summary: "OOP Strategy: interface + một class riêng cho MỖI thuật toán. FP: thuật toán CHÍNH LÀ một giá trị hàm — đổi chiến lược = truyền hàm khác làm tham số. giamCoDinh/giamPhanTram là NHÀ MÁY trả về strategy (closure bắt giữ tham số riêng), tinhGiaCuoiCung nhận strategy làm tham số."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.strategy-as-hof]
requires: [mau.why-patterns-vanish]
concepts: [mau.strategy-as-hof]
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
Cửa hàng có BA cách giảm giá: không giảm, giảm SỐ TIỀN cố định, giảm
PHẦN TRĂM. OOP cần ba class. FP cần gì?
::::

::::explain{#oop-vs-fp-strategy}
OOP Strategy cần MỘT `interface` (`SortStrategy { sort(...) }`) VÀ
MỘT class RIÊNG cho MỖI thuật toán (`BubbleSort`, `QuickSort`) — MỘT
class chứa object giữ THAM CHIẾU tới strategy ĐANG dùng, ĐỔI strategy
nghĩa LÀ gán MỘT object KHÁC.

FP: thuật toán "giảm giá" CHỈ LÀ một GIÁ TRỊ có kiểu
`(gia: number) => number` — KHÔNG interface, KHÔNG class. VÀ vì hàm
LÀ giá trị, MỘT hàm CÓ THỂ **TRẢ VỀ** một hàm KHÁC (đóng vai "nhà
máy" tạo strategy, giữ tham số RIÊNG qua closure):

```typescript title=readonly
type ChienLuocGiamGia = (gia: number) => number;

function khongGiamGia(gia: number): number {
  return gia;
}

function giamCoDinh(soTienGiam: number): ChienLuocGiamGia {
  return (gia) => Math.max(0, gia - soTienGiam);
}

function giamPhanTram(phanTram: number): ChienLuocGiamGia {
  return (gia) => gia * (1 - phanTram / 100);
}

function tinhGiaCuoiCung(gia: number, chienLuoc: ChienLuocGiamGia): number {
  return chienLuoc(gia);
}

console.log(tinhGiaCuoiCung(100, khongGiamGia));
console.log(tinhGiaCuoiCung(100, giamCoDinh(30)));
console.log(tinhGiaCuoiCung(100, giamPhanTram(20)));
```

```text title=readonly
100
70
80
```

`giamCoDinh`/`giamPhanTram` KHÔNG PHẢI strategy — chúng LÀ **NHÀ MÁY**
TẠO strategy: gọi `giamCoDinh(30)` TRẢ VỀ một hàm `(gia) => Math.max(0,
gia - 30)` MỚI, hàm ĐÓ **NHỚ** `soTienGiam = 30` qua closure (KHÔNG
cần field object nào lưu `30` cả). `tinhGiaCuoiCung` KHÔNG BIẾT VÀ
KHÔNG CẦN BIẾT strategy được TẠO RA thế nào — nó CHỈ gọi.
::::

::::example{#nha-may-tao-hai-strategy-doc-lap}
MỖI lời gọi `giamCoDinh(...)` tạo một CLOSURE **RIÊNG**, nhớ tham số
CỦA CHÍNH NÓ — hai strategy được tạo KHÔNG "chia sẻ" gì cả:

```typescript title=readonly
type ChienLuocGiamGia = (gia: number) => number;
function giamCoDinh(soTienGiam: number): ChienLuocGiamGia {
  return (gia) => Math.max(0, gia - soTienGiam);
}

const giam10 = giamCoDinh(10);
const giam50 = giamCoDinh(50);

console.log(giam10(100));
console.log(giam50(100));
```

```text title=readonly
90
50
```

`giam10` VÀ `giam50` LÀ HAI GIÁ TRỊ HÀM ĐỘC LẬP — gọi `giam10` KHÔNG
đụng chạm gì tới `giam50`, dù CẢ HAI được tạo TỪ CÙNG một nhà máy
`giamCoDinh`. Đây LÀ closure ĐÚNG NGHĨA: `soTienGiam` được "đóng gói"
RIÊNG cho MỖI lời gọi nhà máy.
::::

::::predict{#doan-math-max-chan-am commitOnce}
```typescript
type ChienLuocGiamGia = (gia: number) => number;
function giamCoDinh(soTienGiam: number): ChienLuocGiamGia {
  return (gia) => Math.max(0, gia - soTienGiam);
}

const giam10 = giamCoDinh(10);
console.log(giam10(100));
console.log(giam10(5));
```

Hai dòng in ra gì?

:::opt{correct}
`90` rồi `0`
:::

:::opt
`90` rồi `-5` — vì `giam10(5)` tính `5 - 10 = -5`, VÀ `Math.max(0, -5)`
chỉ ẢNH HƯỞNG khi tham số ĐẦU LÀ số LỚN HƠN, ở đây `0` nhỏ HƠN `-5`
nên `Math.max` KHÔNG đổi kết quả
::why
Gần đúng ở việc bạn tính ĐÚNG `5 - 10 = -5` — phép TRỪ bên trong đúng.

Chỗ lệch: SO SÁNH `0` với `-5` NGƯỢC chiều — `0` **LỚN HƠN** `-5`
(không phải nhỏ hơn), VÀ `Math.max(a, b)` trả về giá trị **LỚN NHẤT**
trong hai tham số. `Math.max(0, -5)` so `0` VÀ `-5`, thấy `0 > -5`,
trả về `0`. Đây CHÍNH LÀ lý do `Math.max(0, ...)` xuất hiện Ở đây: NÓ
LÀ một "kẹp dưới" (clamp) NGĂN giá sau giảm rơi XUỐNG ÂM.
::
:::

:::opt
Máy báo lỗi lúc chạy — `giamCoDinh` khai kiểu trả về `ChienLuocGiamGia`
nhưng THÂN hàm trả về MỘT hàm (arrow function), KHÔNG PHẢI MỘT
`number`, nên gọi `giamCoDinh(10)` sẽ ném lỗi kiểu SAI NGAY LÚC CHẠY
::why
Gần đúng ở việc bạn để ý `ChienLuocGiamGia = (gia: number) => number`
LÀ một kiểu HÀM, KHÔNG PHẢI kiểu SỐ — một quan sát ĐÚNG về ĐỊNH NGHĨA
kiểu.

Chỗ lệch: `ChienLuocGiamGia` **CHÍNH LÀ** kiểu "một hàm nhận number,
trả number" — `giamCoDinh` khai TRẢ VỀ **ĐÚNG KIỂU ĐÓ** (một hàm),
KHÔNG PHẢI một `number`. `(gia) => Math.max(0, gia - soTienGiam)` LÀ
MỘT giá trị hàm, khớp CHÍNH XÁC `ChienLuocGiamGia` — biên dịch VÀ chạy
đều SẠCH, không lỗi nào cả.
::
:::
::::

::::code{#viet_strategy}
Hoàn thiện `giamPhanTram` (nhà máy strategy giảm PHẦN TRĂM) VÀ
`tinhGiaCuoiCung` (áp dụng strategy được truyền vào).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type ChienLuocGiamGia = (gia: number) => number;

function giamCoDinh(soTienGiam: number): ChienLuocGiamGia {
  return (gia) => Math.max(0, gia - soTienGiam);
}

function giamPhanTram(phanTram: number): ChienLuocGiamGia {
  return (gia) => ___;
}

function tinhGiaCuoiCung(gia: number, chienLuoc: ChienLuocGiamGia): number {
  return ___;
}

assertEqual(tinhGiaCuoiCung(200, giamPhanTram(25)), 150, "giam 25% tren 200");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type ChienLuocGiamGia = (gia: number) => number;

function giamCoDinh(soTienGiam: number): ChienLuocGiamGia {
  return (gia) => Math.max(0, gia - soTienGiam);
}

function giamPhanTram(phanTram: number): ChienLuocGiamGia {
  return (gia) => gia * (1 - phanTram / 100);
}

function tinhGiaCuoiCung(gia: number, chienLuoc: ChienLuocGiamGia): number {
  return chienLuoc(gia);
}

assertEqual(tinhGiaCuoiCung(200, giamPhanTram(25)), 150, "giam 25% tren 200");
```

```typescript title=test
assertEqual(tinhGiaCuoiCung(80, giamCoDinh(15)), 65, "giam co dinh 15 tren 80");
assertEqual(tinhGiaCuoiCung(50, giamPhanTram(0)), 50, "giam 0% khong doi gia");
assertEqual(tinhGiaCuoiCung(300, giamCoDinh(50)), 250, "giam co dinh 50 tren 300");
assertEqual(tinhGiaCuoiCung(10, giamCoDinh(999)), 0, "giam co dinh vuot qua gia -- Math.max chan am");
```

:::hints
- kind: attention
  body: "giamPhanTram: gia CÒN LẠI sau khi trừ đi ĐÚNG phanTram phần trăm của chính nó. tinhGiaCuoiCung: chienLuoc LÀ một HÀM được truyền vào — gọi nó với gia."
- kind: strategy
  body: "gia * (1 - phanTram / 100) : chienLuoc(gia) — công thức phần trăm còn lại, và GỌI tham số hàm như một hàm bình thường."
- kind: one-line
  body: '___ (giamPhanTram) = gia * (1 - phanTram / 100)\n___ (tinhGiaCuoiCung) = chienLuoc(gia)'
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
Strategy = hàm nhận LÀM tham số. Nhà máy strategy = hàm TRẢ VỀ hàm,
giữ tham số riêng qua closure. Bài tiếp theo: khi nào ĐIỀU NÀY KHÔNG
đủ — vẫn cần interface.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinhGiaCuoiCung` nhận MỘT hàm duy nhất làm strategy. Nếu một "chiến
lược" cần NHIỀU hơn một hành vi (ví dụ: vừa `ket_noi()` vừa `dong()`,
chia sẻ CÙNG một kết nối) — MỘT hàm đơn còn đủ không?
::::

::::checkpoint{mastery=0.8}
::::
