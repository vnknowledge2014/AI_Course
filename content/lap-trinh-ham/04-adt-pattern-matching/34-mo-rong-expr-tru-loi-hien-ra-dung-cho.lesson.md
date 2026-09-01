---
id: lap-trinh-ham.adt-pattern-matching.mo-rong-expr-tru-loi-hien-ra-dung-cho
title: "Mở rộng `Expr` thêm `tru` — lỗi hiện ra ĐÚNG CHỖ nhờ exhaustiveness"
summary: "Thêm biến thể `{ kind: \"tru\"; trai: Expr; phai: Expr }` vào `Expr` — với `assertNever` đã bảo vệ `tinh()` từ bài trước, TypeScript báo NGAY mã TS2345 tại đúng dòng `chuaXuLy(e)`, trỏ thẳng vào chỗ switch cũ CHƯA xử lý `tru`. Bài code: thêm case còn thiếu, tinh() tính đúng cho cả bốn biến thể."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 34
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.extend-recursive-adt-exhaustive]
requires: [ts.write-recursive-eval]
concepts: [ts.extend-recursive-adt-exhaustive]
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
Bài trước: bạn viết `tinh()` — hàm đệ quy pattern-match trên `Expr`,
gọi lại chính nó ở nhánh `cong`/`nhan`. Hôm nay: mở rộng `Expr` thêm
MỘT phép tính mới — xem TypeScript có tự tìm ra chỗ `tinh()` còn
thiếu không.
::::

