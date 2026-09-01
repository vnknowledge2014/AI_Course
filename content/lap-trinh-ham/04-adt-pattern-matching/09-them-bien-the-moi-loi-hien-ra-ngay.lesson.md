---
id: lap-trinh-ham.adt-pattern-matching.them-bien-the-moi-loi-hien-ra-ngay
title: "Thêm một biến thể MỚI — lỗi hiện ra NGAY LÚC BIÊN DỊCH"
summary: "Thêm `TamGiac` vào union `Hinh` đã có `assertNever` bảo vệ, nhưng CHƯA thêm `case \"tamGiac\"` — TypeScript báo NGAY mã TS2345 tại dòng `chuaXuLy(h)`, trước khi bất kỳ dòng nào chạy. Siêu năng lực cốt lõi của ADT: mở rộng union, trình biên dịch tự tìm ra chỗ code cũ chưa theo kịp — không cần grep tay."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.exhaustiveness-catches-new-variant]
requires: [ts.assert-never-pattern]
concepts: [ts.exhaustiveness-catches-new-variant]
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
Bài trước: `assertNever` biến trình biên dịch thành người gác cổng —
thiếu một nhánh, `chuaXuLy(h)` bị chặn ngay. Nhưng `Hinh` bài trước vẫn
chỉ có đúng hai biến thể quen thuộc. Hôm nay: thêm MỘT hình dạng hoàn
toàn mới — xem người gác cổng phản ứng ra sao.
::::

::::explain{#them-tamgiac-loi-ngay}
Nhắc lại nguyên trạng bài trước — `Hinh` có hai biến thể, `switch` xử
lý đủ cả hai, `default` gọi `chuaXuLy(h)`:

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

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    default:
      return chuaXuLy(h);
  }
}

console.log(dienTich({ kind: "vuong", canh: 4 }));
console.log(dienTich({ kind: "tron", banKinh: 2 }));
```

```text
16
12.56
```

Biên dịch sạch, chạy đúng — vì `switch` xử lý đủ cả `"vuong"` lẫn
`"tron"`, nên tại `default`, kiểu của `h` bị thu hẹp còn `never` (đã
đo ở bài trước), khớp đúng tham số `chuaXuLy` đòi hỏi.

Giờ ứng dụng cần thêm hình TAM GIÁC. Thêm `interface TamGiac` và đưa
nó vào union: `type Hinh = Vuong | Tron | TamGiac;` — nhưng CHƯA sửa
`switch`, vẫn chỉ có `case "vuong"` và `case "tron"` như cũ. Chuyện gì
xảy ra?

Bên trong `default`, TypeScript giờ suy luận khác: đã loại `"vuong"`
và `"tron"` (hai `case` phía trên), khả năng CÒN LẠI của `h` không
phải `never` nữa — mà chính xác là `TamGiac`. Gọi `chuaXuLy(h)` lúc
này là đưa một giá trị kiểu `TamGiac` vào một tham số CHỈ chấp nhận
`never` — vi phạm lời hứa kiểu, đúng mã `TS2345` đã học ở T4.0a (bài
"Gọi hàm sai kiểu đối số bị chặn"), giờ xuất hiện lại trong một ngữ
cảnh cao cấp hơn: không phải một lời gọi sai kiểu viết tay, mà lộ ra
CHỈ VÌ một `switch` cũ chưa theo kịp union đã mở rộng. Đã đo thật:
thông điệp chính xác là `Argument of type 'TamGiac' is not assignable
to parameter of type 'never'.`
::::

::::example{#truoc-sau-them-tamgiac}
Thêm `TamGiac`, CHƯA sửa `switch`:

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
  canhDay: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    default:
      return chuaXuLy(h);
  }
}

console.log(dienTich({ kind: "vuong", canh: 4 }));
```

```text title=readonly
(không in ra gì cả)

TS2345 (dòng 30, cột 23): Argument of type 'TamGiac' is not assignable
to parameter of type 'never'.
```

