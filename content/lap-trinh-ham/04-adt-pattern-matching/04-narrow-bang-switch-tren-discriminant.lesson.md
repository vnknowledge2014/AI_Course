---
id: lap-trinh-ham.adt-pattern-matching.narrow-bang-switch-tren-discriminant
title: "Thu hẹp kiểu bằng `switch` — gọn hơn NHIỀU biến thể"
summary: "switch (h.kind) { case \"vuong\": ...h.canh...; case \"tron\": ...h.banKinh... } — mỗi case NARROW kiểu y hệt if, nhưng đọc gọn hơn HẲN khi có BA biến thể trở lên (chuỗi if/else if dài dần rất khó đọc). Đây CHÍNH LÀ 'pattern matching' của TypeScript — không có từ khoá match như Rust, nhưng switch trên discriminant làm ĐÚNG việc đó."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ts.discriminant-switch-narrowing]
requires: [ts.discriminant-if-narrowing]
concepts: [ts.discriminant-switch-narrowing]
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
Bài trước: `if (h.kind === "vuong")` tự thu hẹp kiểu, đúng như bạn đã
đoán. Nhưng `Hinh` có tới BA biến thể — nối thêm `else if`, rồi `else
if` nữa, có thực sự là cách viết TỐT NHẤT không?
::::

