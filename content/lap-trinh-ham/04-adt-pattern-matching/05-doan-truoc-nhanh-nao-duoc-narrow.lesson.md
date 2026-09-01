---
id: lap-trinh-ham.adt-pattern-matching.doan-truoc-nhanh-nao-duoc-narrow
title: "Dự đoán nhánh nào được thu hẹp kiểu"
summary: "Một `switch (h.kind)` ba nhánh trên `Hinh` (`vuong`/`tron`/`tamGiac`) — nhánh nào cho đọc field nào? Mỗi `case` chỉ mở field của ĐÚNG biến thể đang đứng; đọc field của biến thể khác bị TypeScript chặn ngay lúc biên dịch, không đợi tới lúc chạy."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ts.predict-narrowing]
requires: [ts.discriminant-switch-narrowing]
concepts: [ts.predict-narrowing]
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
Bài trước: `switch (h.kind)` thay cho chuỗi `if/else if` dài — gọn hơn khi
`Hinh` có từ ba biến thể trở lên. Nhưng gọn không có nghĩa là DỄ ĐOÁN:
mỗi `case` mở ra field KHÁC NHAU. Đoán trước — trước khi máy nói cho
bạn biết.
::::

::::explain{#switch-thu-hep-tung-nhanh}
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
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function moTa(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "hình vuông cạnh " + h.canh;
    case "tron":
      return "hình tròn bán kính " + h.banKinh;
    case "tamGiac":
      return "hình tam giác cạnh " + h.canhTamGiac;
  }
}

console.log(moTa({ kind: "vuong", canh: 4 }));
console.log(moTa({ kind: "tron", banKinh: 3 }));
console.log(moTa({ kind: "tamGiac", canhTamGiac: 5 }));
```

```text
hình vuông cạnh 4
hình tròn bán kính 3
hình tam giác cạnh 5
```

Mỗi `case` trong `switch (h.kind)` thu hẹp `h` theo ĐÚNG literal đang
so khớp — không phải một kiểu `Hinh` rộng xuyên suốt cả hàm. Bên trong
`case "vuong"`, TypeScript đã đo `h.kind === "vuong"` và suy `h` CHẮC
CHẮN là `Vuong`: đọc `h.canh` được, nhưng `h.banKinh` và
`h.canhTamGiac` KHÔNG tồn tại trên `Vuong` — đọc chúng ở nhánh này bị
chặn. Tương tự, `case "tron"` chỉ mở `h.banKinh`; `case "tamGiac"` chỉ
mở `h.canhTamGiac`. BA nhánh, BA kiểu thu hẹp RIÊNG BIỆT.
::::

::::example{#truong-chung-doc-duoc-moi-nhanh}
Field `kind` — chính field dùng để phân biệt — lại đọc được ở CẢ BA
nhánh, không như `canh`/`banKinh`/`canhTamGiac` chỉ có ở MỘT biến thể:

```typescript title=readonly
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
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function moTaChiTiet(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return h.kind + ": canh=" + h.canh;
    case "tron":
      return h.kind + ": banKinh=" + h.banKinh;
    case "tamGiac":
      return h.kind + ": canhTamGiac=" + h.canhTamGiac;
  }
}

console.log(moTaChiTiet({ kind: "vuong", canh: 4 }));
console.log(moTaChiTiet({ kind: "tron", banKinh: 3 }));
console.log(moTaChiTiet({ kind: "tamGiac", canhTamGiac: 6 }));
```

```text title=readonly
vuong: canh=4
tron: banKinh=3
tamGiac: canhTamGiac=6
```

`h.kind` đọc được ở MỌI nhánh, không cần thu hẹp thêm gì — vì `kind`
CÓ MẶT trên cả `Vuong`, `Tron`, `TamGiac`, KHÔNG RIÊNG biến thể nào.
Ranh giới chỉ xuất hiện với field CHỈ THUỘC một biến thể: field đó
chỉ mở ra ở ĐÚNG nhánh `case` khớp với biến thể đang giữ nó.
::::

::::predict{#doan-nhanh-tron-doc-canh commitOnce}
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
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiac;

function moTa(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "hình vuông cạnh " + h.canh;
    case "tron":
      return "hình tròn bán kính " + h.canh;
    case "tamGiac":
      return "hình tam giác cạnh " + h.canhTamGiac;
  }
}

console.log(moTa({ kind: "tron", banKinh: 3 }));
```

Dòng cuối in ra gì?

:::opt{correct}
Không dòng nào được in ra — TypeScript từ chối BIÊN DỊCH ngay tại
`case "tron"`, báo lỗi TS2339 (`h.canh` không tồn tại trên kiểu
`Tron`), chương trình không chạy tới `console.log`
:::

:::opt
`hình tròn bán kính undefined` — object `{ kind: "tron", banKinh: 3 }`
không có field `canh`, nên `h.canh` trả về `undefined`, nối vào chuỗi
::why
Gần đúng ở việc bạn nhận ra object đó THẬT SỰ không có field `canh` —
quan sát về CẤU TRÚC dữ liệu đó đúng.

Chỗ lệch: đó là ngữ nghĩa JavaScript THUẦN (đọc field không tồn tại
trả `undefined`, không lỗi gì). Nhưng đây là mã TypeScript — trình
biên dịch kiểm KIỂU TRƯỚC KHI CHẠY. Trong `case "tron"`, `h` đã bị
`switch (h.kind)` thu hẹp thành kiểu `Tron`, mà `Tron` KHÔNG khai field
`canh` — TypeScript CHẶN ngay lúc biên dịch (TS2339), chương trình
KHÔNG BAO GIỜ chạy tới `console.log` để có cơ hội in ra `undefined`.
::
:::

:::opt
Biên dịch bình thường và in ra `hình tròn bán kính 3` — vì tham số
khai `h: Hinh` (hợp cả ba biến thể), nên field của BẤT KỲ biến thể nào
trong `Hinh` cũng đọc được ở BẤT KỲ nhánh nào
::why
Gần đúng ở việc bạn nhớ đúng CHỮ KÝ hàm: `h` được khai `Hinh`, hợp cả
ba biến thể — quan sát đó đúng.

Chỗ lệch: `switch (h.kind)` không chỉ ĐIỀU HƯỚNG luồng chạy — nó còn
THU HẸP kiểu của `h` bên trong TỪNG `case`, đúng như bài trước đã học.
Bên trong `case "tron"`, TypeScript đã đo `h.kind === "tron"` và suy
`h` CHẮC CHẮN là `Tron`, KHÔNG còn là `Hinh` rộng nữa trong phạm vi
nhánh đó. `canh` có mặt ở biến thể KHÁC (`Vuong`) không giúp gì — kiểu
đã thu hẹp về `Tron`, không khai `canh`, nên `h.canh` bị chặn (TS2339).
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn vừa đoán ĐÚNG ranh giới: `switch` không mở TOÀN BỘ `Hinh` ra cho
mọi nhánh — nó CHIA NHỎ theo từng `case`, mỗi nhánh chỉ thấy ĐÚNG biến
thể của mình.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa ĐỌC mã người khác viết sẵn và đoán đúng nhánh nào cho field
nào. Nhưng đọc hiểu ví dụ có sẵn khác với TỰ TAY dựng một discriminated
union từ đầu — khai interface, gắn field `kind`, viết `switch` narrow —
không có ai viết mẫu trước.

Bài sau: không đọc mã ai viết sẵn nữa. Tự bạn viết.
::::

::::checkpoint{mastery=0.8}
::::