::::explain{#mo-rong-tru-loi-dung-cho}
Nhắc lại nguyên trạng bài trước — `Expr` có ba biến thể (`so`/`cong`/
`nhan`), `tinh()` xử lý đủ cả ba, có `assertNever` bảo vệ ở `default`:

```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
    default:
      return chuaXuLy(e);
  }
}

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 5 } },
};

console.log(tinh(bieuThuc));
```

```text
23
```

Biên dịch sạch, chạy đúng: 3 + (4 * 5) = 23. `switch` xử lý đủ cả ba
biến thể, nên tại `default`, kiểu của `e` bị thu hẹp còn `never` (đã
đo từ bài 8), khớp đúng tham số `chuaXuLy` đòi hỏi.

Giờ ứng dụng cần thêm PHÉP TRỪ. Thêm biến thể `{ kind: "tru"; trai:
Expr; phai: Expr }` vào union — nhưng CHƯA sửa `switch`, `tinh()` vẫn
y hệt cũ, chỉ xử lý `so`/`cong`/`nhan`. Chuyện gì xảy ra?

Bên trong `default`, TypeScript giờ suy luận khác: đã loại `"so"`,
`"cong"`, `"nhan"` (ba `case` phía trên), khả năng CÒN LẠI của `e`
không phải `never` nữa — mà chính xác là `{ kind: "tru"; trai: Expr;
phai: Expr }`. Gọi `chuaXuLy(e)` lúc này là đưa một giá trị mang kiểu
đó vào tham số CHỈ chấp nhận `never` — đúng mã `TS2345` đã học từ bài
9, xuất hiện LẦN NỮA, nhưng lần này trên một ADT ĐỆ QUY thật: `Expr`
tự tham chiếu chính nó qua `trai`/`phai`, không phải một union phẳng
đơn giản như `Hinh`/`Don` ở các bài trước. Đã đo thật: thông điệp
chính xác là `Argument of type '{ kind: "tru"; trai: Expr; phai:
Expr; }' is not assignable to parameter of type 'never'.`, báo NGAY
tại dòng `chuaXuLy(e)` — không phải một cảnh báo mơ hồ "có lỗi đâu đó
trong file", mà trỏ THẲNG vào đúng chỗ `switch` cũ chưa theo kịp.
::::

::::example{#truoc-sau-them-tru}
Thêm `tru` vào `Expr`, CHƯA sửa `switch`:

```typescript title=readonly
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
    default:
      return chuaXuLy(e);
  }
}

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 5 } },
};

console.log(tinh(bieuThuc));
```

```text title=readonly
(không in ra gì cả)

TS2345 (dòng 20, cột 23): Argument of type '{ kind: "tru"; trai:
Expr; phai: Expr; }' is not assignable to parameter of type 'never'.
```

Để ý: `bieuThuc` ở đây KHÔNG hề chứa một `"tru"` nào — chỉ toàn
`so`/`cong`/`nhan`. Không quan trọng. Lỗi bị bắt LÚC BIÊN DỊCH, nhìn
vào KIỂU của `e` tại `default`, không cần chờ chương trình chạy tới
một nhánh `tru` thật nào.

Thêm đúng `case "tru"` — lỗi biến mất, không cần sửa gì khác:

```typescript title=readonly
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "tru":
      return tinh(e.trai) - tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
    default:
      return chuaXuLy(e);
  }
}

const bieuThuc: Expr = {
  kind: "cong",
  trai: { kind: "tru", trai: { kind: "so", giaTri: 10 }, phai: { kind: "so", giaTri: 4 } },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
};

console.log(tinh(bieuThuc));
```

```text title=readonly
12
```

`(10 - 4) + (2 * 3) = 12`, khớp output thật — và `chuaXuLy(e)` biên
dịch được trở lại, vì giờ CẢ BỐN `case` đã xử lý đủ, `e` tại `default`
lại là `never`. Không cần grep tay tìm mọi `switch` xử lý `Expr` trong
cả dự án, không cần tự nhắc "tôi đã sửa hết chưa" — chỉ cần THÊM biến
thể vào union, trình biên dịch TỰ CHỈ đúng dòng còn thiếu, kể cả khi
`Expr` là một kiểu ĐỆ QUY (biến thể mới cũng tự tham chiếu `Expr`,
không phải một shape phẳng).
::::

::::predict{#doan-mo-rong-chia commitOnce}
```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "nhan"; trai: Expr; phai: Expr }
  | { kind: "chia"; trai: Expr; phai: Expr };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
    default:
      return chuaXuLy(e);
  }
}

console.log(tinh({ kind: "nhan", trai: { kind: "so", giaTri: 6 }, phai: { kind: "so", giaTri: 7 } }));
```

`Expr` vừa được thêm biến thể `"chia"`, nhưng `switch` chỉ xử lý
`"so"` và `"nhan"`. Dòng gọi DUY NHẤT trong chương trình chỉ dùng
`"nhan"` và `"so"`, không hề tạo ra một `"chia"` nào. Chương trình này
biên dịch được không?

:::opt{correct}
Không — bị chặn NGAY LÚC BIÊN DỊCH tại dòng `chuaXuLy(e)` (TS2345), dù
chương trình không hề tạo ra bất kỳ giá trị `kind: "chia"` nào
:::

:::opt
Biên dịch được, in ra `42` — vì lời gọi thực tế chỉ dùng `"nhan"` và
`"so"`, TypeScript chỉ kiểm nhánh THỰC SỰ chạy qua, bỏ qua nhánh
`default` vì chương trình không đi tới đó
::why
Gần đúng ở việc bạn tính đúng 6 * 7 = 42 — NẾU đoạn mã này biên dịch
qua được, kết quả chạy đúng sẽ là 42 thật.

Chỗ lệch: TypeScript kiểm KIỂU lúc BIÊN DỊCH cho MỌI nhánh của
`switch`, bất kể lúc CHẠY chương trình có đi qua nhánh đó hay không —
không có khái niệm "nhánh chưa từng được gọi thì bỏ qua kiểm kiểu". Đã
đo thật: chương trình bị TS2345 ngay tại dòng `chuaXuLy(e)`, TRƯỚC KHI
bất kỳ dòng JavaScript nào được sinh ra hay chạy.
::
:::

:::opt
Biên dịch được, nhưng khi CHẠY dòng `console.log(tinh(...))` chương
trình sẽ ném lỗi runtime từ bên trong `chuaXuLy` — vì `switch` thiếu
`case "chia"`
::why
Gần đúng ở việc bạn nhớ đúng thân hàm `chuaXuLy` CÓ `throw new
Error(...)` — đúng, đó là những gì nó làm NẾU thật sự được gọi tới lúc
chạy.

Chỗ lệch: để CHẠY tới đó, chương trình phải BIÊN DỊCH được trước. `e`
tại `default` ở đây KHÔNG còn là `never` (`Expr` giờ có ba biến thể,
`switch` chỉ xử lý hai) — `chuaXuLy(e)` VI PHẠM chữ ký hàm ngay trên
GIẤY, TypeScript từ chối biên dịch (TS2345), không dòng JavaScript nào
được thực thi.
::
:::
::::

::::code{#them-case-tru}
`tinh(e)` đã có `assertNever` bảo vệ, xử lý đủ `"so"`, `"cong"`,
`"nhan"`. Biến thể `"tru"` vừa được thêm vào `Expr`, nhánh `case
"tru"` đã CÓ SẴN — nhưng công thức tính còn thiếu. Điền đúng để
`chuaXuLy(e)` không còn báo TS2345, và `tinh()` tính đúng cho cả bốn
biến thể: trái trừ phải.

```typescript title=starter
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "tru":
      return ___;
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
    default:
      return chuaXuLy(e);
  }
}

console.log(tinh({ kind: "tru", trai: { kind: "so", giaTri: 10 }, phai: { kind: "so", giaTri: 4 } }));
```

```typescript title=solution
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "tru"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so":
      return e.giaTri;
    case "cong":
      return tinh(e.trai) + tinh(e.phai);
    case "tru":
      return tinh(e.trai) - tinh(e.phai);
    case "nhan":
      return tinh(e.trai) * tinh(e.phai);
    default:
      return chuaXuLy(e);
  }
}

console.log(tinh({ kind: "tru", trai: { kind: "so", giaTri: 10 }, phai: { kind: "so", giaTri: 4 } }));
```

```typescript title=test
const soDon = tinh({ kind: "so", giaTri: 7 });
if (soDon !== 7) throw new Error("tinh cho biến thể so phải trả về đúng giaTri — đang là " + soDon);

const congDon = tinh({ kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } });
if (congDon !== 7) throw new Error("tinh(3 cong 4) phải là 7 — đang là " + congDon);

const truDon = tinh({ kind: "tru", trai: { kind: "so", giaTri: 10 }, phai: { kind: "so", giaTri: 4 } });
if (truDon !== 6) throw new Error("tinh(10 tru 4) phải là 6 — đang là " + truDon);

const nhanDon = tinh({ kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } });
if (nhanDon !== 12) throw new Error("tinh(3 nhan 4) phải là 12 — đang là " + nhanDon);

const bieuThucLong: Expr = {
  kind: "cong",
  trai: { kind: "tru", trai: { kind: "so", giaTri: 10 }, phai: { kind: "so", giaTri: 4 } },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
};
const ketQuaLong = tinh(bieuThucLong);
if (ketQuaLong !== 12) throw new Error("tinh((10 tru 4) cong (2 nhan 3)) phải là 12 — đang là " + ketQuaLong);
```

:::hints
- kind: attention
  body: "Chỗ trống là biểu thức return của case \"tru\" — dùng tinh(e.trai) và tinh(e.phai) đệ quy y hệt case \"cong\", chỉ đổi phép cộng thành phép trừ."
- kind: strategy
  body: 'Trong nhánh case "tru", e đã được TypeScript narrow thành { kind: "tru"; trai: Expr; phai: Expr } — đọc được e.trai và e.phai, mỗi cái đều là một Expr con, gọi tinh() đệ quy trên từng cái rồi trừ.'
- kind: one-line
  body: "Chỗ trống là: tinh(e.trai) - tinh(e.phai)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "6"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một biến thể mới, MỘT lỗi biên dịch trỏ ĐÚNG dòng cần sửa — trên một
ADT ĐỆ QUY thật, không phải union phẳng minh hoạ. Đây chính là lý do
giữ kỷ luật `assertNever` đáng công sức bỏ ra.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Expr` là MỘT ví dụ ADT đệ quy — một cây biểu thức số học. Nhưng đệ
quy không chỉ giới hạn ở biểu thức toán: bất kỳ kiểu dữ liệu nào TỰ
CHỨA một bản sao chính nó (trực tiếp, hoặc qua một mảng các bản sao)
cũng đệ quy được — ví dụ, một danh sách LỒNG NHAU, mỗi phần tử có thể
là một giá trị ĐƠN, hoặc lại là một danh sách con khác.

Bài sau ghép lại mọi Ý đã học trên một hình dạng ĐỆ QUY MỚI như vậy —
trước khi bước vào BOSS khép track.
::::

::::checkpoint{mastery=0.8}
::::
