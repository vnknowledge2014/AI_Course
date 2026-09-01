---
id: lap-trinh-ham.adt-pattern-matching.assertnever-buoc-trinh-bien-dich-kiem
title: "`assertNever` — buộc TRÌNH BIÊN DỊCH tự kiểm đủ nhánh"
summary: "`function chuaXuLy(x: never): never { throw new Error(...) }` gọi ở nhánh `default` — nếu MỌI biến thể đã được xử lý ở các case trước, TypeScript thu hẹp kiểu của biến còn lại tại default thành `never`, và lời gọi `chuaXuLy(h)` biên dịch được. Thiếu MỘT case, biên dịch LỖI ngay."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.assert-never-pattern]
requires: [ts.missing-case-silent-bug]
concepts: [ts.assert-never-pattern]
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
Bài trước: quên một `case`, không có `default` — TypeScript biên dịch
BÌNH THƯỜNG, hàm chạy xong và lặng lẽ trả về `undefined`. Không cảnh
báo gì. Hôm nay: bắt TRÌNH BIÊN DỊCH tự kiểm giúp bạn điều đó.
::::

::::explain{#chuaxuly-va-kieu-never}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiac {
  kind: "tamGiac";
  day: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLy(x: never): never {
  throw new Error("Chưa xử lý biến thể: " + JSON.stringify(x));
}

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    case "tamGiac":
      return (h.day * h.chieuCao) / 2;
    default:
      return chuaXuLy(h);
  }
}

console.log(dienTich({ kind: "vuong", canh: 4 }));
console.log(dienTich({ kind: "tron", banKinh: 10 }));
console.log(dienTich({ kind: "tamGiac", day: 6, chieuCao: 4 }));
```

```text
16
314
12
```

Ba `case` (`"vuong"`, `"tron"`, `"tamGiac"`) đã xử lý ĐỦ ba biến thể
của `Hinh`. Đến nhánh `default`, TypeScript làm một việc đáng chú ý:
nó THU HẸP kiểu của `h` — không còn là `Hinh` nữa, mà thành `never`.

`never` là một kiểu ĐẶC BIỆT, nghĩa là "không còn khả năng nào" — một
giá trị THẬT không bao giờ mang kiểu này. TypeScript suy luận: nếu
`h.kind` KHÔNG khớp `"vuong"`, KHÔNG khớp `"tron"`, KHÔNG khớp
`"tamGiac"` — thì xét trên KIỂU đã khai của `Hinh`, không còn hình
dạng nào khác có thể lọt tới đây. Nên `h` tại `default` mang kiểu
`never`.

Hàm `chuaXuLy(x: never): never` chỉ NHẬN tham số kiểu `never` — và `h`
tại đây ĐÚNG LÀ `never`, nên `chuaXuLy(h)` BIÊN DỊCH ĐƯỢC. Lúc CHẠY,
nhánh `default` không bao giờ THỰC SỰ chạy tới (mọi giá trị hợp lệ đều
khớp một `case` phía trên) — `chuaXuLy` tồn tại chủ yếu như một LỜI
CHỨNG MINH cho trình biên dịch, không phải logic nghiệp vụ.
::::

::::example{#tai-su-dung-chuaxuly}
`chuaXuLy` không hề "biết" gì về `Hinh` — chữ ký của nó chỉ đòi tham
số kiểu `never`. Nên nó DÙNG LẠI được cho BẤT KỲ `switch` cạn kiệt
nào, trên BẤT KỲ union nào khác:

```typescript title=readonly
interface DonChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DonDaGiao {
  kind: "daGiao";
  ma: string;
}

interface DonDaHuy {
  kind: "daHuy";
  ma: string;
  lyDoHuy: string;
}

type Don = DonChoXuLy | DonDaGiao | DonDaHuy;

function chuaXuLy(x: never): never {
  throw new Error("Chưa xử lý biến thể: " + JSON.stringify(x));
}

function moTaDon(d: Don): string {
  switch (d.kind) {
    case "choXuLy":
      return "đơn " + d.ma + " đang chờ xử lý";
    case "daGiao":
      return "đơn " + d.ma + " đã giao";
    case "daHuy":
      return "đơn " + d.ma + " đã huỷ: " + d.lyDoHuy;
    default:
      return chuaXuLy(d);
  }
}

console.log(moTaDon({ kind: "choXuLy", ma: "DH-1" }));
console.log(moTaDon({ kind: "daHuy", ma: "DH-2", lyDoHuy: "hết hàng" }));
```

```text title=readonly
đơn DH-1 đang chờ xử lý
đơn DH-2 đã huỷ: hết hàng
```

Ba `case` xử lý đủ ba biến thể của `Don` — `d` tại `default` cũng bị
thu hẹp thành `never`, `chuaXuLy(d)` biên dịch được, y hệt cơ chế vừa
thấy với `Hinh`. `chuaXuLy` là một HÀM DÙNG CHUNG cho MỌI `switch` cạn
kiệt trong dự án, không phải thứ gắn riêng cho một union nào.
::::

::::predict{#doan-thieu-mot-case commitOnce}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

type Hinh = Vuong | Tron;

function chuaXuLy(x: never): never {
  throw new Error("Chưa xử lý biến thể: " + JSON.stringify(x));
}

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    default:
      return chuaXuLy(h);
  }
}

console.log(dienTich({ kind: "tron", banKinh: 5 }));
```

