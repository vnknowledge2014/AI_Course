---
id: lap-trinh-ham.adt-pattern-matching.adt-de-quy-mot-cay-bieu-thuc
title: "ADT đệ quy — một kiểu TỰ THAM CHIẾU chính nó"
summary: "`type Expr = { kind: \"so\"; giaTri: number } | { kind: \"cong\"; trai: Expr; phai: Expr } | { kind: \"nhan\"; trai: Expr; phai: Expr }` — hai biến thể `cong`/`nhan` có field kiểu CHÍNH là `Expr`, kiểu TỰ THAM CHIẾU chính nó, biểu diễn một CÂY biểu thức số học lồng bao nhiêu tầng cũng được."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 32
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.recursive-adt]
requires: [ts.review-branded-smart-constructor]
concepts: [ts.recursive-adt]
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
Bài trước bạn ôn lại cặp branded type + smart constructor, tự thiết kế
từ đầu, không có khung sẵn. Hôm nay track rẽ sang một câu hỏi khác hẳn:
một KIỂU dữ liệu có được phép chứa CHÍNH NÓ bên trong nó không?
::::

::::explain{#kieu-tu-tham-chieu}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: {
    kind: "nhan",
    trai: { kind: "so", giaTri: 4 },
    phai: { kind: "so", giaTri: 5 },
  },
};

console.log(bieuThuc.kind);
console.log(bieuThuc.phai.kind);
```

```text
cong
nhan
```

`Expr` có BA biến thể, y hệt khuôn "discriminant" đã quen: mỗi biến
thể tự mang nhãn `kind`. Biến thể `so` là một LÁ — chỉ có một số
(`giaTri`). Nhưng `cong` và `nhan` không chứa số trực tiếp — hai field
`trai`/`phai` của chúng được khai kiểu `Expr`, CHÍNH là cái `type` đang
được định nghĩa. `Expr` xuất hiện ngay TRONG định nghĩa của `Expr` —
đây gọi là kiểu TỰ THAM CHIẾU (recursive type), và TypeScript chấp
nhận nó bình thường, không lỗi gì.

Nhờ vậy, `bieuThuc` biểu diễn được phép toán `3 + (4 * 5)`: gốc là một
`cong`, nhánh `trai` là số `3` (lá), nhánh `phai` lại là một `nhan`
khác — bản thân nó có `trai`/`phai` riêng, là hai số `4` và `5`. Đây
là một CÂY ba tầng thật: tầng 1 là `cong` ở gốc, tầng 2 gồm lá `3` và
nút `nhan`, tầng 3 là hai lá `4`/`5` bên trong `nhan` đó. Không có
GIỚI HẠN nào về việc cây lồng sâu bao nhiêu tầng — mỗi `trai`/`phai`
lại là một `Expr` đầy đủ, có thể tiếp tục chứa `cong`/`nhan` khác bên
trong, tới bất kỳ độ sâu nào.

`bieuThuc.kind` đọc nhãn của GỐC (`"cong"`) — field `kind` có mặt trên
CẢ BA biến thể nên đọc được ngay, không cần kiểm gì thêm.
`bieuThuc.phai` là một `Expr` LỒNG (chính nút `nhan` ở tầng 2), và
`.kind` của NÓ (`"nhan"`) khác với `kind` của gốc — mỗi tầng trong cây
mang nhãn RIÊNG của tầng đó.
::::

::::example{#cay-long-sau-hai-ben}
Cây không nhất thiết lồng về MỘT phía — cả `trai` VÀ `phai` đều có thể
là nút lồng cùng lúc, biểu diễn `(2 + 3) * (4 + 5)`:

```typescript title=readonly
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

const bieuThuc: Expr = {
  kind: "nhan",
  trai: {
    kind: "cong",
    trai: { kind: "so", giaTri: 2 },
    phai: { kind: "so", giaTri: 3 },
  },
  phai: {
    kind: "cong",
    trai: { kind: "so", giaTri: 4 },
    phai: { kind: "so", giaTri: 5 },
  },
};

