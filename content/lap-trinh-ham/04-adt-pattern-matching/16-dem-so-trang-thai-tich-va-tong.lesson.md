---
id: lap-trinh-ham.adt-pattern-matching.dem-so-trang-thai-tich-va-tong
title: "Đếm số trạng thái có thể có: TÍCH và TỔNG"
summary: "`interface { a: boolean; b: boolean }` có `2 × 2 = 4` giá trị khả dĩ (product — TÍCH số lựa chọn từng trường), còn `type X = A | B | C` với mỗi biến thể chỉ `1` hình dạng có `1 + 1 + 1 = 3` giá trị khả dĩ (sum — TỔNG số biến thể). 'sum'/'product' không phải ẩn dụ — chúng đúng nghĩa CỘNG/NHÂN số trạng thái có thể có."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ts.counting-states-product-sum]
requires: [ts.sum-type-concept]
concepts: [ts.counting-states-product-sum]
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
Bài trước gọi interface là "product type", union có nhãn là "sum type" —
nghe như PHÉP TOÁN. Hôm nay: kiểm tra xem có thật là NHÂN và CỘNG
không, bằng cách ĐẾM.
::::

::::explain{#dem-trang-thai-product}
```typescript
interface CoHaiCo {
  coA: boolean;
  coB: boolean;
}

const moiGiaTri: CoHaiCo[] = [
  { coA: false, coB: false },
  { coA: false, coB: true },
  { coA: true, coB: false },
  { coA: true, coB: true },
];

console.log(moiGiaTri.length);
```

```text
4
```

`CoHaiCo` là một `interface` (bài trước gọi là "product type") với hai
trường boolean: `coA` và `coB`. Mỗi trường CÓ ĐÚNG 2 giá trị khả dĩ —
`false` hoặc `true`. Một giá trị `CoHaiCo` HỢP LỆ phải có CẢ HAI trường
CÙNG LÚC — nên số giá trị khả dĩ là số CẶP `(coA, coB)` khác nhau: chọn
`coA` (2 cách) VÀ chọn `coB` (2 cách), ĐỘC LẬP với nhau — 2 × 2 = 4,
đúng số phần tử `moiGiaTri` liệt kê ở trên, không thiếu không thừa.

Đây chính là "quy tắc nhân" trong tổ hợp (R2 đã học): khi một lựa chọn
KHÔNG PHỤ THUỘC lựa chọn kia, tổng số cách chọn CẢ HAI là TÍCH số cách
chọn từng cái riêng. "Product type" không phải một cái tên ẩn dụ hoa
mỹ — nó ĐÚNG NGHĨA phép nhân đó. Tổng quát hơn: một `interface` có N
trường, trường thứ i có `k_i` giá trị khả dĩ, thì tổng số giá trị khả
dĩ là tích của tất cả: `k_1 × k_2 × ... × k_N`.
::::

::::example{#dem-trang-thai-sum}
Giờ đến sum type (bài trước) — union có nhãn, MỖI biến thể chỉ có ĐÚNG
một hình dạng:

```typescript title=readonly
interface HinhTron {
  kind: "tron";
}

interface HinhVuong {
  kind: "vuong";
}

interface HinhTamGiac {
  kind: "tamgiac";
}

type Hinh = HinhTron | HinhVuong | HinhTamGiac;

const moiGiaTri: Hinh[] = [
  { kind: "tron" },
  { kind: "vuong" },
  { kind: "tamgiac" },
];

console.log(moiGiaTri.length);
```

```text title=readonly
3
```

`HinhTron` chỉ có MỘT hình dạng khả dĩ (`{ kind: "tron" }`) — field
`kind` chỉ nhận đúng chuỗi `"tron"` (kiểu literal, đã học ở bài 2),
không field nào khác góp thêm biến thiên. Tương tự `HinhVuong`,
`HinhTamGiac` — mỗi cái CŨNG chỉ 1 hình dạng. Một giá trị `Hinh` CHỈ
THUỘC ĐÚNG MỘT trong ba biến thể tại một thời điểm (bài trước's sum
type) — không có giá trị nào "vừa Tron vừa Vuong".

Mỗi biến thể góp đúng 1 hình dạng vào tổng, nên số giá trị khả dĩ là:
`1 + 1 + 1 = 3` — CỘNG các biến thể LOẠI TRỪ LẪN NHAU lại, không NHÂN.

Liên hệ "quy tắc cộng" trong tổ hợp (R2): chọn CÁCH 1 HOẶC CÁCH 2 HOẶC
CÁCH 3 (các cách KHÔNG chồng lấn) — tổng số cách = TỔNG số cách từng
nhánh. Đây cũng chính xác lý do "sum type" mang tên đó — không phải ẩn
dụ, mà là phép cộng thật.
::::

::::predict{#doan-so-trang-thai commitOnce}
Đoạn mã dưới liệt kê MỌI `TrangThai` khả dĩ bằng vòng lặp lồng nhau,
rồi in ra SỐ LƯỢNG:

```typescript
type CheDo = "ngay" | "dem" | "tuDong";

interface TrangThai {
  daBat: boolean;
  daKhoa: boolean;
  cheDo: CheDo;
}

const boolKhaDi = [false, true];
const cheDoKhaDi: CheDo[] = ["ngay", "dem", "tuDong"];

const tatCa: TrangThai[] = [];
for (const daBat of boolKhaDi) {
  for (const daKhoa of boolKhaDi) {
    for (const cheDo of cheDoKhaDi) {
      tatCa.push({ daBat, daKhoa, cheDo });
    }
  }
}

console.log(tatCa.length);
```

Dòng cuối in ra gì?

:::opt{correct}
`12`
:::

:::opt
`7`
::why
Gần đúng ở việc bạn ĐẾM đúng số giá trị khả dĩ của từng trường riêng
lẻ: `daBat` có 2, `daKhoa` có 2, `cheDo` có 3 — ba con số đó đúng.

Chỗ lệch: bạn CỘNG chúng lại (2 + 2 + 3 = 7) — cách cộng đó chỉ đúng
khi ba lựa chọn LOẠI TRỪ LẪN NHAU, như ba biến thể của một sum type
(bài trước). Nhưng `TrangThai` là một `interface` (product type) —
`daBat`, `daKhoa`, `cheDo` là BA trường PHẢI CÓ CÙNG LÚC, không phải ba
lựa chọn thay thế nhau. Số tổ hợp `(daBat, daKhoa, cheDo)` khác nhau là
TÍCH 2 × 2 × 3 = 12, không phải TỔNG 2 + 2 + 3 = 7.
::
:::

:::opt
`3`
::why
Gần đúng ở việc bạn đếm đúng `cheDo` có 3 giá trị khả dĩ (`"ngay" |
"dem" | "tuDong"`) — quan sát đó chính xác.

Chỗ lệch: `TrangThai` không CHỈ có trường `cheDo` — nó còn HAI trường
boolean `daBat`, `daKhoa`, MỖI trường góp phần NHÂN vào tổng số tổ hợp
(product type đòi CẢ BA trường CÙNG LÚC). Bỏ qua `daBat`, `daKhoa` làm
mất hệ số 2 × 2 = 4; số ĐÚNG là 2 × 2 × 3 = 12, không phải chỉ 3.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
"Product"/"sum" không phải tên gọi cho vui — chúng đúng nghĩa NHÂN và
CỘNG số trạng thái khả dĩ. Đếm được, không chỉ cảm nhận được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu MỘT biến thể trong sum type (như `HinhTron`) không chỉ có 1 hình
dạng — mà bản thân nó LẠI là một product type (nhiều trường)? Số trạng
thái khả dĩ của TOÀN BỘ union tính thế nào — TÍCH lồng trong TỔNG?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