Đoạn này CHỈ xử lý `case "vuong"`, thiếu hẳn `case "tron"`. Dòng cuối
làm gì?

:::opt{correct}
Máy báo lỗi biên dịch — mã TS2345: kiểu `Tron` không gán được cho
tham số kiểu `never`
:::

:::opt
Trả về `undefined` — vì `switch` không xử lý `case "tron"`, giống hệt
lỗi ở bài trước
::why
Gần đúng ở việc bạn nhớ ĐÚNG lỗi bài trước: `switch` thiếu case,
không `default`, rơi ra ngoài mà trả `undefined` — đó LÀ một lỗi có
thật, từng gặp.

Chỗ lệch: `switch` NÀY CÓ `default`, gọi `chuaXuLy(h)`. Vì `case
"tron"` chưa xử lý, `h` tại `default` KHÔNG được thu hẹp về `never` —
nó VẪN là `Tron`. `Tron` không gán được cho tham số kiểu `never`,
TypeScript CHẶN NGAY LÚC BIÊN DỊCH (TS2345) — chương trình KHÔNG chạy
để trả về bất cứ giá trị nào, kể cả `undefined`.
::
:::

:::opt
Biên dịch được, chương trình CHẠY, và dòng cuối NÉM lỗi runtime từ
bên trong `chuaXuLy` — vì `"tron"` không khớp case nào
::why
Gần đúng ở việc bạn hiểu ĐÚNG hành vi CỦA `chuaXuLy` nếu nó THỰC SỰ bị
gọi lúc chạy — hàm có `throw new Error(...)` thật, sẽ ném lỗi nếu
chạy tới đó.

Chỗ lệch: để CHẠY tới đó, chương trình phải BIÊN DỊCH được trước. `h`
tại `default` ở đây VẪN mang kiểu `Tron` (không phải `never`, vì
thiếu `case "tron"`) — `chuaXuLy(h)` VI PHẠM chữ ký hàm ngay trên
GIẤY, TypeScript từ chối biên dịch (TS2345), không có dòng nào được
thực thi.
::
:::
::::

::::code{#dat_ten_hinh}
Viết nhánh `default` của `tenHinh` — gọi `chuaXuLy` với đúng biến
`h`, để TypeScript tự xác nhận `switch` đã xử lý ĐỦ ba biến thể.

```typescript title=starter
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiac {
  kind: "tamGiac";
  day: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLy(x: never): never {
  throw new Error("Chưa xử lý biến thể: " + JSON.stringify(x));
}

function tenHinh(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "vuông";
    case "tron":
      return "tròn";
    case "tamGiac":
      return "tam giác";
    default:
      return ___;
  }
}

console.log(tenHinh({ kind: "vuong", canh: 4 }));
```

```typescript title=solution
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiac {
  kind: "tamGiac";
  day: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLy(x: never): never {
  throw new Error("Chưa xử lý biến thể: " + JSON.stringify(x));
}

function tenHinh(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "vuông";
    case "tron":
      return "tròn";
    case "tamGiac":
      return "tam giác";
    default:
      return chuaXuLy(h);
  }
}

console.log(tenHinh({ kind: "vuong", canh: 4 }));
```

```typescript title=test
const s1 = tenHinh({ kind: "vuong", canh: 4 });
if (s1 !== "vuông") throw new Error("tenHinh cho vuong phải trả về \"vuông\" — đang là " + s1);
const s2 = tenHinh({ kind: "tron", banKinh: 5 });
if (s2 !== "tròn") throw new Error("tenHinh cho tron phải trả về \"tròn\" — đang là " + s2);
const s3 = tenHinh({ kind: "tamGiac", day: 3, chieuCao: 4 });
if (s3 !== "tam giác") throw new Error("tenHinh cho tamGiac phải trả về \"tam giác\" — đang là " + s3);

let nemLoi = false;
try {
  tenHinh({ kind: "khac" } as any);
} catch (e) {
  nemLoi = true;
}
if (!nemLoi) throw new Error("tenHinh phải NÉM lỗi khi gặp một kind lạ, không được âm thầm trả về giá trị nào khác");
```

:::hints
- kind: attention
  body: "Chỗ trống là biểu thức return của nhánh default — phải gọi chuaXuLy với đúng biến h, không phải trả về một literal string."
- kind: strategy
  body: "Ba case phía trên đã xử lý đủ ba biến thể của Hinh — tại default, kiểu của h bị TypeScript thu hẹp còn never, khớp đúng tham số mà chuaXuLy(x: never) đòi hỏi."
- kind: one-line
  body: "Chỗ trống là: chuaXuLy(h)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "vuông"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`chuaXuLy(h)` biên dịch được — nghĩa là TypeScript vừa TỰ XÁC NHẬN
`switch` xử lý đủ mọi biến thể, không cần bạn tự đếm case bằng mắt.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`assertNever` đang bảo vệ `dienTich` — ĐÚNG lúc này, với ĐÚNG ba biến
thể mà `Hinh` đang có. Nhưng nếu vài tuần sau, ai đó thêm một biến
thể THỨ TƯ vào `Hinh` — mà QUÊN thêm `case` xử lý nó — thì sao?
`assertNever` đã nằm sẵn ở nhánh `default`. Nó có LÊN TIẾNG không?
::::

::::checkpoint{mastery=0.8}
::::
