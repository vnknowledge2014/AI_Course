---
id: lap-trinh-ham.adt-pattern-matching.narrow-bang-if-tren-discriminant
title: "Thu hẹp kiểu bằng `if` trên trường nhận diện"
summary: "`if (h.kind === \"vuong\") { h.canh }` — bên TRONG nhánh if, TypeScript đã biết h CHẮC CHẮN là Vuong nhờ kind có kiểu literal, cho phép đọc h.canh an toàn dù Hinh là union. Không phải phép màu — là kết quả trực tiếp của so sánh giá trị trên field literal."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ts.discriminant-if-narrowing]
requires: [ts.discriminant-field]
concepts: [ts.discriminant-if-narrowing]
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
Bài trước hỏi: có cách nào TẬN DỤNG trực tiếp nhãn `kind`, thay vì soi
field nào có mặt như `in`? Có — so sánh thẳng `kind` với đúng chuỗi
của biến thể.
::::

::::explain{#so-sanh-truc-tiep-tren-kind}
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

function moTa(h: Hinh): string {
  if (h.kind === "vuong") {
    return "hình vuông cạnh " + h.canh;
  }
  return "hình tròn bán kính " + h.banKinh;
}

console.log(moTa({ kind: "vuong", canh: 4 }));
console.log(moTa({ kind: "tron", banKinh: 3 }));
```

```text
hình vuông cạnh 4
hình tròn bán kính 3
```

Thay vì `"canh" in h` (soi field nào CÓ MẶT — bài trước), giờ so sánh
TRỰC TIẾP nhãn: `h.kind === "vuong"`. Ngay bên TRONG nhánh `if` đó,
TypeScript đã biết `h` CHẮC CHẮN là `Vuong` — dòng `return "hình vuông
cạnh " + h.canh` đọc `h.canh` mà KHÔNG báo lỗi, dù `h` được khai là
`Hinh` (union của cả `Vuong` LẪN `Tron`).

Đây KHÔNG phải phép màu. `Vuong.kind` có KIỂU LITERAL `"vuong"` (đã học
bài trước) — nghĩa là trong `Hinh`, CHỈ CÓ MỘT biến thể mà `kind` có
thể bằng đúng chuỗi `"vuong"`. Khi phép so sánh `h.kind === "vuong"`
trả `true`, TypeScript LOẠI TRỪ mọi biến thể khác khỏi khả năng — chỉ
còn `Vuong` khớp. Bước loại trừ này gọi là "narrowing" (thu hẹp kiểu),
và nó CHỈ có hiệu lực TRONG PHẠM VI nhánh `if` — ra khỏi nhánh, `h`
quay lại kiểu `Hinh` rộng như cũ.
::::

::::example{#hai-nhanh-deu-duoc-thu-hep}
```typescript title=readonly
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

type Hinh = Vuong | Tron;

function dienTich(h: Hinh): number {
  if (h.kind === "vuong") {
    return h.canh * h.canh;
  }
  return Math.PI * h.banKinh * h.banKinh;
}

console.log(dienTich({ kind: "vuong", canh: 4 }));
console.log(dienTich({ kind: "tron", banKinh: 2 }));
```

```text title=readonly
16
12.566370614359172
```

Nhánh `if` thu hẹp `h` thành `Vuong` — đọc `h.canh` được. Nhưng phần
CÒN LẠI sau `if` (ở đây viết ngầm bằng `return` thứ hai, không có
`else` tường minh) CŨNG được thu hẹp — theo CHIỀU NGƯỢC LẠI. `Hinh` chỉ
có ĐÚNG hai biến thể; nếu `h.kind` KHÔNG bằng `"vuong"`, phần còn lại
duy nhất là `Tron`. Vì vậy `h.banKinh` ở dòng cuối cũng không báo lỗi —
TypeScript loại trừ theo CẢ HAI chiều của phép so sánh, không chỉ
chiều `true`.
::::

::::predict{#tham-nham-truong-trong-nhanh commitOnce}
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

function moTa(h: Hinh): string {
  if (h.kind === "vuong") {
    return "canh " + h.banKinh;
  }
  return "ban kinh " + h.banKinh;
}

console.log(moTa({ kind: "vuong", canh: 4 }));
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi biên dịch — mã TS2339, vì bên TRONG nhánh
`if (h.kind === "vuong")`, `h` đã bị thu hẹp thành `Vuong`, không có
field `banKinh`
:::

:::opt
`canh 4` — vì `h` vẫn giữ kiểu `Hinh` (union), field nào có ở MỘT
trong hai biến thể cũng đọc được, TypeScript không phân biệt nhánh nào
::why
Gần đúng ở việc bạn nhớ `h` được khai là `Hinh` — một union của `Vuong`
LẪN `Tron`, đúng là kiểu GỐC của tham số hàm.

Chỗ lệch: BÊN TRONG nhánh `if (h.kind === "vuong")`, `h` KHÔNG còn là
`Hinh` rộng nữa — TypeScript đã thu hẹp nó xuống ĐÚNG `Vuong` (như
explain đã học). `Vuong` không CÓ field `banKinh` — nên `h.banKinh`
trong nhánh này bị chặn ngay, không liên quan gì tới việc `Hinh` gốc
có `banKinh` ở biến thể kia.
::
:::

:::opt
`canh undefined` — object `{ kind: "vuong", canh: 4 }` không có field
`banKinh`, JavaScript truy cập field không tồn tại trả về `undefined`,
không lỗi gì
::why
Gần đúng ở việc bạn nhớ đúng ngữ nghĩa JAVASCRIPT thuần: truy cập field
không tồn tại trên một object trả `undefined`, không throw exception —
đúng, NẾU mã có cơ hội CHẠY tới đó.

Chỗ lệch: mã này không CHẠY tới bước đó. TypeScript CHẶN NGAY ở bước
BIÊN DỊCH (mã TS2339) — sau khi thu hẹp `h` thành `Vuong` trong nhánh
`if`, trình biên dịch biết chắc `Vuong` không có `banKinh` và từ chối
biên dịch, không đợi tới lúc chạy để lộ ra `undefined`.
::
:::
::::

::::code{#dien_tich_theo_kind}
Viết `dienTich(h)` — dùng đúng nhãn `kind` để chọn công thức: cạnh
bình phương nếu là `Vuong`, `Math.PI * banKinh * banKinh` nếu là
`Tron`.

```typescript title=starter
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

type Hinh = Vuong | Tron;

function dienTich(h: Hinh): number {
  if (h.kind === ___) {
    return h.canh * h.canh;
  }
  return Math.PI * h.banKinh * h.banKinh;
}

console.log(dienTich({ kind: "vuong", canh: 4 }));
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

type Hinh = Vuong | Tron;

function dienTich(h: Hinh): number {
  if (h.kind === "vuong") {
    return h.canh * h.canh;
  }
  return Math.PI * h.banKinh * h.banKinh;
}

console.log(dienTich({ kind: "vuong", canh: 4 }));
```

```typescript title=test
const dt1 = dienTich({ kind: "vuong", canh: 4 });
if (dt1 !== 16) throw new Error("dienTich hình vuông cạnh 4 phải là 16 — đang là " + dt1);
const dt2 = dienTich({ kind: "tron", banKinh: 2 });
const kyVong2 = Math.PI * 2 * 2;
if (Math.abs(dt2 - kyVong2) > 0.0001) throw new Error("dienTich hình tròn bán kính 2 phải xấp xỉ " + kyVong2 + " — đang là " + dt2);
const dt3 = dienTich({ kind: "vuong", canh: 7 });
if (dt3 !== 49) throw new Error("dienTich hình vuông cạnh 7 phải là 49 — đang là " + dt3);
```

:::hints
- kind: attention
  body: "Chỗ trống là giá trị so sánh với h.kind — Vuong khai kind: \"vuong\" (kiểu literal), phải so đúng chuỗi đó."
- kind: strategy
  body: 'So sánh h.kind với đúng chuỗi literal mà interface Vuong khai cho field kind — khớp thì TypeScript thu hẹp h thành Vuong trong nhánh if.'
- kind: one-line
  body: 'Chỗ trống là: "vuong"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "16"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
So sánh trực tiếp trên `kind` — không cần đoán qua field nào có mặt,
TypeScript tự thu hẹp cả hai chiều. Ngắn gọn hơn hẳn `in`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`if/else` chạy tốt với ĐÚNG hai biến thể. Nhưng nếu `Hinh` có BA, BỐN,
NĂM biến thể — chuỗi `if (h.kind === ...) ... else if (h.kind === ...)
... else if ...` sẽ dài dần, lặp đi lặp lại `h.kind ===`. Có cách viết
GỌN hơn, dành RIÊNG cho việc so sánh một giá trị với NHIỀU khả năng,
không?
::::

::::checkpoint{mastery=0.8}
::::
