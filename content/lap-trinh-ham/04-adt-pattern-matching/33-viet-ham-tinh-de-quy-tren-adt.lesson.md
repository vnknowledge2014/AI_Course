---
id: lap-trinh-ham.adt-pattern-matching.viet-ham-tinh-de-quy-tren-adt
title: "Viết hàm `tinh()` ĐỆ QUY, pattern-match trên ADT đệ quy"
summary: "Viết tinh(e: Expr): number — switch trên e.kind, biến thể so trả giaTri, biến thể cong/nhan GỌI LẠI tinh() đệ quy trên trai/phai rồi cộng/nhân kết quả. Cấu trúc hàm phản ánh đúng cấu trúc kiểu — kiểu đệ quy, hàm xử lý nó cũng đệ quy."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 33
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.write-recursive-eval]
requires: [ts.recursive-adt]
concepts: [ts.write-recursive-eval]
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
Bài trước: `Expr` tự tham chiếu chính nó — `cong`/`nhan` chứa HAI `Expr`
con, lồng bao nhiêu tầng cũng hợp lệ. Nhưng một GIÁ TRỊ kiểu `Expr` chỉ
là dữ liệu — nó không tự TÍNH ra số nào cả. Cần một HÀM biết đọc cấu
trúc đó.
::::

::::explain{#ham-tinh-de-quy}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
  }
}

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: {
    kind: "nhan",
    trai: { kind: "so", giaTri: 4 },
    phai: { kind: "so", giaTri: 5 },
  },
};

console.log(tinh(bieuThuc));
```

```text
23
```

`tinh(e: Expr): number` — một `switch` trên `e.kind`, giống mọi hàm
pattern-match đã viết từ đầu track. Nhưng `Expr` có kiểu ĐỆ QUY (hai
biến thể chứa chính `Expr` bên trong), nên hàm xử lý nó CŨNG đệ quy:
biến thể `so` là ĐIỂM DỪNG (không gọi lại `tinh()`, chỉ trả thẳng
`giaTri`); biến thể `cong`/`nhan` GỌI LẠI `tinh()` trên `e.trai` VÀ
`e.phai` — mỗi lời gọi đó lại chạy đúng `switch` này, cho tới khi chạm
một `so` ở đáy cây. Biểu thức `3 + (4 * 5)` (bài trước dùng làm ví dụ
cho `Expr`) giờ TÍNH ra đúng `23` — `tinh()` không cần biết trước cây
sâu bao nhiêu tầng, vì mỗi lời gọi đệ quy tự lo phần cây CON của nó.

Đây là điều cốt lõi: `Expr` có MỘT biến thể không đệ quy (`so`) và HAI
biến thể đệ quy (`cong`, `nhan`) — `tinh()` cũng có ĐÚNG hình dạng đó:
một nhánh KHÔNG gọi lại chính nó, hai nhánh CÓ gọi lại. Cấu trúc hàm
PHẢN ÁNH cấu trúc kiểu — không phải trùng hợp, mà là hệ quả trực tiếp
của việc một kiểu đệ quy đòi hàm xử lý nó cũng đệ quy theo đúng hình
dạng đó.
::::

::::example{#cay-nhieu-tang}
Cây SÂU hơn — cả HAI nhánh của phép `nhan` ngoài cùng đều là `cong`,
không chỉ một bên:

```typescript title=readonly
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
  }
}

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
    phai: { kind: "so", giaTri: 1 },
  },
};

console.log(tinh(bieuThuc));
```

```text title=readonly
25
```

`(2 + 3) * (4 + 1)` — `tinh()` không biết trước cây này có hình dạng
gì, nó chỉ làm ĐÚNG MỘT việc mỗi lần gọi: nhìn `kind` của node hiện
tại, và nếu cần, gọi lại `tinh()` trên con. Nhánh `trai` (một `cong`)
tự đệ quy xuống hai lá, cho `5`; nhánh `phai` (cũng một `cong`) tự đệ
quy xuống hai lá khác, cho `5`; rồi `nhan` NHÂN hai kết quả đó lại:
5 * 5 = 25. Không có giới hạn nào buộc `trai`/`phai` phải là `so` —
chúng có thể là BẤT KỲ `Expr` nào, kể cả một cây con phức tạp, vì
`tinh()` gọi lại CHÍNH NÓ, không phải một hàm "xử lý lá" riêng biệt.
::::

::::predict{#doan-ket-qua-nhan-long-cong commitOnce}
Cùng `tinh()` như trên:

```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
  }
}

const a: Expr = {
  kind: "nhan",
  trai: { kind: "so", giaTri: 3 },
  phai: {
    kind: "cong",
    trai: { kind: "so", giaTri: 2 },
    phai: { kind: "so", giaTri: 5 },
  },
};

