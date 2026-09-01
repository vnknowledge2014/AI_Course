---
id: lap-trinh-ham.adt-pattern-matching.do-tong-hop-discriminated-union
title: "Đo tổng hợp: discriminated union cơ bản"
summary: "Bài code có chấm điểm sống: tự tay định nghĩa MỘT discriminated union ba biến thể (kind literal) và viết một hàm switch trên kind, mỗi nhánh trả về một chuỗi mô tả — không khái niệm mới, đo khả năng tự dựng discriminant + narrow, không chỉ đọc hiểu ví dụ có sẵn."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.review-discriminated-union]
requires: [ts.predict-narrowing]
concepts: [ts.review-discriminated-union]
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
Bài trước bạn ĐOÁN đúng nhánh nào được narrow, field nào hợp lệ ở
đâu. Hôm nay không đoán nữa — tự tay DỰNG cả discriminant lẫn switch,
từ đầu.
::::

::::explain{#on-lai-ba-manh-ghep}
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

Ba mảnh ghép của bốn bài vừa qua, ghép lại thành MỘT: mỗi `interface`
có field `kind` kiểu LITERAL (bài 2) — một CHUỖI CỤ THỂ, không phải
`string` chung chung. `switch (h.kind)` so khớp giá trị đó (bài 3, bài
4) — bên trong TỪNG `case`, TypeScript tự NARROW `h` thành ĐÚNG
interface tương ứng, cho phép đọc field RIÊNG của biến thể đó
(`h.canh` chỉ hợp lệ trong `case "vuong"`, `h.banKinh` chỉ hợp lệ
trong `case "tron"` — bài 5 đã dạy đúng ranh giới này).

Không có khái niệm MỚI nào ở đây. Bài này chỉ hỏi: bạn có tự VIẾT
được cả ba mảnh — field `kind`, các `interface`, và `switch` — từ một
mô tả bài toán, không chỉ đọc hiểu mã có sẵn?
::::

::::example{#dung-cho-nhieu-viec-hon-mo-ta}
`switch` trên `kind` không chỉ dùng để MÔ TẢ — bất kỳ hàm nào cần xử
lý khác nhau theo biến thể đều dùng được cùng khuôn:

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

function chuVi(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * 4;
    case "tron":
      return Math.round(2 * Math.PI * h.banKinh);
    case "tamGiac":
      return h.canhTamGiac * 3;
  }
}

const dsHinh: Hinh[] = [
  { kind: "vuong", canh: 4 },
  { kind: "tron", banKinh: 3 },
  { kind: "tamGiac", canhTamGiac: 5 },
];

for (const h of dsHinh) {
  console.log(h.kind + ": " + chuVi(h));
}
```

```text title=readonly
vuong: 16
tron: 19
tamGiac: 15
```

`chuVi` tính CHU VI — công thức khác hẳn theo từng biến thể (`canh *
4`, chu vi tròn làm tròn, `canhTamGiac * 3`) — nhưng KHUÔN switch y
hệt `moTa` ở trên. Đây chính là lý do discriminated union hữu ích:
một khuôn xử lý, tái dùng cho bất kỳ hàm nào cần rẽ nhánh theo biến
thể.
::::

::::predict{#doan-dau-ra-trang-thai commitOnce}
```typescript
interface DangTai {
  kind: "dangTai";
}

interface ThanhCong {
  kind: "thanhCong";
  duLieu: string;
}

interface Loi {
  kind: "loi";
  thongDiepLoi: string;
}

type TrangThai = DangTai | ThanhCong | Loi;

function moTa(t: TrangThai): string {
  switch (t.kind) {
    case "dangTai":
      return "đang tải...";
    case "thanhCong":
      return "xong: " + t.duLieu;
    case "loi":
      return "lỗi: " + t.thongDiepLoi;
  }
}

console.log(moTa({ kind: "loi", thongDiepLoi: "hết thời gian chờ" }));
console.log(moTa({ kind: "thanhCong", duLieu: "42 bản ghi" }));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`lỗi: hết thời gian chờ` rồi `xong: 42 bản ghi`
:::

:::opt
`xong: 42 bản ghi` rồi `lỗi: hết thời gian chờ`
::why
Gần đúng ở việc bạn nhận ra ĐÚNG hai câu sẽ xuất hiện — một dòng bắt
đầu bằng "xong:", một dòng bắt đầu bằng "lỗi:" — bạn đọc đúng NỘI DUNG
hai nhánh `switch`.

Chỗ lệch: thứ tự IN RA đi theo ĐÚNG thứ tự các dòng `console.log`
trong mã nguồn, không theo thứ tự khai báo biến thể trong `type
TrangThai`. Dòng `console.log` ĐẦU TIÊN gọi `moTa` với `kind: "loi"`
nên in `lỗi: hết thời gian chờ` TRƯỚC; dòng THỨ HAI gọi với `kind:
"thanhCong"` nên in `xong: 42 bản ghi` SAU.
::
:::

:::opt
Máy báo lỗi biên dịch ở dòng `{ kind: "loi", thongDiepLoi: "hết thời
gian chờ" }` — vì object đó thiếu field `duLieu` mà `ThanhCong` đòi
hỏi
::why
Gần đúng ở việc bạn để ý `ThanhCong` CÓ field `duLieu` mà object này
thiếu — quan sát về CẤU TRÚC field đó đúng.

Chỗ lệch: `{ kind: "loi", thongDiepLoi: "..." }` khớp CHÍNH XÁC hình
dạng `Loi` (field `kind: "loi"` và `thongDiepLoi`) — nó là một
`TrangThai` HỢP LỆ, không phải lỗi. `TrangThai` là UNION của ba
interface, một giá trị chỉ cần khớp MỘT trong ba, không cần khớp CẢ
BA.
::
:::
::::

::::code{#mo_ta_thong_bao}
Viết hàm `moTa(t)` xử lý discriminated union `ThongBao` (ba biến thể:
`loi`, `canhBao`, `thanhCong`) — nhánh `"canhBao"` còn thiếu một
field, đọc đúng field của biến thể `CanhBao` để hoàn thành nó.

```typescript title=starter
interface Loi {
  kind: "loi";
  maLoi: string;
}

interface CanhBao {
  kind: "canhBao";
  mucDo: number;
}

interface ThanhCong {
  kind: "thanhCong";
  thongDiep: string;
}

type ThongBao = Loi | CanhBao | ThanhCong;

function moTa(t: ThongBao): string {
  switch (t.kind) {
    case "loi":
      return "lỗi " + t.maLoi;
    case "canhBao":
      return "cảnh báo mức " + t.___;
    case "thanhCong":
      return "thành công: " + t.thongDiep;
  }
}

console.log(moTa({ kind: "loi", maLoi: "E404" }));
console.log(moTa({ kind: "canhBao", mucDo: 3 }));
console.log(moTa({ kind: "thanhCong", thongDiep: "đã lưu" }));
```

```typescript title=solution
interface Loi {
  kind: "loi";
  maLoi: string;
}

interface CanhBao {
  kind: "canhBao";
  mucDo: number;
}

interface ThanhCong {
  kind: "thanhCong";
  thongDiep: string;
}

type ThongBao = Loi | CanhBao | ThanhCong;

function moTa(t: ThongBao): string {
  switch (t.kind) {
    case "loi":
      return "lỗi " + t.maLoi;
    case "canhBao":
      return "cảnh báo mức " + t.mucDo;
    case "thanhCong":
      return "thành công: " + t.thongDiep;
  }
}

console.log(moTa({ kind: "loi", maLoi: "E404" }));
console.log(moTa({ kind: "canhBao", mucDo: 3 }));
console.log(moTa({ kind: "thanhCong", thongDiep: "đã lưu" }));
```

```typescript title=test
const r1 = moTa({ kind: "loi", maLoi: "E404" });
if (r1 !== "lỗi E404") throw new Error("moTa cho Loi phải trả về \"lỗi E404\" — đang là " + r1);
const r2 = moTa({ kind: "canhBao", mucDo: 3 });
if (r2 !== "cảnh báo mức 3") throw new Error("moTa cho CanhBao(3) phải trả về \"cảnh báo mức 3\" — đang là " + r2);
const r3 = moTa({ kind: "canhBao", mucDo: 7 });
if (r3 !== "cảnh báo mức 7") throw new Error("moTa cho CanhBao(7) phải trả về \"cảnh báo mức 7\" — đang là " + r3);
const r4 = moTa({ kind: "thanhCong", thongDiep: "đã lưu" });
if (r4 !== "thành công: đã lưu") throw new Error("moTa cho ThanhCong phải trả về \"thành công: đã lưu\" — đang là " + r4);
```

:::hints
- kind: attention
  body: "Chỗ trống là field bạn đọc RA để mô tả mức cảnh báo — interface CanhBao có field nào ngoài kind?"
- kind: strategy
  body: 'interface CanhBao khai kind: "canhBao" và mucDo: number — bên trong nhánh case "canhBao", TypeScript đã NARROW t thành CanhBao, field còn lại để đọc là mucDo.'
- kind: one-line
  body: "Chỗ trống là: mucDo"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "cảnh báo mức 3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Discriminant, narrow, switch — bạn vừa tự tay dựng cả ba, không chỉ
đọc hiểu ví dụ có sẵn. Cụm 1 khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`switch` của bạn vừa rồi xử lý ĐỦ cả ba biến thể — vì bạn TỰ NHỚ viết
đủ ba `case`. Nhưng nếu một ngày bạn QUÊN một nhánh — không viết
`case` cho nó, không có `default` — TypeScript có báo gì không? Hay
mã vẫn biên dịch bình thường, và lỗi chỉ lộ ra lúc CHẠY, đúng với dữ
liệu bị bỏ sót đó?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
