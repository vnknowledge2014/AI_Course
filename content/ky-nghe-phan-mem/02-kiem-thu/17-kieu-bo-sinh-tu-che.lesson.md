---
id: ky-nghe-phan-mem.kiem-thu.kieu-bo-sinh-tu-che
title: "Kiểu BoSinh<T> — bộ sinh dữ liệu ngẫu nhiên tự chế"
summary: "MỘT 'bộ sinh' chỉ là {generate: () => T} — interface tối giản này CHÍNH LÀ khái niệm 'Arbitrary' của fast-check, tái tạo được trong MỘT dòng cho từng kiểu. soNguyen(min, max): BoSinh<number> — generate dùng Math.random() + Math.floor để sinh số nguyên ngẫu nhiên trong khoảng."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 17
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.arbitrary-type]
requires: [kt.property-check-loop]
concepts: [kt.arbitrary-type]
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
`chayThuTinhChat` nhận `sinhInput: () => A` — MỘT hàm rời rạc. Đóng
gói "cách sinh giá trị" thành MỘT kiểu dữ liệu riêng sẽ ra sao?
::::

::::explain{#bosinh-toi-gian}
MỘT "bộ sinh" CHỈ LÀ `{ generate: () => T }` — interface **TỐI
GIẢN** NÀY CHÍNH LÀ khái niệm "Arbitrary" của `fast-check` (thư viện
property-testing phổ biến, KHÔNG dùng được trong sandbox NÀY), tái
tạo được TRONG MỘT DÒNG cho TỪNG kiểu:

```typescript
type BoSinh<T> = { generate: () => T };

function soNguyen(min: number, max: number): BoSinh<number> {
  return {
    generate: () => Math.floor(Math.random() * (max - min + 1)) + min,
  };
}

const bs = soNguyen(1, 6); // gia lap XUC XAC sau mat
console.log(typeof bs.generate); // "function"
console.log(typeof bs.generate()); // "number"
```

```text
function
number
```

`BoSinh<T>` KHÔNG "chứa SẴN" giá trị — nó CHỨA MỘT **HÀM** (`generate`)
biết CÁCH tạo giá trị MỚI MỖI lần được GỌI. `soNguyen(min, max)` TRẢ
VỀ MỘT `BoSinh<number>` — gọi `bs.generate()` NHIỀU lần cho NHIỀU số
NGẪU NHIÊN KHÁC NHAU trong khoảng `[min, max]`.
::::

::::example{#ket-hop-voi-vong-lap-cu}
`BoSinh<T>.generate` GHÉP THẲNG được VỚI `chayThuTinhChat` (bài 16)
— TRUYỀN `bs.generate` LÀM tham số `sinhInput`:

```typescript title=readonly
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

const bs = soNguyen(-50, 50);
chayThuTinhChat(bs.generate, (n) => n >= -50 && n <= 50, 100);

// nguong khac -- bien KHONG chua "0" -- van phai dung TRON VEN
const bs2 = soNguyen(5, 10);
chayThuTinhChat(bs2.generate, (n) => n >= 5 && n <= 10, 100);
```

```text title=readonly
[PASS] tinh chat dung tren ca 100 lan
[PASS] tinh chat dung tren ca 100 lan
```

CẢ HAI lần chạy `[PASS]` — DÙ `bs.generate`/`bs2.generate` sinh giá
trị NGẪU NHIÊN (KHÁC NHAU MỖI lần chạy chương trình), tính chất "nằm
trong khoảng ĐÃ khai" LUÔN đúng, nên KẾT QUẢ IN RA VẪN ỔN ĐỊNH —
`BoSinh` LÀ MỘT "hộp đóng gói" NGUỒN dữ liệu, KHÔNG PHẢI dữ liệu.
::::

::::predict{#doan-so-nguyen-luon-la-so-nguyen commitOnce}
```typescript
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}

const bs = soNguyen(1, 6);
let tatCaLaSoNguyen = true;
for (let i = 0; i < 1000; i++) {
  const n = bs.generate();
  if (!Number.isInteger(n)) tatCaLaSoNguyen = false;
}
console.log(tatCaLaSoNguyen);
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
KHÔNG chắc chắn — `Math.random()` trả về MỘT số THỰC (có phần thập
phân) trong khoảng `[0, 1)`, nên `bs.generate()` CÓ THỂ thỉnh thoảng
trả VỀ MỘT số THẬP PHÂN (ví dụ `3.7`), khiến `Number.isInteger(n)`
`false` Ở MỘT vài LẦN trong `1000` lần lặp — kết quả IN RA phụ thuộc
"may mắn"
::why
Gần đúng ở việc bạn nhớ ĐÚNG `Math.random()` TRẢ VỀ số THỰC CÓ phần
thập phân — một quan sát ĐÚNG về BẢN CHẤT của `Math.random()`.

Chỗ lệch: `generate` KHÔNG trả TRỰC TIẾP kết quả của `Math.random()`
— nó BỌC kết quả ĐÓ trong `Math.floor(...)` **TRƯỚC KHI** cộng `min`.
`Math.floor` LUÔN cắt bỏ HOÀN TOÀN phần thập phân, trả VỀ số NGUYÊN
(KIỂU `number`, NHƯNG giá trị LUÔN nguyên) — CỘNG THÊM `min` (MỘT số
nguyên) VẪN LÀ số nguyên. `Number.isInteger(n)` LUÔN `true` VỚI MỌI
giá trị `generate()` có THỂ sinh ra — KHÔNG có "may mắn" NÀO ở đây,
đây LÀ MỘT bảo đảm TOÁN HỌC của công thức, ĐÚNG với TOÀN BỘ `1000`
lần lặp.
::
:::

:::opt
Máy báo lỗi biên dịch — `BoSinh<T>` khai `generate: () => T`, NHƯNG
`soNguyen` gán `generate: () => Math.floor(...) + min` (MỘT biểu
thức TÍNH TOÁN, KHÔNG PHẢI literal `T` trực tiếp) — TypeScript đòi
`generate` PHẢI trả VỀ ĐÚNG literal kiểu `T`, KHÔNG chấp nhận biểu
thức
::why
Gần đúng ở việc bạn để ý `BoSinh<T>` LÀ MỘT kiểu GENERIC, VÀ
`generate` PHẢI trả ĐÚNG kiểu `T` — một quan sát ĐÚNG về RÀNG BUỘC
kiểu.

Chỗ lệch: TypeScript KHÔNG PHÂN BIỆT "literal trực tiếp" VÀ "biểu
thức tính toán" — CHỈ quan tâm **KIỂU KẾT QUẢ** của biểu thức CÓ khớp
KIỂU khai hay KHÔNG. `Math.floor(Math.random() * (max - min + 1)) +
min` LÀ MỘT biểu thức TÍNH RA `number` (MỖI hàm/toán tử THAM GIA đều
trả `number`), khớp HOÀN TOÀN `T = number` (được suy TỪ chữ ký khai
báo `soNguyen(...): BoSinh<number>`). Biên dịch sạch — ĐÂY LÀ CÁCH
MỌI hàm phức tạp trong track NÀY TỪ TRƯỚC tới GIỜ đã trả giá trị.
::
:::
::::

::::code{#viet_songuyen}
Tự viết `soNguyen(min, max): BoSinh<number>`.

```typescript title=starter
type BoSinh<T> = { generate: () => T };

function soNguyen(min: number, max: number): BoSinh<number> {
  return {
    generate: () => ___,
  };
}

function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

const bs = soNguyen(1, 6);
chayThuTinhChat(bs.generate, (n) => n >= 1 && n <= 6, 50);
```

```typescript title=solution
type BoSinh<T> = { generate: () => T };

function soNguyen(min: number, max: number): BoSinh<number> {
  return {
    generate: () => Math.floor(Math.random() * (max - min + 1)) + min,
  };
}

function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

const bs = soNguyen(1, 6);
chayThuTinhChat(bs.generate, (n) => n >= 1 && n <= 6, 50);
```

```typescript title=test
// bien KHONG chua 0 -- neu generate luon tra 0 se lo ngay
const bsKhongChua0 = soNguyen(5, 10);
let luonTrongKhoang = true;
let luonLaSoNguyen = true;
for (let i = 0; i < 500; i++) {
  const n = bsKhongChua0.generate();
  if (n < 5 || n > 10) luonTrongKhoang = false;
  if (!Number.isInteger(n)) luonLaSoNguyen = false;
}
if (!luonTrongKhoang) throw new Error("generate() phai LUON nam trong [min, max] da khai");
if (!luonLaSoNguyen) throw new Error("generate() phai LUON tra ve so NGUYEN, khong duoc co phan thap phan");
console.log("[PASS] generate() luon trong khoang va luon la so nguyen");

// khoang am -- kiem generate KHONG bi ghim cung mot phia
const bsAm = soNguyen(-30, -20);
let luonAm = true;
for (let i = 0; i < 200; i++) {
  const n = bsAm.generate();
  if (n < -30 || n > -20) luonAm = false;
}
if (!luonAm) throw new Error("generate() phai dung ca khoang AM");
console.log("[PASS] generate() dung ca khoang am");

// phai sinh NHIEU gia tri KHAC NHAU -- khong duoc ghim cung mot so (vi du luon tra ve min)
const bsBienThien = soNguyen(1, 100);
const giaTriKhacNhau = new Set<number>();
for (let i = 0; i < 50; i++) giaTriKhacNhau.add(bsBienThien.generate());
if (giaTriKhacNhau.size < 3) throw new Error("generate() phai sinh NHIEU gia tri KHAC NHAU, khong duoc ghim cung mot so");
console.log("[PASS] generate() sinh nhieu gia tri khac nhau");
```

:::hints
- kind: attention
  body: "Công thức chuẩn để sinh số nguyên ngẫu nhiên trong [min, max]: nhân Math.random() với (max - min + 1), cắt bỏ phần thập phân bằng Math.floor, rồi cộng min để dịch về đúng khoảng."
- kind: strategy
  body: 'Math.floor(Math.random() * (max - min + 1)) + min'
- kind: one-line
  body: '___ = Math.floor(Math.random() * (max - min + 1)) + min'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
BoSinh<T> = {generate: () => T} — hộp đóng gói nguồn dữ liệu ngẫu
nhiên. Bước tiếp theo: kết hợp bộ sinh có sẵn để sinh cả MẢNG.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`soNguyen` sinh MỘT số. Nếu cần sinh CẢ MỘT MẢNG số (độ dài ngẫu
nhiên, MỖI phần tử sinh TỪ `BoSinh<number>` CÓ SẴN) — không viết lại
công thức TỪ ĐẦU — làm sao?
::::

::::checkpoint{mastery=0.8}
::::
