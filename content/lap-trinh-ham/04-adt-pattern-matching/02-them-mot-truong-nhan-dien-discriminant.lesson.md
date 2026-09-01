---
id: lap-trinh-ham.adt-pattern-matching.them-mot-truong-nhan-dien-discriminant
title: "Thêm MỘT trường nhận diện — `kind: \"vuong\" | \"tron\"`"
summary: "interface Vuong { kind: \"vuong\"; canh: number } — thêm một field kind với KIỂU LITERAL vào MỖI biến thể. Field này không mang thông tin nghiệp vụ — nó CHỈ là một cái NHÃN, để sau này TypeScript tự soi được đang cầm biến thể nào."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ts.discriminant-field]
requires: [ts.union-ambiguity]
concepts: [ts.discriminant-field]
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
Bài trước: đoán biến thể qua field nào CÓ MẶT — dễ đoán sai. Hôm nay:
mỗi biến thể tự MANG một cái nhãn, không cần đoán nữa.
::::

::::explain{#truong-nhan-dien}
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

const v: Hinh = { kind: "vuong", canh: 4 };
const t: Hinh = { kind: "tron", banKinh: 3 };

console.log(v.kind);
console.log(t.kind);
```

```text
vuong
tron
```

Mỗi `interface` giờ có thêm MỘT field `kind` — nhưng KHÔNG khai
`kind: string` (kiểu chung chung). Khai `kind: "vuong"` (một CHUỖI CỤ
THỂ làm KIỂU, gọi là "literal type") — nghĩa là field `kind` của
`Vuong` CHỈ CÓ THỂ là đúng chuỗi `"vuong"`, không giá trị nào khác được
chấp nhận. Gán `kind: "hinh_khac"` cho một biến kiểu `Vuong` bị
TypeScript CHẶN NGAY (mã lỗi TS2322 — đã học ở T4.0a).

Field `kind` KHÔNG mang thông tin nghiệp vụ gì (không phải cạnh, không
phải bán kính) — nó CHỈ là một cái NHÃN, gắn cố định vào TỪNG biến thể,
để sau này (bài tiếp theo) TypeScript có thể tự soi nhãn mà biết đang
cầm biến thể nào, không cần đoán qua field nào có mặt.
::::

::::example{#nhan-khong-doi}
Mỗi giá trị GIỮ ĐÚNG nhãn của biến thể nó thuộc về, không tự đổi:

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

const dsHinh: Hinh[] = [
  { kind: "vuong", canh: 4 },
  { kind: "tron", banKinh: 2 },
  { kind: "vuong", canh: 10 },
];

for (const h of dsHinh) {
  console.log(h.kind);
}
```

```text title=readonly
vuong
tron
vuong
```

Một MẢNG `Hinh[]` (trộn lẫn cả hai biến thể) — mỗi phần tử vẫn tự MANG
đúng nhãn của nó, đọc ra qua `h.kind`, không cần biết TRƯỚC thứ tự hay
số lượng từng loại trong mảng.
::::

::::predict{#doan-gan-sai-nhan commitOnce}
```typescript
interface DonChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DonDaGiao {
  kind: "daGiao";
  ma: string;
}

type Don = DonChoXuLy | DonDaGiao;

const d: DonChoXuLy = { kind: "daGiao", ma: "DH-1" };
console.log(d);
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi biên dịch — `"daGiao"` không khớp kiểu `"choXuLy"` mà
`DonChoXuLy` đòi hỏi ở field `kind`
:::

:::opt
`{ kind: "daGiao", ma: "DH-1" }` — object vẫn được TẠO RA bình thường,
`kind` chỉ là một chuỗi như bao chuỗi khác, gán gì cũng được
::why
Gần đúng ở việc bạn nghĩ `kind` "chỉ là một chuỗi" — về BẢN CHẤT LÚC
CHẠY, `kind` đúng là một chuỗi JavaScript bình thường, không có gì đặc
biệt.

Chỗ lệch: LÚC BIÊN DỊCH, TypeScript đã KHAI `d` là kiểu `DonChoXuLy`
tường minh — mà `DonChoXuLy.kind` CHỈ CHẤP NHẬN đúng chuỗi `"choXuLy"`
(kiểu literal). Gán `"daGiao"` (một chuỗi khác) vào field đó VI PHẠM
lời hứa kiểu, TypeScript từ chối biên dịch — mã TS2322, không có dòng
nào chạy.
::
:::

:::opt
`{ kind: "choXuLy", ma: "DH-1" }` — TypeScript tự SỬA `kind` về đúng
giá trị hợp lệ theo kiểu đã khai, âm thầm bỏ qua giá trị bạn gõ
::why
Gần đúng ở việc bạn tin TypeScript "biết" giá trị đúng phải là gì —
đúng là kiểu `DonChoXuLy` chỉ chấp nhận `"choXuLy"`.

Chỗ lệch: TypeScript KHÔNG BAO GIỜ tự "sửa" giá trị bạn viết — nó chỉ
CHẶN nếu giá trị đó sai kiểu, buộc BẠN sửa lại mã nguồn. Không có cơ
chế nào "âm thầm thay thế" — hoặc mã biên dịch được với giá trị bạn
viết, hoặc trình biên dịch từ chối và báo lỗi rõ ràng.
::
:::
::::

::::code{#tao_tron}
Viết `taoTron(banKinh)` — trả về một `Tron` với `kind` đúng nhãn
`"tron"` và `banKinh` đúng giá trị truyền vào.

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

function taoTron(banKinh: number): Tron {
  return { kind: ___, banKinh: banKinh };
}

console.log(taoTron(5).kind);
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

function taoTron(banKinh: number): Tron {
  return { kind: "tron", banKinh: banKinh };
}

console.log(taoTron(5).kind);
```

```typescript title=test
const t1 = taoTron(5);
if (t1.kind !== "tron") throw new Error("taoTron phải trả về kind là \"tron\" — đang là " + t1.kind);
if (t1.banKinh !== 5) throw new Error("taoTron(5) phải có banKinh là 5 — đang là " + t1.banKinh);
const t2 = taoTron(10);
if (t2.banKinh !== 10) throw new Error("taoTron(10) phải có banKinh là 10 — đang là " + t2.banKinh);
if (t2.kind !== "tron") throw new Error("taoTron(10) phải có kind là \"tron\" — đang là " + t2.kind);
```

:::hints
- kind: attention
  body: "Chỗ trống là giá trị của field kind — Tron chỉ chấp nhận đúng chuỗi \"tron\" (kiểu literal), không phải string chung chung."
- kind: strategy
  body: 'interface Tron khai kind: "tron" — object trả về phải khớp ĐÚNG chuỗi đó, viết trong dấu ngoặc kép.'
- kind: one-line
  body: 'Chỗ trống là: "tron"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "tron"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mỗi biến thể giờ tự MANG một cái nhãn, không mơ hồ. Bước sau: dùng cái
nhãn đó để TypeScript tự soi ra đang cầm biến thể nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có nhãn `kind` — nhưng vẫn phải viết `if ("canh" in h)` kiểu cũ
để đọc nó, hay có cách viết KHÁC, TẬN DỤNG trực tiếp cái nhãn đó?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