console.log(bieuThuc.trai.kind);
console.log(bieuThuc.phai.kind);
```

```text title=readonly
cong
cong
```

Gốc là `nhan`, nhưng CẢ `trai` LẪN `phai` của nó đều là nút `cong`
riêng — không phải lá. `Expr` không quy định "chỉ một bên được lồng" —
mỗi field `trai`/`phai` ĐỘC LẬP, mang bất kỳ biến thể nào trong ba biến
thể của `Expr`, kể cả lồng ở cả hai bên cùng lúc. Đây chính là sức mạnh
của kiểu tự tham chiếu: MỘT định nghĩa kiểu, áp dụng đệ quy, dựng được
cây hình dạng bất kỳ.
::::

::::predict{#doan-doc-cay-long commitOnce}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

const e: Expr = {
  kind: "nhan",
  trai: { kind: "so", giaTri: 6 },
  phai: {
    kind: "cong",
    trai: { kind: "so", giaTri: 1 },
    phai: { kind: "so", giaTri: 2 },
  },
};

console.log(e.kind);
console.log(e.phai.kind);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`nhan` rồi `cong`
:::

:::opt
`nhan` rồi `so`
::why
Gần đúng ở dòng ĐẦU — `e.kind` đúng là `"nhan"`, object gán cho `e`
thật sự khai `kind: "nhan"` ở gốc.

Chỗ lệch: field `phai` không lưu MỘT SỐ trực tiếp — nó lưu một `Expr`
LỒNG bên trong, mà ở đây chính là object `{ kind: "cong", trai: ...,
phai: ... }`. Đọc `.kind` của NÚT LỒNG đó (không phải của lá bên
trong nó) phải trả về `"cong"`, không phải `"so"` — muốn chạm tới lá
`"so"` phải đi thêm một bước nữa, ví dụ `e.phai.trai.kind`.
::
:::

:::opt
Máy báo lỗi biên dịch ở dòng `e.phai.kind` — vì `e.phai` có kiểu `Expr`
(union ba biến thể), chưa rõ biến thể nào nên không được phép đọc
`.kind`
::why
Gần đúng ở việc bạn nhớ ĐÚNG: field `trai`/`phai` mang kiểu `Expr` — một
UNION ba biến thể, và nhiều field KHÁC của union đó (như `giaTri`) thật
sự cần thu hẹp (narrow) trước khi đọc.

Chỗ lệch: `kind` là NGOẠI LỆ — nó là trường nhận diện (discriminant, đã
học từ bài 2), có mặt trên CẢ BA biến thể của `Expr` với cùng một tên.
Đọc `.kind` không bao giờ cần narrow trước, bất kể object đó thuộc biến
thể nào — kể cả khi nó là một `Expr` lồng bên trong một `Expr` khác.
::
:::
::::

::::code{#tao_bieu_thuc}
Viết `taoBieuThuc(a, b, c)` — trả về một `Expr` biểu diễn phép toán
`a + (b * c)`: gốc là `cong`, nhánh `trai` là số `a`, nhánh `phai` là
một `nhan` lồng chứa hai số `b` và `c`.

```typescript title=starter
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function taoBieuThuc(a: number, b: number, c: number): Expr {
  return {
    kind: "cong",
    trai: { kind: "so", giaTri: a },
    phai: {
      kind: "nhan",
      trai: { kind: "so", giaTri: b },
      phai: ___,
    },
  };
}

console.log(taoBieuThuc(3, 4, 5).kind);
```

```typescript title=solution
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function taoBieuThuc(a: number, b: number, c: number): Expr {
  return {
    kind: "cong",
    trai: { kind: "so", giaTri: a },
    phai: {
      kind: "nhan",
      trai: { kind: "so", giaTri: b },
      phai: { kind: "so", giaTri: c },
    },
  };
}

console.log(taoBieuThuc(3, 4, 5).kind);
```

```typescript title=test
const e = taoBieuThuc(3, 4, 5);

if (e.kind !== "cong") throw new Error("taoBieuThuc phải trả về Expr với kind là \"cong\" ở gốc — đang là " + e.kind);
if (e.trai.kind !== "so") throw new Error("Nhánh trai của gốc phải là kind \"so\" — đang là " + e.trai.kind);
if (e.trai.giaTri !== 3) throw new Error("Nhánh trai (so) phải có giaTri là 3 — đang là " + e.trai.giaTri);

if (e.phai.kind !== "nhan") throw new Error("Nhánh phai của gốc phải là kind \"nhan\" — đang là " + e.phai.kind);
if (e.phai.trai.kind !== "so") throw new Error("Nhánh trai của nhan phải là kind \"so\" — đang là " + e.phai.trai.kind);
if (e.phai.trai.giaTri !== 4) throw new Error("Nhánh trai của nhan phải có giaTri là 4 — đang là " + e.phai.trai.giaTri);
if (e.phai.phai.kind !== "so") throw new Error("Nhánh phai của nhan phải là kind \"so\" — đang là " + e.phai.phai.kind);
if (e.phai.phai.giaTri !== 5) throw new Error("Nhánh phai của nhan phải có giaTri là 5 — đang là " + e.phai.phai.giaTri);

const e2 = taoBieuThuc(10, 20, 30);
if (e2.kind !== "cong") throw new Error("taoBieuThuc(10,20,30) phải có kind là \"cong\" ở gốc — đang là " + e2.kind);
if (e2.phai.kind !== "nhan") throw new Error("taoBieuThuc(10,20,30) phải có phai.kind là \"nhan\" — đang là " + e2.phai.kind);
if (e2.phai.phai.kind !== "so") throw new Error("taoBieuThuc(10,20,30) phải có phai.phai.kind là \"so\" — đang là " + e2.phai.phai.kind);
if (e2.phai.phai.giaTri !== 30) throw new Error("taoBieuThuc(10,20,30) phải có phai.phai.giaTri là 30 — đang là " + e2.phai.phai.giaTri);
```

:::hints
- kind: attention
  body: "Chỗ trống là nhánh phai của nút nhan lồng bên trong — nó phải là một lá số, mang giá trị c."
- kind: strategy
  body: "Biến thể so khai kind: \"so\"; giaTri: number — chỗ trống phải là một object đúng hình dạng đó, dùng tham số c làm giaTri."
- kind: one-line
  body: 'Chỗ trống là: { kind: "so", giaTri: c }'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "cong"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một kiểu tự tham chiếu chính nó, dựng được cây lồng bao nhiêu tầng
cũng xong — không còn là lý thuyết, bạn vừa tự tay viết một cây thật.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã biết XÂY một cây `Expr` — nhưng cây đó chỉ là DỮ LIỆU, chưa ai
"tính" ra kết quả số học của nó. `3 + (4 * 5)` phải cho ra `23` — làm
sao viết một hàm ĐI QUA từng tầng của cây, cộng/nhân đúng theo đúng
`kind` gặp ở mỗi tầng, kể cả khi cây lồng sâu bao nhiêu cũng không bỏ
sót tầng nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
