---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.viet-foldexpr-tach-dot-de-quy-ra-rieng
title: "Viết `foldExpr` — TÁCH phần đệ quy RA RIÊNG, chạy MỘT LẦN cho mọi `BoXuLy`"
summary: "function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {...} — MỘT hàm chứa TOÀN BỘ phần đệ quy trên Expr, viết ĐÚNG MỘT LẦN, dùng lại cho BẤT KỲ BoXuLyExpr nào. Đây là catamorphism — 'gấp' một cấu trúc đệ quy thành một giá trị."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 44
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.write-fold-expr]
requires: [alg.handler-object-per-variant]
concepts: [alg.write-fold-expr]
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
Bài trước gói MỖI biến thể của `Expr` vào một `BoXuLyExpr<R>` — ba
hàm nhỏ, không hàm nào tự gọi lại chính nó. Nhưng CÓ AI đi qua cây,
gọi ĐÚNG hàm nào ở ĐÚNG node nào chưa? Chưa — `BoXuLyExpr` chỉ MÔ TẢ,
chưa THỰC THI. Hôm nay viết đúng MỘT hàm làm việc đó.
::::

::::explain{#dinh-nghia-fold-expr}
```typescript
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const boXuLyTinh: BoXuLyExpr<number> = {
  so: (g) => g,
  cong: (t, p) => t + p,
  nhan: (t, p) => t * p,
};

const cay: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 2 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
};

console.log(foldExpr(cay, boXuLyTinh));
```

```text
14
```

`foldExpr<R>` nhận HAI thứ: một `Expr` (cây), một `BoXuLyExpr<R>` (bộ
mô tả). Ở node `"so"`, nó gọi thẳng `bx.so(e.giaTri)` — không có gì
để đệ quy. Ở node `"cong"`/`"nhan"`, nó GỌI LẠI CHÍNH NÓ trên `e.trai`
và `e.phai` TRƯỚC — `foldExpr(e.trai, bx)` và `foldExpr(e.phai, bx)`
— rồi đưa HAI KẾT QUẢ đó (đã là `R`, không còn là `Expr`) vào
`bx.cong` hoặc `bx.nhan`. Toàn bộ việc "đi xuống cây, quay lại, gộp
kết quả" nằm GỌN trong `foldExpr` — `bx` không biết gì về việc đệ
quy, chỉ khai TỪNG bước tính gì.

Với `cay = cong(so(2), nhan(so(3), so(4)))`, `foldExpr` gọi `bx.nhan`
trước (nhánh sâu hơn tính xong trước): `3 * 4 = 12`. Rồi gọi `bx.cong`
sau: `2 + 12 = 14` — khớp đúng output.

Hàm kiểu này có tên riêng: **catamorphism** — "gấp" (fold) một cấu
trúc đệ quy (ở đây là `Expr`) thành MỘT giá trị, theo một quy tắc gộp
cho trước (`bx`). `foldExpr` viết ĐÚNG MỘT LẦN — phần đệ quy KHÔNG
lặp lại mỗi khi có một `BoXuLyExpr` mới.
::::

::::example{#dung-lai-cho-boxuly-khac}
```typescript title=readonly
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const boXuLyHienThi: BoXuLyExpr<string> = {
  so: (g) => String(g),
  cong: (t, p) => "(" + t + "+" + p + ")",
  nhan: (t, p) => "(" + t + "*" + p + ")",
};

const cay: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 2 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
};

console.log(foldExpr(cay, boXuLyHienThi));
```

```text title=readonly
(2+(3*4))
```

CÙNG `cay`, CÙNG `foldExpr` — không sửa MỘT DÒNG nào bên trong
`foldExpr` — chỉ đổi `bx` từ `boXuLyTinh` (`R = number`, cộng/nhân
thật) sang `boXuLyHienThi` (`R = string`, ghép chuỗi có ngoặc). Kết
quả đổi hẳn kiểu (`number` thành `string`), đổi hẳn Ý NGHĨA (tính giá
trị thành hiển thị biểu thức) — nhưng `foldExpr` không hề biết, không
hề cần sửa. Đây chính xác là điều dòng tiêu đề bài này hứa: phần đệ
quy chạy MỘT LẦN, dùng lại cho MỌI `BoXuLyExpr`.
::::

::::predict{#doan-fold-so-lon-nhat commitOnce}
```typescript
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const boXuLySoLonNhat: BoXuLyExpr<number> = {
  so: (g) => g,
  cong: (t, p) => Math.max(t, p),
  nhan: (t, p) => Math.max(t, p),
};

const cay: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 3 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 5 }, phai: { kind: "so", giaTri: 2 } },
};

console.log(foldExpr(cay, boXuLySoLonNhat));
```

Dòng cuối in ra gì?

:::opt{correct}
`5`
:::

:::opt
`13` — vì `cong` cộng hai nhánh lại, `nhan` nhân hai nhánh lại, đúng
theo TÊN của chúng (`nhan(5, 2) = 10`, rồi `cong(3, 10) = 13`)
::why
Gần đúng ở việc bạn xác định ĐÚNG hình dạng của cây (`cay` là node
`cong`, nhánh trái là `so(3)`, nhánh phải là `nhan(so(5), so(2))`) —
đọc cấu trúc cây đó đúng.

Chỗ lệch: `bx.cong` và `bx.nhan` trong `boXuLySoLonNhat` KHÔNG cộng
hay nhân — CẢ HAI đều là `Math.max(t, p)`. Tên trường (`cong`, `nhan`)
chỉ là NHÃN — `foldExpr` gọi ĐÚNG HÀM đã gán cho nhãn đó, không suy
luận theo NGHĨA của tên gọi. `nhan(5, 2)` ở đây ra `Math.max(5, 2) =
5` (không phải `5 * 2 = 10`), rồi `cong(3, 5)` ra `Math.max(3, 5) =
5`.
::
:::

:::opt
Máy báo lỗi biên dịch — nhánh `phai` của node `cong` là kết quả của
`foldExpr` trên một node `nhan`, khác kiểu với kết quả trên một node
`so`, nên `bx.cong` không nhận đủ hai đối số cùng kiểu
::why
Gần đúng ở việc bạn để ý `cay` có node `nhan` LỒNG bên trong nhánh
`phai` của node `cong` — quan sát cấu trúc lồng đó đúng.

Chỗ lệch: `foldExpr<R>` LUÔN trả về CÙNG một kiểu `R`, bất kể node
đang xử lý là `so`, `cong`, hay `nhan` — cả ba nhánh của `switch` đều
khai kiểu trả về `R`. `foldExpr(e.phai, bx)` gọi trên node `nhan` vẫn
trả về `number` (giống hệt kiểu trên node `so`) — không có xung đột
kiểu nào, biên dịch sạch.
::
:::
::::

::::code{#viet_foldexpr}
Hoàn thành `foldExpr` — nhánh `"nhan"` đã viết sẵn làm mẫu, viết PHẦN
ĐỆ QUY còn thiếu cho nhánh `"cong"`.

```typescript title=starter
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(___, ___);
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const boXuLyHienThi: BoXuLyExpr<string> = {
  so: (g) => String(g),
  cong: (t, p) => "(" + t + "+" + p + ")",
  nhan: (t, p) => "(" + t + "*" + p + ")",
};

console.log(foldExpr({ kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } }, boXuLyHienThi));
```

```typescript title=solution
interface BoXuLyExpr<R> {
  so: (giaTri: number) => R;
  cong: (trai: R, phai: R) => R;
  nhan: (trai: R, phai: R) => R;
}

type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function foldExpr<R>(e: Expr, bx: BoXuLyExpr<R>): R {
  switch (e.kind) {
    case "so": return bx.so(e.giaTri);
    case "cong": return bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
    case "nhan": return bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx));
  }
}

const boXuLyHienThi: BoXuLyExpr<string> = {
  so: (g) => String(g),
  cong: (t, p) => "(" + t + "+" + p + ")",
  nhan: (t, p) => "(" + t + "*" + p + ")",
};

console.log(foldExpr({ kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } }, boXuLyHienThi));
```

```typescript title=test
const e1: Expr = { kind: "so", giaTri: 7 };
if (foldExpr(e1, boXuLyHienThi) !== "7") throw new Error("foldExpr trên node so phải trả về bx.so(giaTri)");

const e2: Expr = { kind: "cong", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } };
if (foldExpr(e2, boXuLyHienThi) !== "(2+3)") throw new Error("foldExpr trên node cong phải gọi bx.cong với kết quả đệ quy của trai rồi phai, đúng thứ tự");

const e3: Expr = {
  kind: "cong",
  trai: { kind: "nhan", trai: { kind: "so", giaTri: 2 }, phai: { kind: "so", giaTri: 3 } },
  phai: { kind: "so", giaTri: 4 },
};
if (foldExpr(e3, boXuLyHienThi) !== "((2*3)+4)") throw new Error("foldExpr phải đệ quy xuyên qua nhánh trai lồng bên trong (một node nhan bên trong cong), không chỉ xử lý một tầng");

const boXuLyTinh: BoXuLyExpr<number> = {
  so: (g) => g,
  cong: (t, p) => t + p,
  nhan: (t, p) => t * p,
};
if (foldExpr(e3, boXuLyTinh) !== 10) throw new Error("foldExpr phải dùng lại được với một BoXuLyExpr khác (boXuLyTinh) mà không sửa foldExpr");
```

:::hints
- kind: attention
  body: "Nhánh \"cong\" phải GỌI LẠI foldExpr trên e.trai và e.phai TRƯỚC, rồi đưa HAI KẾT QUẢ đó (không phải e.trai/e.phai thô) vào bx.cong. Nhìn nhánh \"nhan\" đã viết sẵn để thấy đúng khuôn."
- kind: strategy
  body: "Nhánh \"nhan\" đã có: bx.nhan(foldExpr(e.trai, bx), foldExpr(e.phai, bx)). Nhánh \"cong\" đi CÙNG khuôn — chỉ đổi bx.nhan thành bx.cong, giữ nguyên foldExpr(e.trai, bx) và foldExpr(e.phai, bx)."
- kind: one-line
  body: "bx.cong(foldExpr(e.trai, bx), foldExpr(e.phai, bx))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "(2+3)"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`foldExpr` viết ĐÚNG MỘT LẦN — bất kỳ `BoXuLyExpr` mới nào, kiểu `R`
nào, đều DÙNG LẠI được nó, không đụng lại phần đệ quy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài 41 viết `tinh(e: Expr): number` bằng tay — `switch`, tự gọi lại
chính nó trên `e.trai`/`e.phai`. Giờ đã có `foldExpr` CHỨA SẴN phần
đệ quy đó. `tinh` có còn cần `switch`, còn cần tự gọi lại chính nó
không — hay chỉ cần MỘT `BoXuLyExpr<number>` mô tả "với `so`, giữ
nguyên; với `cong`/`nhan`, cộng/nhân hai kết quả"?

Bài sau viết lại `tinh` bằng CHÍNH `foldExpr` vừa xong.
::::

::::checkpoint{mastery=0.8}
::::
