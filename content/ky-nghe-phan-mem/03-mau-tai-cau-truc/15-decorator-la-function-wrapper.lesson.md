---
id: ky-nghe-phan-mem.mau-tai-cau-truc.decorator-la-function-wrapper
title: "Decorator = Function Wrapper — bọc hành vi, không sửa hàm gốc"
summary: "OOP Decorator: một class BỌC một object khác, THÊM hành vi TRƯỚC/SAU khi gọi object gốc. FP: \"bọc\" một hàm CHỈ LÀ viết một hàm MỚI GỌI hàm gốc BÊN TRONG — voiGhiLog<A,B>(f) trả về hàm MỚI: log TRƯỚC gọi f, log SAU khi có kết quả, TRẢ NGUYÊN kết quả. Hàm GỐC KHÔNG đổi MỘT dòng — gọi TRỰC TIẾP vẫn hoạt động y hệt như TRƯỚC, không log."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.decorator-as-wrapper]
requires: [mau.add-operation-no-touch-adt]
concepts: [mau.decorator-as-wrapper]
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
Muốn GHI LOG mỗi lần MỘT hàm được gọi — mà KHÔNG sửa THÂN hàm đó
(có THỂ hàm đó dùng Ở NHIỀU nơi, sửa SẼ rủi ro). Làm sao?
::::