console.log(tinh(a));
```

Dòng cuối in ra gì?

:::opt{correct}
`21`
:::

:::opt
`11`
::why
Gần đúng ở việc bạn tính đúng hai con số bên trong nhánh `phai` (`2` và
`5`, hai lá của `cong`) — hai giá trị đó có thật trong cây.

Chỗ lệch: `tinh()` không "vói" thẳng vào field `trai`/`phai` của nhánh
`phai` rồi bỏ qua bước cộng — nó gọi ĐỆ QUY `tinh(e.phai)` trước, tính
trọn `2 + 5 = 7` cho CẢ nhánh `cong` đó, rồi mới nhân với `tinh(e.trai)`
(là `3`). Kết quả đúng là 3 * 7 = 21, không phải 3 * 2 + 5 = 11.
::
:::

:::opt
Máy báo lỗi biên dịch tại `tinh(e.phai)` trong `case "nhan"` — vì
`e.phai` có kiểu `Expr` (hợp union rộng), TypeScript chưa NARROW được
biến thể cụ thể nên không cho gọi hàm
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng bên trong `case "nhan"`, TypeScript
đã NARROW `e` thành đúng biến thể `nhan` — quan sát đó đúng, `e.trai`
và `e.phai` truy cập được bình thường ngay trong nhánh này.

Chỗ lệch: `tinh()` nhận tham số kiểu `Expr` — CHÍNH LÀ kiểu union rộng
đó, không đòi một biến thể cụ thể nào. `e.phai` đã có kiểu `Expr` sẵn
(interface khai đúng vậy), truyền thẳng vào `tinh()` không cần narrow
thêm bước nào — đoạn mã biên dịch và chạy bình thường, in ra `21`.
::
:::
::::

::::code{#ham_tinh}
Viết đủ `tinh(e: Expr): number`. Nhánh `case "cong"` đã có sẵn làm
mẫu — nhánh `case "nhan"` còn thiếu, viết đúng CẤU TRÚC như `cong`,
chỉ đổi phép toán.

```typescript title=starter
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "nhan":
      return ___;
  }
}

console.log(tinh({ kind: "so", giaTri: 7 }));
```

```typescript title=solution
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
  }
}

console.log(tinh({ kind: "so", giaTri: 7 }));
```

```typescript title=test
const soBay: Expr = { kind: "so", giaTri: 7 };
if (tinh(soBay) !== 7) throw new Error("tinh phải trả 7 cho { kind: \"so\", giaTri: 7 } — đang là " + tinh(soBay));

const congBa: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "so", giaTri: 4 },
};
if (tinh(congBa) !== 7) throw new Error("tinh phải trả 7 cho 3 cong 4 — đang là " + tinh(congBa));

const nhanBa: Expr = {
  kind: "nhan",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "so", giaTri: 4 },
};
if (tinh(nhanBa) !== 12) throw new Error("tinh phải trả 12 cho 3 nhan 4 — đang là " + tinh(nhanBa));

const longNhau1: Expr = {
  kind: "nhan",
  trai: { kind: "so", giaTri: 3 },
  phai: {
    kind: "cong",
    trai: { kind: "so", giaTri: 2 },
    phai: { kind: "so", giaTri: 5 },
  },
};
if (tinh(longNhau1) !== 21) throw new Error("tinh phải trả 21 cho 3 nhan (2 cong 5) — đang là " + tinh(longNhau1));

const longNhau2: Expr = {
  kind: "cong",
  trai: {
    kind: "nhan",
    trai: { kind: "so", giaTri: 2 },
    phai: { kind: "so", giaTri: 3 },
  },
  phai: {
    kind: "nhan",
    trai: { kind: "so", giaTri: 4 },
    phai: { kind: "so", giaTri: 4 },
  },
};
if (tinh(longNhau2) !== 22) throw new Error("tinh phải trả 22 cho (2 nhan 3) cong (4 nhan 4) — đang là " + tinh(longNhau2));
```

:::hints
- kind: attention
  body: "Chỗ trống là phép TÍNH của nhánh nhan — case cong ngay phía trên đã làm mẫu ĐÚNG cấu trúc cần lặp lại, chỉ khác một phép toán."
- kind: strategy
  body: "case cong trả về tinh(e.trai) + tinh(e.phai) — case nhan cần ĐÚNG cấu trúc đó (gọi đệ quy tinh() trên cả trai lẫn phai), chỉ đổi + thành *."
- kind: one-line
  body: "Chỗ trống là: tinh(e.trai) * tinh(e.phai)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "7"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`tinh()` đọc đúng một cây `Expr` sâu bao nhiêu tầng cũng được — không
phải vì có gì đặc biệt, mà vì cấu trúc hàm khớp ĐÚNG cấu trúc kiểu:
một điểm dừng, hai nhánh gọi lại chính nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Expr` hiện có ba biến thể: `so`, `cong`, `nhan`. Nếu thêm MỘT biến
thể mới — ví dụ `tru` (phép trừ) — mà `tinh()` KHÔNG được cập nhật để
xử lý nó, chuyện gì xảy ra? Lỗi có hiện ra không, và hiện ra Ở ĐÂU —
lúc biên dịch, hay chỉ lúc chạy với đúng biến thể mới đó?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