Để ý: dòng gọi DUY NHẤT trong chương trình là `dienTich({ kind:
"vuong", canh: 4 })` — không hề có `TamGiac` thật nào được tạo ra hay
chạy qua. Không quan trọng. Lỗi bị bắt LÚC BIÊN DỊCH, nhìn vào KIỂU của
`h` tại `default`, không cần chờ chương trình chạy tới đó.

Thêm đúng `case "tamGiac"` — lỗi biến mất, không cần sửa gì khác:

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
  canhDay: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    case "tamGiac":
      return 0.5 * h.canhDay * h.chieuCao;
    default:
      return chuaXuLy(h);
  }
}

console.log(dienTich({ kind: "tamGiac", canhDay: 6, chieuCao: 4 }));
```

```text title=readonly
12
```

Không cần grep tay tìm mọi `switch` xử lý `Hinh` trong cả dự án, không
cần tự nhắc "tôi đã sửa hết chưa" — chỉ cần THÊM biến thể vào union,
trình biên dịch TỰ CHỈ đúng dòng còn thiếu.
::::

::::predict{#doan-loi-bien-dich-hay-khong commitOnce}
```typescript
interface DonChoXuLy {
  kind: "choXuLy";
  ma: string;
}

interface DonDaGiao {
  kind: "daGiao";
  ma: string;
}

interface DonDaHoanTien {
  kind: "daHoanTien";
  ma: string;
  soTienHoan: number;
}

type Don = DonChoXuLy | DonDaGiao | DonDaHoanTien;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function moTa(d: Don): string {
  switch (d.kind) {
    case "choXuLy":
      return "đang chờ xử lý";
    case "daGiao":
      return "đã giao";
    default:
      return chuaXuLy(d);
  }
}

console.log(moTa({ kind: "choXuLy", ma: "DH-9" }));
```

`Don` vừa được thêm biến thể `DonDaHoanTien`, nhưng `switch` chỉ mới
xử lý `"choXuLy"` và `"daGiao"`. Dòng gọi DUY NHẤT trong chương trình
truyền vào một `DonChoXuLy`. Chương trình trên biên dịch được không?

:::opt{correct}
Không — bị chặn NGAY LÚC BIÊN DỊCH tại dòng `chuaXuLy(d)` (TS2345), dù
chương trình không hề gọi `moTa` với một `DonDaHoanTien` nào
:::

:::opt
Biên dịch được, in ra `đang chờ xử lý` — vì dòng gọi thực tế chỉ
truyền vào `DonChoXuLy`, TypeScript chỉ kiểm NHÁNH THỰC SỰ được chạy
qua, bỏ qua nhánh `default` vì chương trình không đi tới đó
::why
Gần đúng ở việc bạn đọc đúng lời gọi duy nhất trong chương trình
(`moTa({ kind: "choXuLy", ... })`) — NẾU mã này biên dịch qua được,
kết quả chạy đúng là `"đang chờ xử lý"` thật.

Chỗ lệch: TypeScript kiểm KIỂU lúc BIÊN DỊCH cho MỌI nhánh của
`switch`, bất kể lúc CHẠY chương trình có đi qua nhánh đó hay không —
không có khái niệm "nhánh chưa từng được gọi thì bỏ qua kiểm kiểu". Đã
đo thật: chương trình bị TS2345 ngay tại dòng `chuaXuLy(d)`, TRƯỚC KHI
bất kỳ dòng JavaScript nào được sinh ra hay chạy.
::
:::

:::opt
Biên dịch được, nhưng khi CHẠY dòng `console.log(moTa(...))` chương
trình sẽ ném `Error` — vì thân hàm `chuaXuLy` luôn `throw`, bất kể `x`
là gì
::why
Gần đúng ở việc bạn nhớ đúng thân hàm `chuaXuLy` CÓ `throw new
Error(...)` — đúng, đó là những gì nó làm NẾU thật sự được gọi tới lúc
chạy.

Chỗ lệch: câu hỏi không phải "chuaXuLy làm gì khi chạy" — chương trình
này KHÔNG BAO GIỜ tới lúc chạy, vì `Don` giờ có BA biến thể mà `switch`
chỉ xử lý HAI (thiếu `case "daHoanTien"`), khiến `d` tại `default`
không còn là `never`. TypeScript từ chối biên dịch NGAY, không dòng
JavaScript nào được sinh ra — TS2345 xảy ra trước khi có runtime nào
cả, không phải một `Error` ném ra lúc chạy.
::
:::
::::

::::code{#them-case-tamgiac}
`dienTich(h)` đã có `assertNever` bảo vệ, xử lý đủ `"vuong"` và
`"tron"`. `TamGiac` vừa được thêm vào `Hinh`, nhánh `case "tamGiac"`
đã CÓ SẴN — nhưng công thức tính diện tích còn thiếu. Điền đúng để
`chuaXuLy(h)` không còn báo TS2345, và diện tích tính đúng: nửa đáy
nhân chiều cao.

```typescript title=starter
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
  canhDay: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    case "tamGiac":
      return ___;
    default:
      return chuaXuLy(h);
  }
}

