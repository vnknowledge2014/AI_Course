---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.phan-de-quy-lap-lai-o-moi-ham
title: "Phần ĐỆ QUY lặp lại ở MỌI hàm trên `Expr` — chỉ phép tính là khác"
summary: "tinh, demNode, chieuCao — ba hàm KHÁC việc trên CÙNG Expr, nhưng CẢ BA đều switch(e.kind), CẢ BA đều gọi lại chính mình trên e.trai/e.phai. Viết hàm đệ quy MỚI trên Expr LUÔN phải chép lại phần đệ quy — có cách tách phần đó ra không?"
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 42
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.recursion-pattern-repeats]
requires: [alg.recall-recursive-expr]
concepts: [alg.recursion-pattern-repeats]
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
`tinh` và `demNode` — hai việc HOÀN TOÀN khác. Đặt CẠNH nhau xem có
GIỐNG chỗ nào không.
::::

::::explain{#ba-ham-canh-nhau}
Thêm MỘT hàm thứ BA — `chieuCao(e: Expr): number`, đo ĐỘ SÂU của cây
(số tầng, tính từ node lá):

```typescript
type Expr =
  | { kind: "so"; giaTri: number }
  | { kind: "cong"; trai: Expr; phai: Expr }
  | { kind: "nhan"; trai: Expr; phai: Expr };

function tinh(e: Expr): number {
  switch (e.kind) {
    case "so": return e.giaTri;
    case "cong": return tinh(e.trai) + tinh(e.phai);
    case "nhan": return tinh(e.trai) * tinh(e.phai);
  }
}

function demNode(e: Expr): number {
  switch (e.kind) {
    case "so": return 1;
    case "cong": return demNode(e.trai) + demNode(e.phai) + 1;
    case "nhan": return demNode(e.trai) + demNode(e.phai) + 1;
  }
}

function chieuCao(e: Expr): number {
  switch (e.kind) {
    case "so": return 1;
    case "cong": return Math.max(chieuCao(e.trai), chieuCao(e.phai)) + 1;
    case "nhan": return Math.max(chieuCao(e.trai), chieuCao(e.phai)) + 1;
  }
}
```

BA hàm, BA việc KHÁC HẲN nhau (tính giá trị, đếm node, đo độ sâu) —
nhưng nhìn PHẦN KHUNG: CẢ BA đều `switch (e.kind)` với ĐÚNG ba nhánh
(`"so"`, `"cong"`, `"nhan"`); CẢ BA đều GỌI LẠI CHÍNH MÌNH trên
`e.trai` và `e.phai` ở nhánh `"cong"`/`"nhan"`. Phần LẶP LẠI ĐÓ —
`switch`, ba nhánh, gọi đệ quy trên hai nhánh con — KHÔNG hề đổi giữa
ba hàm. Chỉ MỘT chỗ đổi: PHÉP TÍNH áp dụng SAU khi có hai kết quả đệ
quy — `t + p` hay `t * p` (`tinh`), `t + p + 1` (`demNode`),
`Math.max(t, p) + 1` (`chieuCao`).
::::

::::example{#viet-ham-thu-tu-lai-chep-y-het}
Viết một hàm đệ quy THỨ TƯ trên `Expr` — ví dụ `demSoLe(e: Expr):
number` (đếm số LÁ mang giá trị LẺ) — LẠI phải chép nguyên phần khung:

```typescript title=readonly
function demSoLe(e: Expr): number {
  switch (e.kind) {
    case "so": return e.giaTri % 2 !== 0 ? 1 : 0;
    case "cong": return demSoLe(e.trai) + demSoLe(e.phai);
    case "nhan": return demSoLe(e.trai) + demSoLe(e.phai);
  }
}

const bieuThuc: Expr = {
  kind: "nhan",
  trai: { kind: "cong", trai: { kind: "so", giaTri: 3 }, phai: { kind: "so", giaTri: 4 } },
  phai: { kind: "so", giaTri: 5 },
};

console.log(demSoLe(bieuThuc));
```

```text title=readonly
2
```

`demSoLe(bieuThuc)` ra `2` (`3` và `5` là số lẻ, `4` là số chẵn). Vẫn
`switch (e.kind)`, vẫn gọi lại CHÍNH MÌNH trên `e.trai`/`e.phai` — LẦN
THỨ TƯ chép lại CÙNG một khung. Bốn hàm, BỐN LẦN viết phần đệ quy
GIỐNG HỆT nhau, chỉ đổi PHÉP TÍNH — mỗi hàm mới thêm vào `Expr` sẽ LẶP
LẠI y hệt điều này LẦN NỮA.
::::

::::predict{#doan-ham-thu-nam commitOnce}
Một hàm đệ quy THỨ NĂM, cùng khung — `daySoLe(e: Expr): boolean`
("có tồn tại MỘT lá số LẺ trong cây không?"):

```typescript
function daySoLe(e: Expr): boolean {
  switch (e.kind) {
    case "so": return e.giaTri % 2 !== 0;
    case "cong": return daySoLe(e.trai) || daySoLe(e.phai);
    case "nhan": return daySoLe(e.trai) || daySoLe(e.phai);
  }
}

const chiSoChan: Expr = {
  kind: "cong",
  trai: { kind: "so", giaTri: 2 },
  phai: { kind: "nhan", trai: { kind: "so", giaTri: 4 }, phai: { kind: "so", giaTri: 6 } },
};

console.log(daySoLe(chiSoChan));
```

Dòng cuối in ra gì (`chiSoChan` chỉ chứa các lá `2`, `4`, `6` — TOÀN
số chẵn)?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `daySoLe` LUÔN trả `true` ở nhánh `"cong"`/`"nhan"`, bất
kể giá trị lá bên dưới là gì
::why
Gần đúng ở việc bạn để ý nhánh `"cong"`/`"nhan"` có TRẢ VỀ một giá trị
`boolean` CỐ ĐỊNH về CẤU TRÚC (`||` giữa hai lời gọi đệ quy) — quan
sát về HÌNH DẠNG chữ ký đó đúng.

Chỗ lệch: `daySoLe(e.trai) || daySoLe(e.phai)` KHÔNG phải một hằng số
`true` — nó là PHÉP TOÁN LOGIC "hoặc", giá trị THẬT SỰ phụ thuộc kết
quả đệ quy của HAI nhánh con. Ở CẢ BA lá của `chiSoChan` (`2`, `4`,
`6`), `e.giaTri % 2 !== 0` đều `false` (toàn số CHẴN) — đệ quy đi
xuống TỚI TẬN lá, không tìm thấy lá LẺ nào, `||` giữa các `false`
VẪN là `false`.
::
:::

:::opt
Máy báo lỗi biên dịch — `daySoLe` trả về `boolean` nhưng nhánh `"so"`
lại dùng phép so sánh `%` (chia lấy dư), không hợp kiểu với `boolean`
::why
Gần đúng ở việc bạn để ý nhánh `"so"` CÓ dùng phép toán số học (`%`)
— quan sát về việc CÓ phép tính số trong nhánh đó đúng.

Chỗ lệch: `e.giaTri % 2 !== 0` là một BIỂU THỨC SO SÁNH HOÀN CHỈNH —
`e.giaTri % 2` cho ra một `number`, `!== 0` so sánh nó với `0`, kết
quả CỦA CẢ biểu thức là một `boolean` (`true`/`false`), khớp ĐÚNG kiểu
trả về `boolean` mà `daySoLe` khai báo. Biên dịch hoàn toàn bình
thường.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm hàm, năm lần viết CÙNG một khung đệ quy — chỉ phép tính đổi. Có
cách tách phần khung đó RA, viết đúng MỘT LẦN không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Câu trả lời KHÔNG bắt đầu từ phần đệ quy. Nó bắt đầu từ phần NGƯỢC
LẠI — phần THẬT SỰ khác nhau giữa các hàm.
::::

::::checkpoint{mastery=0.8}
::::