::::explain{#switch-tren-discriminant}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiacDeu {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiacDeu;

function moTa(h: Hinh): string {
  switch (h.kind) {
    case "vuong":
      return "hình vuông cạnh " + h.canh;
    case "tron":
      return "hình tròn bán kính " + h.banKinh;
    case "tamGiac":
      return "hình tam giác đều cạnh " + h.canhTamGiac;
  }
}

console.log(moTa({ kind: "vuong", canh: 4 }));
console.log(moTa({ kind: "tron", banKinh: 3 }));
console.log(moTa({ kind: "tamGiac", canhTamGiac: 5 }));
```

```text
hình vuông cạnh 4
hình tròn bán kính 3
hình tam giác đều cạnh 5
```

`switch (h.kind)` so khớp giá trị của `h.kind` với từng nhãn `case`.
Bên TRONG mỗi nhánh, TypeScript NARROW kiểu của `h` — y hệt cơ chế
`if (h.kind === "vuong")` bài trước, chỉ khác cách viết. Ở nhánh
`case "vuong":`, `h` được thu hẹp còn `Vuong`, nên `h.canh` đọc được;
ở nhánh `case "tron":`, `h` thu hẹp còn `Tron`, `h.banKinh` đọc được;
tương tự với `case "tamGiac":` và `h.canhTamGiac`. Không có phép màu
nào khác — vẫn LÀ cơ chế narrow theo kiểu literal của field `kind`,
`switch` chỉ là một CÚ PHÁP khác để viết ra cùng một phép so khớp.

Đây CHÍNH LÀ "pattern matching" của TypeScript. Rust có từ khoá `match`
riêng cho việc này; TypeScript KHÔNG có từ khoá đó — nhưng `switch`
trên một discriminant (field `kind` kiểu literal) làm ĐÚNG việc mà
`match` làm: so khớp giá trị, rồi thu hẹp kiểu theo đúng nhánh khớp.
::::

::::example{#so-sanh-if-else-va-switch}
Cùng MỘT logic như trên, nhưng viết bằng chuỗi `if/else if` — để thấy
rõ vì sao `switch` đọc gọn hơn khi có từ ba biến thể trở lên:

```typescript title=readonly
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiacDeu {
  kind: "tamGiac";
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiacDeu;

function moTaBangIf(h: Hinh): string {
  if (h.kind === "vuong") {
    return "hình vuông cạnh " + h.canh;
  } else if (h.kind === "tron") {
    return "hình tròn bán kính " + h.banKinh;
  } else if (h.kind === "tamGiac") {
    return "hình tam giác đều cạnh " + h.canhTamGiac;
  } else {
    return "không rõ";
  }
}

console.log(moTaBangIf({ kind: "vuong", canh: 4 }));
console.log(moTaBangIf({ kind: "tron", banKinh: 3 }));
console.log(moTaBangIf({ kind: "tamGiac", canhTamGiac: 5 }));
```

```text title=readonly
hình vuông cạnh 4
hình tròn bán kính 3
hình tam giác đều cạnh 5
```

Kết quả GIỐNG HỆT `moTa` viết bằng `switch` ở trên — hai cách viết
NARROW y hệt nhau, cùng đúng. Khác biệt nằm ở chỗ ĐỌC: `if/else if`
phải lặp lại `h.kind === ...` ở MỖI nhánh, còn thừa một nhánh `else`
"phòng hờ" không rõ ứng với biến thể nào. `switch (h.kind)` viết điều
kiện so khớp đúng MỘT LẦN ở đầu, mỗi `case` chỉ còn lại đúng phần THÂN
xử lý — với hai biến thể, khác biệt còn nhỏ; với ba biến thể trở lên
(và ADT thật thường có nhiều hơn ba), `switch` đọc rõ ràng hơn HẲN.
::::

::::predict{#doan-ket-qua-switch commitOnce}
```typescript
interface DonMoi {
  kind: "moi";
  ma: string;
}

interface DonDangGiao {
  kind: "dangGiao";
  ma: string;
  nguoiGiao: string;
}

interface DonDaGiao {
  kind: "daGiao";
  ma: string;
}

type Don = DonMoi | DonDangGiao | DonDaGiao;

function trangThai(d: Don): string {
  switch (d.kind) {
    case "moi":
      return "đơn " + d.ma + ": mới tạo";
    case "dangGiao":
      return "đơn " + d.ma + ": đang giao bởi " + d.nguoiGiao;
    case "daGiao":
      return "đơn " + d.ma + ": đã giao xong";
  }
}

console.log(trangThai({ kind: "dangGiao", ma: "DH-9", nguoiGiao: "Minh" }));
console.log(trangThai({ kind: "daGiao", ma: "DH-2" }));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`đơn DH-9: đang giao bởi Minh` rồi `đơn DH-2: đã giao xong`
:::

:::opt
Máy báo lỗi biên dịch tại `d.nguoiGiao` trong `case "dangGiao"` — vì
`switch` không tự biết đang ở nhánh nào, phải ép kiểu tay
(`as DonDangGiao`) trước khi đọc field đó
::why
Gần đúng ở việc bạn nhớ `d` được khai kiểu `Don` (union rộng) ngay đầu
hàm — đúng, tham số hàm là `Don`, không phải `DonDangGiao` cụ thể.

Chỗ lệch: `switch (d.kind)` cùng nhãn `case "dangGiao":` đã LÀM ĐÚNG
việc so khớp giá trị literal — TypeScript tự NARROW `d` thành
`DonDangGiao` bên trong nhánh đó, giống hệt cơ chế `if` đã học, không
cần `as` nào cả. Mã biên dịch bình thường, không có lỗi TS2339 hay bất
kỳ lỗi nào khác.
::
:::

:::opt
`đơn DH-9: đang giao bởi Minh đơn DH-9: đã giao xong` — vì các `case`
ở đây không có từ khoá `break`, nên "rơi" (fallthrough) xuống nhánh kế
tiếp
::why
Gần đúng ở việc bạn để ý các `case` ở đây KHÔNG viết `break` — quan sát
đúng, cú pháp `switch` thông thường CẦN `break` để tránh rơi tự do
xuống `case` sau.

Chỗ lệch: mỗi `case` ở đây dùng `return`, mà `return` thoát khỏi NGAY
CẢ HÀM (không chỉ `switch`) — không cần `break` đi kèm, không có
chuyện rơi xuống nhánh sau. Fallthrough chỉ xảy ra khi một `case` kết
thúc bằng câu lệnh thường (không `return`, không `break`).
::
:::
::::

::::code{#hien_thi_thong_bao}
Viết `hienThi(tb)` — với `tb.kind === "canhBao"`, trả về chuỗi
`"CANH BAO muc "` nối với ĐÚNG field mức độ của `CanhBao`.

```typescript title=starter
interface ThanhCong {
  kind: "thanhCong";
  thongDiep: string;
}

interface Loi {
  kind: "loi";
  maLoi: number;
}

interface CanhBao {
  kind: "canhBao";
  mucDo: number;
}

type ThongBao = ThanhCong | Loi | CanhBao;

function hienThi(tb: ThongBao): string {
  switch (tb.kind) {
    case "thanhCong":
      return "OK: " + tb.thongDiep;
    case "loi":
      return "LOI " + tb.maLoi;
    case "canhBao":
      return "CANH BAO muc " + tb.___;
  }
}

console.log(hienThi({ kind: "thanhCong", thongDiep: "luu xong" }));
console.log(hienThi({ kind: "loi", maLoi: 404 }));
console.log(hienThi({ kind: "canhBao", mucDo: 2 }));
```

```typescript title=solution
interface ThanhCong {
  kind: "thanhCong";
  thongDiep: string;
}

interface Loi {
  kind: "loi";
  maLoi: number;
}

interface CanhBao {
  kind: "canhBao";
  mucDo: number;
}

type ThongBao = ThanhCong | Loi | CanhBao;

function hienThi(tb: ThongBao): string {
  switch (tb.kind) {
    case "thanhCong":
      return "OK: " + tb.thongDiep;
    case "loi":
      return "LOI " + tb.maLoi;
    case "canhBao":
      return "CANH BAO muc " + tb.mucDo;
  }
}

console.log(hienThi({ kind: "thanhCong", thongDiep: "luu xong" }));
console.log(hienThi({ kind: "loi", maLoi: 404 }));
console.log(hienThi({ kind: "canhBao", mucDo: 2 }));
```

```typescript title=test
const t1 = hienThi({ kind: "thanhCong", thongDiep: "luu xong" });
if (t1 !== "OK: luu xong") throw new Error("hienThi phải trả về \"OK: luu xong\" cho thanhCong — đang là " + t1);
const t2 = hienThi({ kind: "loi", maLoi: 404 });
if (t2 !== "LOI 404") throw new Error("hienThi phải trả về \"LOI 404\" cho loi — đang là " + t2);
const t3 = hienThi({ kind: "canhBao", mucDo: 2 });
if (t3 !== "CANH BAO muc 2") throw new Error("hienThi phải trả về \"CANH BAO muc 2\" cho canhBao — đang là " + t3);
```

:::hints
- kind: attention
  body: "Chỗ trống nằm trong nhánh case \"canhBao\" — bên trong nhánh đó, tb đã được narrow thành CanhBao, field mức độ của nó tên là gì?"
- kind: strategy
  body: 'interface CanhBao khai field mucDo: number — đọc field đó qua tb.mucDo, giống hệt cách case "loi" đọc tb.maLoi.'
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
  expect: "CANH BAO muc 2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`switch (h.kind)` narrow đúng như `if` — chỉ gọn hơn khi biến thể tăng
lên. Đó CHÍNH LÀ pattern matching của TypeScript, dù không có từ khoá
`match`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã thấy MỖI `case` narrow đúng field của MÌNH — `h.canh` trong
`case "vuong"`, `h.banKinh` trong `case "tron"`. Nhưng nếu, ngay TRONG
nhánh `case "tron":`, bạn thử gọi `h.canh` (field của `Vuong`, không
phải `Tron`) — TypeScript có cho qua không? Bài sau kiểm tra kỹ TỪNG
nhánh, không đoán chung chung nữa.
::::

::::checkpoint{mastery=0.8}
::::