console.log(dienTich({ kind: "tamGiac", canhDay: 6, chieuCao: 4 }));
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

interface TamGiac {
  kind: "tamGiac";
  canhDay: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

function chuaXuLy(x: never): never {
  throw new Error("chưa xử lý: " + JSON.stringify(x));
}

function dienTich(h: Hinh): number {
  switch (h.kind) {
    case "vuong":
      return h.canh * h.canh;
    case "tron":
      return 3.14 * h.banKinh * h.banKinh;
    case "tamGiac":
      return 0.5 * h.canhDay * h.chieuCao;
    default:
      return chuaXuLy(h);
  }
}

console.log(dienTich({ kind: "tamGiac", canhDay: 6, chieuCao: 4 }));
```

```typescript title=test
const dienTichVuong = dienTich({ kind: "vuong", canh: 4 });
if (dienTichVuong !== 16) throw new Error("dienTich hình vuông cạnh 4 phải là 16 — đang là " + dienTichVuong);
const dienTichTron = dienTich({ kind: "tron", banKinh: 2 });
if (Math.abs(dienTichTron - 12.56) > 0.001) throw new Error("dienTich hình tròn bán kính 2 phải là 12.56 — đang là " + dienTichTron);
const dienTichTamGiac = dienTich({ kind: "tamGiac", canhDay: 6, chieuCao: 4 });
if (dienTichTamGiac !== 12) throw new Error("dienTich tam giác đáy 6 cao 4 phải là 12 — đang là " + dienTichTamGiac);
const dienTichTamGiac2 = dienTich({ kind: "tamGiac", canhDay: 10, chieuCao: 3 });
if (dienTichTamGiac2 !== 15) throw new Error("dienTich tam giác đáy 10 cao 3 phải là 15 — đang là " + dienTichTamGiac2);
```

:::hints
- kind: attention
  body: "Chỗ trống là công thức tính diện tích tam giác — dùng canhDay và chieuCao của TamGiac, không phải canh hay banKinh (những field đó không tồn tại trên TamGiac)."
- kind: strategy
  body: 'Trong nhánh case "tamGiac", h đã được TypeScript narrow thành TamGiac — đọc được h.canhDay và h.chieuCao. Diện tích tam giác = 0.5 nhân canhDay nhân chieuCao.'
- kind: one-line
  body: 'Chỗ trống là: 0.5 * h.canhDay * h.chieuCao'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "12"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đây là "siêu năng lực" cốt lõi của ADT: không cần grep tay tìm mọi
switch cũ, không cần tự nhắc "tôi đã sửa hết chưa" — cứ MỞ RỘNG union,
trình biên dịch TỰ CHỈ đúng chỗ code cũ chưa theo kịp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa THẤY cơ chế này hoạt động qua hai ví dụ cụ thể (`Hinh`, `Don`)
— cả hai lần thêm biến thể mới đều làm `chuaXuLy(h)` báo lỗi NGAY, dù
chương trình không hề chạy qua nhánh mới.

Đó có phải một QUY LUẬT — luôn luôn đúng, với BẤT KỲ union nào đã có
`assertNever` bảo vệ đủ? Hay chỉ là trùng hợp của hai ví dụ vừa xem?
Bài sau kiểm tra trực giác đó, không cần viết thêm dòng code nào.
::::

::::checkpoint{mastery=0.8}
::::
