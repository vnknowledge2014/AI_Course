---
id: ky-nghe-phan-mem.ddd.value-object-va-composite
title: "Value Object — khuôn đúc chỉ nhận giá trị hợp lệ, kể cả nhiều trường"
summary: "Composite Value Object: VO nhiều trường vẫn KHÔNG có identity, so sánh bằng GIÁ TRỊ (deep compare). DiaChi (tinh/huyen/duong) với sameDiaChi(); KhoangThoiGian (start/end) với giaoNhau()."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.value-object-handwritten]
requires: [ddd.architecture-capstone]
concepts: [ddd.value-object-handwritten]
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
Cụm mới. Bài 4 đã dạy Value Object — `Email`, `TienVND`, một TRƯỜNG
duy nhất. Nếu Value Object cần NHIỀU trường thì sao?
::::

::::explain{#composite-value-object}
**Composite Value Object**: VO có NHIỀU trường VẪN KHÔNG có identity —
so sánh bằng GIÁ TRỊ (deep compare TẤT CẢ field), immutable:

```typescript
type DiaChi = { tinh: string; huyen: string; duong: string };

function sameDiaChi(a: DiaChi, b: DiaChi): boolean {
  return a.tinh === b.tinh && a.huyen === b.huyen && a.duong === b.duong;
}

const dc1: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
const dc2: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
const dc3: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Trần Duy Hưng" };

console.log(sameDiaChi(dc1, dc2));
console.log(sameDiaChi(dc1, dc3));
```

```text
true
false
```

`dc1` và `dc2` là HAI OBJECT khác nhau (hai lần khai báo `{...}` RIÊNG
biệt trong bộ nhớ) — nhưng `sameDiaChi` trả `true`, vì `DiaChi` là VO:
SO SÁNH bằng GIÁ TRỊ, không quan tâm chúng có phải "cùng một object
trong bộ nhớ" hay không. `dc3` khác `duong` — MỘT field khác là ĐỦ để
`sameDiaChi` trả `false`.
::::

::::example{#khoangthoigian-va-giao-nhau}
Một Composite VO khác — `KhoangThoiGian`, có phép toán RIÊNG (`giaoNhau`
— hai khoảng thời gian có CHỒNG LẤN không):

```typescript title=readonly
type KhoangThoiGian = { batDau: Date; ketThuc: Date };

function giaoNhau(a: KhoangThoiGian, b: KhoangThoiGian): boolean {
  return a.batDau <= b.ketThuc && b.batDau <= a.ketThuc;
}

const kt1: KhoangThoiGian = { batDau: new Date("2024-01-01"), ketThuc: new Date("2024-01-10") };
const kt2: KhoangThoiGian = { batDau: new Date("2024-01-05"), ketThuc: new Date("2024-01-15") };
const kt3: KhoangThoiGian = { batDau: new Date("2024-02-01"), ketThuc: new Date("2024-02-10") };

console.log(giaoNhau(kt1, kt2));
console.log(giaoNhau(kt1, kt3));
```

```text title=readonly
true
false
```

`kt1` (1/1 → 10/1) và `kt2` (5/1 → 15/1) CHỒNG LẤN (5/1 → 10/1 thuộc
CẢ HAI) — `giaoNhau` trả `true`. `kt1` và `kt3` (1/2 → 10/2) KHÔNG hề
chạm nhau — trả `false`. `giaoNhau` KHÔNG sửa `kt1`/`kt2`/`kt3` — chỉ
TÍNH TOÁN và trả về `boolean`, đúng tinh thần VO: operations không đổi
dữ liệu gốc.
::::

::::predict{#doan-dia-chi-khac-huyen commitOnce}
```typescript
type DiaChi = { tinh: string; huyen: string; duong: string };
function sameDiaChi(a: DiaChi, b: DiaChi): boolean {
  return a.tinh === b.tinh && a.huyen === b.huyen && a.duong === b.duong;
}

const dcA: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
const dcB: DiaChi = { tinh: "Hà Nội", huyen: "Đống Đa", duong: "Xuân Thuỷ" };

console.log(sameDiaChi(dcA, dcB));
```

`dcA` và `dcB` khác nhau ở `huyen` (giữ nguyên `tinh` VÀ `duong`). Dòng
cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `tinh` và `duong` (hai trong ba field) GIỐNG NHAU, đa số
field khớp là ĐỦ để coi là cùng địa chỉ
::why
Gần đúng ở việc bạn đếm ĐÚNG có hai trong ba field GIỐNG NHAU — quan
sát về TỶ LỆ khớp đó đúng.

Chỗ lệch: `sameDiaChi` dùng `&&` (VÀ) nối CẢ BA điều kiện — YÊU CẦU
TẤT CẢ BA field GIỐNG NHAU, không có khái niệm "đa số khớp là đủ". Chỉ
CẦN MỘT trong ba điều kiện là `false` (ở đây: `a.huyen === b.huyen` là
`false`, vì `"Cầu Giấy" !== "Đống Đa"`), TOÀN BỘ biểu thức `&&` LÀ
`false` — Value Object so sánh CHÍNH XÁC, không có "gần đúng".
::
:::

:::opt
Máy báo lỗi biên dịch — so sánh `string` bằng `===` ba lần liên tiếp
nối bằng `&&` không hợp lệ trong TypeScript
::why
Gần đúng ở việc bạn để ý biểu thức CÓ BA phép so sánh `===` nối bằng
`&&` — một cấu trúc khá DÀI — quan sát về ĐỘ DÀI biểu thức đó đúng.

Chỗ lệch: nối NHIỀU biểu thức `boolean` bằng `&&` là cú pháp HOÀN TOÀN
HỢP LỆ, phổ biến trong TypeScript/JavaScript — không có giới hạn về
SỐ LƯỢNG điều kiện nối được. `a.tinh === b.tinh && a.huyen === b.huyen
&& a.duong === b.duong` biên dịch và chạy bình thường.
::
:::
::::

::::code{#viet_samediachi_va_giaonhau}
Tự viết `sameDiaChi` và `giaoNhau`.

```typescript title=starter
type DiaChi = { tinh: string; huyen: string; duong: string };
function sameDiaChi(a: DiaChi, b: DiaChi): boolean {
  return ___;
}

type KhoangThoiGian = { batDau: Date; ketThuc: Date };
function giaoNhau(a: KhoangThoiGian, b: KhoangThoiGian): boolean {
  return ___;
}

const dc: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
console.log(sameDiaChi(dc, dc));
```

```typescript title=solution
type DiaChi = { tinh: string; huyen: string; duong: string };
function sameDiaChi(a: DiaChi, b: DiaChi): boolean {
  return a.tinh === b.tinh && a.huyen === b.huyen && a.duong === b.duong;
}

type KhoangThoiGian = { batDau: Date; ketThuc: Date };
function giaoNhau(a: KhoangThoiGian, b: KhoangThoiGian): boolean {
  return a.batDau <= b.ketThuc && b.batDau <= a.ketThuc;
}

const dc: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
console.log(sameDiaChi(dc, dc));
```

```typescript title=test
const dc1: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
const dc2: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
const dc3: DiaChi = { tinh: "Hà Nội", huyen: "Cầu Giấy", duong: "Trần Duy Hưng" };
if (sameDiaChi(dc1, dc2) !== true) throw new Error("hai địa chỉ CÙNG giá trị phải bằng nhau");
if (sameDiaChi(dc1, dc3) !== false) throw new Error("hai địa chỉ KHÁC duong không được coi là bằng nhau");

const dc4: DiaChi = { tinh: "TP.HCM", huyen: "Cầu Giấy", duong: "Xuân Thuỷ" };
if (sameDiaChi(dc1, dc4) !== false) throw new Error("hai địa chỉ KHÁC tinh (dù CÙNG huyen, duong) không được coi là bằng nhau");
const dc5: DiaChi = { tinh: "Hà Nội", huyen: "Ba Đình", duong: "Xuân Thuỷ" };
if (sameDiaChi(dc1, dc5) !== false) throw new Error("hai địa chỉ KHÁC huyen (dù CÙNG tinh, duong) không được coi là bằng nhau");

const kt1: KhoangThoiGian = { batDau: new Date("2024-01-01"), ketThuc: new Date("2024-01-10") };
const kt2: KhoangThoiGian = { batDau: new Date("2024-01-05"), ketThuc: new Date("2024-01-15") };
const kt3: KhoangThoiGian = { batDau: new Date("2024-02-01"), ketThuc: new Date("2024-02-10") };
if (giaoNhau(kt1, kt2) !== true) throw new Error("hai khoảng thời gian chồng lấn phải trả về true");
if (giaoNhau(kt1, kt3) !== false) throw new Error("hai khoảng thời gian KHÔNG chạm nhau phải trả về false");
```

:::hints
- kind: attention
  body: "sameDiaChi: nối CẢ BA phép so sánh field bằng &&. giaoNhau: chồng lấn khi a bắt đầu TRƯỚC (hoặc đúng lúc) b kết thúc, VÀ b bắt đầu TRƯỚC (hoặc đúng lúc) a kết thúc."
- kind: strategy
  body: "a.tinh === b.tinh && a.huyen === b.huyen && a.duong === b.duong — cho sameDiaChi. a.batDau <= b.ketThuc && b.batDau <= a.ketThuc — cho giaoNhau."
- kind: one-line
  body: "___ (sameDiaChi) = a.tinh === b.tinh && a.huyen === b.huyen && a.duong === b.duong\n___ (giaoNhau) = a.batDau <= b.ketThuc && b.batDau <= a.ketThuc"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Composite Value Object: nhiều trường, vẫn so sánh bằng GIÁ TRỊ — CHỈ
MỘT field khác là đủ để coi là hai giá trị khác nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Nếu một domain object dùng NHIỀU cờ `boolean` để mô tả trạng thái —
`daThanhToan`, `daGiao`, `daHuy` — điều gì có thể sai?
::::

::::checkpoint{mastery=0.8}
::::
