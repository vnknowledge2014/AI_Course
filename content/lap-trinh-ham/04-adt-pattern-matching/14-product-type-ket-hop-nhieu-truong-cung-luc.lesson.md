---
id: lap-trinh-ham.adt-pattern-matching.product-type-ket-hop-nhieu-truong-cung-luc
title: "Product type — kết hợp NHIỀU trường CÙNG LÚC"
summary: "interface NguoiDung { ten: string; tuoi: number; email: string } — một giá trị thuộc kiểu này PHẢI có CẢ BA trường CÙNG LÚC. Gọi là 'product type' (kiểu TÍCH) vì không gian mọi giá trị có thể có là TÍCH của không gian từng trường — interface/object literal type dùng suốt track chính là product type, bài này chỉ đặt tên hình thức."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ts.product-type-concept]
requires: [ts.review-exhaustiveness]
concepts: [ts.product-type-concept]
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
Bạn vừa đo xong cả cụm exhaustiveness checking — discriminant, `switch`,
`assertNever`, thêm biến thể mới lộ lỗi ngay lúc biên dịch. Giờ lùi lại
một bước, hỏi một câu nền tảng hơn: `interface NguoiDung { ten: string;
tuoi: number; email: string }` — kiểu NÀY thuộc LOẠI gì?
::::