::::explain{#boc-ham-khong-sua-goc}
OOP Decorator: MỘT class BỌC một object khác, implement CÙNG
interface, THÊM hành vi TRƯỚC/SAU khi gọi object GỐC (ví dụ:
`LoggingDecorator` bọc `RealService`).

FP: "bọc" MỘT hàm CHỈ LÀ viết MỘT hàm **MỚI** GỌI hàm gốc Ở BÊN
TRONG — hàm GỐC **KHÔNG đổi MỘT dòng nào**:

```typescript title=readonly
function binhPhuong(n: number): number {
  return n * n;
}

function voiGhiLog<A, B>(f: (a: A) => B): (a: A) => B {
  return (a: A) => {
    console.log("goi ham voi:", a);
    const ketQua = f(a);
    console.log("ket qua:", ketQua);
    return ketQua;
  };
}

const binhPhuongCoLog = voiGhiLog(binhPhuong);
const kq = binhPhuongCoLog(5);
console.log("gia tri tra ve:", kq);
```

```text title=readonly
goi ham voi: 5
ket qua: 25
gia tri tra ve: 25
```

`voiGhiLog<A,B>(f)` NHẬN một hàm BẤT KỲ (`(a:A)=>B`), TRẢ VỀ một hàm
**MỚI CÙNG chữ ký** — log TRƯỚC khi gọi `f`, log SAU khi CÓ kết quả,
`return ketQua` **NGUYÊN VẸN** (KHÔNG đổi giá trị). `binhPhuong` GỐC
KHÔNG hề bị SỬA — `binhPhuongCoLog` LÀ một GIÁ TRỊ hàm **MỚI, RIÊNG**.
::::

::::example{#ham-goc-khong-anh-huong}
Vì `binhPhuong` KHÔNG BỊ SỬA, gọi TRỰC TIẾP `binhPhuong` (KHÔNG qua
`binhPhuongCoLog`) VẪN hoạt động Y HỆT như TRƯỚC — KHÔNG log gì cả:

```typescript title=readonly
function binhPhuong(n: number): number { return n * n; }
function voiGhiLog<A,B>(f:(a:A)=>B):(a:A)=>B {
  return (a:A) => {
    console.log("goi ham voi:", a);
    const ketQua = f(a);
    console.log("ket qua:", ketQua);
    return ketQua;
  };
}
const binhPhuongCoLog = voiGhiLog(binhPhuong);
console.log(binhPhuong(3));
console.log(binhPhuongCoLog(3));
```

```text title=readonly
9
goi ham voi: 3
ket qua: 9
9
```

Dòng ĐẦU (`binhPhuong(3)`) in THẲNG `9`, KHÔNG log gì — `binhPhuong`
VẪN LÀ chính nó, KHÔNG bị "nhiễm" hành vi CỦA `voiGhiLog`. Dòng SAU
(`binhPhuongCoLog(3)`) MỚI có log, VÌ `binhPhuongCoLog` LÀ một hàm
KHÁC, RIÊNG BIỆT — được TẠO RA bằng cách BỌC `binhPhuong`.
::::

::::predict{#doan-goc-khong-nhiem-hanh-vi commitOnce}
```typescript
function nhanBa(n: number): number { return n * 3; }
function voiGhiLog<A,B>(f:(a:A)=>B):(a:A)=>B {
  return (a:A) => {
    console.log("goi ham voi:", a);
    const ketQua = f(a);
    console.log("ket qua:", ketQua);
    return ketQua;
  };
}
const nhanBaCoLog = voiGhiLog(nhanBa);
nhanBaCoLog(2);
console.log(nhanBa(10));
```

Ba dòng in ra gì?

:::opt{correct}
`goi ham voi: 2` rồi `ket qua: 6` rồi `30`
:::

:::opt
`goi ham voi: 2` rồi `ket qua: 6` rồi `goi ham voi: 10` rồi
`ket qua: 30` — vì gọi `voiGhiLog(nhanBa)` MỘT LẦN đã "GẮN" hành vi
ghi log VÀO `nhanBa` VĨNH VIỄN, nên MỌI lời gọi `nhanBa` SAU ĐÓ (dù
TRỰC TIẾP, không qua `nhanBaCoLog`) ĐỀU tự động có log
::why
Gần đúng ở việc bạn nhớ ĐÚNG `voiGhiLog(nhanBa)` ĐƯỢC gọi Ở đầu chương
trình — quan sát ĐÓ về TRÌNH TỰ code đúng.

Chỗ lệch: `voiGhiLog(nhanBa)` KHÔNG "GẮN" gì vào `nhanBa` cả — nó CHỈ
**ĐỌC** `nhanBa` (như một tham số) VÀ TRẢ VỀ MỘT giá trị hàm HOÀN
TOÀN MỚI (`nhanBaCoLog`), KHÔNG hề ĐỘNG tới `nhanBa`. `nhanBa` (biến
GỐC) VẪN TRỎ tới hàm BAN ĐẦU, KHÔNG đổi — gọi `nhanBa(10)` TRỰC TIẾP
CHỈ chạy ĐÚNG thân hàm GỐC (`return n * 3`), KHÔNG log gì, in THẲNG
`30`.
::
:::

:::opt
Máy báo lỗi biên dịch — `voiGhiLog<A,B>` khai HAI tham số kiểu generic
(`A`, `B`) nhưng lời gọi `voiGhiLog(nhanBa)` KHÔNG chỉ định generic
TƯỜNG MINH (`voiGhiLog<number,number>(nhanBa)`), TypeScript ĐÒI khai
generic RÕ RÀNG khi hàm CÓ nhiều hơn MỘT tham số kiểu
::why
Gần đúng ở việc bạn để ý `voiGhiLog` khai HAI tham số kiểu (`<A, B>`)
— một quan sát ĐÚNG về CHỮ KÝ hàm.

Chỗ lệch: TypeScript **SUY LUẬN** (infer) CẢ HAI tham số kiểu TỪ kiểu
của `nhanBa` (`(n: number) => number` → `A = number`, `B = number`)
— KHÔNG BAO GIỜ bắt buộc khai generic tường minh khi suy luận được.
Đây LÀ hành vi CHUẨN, dùng THƯỜNG XUYÊN với generic hàm — biên dịch
SẠCH.
::
:::
::::

::::code{#viet_voi_ghi_log}
Hoàn thiện `voiGhiLog` — trả về hàm mới: log TRƯỚC, gọi `f`, log SAU,
trả kết quả NGUYÊN VẸN.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function voiGhiLog<A, B>(f: (a: A) => B, nhatKy: string[]): (a: A) => B {
  return (a: A) => {
    nhatKy.push(`goi voi ${a}`);
    const ketQua = ___;
    nhatKy.push(`ket qua ${ketQua}`);
    return ___;
  };
}

function nhanDoi(n: number): number {
  return n * 2;
}

const log1: string[] = [];
const nhanDoiCoLog = voiGhiLog(nhanDoi, log1);
const kq1 = nhanDoiCoLog(4);
assertEqual(kq1, 8, "ket qua tra ve dung");
assertEqual(log1.length, 2, "hai dong nhat ky duoc ghi");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

function voiGhiLog<A, B>(f: (a: A) => B, nhatKy: string[]): (a: A) => B {
  return (a: A) => {
    nhatKy.push(`goi voi ${a}`);
    const ketQua = f(a);
    nhatKy.push(`ket qua ${ketQua}`);
    return ketQua;
  };
}

function nhanDoi(n: number): number {
  return n * 2;
}

const log1: string[] = [];
const nhanDoiCoLog = voiGhiLog(nhanDoi, log1);
const kq1 = nhanDoiCoLog(4);
assertEqual(kq1, 8, "ket qua tra ve dung");
assertEqual(log1.length, 2, "hai dong nhat ky duoc ghi");
```

```typescript title=test
assertEqual(log1[0], "goi voi 4", "dong nhat ky dau ghi dung tham so");
assertEqual(log1[1], "ket qua 8", "dong nhat ky sau ghi dung ket qua");

assertEqual(nhanDoi(100), 200, "ham goc KHONG bi anh huong, goi truc tiep van dung");

const log2: string[] = [];
const nhanDoiCoLog2 = voiGhiLog(nhanDoi, log2);
nhanDoiCoLog2(50);
assertEqual(log2.length, 2, "nhat ky rieng cho lan boc thu hai");
assertEqual(log1.length, 2, "nhat ky lan dau KHONG bi anh huong boi lan boc thu hai");
```

:::hints
- kind: attention
  body: "ketQua: gọi f (tham số hàm gốc) với a. return: trả NGUYÊN ketQua vừa tính, không đổi giá trị."
- kind: strategy
  body: "f(a) : ketQua — gọi hàm gốc với tham số, và trả nguyên kết quả đã tính."
- kind: one-line
  body: '___ (ketQua) = f(a)\n___ (return) = ketQua'
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
Decorator = hàm mới bọc hàm cũ, hàm gốc KHÔNG đổi. Bài chốt cụm: kết
hợp Visitor (duyệt cây) VÀ Decorator (đếm lượt gọi) trong MỘT bài.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`voiGhiLog` bọc MỘT hàm BẤT KỲ (generic `<A,B>`). `tinhGiaTri`
(bài 13, `(bt: BieuThuc) => number`) có PHẢI MỘT hàm khớp chữ ký ĐÓ
không? Bọc `tinhGiaTri` bằng một decorator ĐẾM số lần gọi — bạn hình
dung được chưa?
::::

::::checkpoint{mastery=0.8}
::::
