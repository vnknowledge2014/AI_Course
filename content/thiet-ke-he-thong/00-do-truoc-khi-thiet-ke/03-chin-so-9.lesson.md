---
id: thiet-ke-he-thong.do-truoc-khi-thiet-ke.chin-so-9
title: "Chín số 9 — availability ra tiền"
summary: "downtimeGiay(availability) = (1 - availability) x 365.25 x 24 x 3600 -- mỗi số 9 thêm vào phần trăm khả dụng giảm downtime khoảng 10 lần. Tính thật: 99% = 3.65 ngày/năm, 99.9% = 8.77 giờ/năm, 99.99% = 52.6 phút/năm, 99.999% = 5.26 phút/năm, 99.9999% = 31.56 giây/năm -- từ 99.9% lên 99.99% downtime giảm đúng 10 lần (không phải giảm một nửa như trực giác 'thêm một chữ số' hay gợi ý)."
locale: vi
track: thiet-ke-he-thong
module: do-truoc-khi-thiet-ke
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.chin-so-9]
requires: [sd.do-tre-nao-cung-co-gia]
concepts: [sd.chin-so-9]
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
Độ trễ đo "một thao tác chậm cỡ nào" (bài trước). "Availability" đo
một câu hỏi khác hẳn: hệ thống SỐNG (trả lời được request) bao nhiêu
PHẦN trăm thời gian trong một năm?
::::