::::explain{#product-type-la-gi}
```typescript
interface NguoiDung {
  ten: string;
  tuoi: number;
  email: string;
}

const nguoiDung1: NguoiDung = { ten: "An", tuoi: 30, email: "an@vd.com" };

console.log(nguoiDung1.ten);
console.log(nguoiDung1.tuoi);
console.log(nguoiDung1.email);
```

```text
An
30
an@vd.com
```

Một giá trị thuộc kiểu `NguoiDung` PHẢI có CẢ BA trường `ten`, `tuoi`,
`email` CÙNG LÚC — không thể chỉ có hai trong ba, không thể "tạm để
trống" một trường rồi bổ sung sau. Kiểu này gọi là **product type**
(kiểu TÍCH): không gian MỌI giá trị có thể có của `NguoiDung` chính là
TÍCH (Cartesian product) của không gian từng trường riêng lẻ — mọi
`ten` khả dĩ, NHÂN với mọi `tuoi` khả dĩ, NHÂN với mọi `email` khả dĩ.

Đây không phải một khái niệm MỚI — `interface` và object literal type
mà track này đã dùng từ bài đầu tiên (`Vuong`, `Tron`, `DonChoXuLy`,
mọi kiểu bạn từng khai) ĐỀU là product type. Bài này chỉ ĐẶT TÊN HÌNH
THỨC cho một thứ bạn đã quen tay từ lâu.
::::

::::example{#tich-doc-lap-tung-truong}
Ba giá trị `NguoiDung`, mỗi giá trị đổi ĐÚNG MỘT trường so với giá trị
còn lại — cả ba vẫn đều HỢP LỆ:

```typescript title=readonly
interface NguoiDung {
  ten: string;
  tuoi: number;
  email: string;
}

const nd1: NguoiDung = { ten: "An", tuoi: 30, email: "a@vd.com" };
const nd2: NguoiDung = { ten: "An", tuoi: 30, email: "b@vd.com" };
const nd3: NguoiDung = { ten: "An", tuoi: 25, email: "a@vd.com" };

console.log(nd1.email === nd2.email);
console.log(nd1.tuoi === nd3.tuoi);
```

```text title=readonly
false
false
```

`nd1` và `nd2` CÙNG `ten`/`tuoi`, khác `email` — vẫn là hai `NguoiDung`
hợp lệ, KHÁC nhau. `nd1` và `nd3` CÙNG `ten`/`email`, khác `tuoi` —
cũng vậy. Không có RÀNG BUỘC CHÉO nào giữa ba trường: đổi trường này
không bắt buộc đổi trường kia. Chính vì các trường ĐỘC LẬP như vậy,
tổng số giá trị khả dĩ mới NHÂN lên với nhau — đúng ý nghĩa của TÍCH.
::::

::::predict{#doan-thieu-truong-loi-bien-dich commitOnce}
```typescript
interface NguoiDung {
  ten: string;
  tuoi: number;
  email: string;
}

const nd: NguoiDung = { ten: "An", tuoi: 30 };

console.log(nd.ten);
```

Dòng cuối in ra gì?

:::opt{correct}
Máy báo lỗi biên dịch — object literal thiếu field `email` mà
`NguoiDung` đòi hỏi
:::

:::opt
`An` — vì `console.log(nd.ten)` chỉ ĐỌC field `ten`, không đụng tới
`email`, nên thiếu `email` không ảnh hưởng gì
::why
Gần đúng ở việc bạn nhận ra `console.log(nd.ten)` chỉ ĐỌC field `ten`,
không hề dùng tới `email` — quan sát đó đúng về mặt LOGIC lúc chạy.

Chỗ lệch: TypeScript kiểm tra kiểu ngay tại BƯỚC GÁN GIÁ TRỊ (`const
nd: NguoiDung = { ... }`), không phải tại bước ĐỌC field về sau. Object
literal thiếu `email` so với `NguoiDung` bị CHẶN NGAY tại dòng gán —
mã TS2741 ("Property 'email' is missing in type ... but required in
type 'NguoiDung'") — không dòng nào phía sau được chạy, kể cả
`console.log(nd.ten)` tưởng như vô hại vì không đụng tới field thiếu.
::
:::

:::opt
`undefined` — vì field `email` không được gán nên tự động nhận giá trị
`undefined`, và dòng lệnh vẫn chạy tiếp bình thường
::why
Gần đúng ở việc bạn nghĩ tới cách JavaScript THƯỜNG xử lý field thiếu
trên một object không khai kiểu (trả về `undefined` khi đọc field
không tồn tại) — đúng với object THƯỜNG.

Chỗ lệch: `nd` được khai TƯỜNG MINH là kiểu `NguoiDung`, mà
`NguoiDung.email` là trường BẮT BUỘC (không phải `email?: string`
optional). TypeScript không cho phép "để trống rồi tự nhận
`undefined`" cho một trường bắt buộc — nó CHẶN việc gán ngay lúc biên
dịch, không có cơ hội nào để chương trình chạy tới `console.log` và lộ
ra giá trị `undefined` đó.
::
:::
::::

::::code{#tao_nguoi_dung}
Viết `taoNguoiDung(ten, tuoi, email)` — trả về một `NguoiDung` với CẢ
BA trường đúng giá trị truyền vào.

```typescript title=starter
interface NguoiDung {
  ten: string;
  tuoi: number;
  email: string;
}

function taoNguoiDung(ten: string, tuoi: number, email: string): NguoiDung {
  return { ten: ten, tuoi: tuoi, email: ___ };
}

console.log(taoNguoiDung("An", 30, "an@vd.com").email);
```

```typescript title=solution
interface NguoiDung {
  ten: string;
  tuoi: number;
  email: string;
}

function taoNguoiDung(ten: string, tuoi: number, email: string): NguoiDung {
  return { ten: ten, tuoi: tuoi, email: email };
}

console.log(taoNguoiDung("An", 30, "an@vd.com").email);
```

```typescript title=test
const nd1 = taoNguoiDung("An", 30, "an@vd.com");
if (nd1.ten !== "An") throw new Error("taoNguoiDung phải có ten là \"An\" — đang là " + nd1.ten);
if (nd1.tuoi !== 30) throw new Error("taoNguoiDung phải có tuoi là 30 — đang là " + nd1.tuoi);
if (nd1.email !== "an@vd.com") throw new Error("taoNguoiDung phải có email là \"an@vd.com\" — đang là " + nd1.email);
const nd2 = taoNguoiDung("Binh", 25, "binh@vd.com");
if (nd2.ten !== "Binh") throw new Error("taoNguoiDung(\"Binh\", 25, \"binh@vd.com\") phải có ten là \"Binh\" — đang là " + nd2.ten);
if (nd2.tuoi !== 25) throw new Error("taoNguoiDung(\"Binh\", 25, \"binh@vd.com\") phải có tuoi là 25 — đang là " + nd2.tuoi);
if (nd2.email !== "binh@vd.com") throw new Error("taoNguoiDung(\"Binh\", 25, \"binh@vd.com\") phải có email là \"binh@vd.com\" — đang là " + nd2.email);
```

:::hints
- kind: attention
  body: "Chỗ trống là GIÁ TRỊ gán cho field email trong object trả về — ten và tuoi đã được điền sẵn (ten: ten, tuoi: tuoi), email cần điền tương tự."
- kind: strategy
  body: "Hàm nhận tham số email — object trả về phải LẤY đúng giá trị tham số đó, không phải một chuỗi cố định nào khác. Cả ba trường của product type phải CÙNG có mặt trong giá trị trả về."
- kind: one-line
  body: "Chỗ trống là: email"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "an@vd.com"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn vừa đặt tên chính thức cho khái niệm quen thuộc — product type: một
giá trị PHẢI có ĐỦ mọi trường CÙNG LÚC. Nhưng cái `Hinh = Vuong | Tron`
bạn dùng suốt track này — một giá trị CHỈ cần là Vuong HOẶC Tron, không
phải cả hai — đó là loại type KHÁC hẳn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`NguoiDung` bắt bạn có CẢ ten LẪN tuổi LẪN email — kiểu TÍCH. Còn
`Hinh = Vuong | Tron` chỉ bắt bạn có MỘT trong hai, không cần cả hai
cùng lúc. Nếu product type là phép NHÂN không gian giá trị, thì kiểu
union này — phép gì?
::::

::::checkpoint{mastery=0.8}
::::