::::explain{#downtime-giay}
"99.9% availability" nghe GẦN với "100%" — nhưng quy RA thời gian
NGỪNG hoạt động (downtime) thực tế trong MỘT năm thì khác XA trực
giác. Công thức: `downtime giây = (1 - availability) × 365.25 × 24 ×
3600` (dùng `365.25` ngày/năm để tính CẢ năm nhuận):

```typescript title=readonly
const GIAY_MOI_NAM = 365.25 * 24 * 3600;

function downtimeGiay(availability: number): number {
  return (1 - availability) * GIAY_MOI_NAM;
}

function lamTron(x: number, chuSo: number): number {
  const heSo = 10 ** chuSo;
  return Math.round(x * heSo) / heSo;
}

console.log("99% ->", lamTron(downtimeGiay(0.99) / 86400, 2), "ngay/nam");
console.log("99.9% ->", lamTron(downtimeGiay(0.999) / 3600, 2), "gio/nam");
console.log("99.99% ->", lamTron(downtimeGiay(0.9999) / 60, 2), "phut/nam");
```

```text title=readonly
99% -> 3.65 ngay/nam
99.9% -> 8.77 gio/nam
99.99% -> 52.6 phut/nam
```

`(1 - 0.99) = 0.01` — CHỈ `1%` thời gian trong năm, nhưng MỘT năm có
`31.557.600` giây (`365.25 × 24 × 3600`), NÊN `1%` CỦA nó vẫn LÀ
`315.576` giây — tương đương gần `3.65` NGÀY hệ thống hoàn toàn
KHÔNG trả lời được request nào.
::::

::::example{#moi-so-9-giam-10-lan}
Tiếp tục thêm số `9`:

```typescript title=readonly
const GIAY_MOI_NAM = 365.25 * 24 * 3600;
function downtimeGiay(availability: number): number {
  return (1 - availability) * GIAY_MOI_NAM;
}
function lamTron(x: number, chuSo: number): number {
  const heSo = 10 ** chuSo;
  return Math.round(x * heSo) / heSo;
}

console.log("99.999% ->", lamTron(downtimeGiay(0.99999) / 60, 2), "phut/nam");
console.log("99.9999% ->", lamTron(downtimeGiay(0.999999), 2), "giay/nam");
console.log("ti le 99.9% / 99.99% (tinh theo giay):", Math.round(downtimeGiay(0.999) / downtimeGiay(0.9999)));
```

```text title=readonly
99.999% -> 5.26 phut/nam
99.9999% -> 31.56 giay/nam
ti le 99.9% / 99.99% (tinh theo giay): 10
```

Mỗi lần THÊM đúng một số `9` (`99.9%` → `99.99%` → `99.999%` →
`99.9999%`), downtime GIẢM đúng khoảng `10` lần — VÌ `(1 -
availability)` giảm đúng `10` lần MỖI lần thêm một số `9` sau dấu
phẩy. Đây LÀ lý do "sáu số 9" (`99.9999%`, chỉ `31.56` giây
downtime/năm) LÀ một mục tiêu vận hành cực kỳ khó — mỗi bậc CAO hơn
đòi hỏi hạ tầng tốt hơn hẳn một BẬC độ lớn, không phải "tốt hơn một
chút".
::::

::::predict{#doan-tang-mot-so-9 commitOnce}
Đi TỪ `99.9%` LÊN `99.99%` (thêm ĐÚNG một số `9`), downtime MỖI năm
giảm khoảng BAO nhiêu lần?

:::opt{correct}
Khoảng `10` lần — `(1 - 0.999) = 0.001` VÀ `(1 - 0.9999) = 0.0001`,
tỉ lệ giữa hai số NÀY đúng LÀ `10`
:::
:::opt
Khoảng `2` lần — "thêm một chữ SỐ 9" trực giác nghe giống "tăng gấp
đôi độ chính XÁC"
::why
Nhầm "thêm một KÝ tự vào chuỗi số" VỚI "tăng gấp đôi một đại lượng"
— hai phép TOÁN hoàn toàn khác nhau.

Chỗ lệch: `downtimeGiay` phụ thuộc VÀO `(1 - availability)`, VÀ mỗi
số `9` thêm VÀO SAU dấu phẩy LÀM `(1 - availability)` giảm đúng một
BẬC thập phân (`chia cho 10`) — không phải chia CHO 2. `99.9% →
99.99%` LÀ bước nhảy `10` lần, không phải `2` lần.
::
:::
::::

::::code{#viet_downtime_giay}
Hoàn thiện `downtimeGiay` — số giây ngừng hoạt động MỖI năm ứng VỚI
một mức `availability` (số thập phân, VÍ dụ `0.999` cho `99.9%`).

```typescript title=starter
const GIAY_MOI_NAM = 365.25 * 24 * 3600;

function downtimeGiay(availability: number): number {
  return ___;
}

function lamTron(x: number, chuSo: number): number {
  const heSo = 10 ** chuSo;
  return Math.round(x * heSo) / heSo;
}

console.log(lamTron(downtimeGiay(0.999) / 3600, 2));
```

```typescript title=solution
const GIAY_MOI_NAM = 365.25 * 24 * 3600;

function downtimeGiay(availability: number): number {
  return (1 - availability) * GIAY_MOI_NAM;
}

function lamTron(x: number, chuSo: number): number {
  const heSo = 10 ** chuSo;
  return Math.round(x * heSo) / heSo;
}

console.log(lamTron(downtimeGiay(0.999) / 3600, 2));
```

```typescript title=test
if (lamTron(downtimeGiay(0.99) / 86400, 2) !== 3.65) throw new Error("99% phai la 3.65 ngay/nam");
if (lamTron(downtimeGiay(0.999) / 3600, 2) !== 8.77) throw new Error("99.9% phai la 8.77 gio/nam");
if (lamTron(downtimeGiay(0.9999) / 60, 2) !== 52.6) throw new Error("99.99% phai la 52.6 phut/nam");
if (lamTron(downtimeGiay(0.99999) / 60, 2) !== 5.26) throw new Error("99.999% phai la 5.26 phut/nam");
if (lamTron(downtimeGiay(0.999999), 2) !== 31.56) throw new Error("99.9999% phai la 31.56 giay/nam");
if (downtimeGiay(1) !== 0) throw new Error("availability 100% phai co downtime bang 0");
if (lamTron(downtimeGiay(0.9995) / 3600, 2) !== 4.38) throw new Error("99.95% phai la 4.38 gio/nam");
```

:::hints
- kind: attention
  body: "(1 - availability) nhan voi GIAY_MOI_NAM -- mot dong."
- kind: strategy
  body: "(1 - availability) * GIAY_MOI_NAM"
- kind: one-line
  body: "return (1 - availability) * GIAY_MOI_NAM;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "8.77"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đơn vị, độ trễ, availability — ba trục ĐO đã sẵn sàng. Giờ ráp CHÚNG
lại thành một bài toán ước lượng THẬT: một hệ thống GIẢ định cần bao
nhiêu QPS, bao nhiêu dung lượng?
::::

::::reflect{#nghi-lai}
`downtimeGiay` chỉ LÀ một phép trừ VÀ một phép nhân — nhưng nó biến
một con số PHẦN trăm trừu tượng (`99.9%`) thành một câu hỏi VẬN hành
CỤ thể: "hệ thống được PHÉP ngừng hoạt động bao lâu MỖI năm, VÀ đội
vận hành CÓ chấp nhận được con số ĐÓ không?"
::::

::::checkpoint{mastery=0.8}
::::
