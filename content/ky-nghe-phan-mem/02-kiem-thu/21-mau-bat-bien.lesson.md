---
id: ky-nghe-phan-mem.kiem-thu.mau-bat-bien
title: "Mẫu Bất Biến — điều PHẢI đúng, VÔ ĐIỀU KIỆN, với MỌI input"
summary: "Mẫu tính chất THỨ NHẤT: BẤT BIẾN — điều LUÔN LUÔN đúng, KHÔNG phụ thuộc input cụ thể. Math.abs(n) >= 0 với MỌI n; Math.abs(n) === Math.abs(-n). Ứng dụng: dieuChinhKhoangGiaTri (clamp) LUÔN trả kết quả nằm trong khoảng cho phép, BẤT KỂ input."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 21
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.pattern-invariant]
requires: [kt.gate-boss-pbt-foundations]
concepts: [kt.pattern-invariant]
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
Cụm mới. "Đi-về" (bài 20) LÀ MỘT trong bốn mẫu tính chất phổ biến.
Mẫu ĐẦU TIÊN — Bất Biến — trông như thế nào?
::::

::::explain{#bat-bien-luon-dung}
Mẫu tính chất **THỨ NHẤT**: **BẤT BIẾN** (Invariant) — điều LUÔN
LUÔN đúng, **KHÔNG** phụ thuộc input CỤ THỂ:

```typescript
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

// bat bien: gia tri tuyet doi LUON >= 0
kiemTraTinhChat(soNguyen(-1000, 1000), (n) => Math.abs(n) >= 0, 50);
// bat bien: gia tri tuyet doi DOI XUNG qua dau
kiemTraTinhChat(soNguyen(-1000, 1000), (n) => Math.abs(n) === Math.abs(-n), 50);
```

```text
[PASS] tinh chat dung tren ca 50 lan
[PASS] tinh chat dung tren ca 50 lan
```

CẢ HAI tính chất **KHÔNG** cần biết `n` LÀ giá trị GÌ CỤ THỂ — CHÚNG
đúng VỚI **MỌI** số nguyên, KHÔNG có "trường hợp ĐẶC BIỆT" nào cần
NGHĨ tới. Đây LÀ đặc trưng của Bất Biến: PHÁT BIỂU được MÀ **KHÔNG**
cần liệt kê ví dụ.
::::

::::example{#clamp-luon-nam-trong-khoang}
Ứng dụng THỰC TẾ HƠN: `dieuChinhKhoangGiaTri` (clamp) — BẤT BIẾN LÀ
"kết quả LUÔN nằm TRONG khoảng cho phép", **BẤT KỂ** input:

```typescript title=readonly
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function dieuChinhKhoangGiaTri(gia: number, min: number, max: number): number {
  if (gia < min) return min;
  if (gia > max) return max;
  return gia;
}

// sinh gia NGAU NHIEN, RONG hon ca khoang [10, 20] rat nhieu
kiemTraTinhChat(soNguyen(-1000, 1000), (gia) => {
  const ket = dieuChinhKhoangGiaTri(gia, 10, 20);
  return ket >= 10 && ket <= 20;
}, 100);
```

```text title=readonly
[PASS] tinh chat dung tren ca 100 lan
```

`gia` sinh RA nằm trong `[-1000, 1000]` — RỘNG hơn khoảng CHO PHÉP
`[10, 20]` RẤT NHIỀU. DÙ `gia` LÀ `-847` hay `632` hay bất kỳ số nào
KHÁC, `dieuChinhKhoangGiaTri(gia, 10, 20)` LUÔN trả VỀ MỘT giá trị
NẰM TRONG `[10, 20]` — ĐÂY LÀ bất biến, KIỂM được TRÊN `100` giá trị
NGẪU NHIÊN KHÁC NHAU MÀ KHÔNG cần liệt kê TỪNG trường hợp.
::::

::::predict{#doan-ba-truong-hop-clamp commitOnce}
```typescript
function dieuChinhKhoangGiaTri(gia: number, min: number, max: number): number {
  if (gia < min) return min;
  if (gia > max) return max;
  return gia;
}

console.log(
  dieuChinhKhoangGiaTri(5, 10, 20),
  dieuChinhKhoangGiaTri(25, 10, 20),
  dieuChinhKhoangGiaTri(15, 10, 20)
);
```

Dòng cuối in ra gì?

:::opt{correct}
`10 20 15`
:::

:::opt
`5 25 15` — vì `dieuChinhKhoangGiaTri` CHỈ kiểm tra ĐIỀU KIỆN NHƯNG
KHÔNG THAY ĐỔI `gia` — nó LUÔN trả VỀ CHÍNH `gia` được TRUYỀN vào,
BẤT KỂ điều kiện nào ĐÚNG
::why
Gần đúng ở việc bạn để ý hàm CÓ dòng `return gia;` Ở CUỐI — một quan
sát ĐÚNG rằng CÓ MỘT nhánh trả VỀ CHÍNH `gia`.

Chỗ lệch: `return gia;` Ở CUỐI **CHỈ** chạy tới KHI CẢ HAI điều kiện
TRƯỚC ĐÓ (`gia < min`, `gia > max`) ĐỀU `false` (nghĩa LÀ `gia` ĐÃ
NẰM TRONG khoảng SẴN) — hàm KHÔNG "luôn" trả `gia`. Trace TỪNG lời
gọi: `dieuChinhKhoangGiaTri(5, 10, 20)`: `5 < 10` LÀ `true` → trả
`min` = `10`. `dieuChinhKhoangGiaTri(25, 10, 20)`: `25 < 10` `false`,
`25 > 20` `true` → trả `max` = `20`. `dieuChinhKhoangGiaTri(15, 10,
20)`: CẢ HAI điều kiện `false` (`15` ĐÃ trong khoảng) → trả `gia` =
`15`. Kết quả: `10 20 15`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `dieuChinhKhoangGiaTri(5, 10, 20)` không
hợp lệ, vì `10 < 20`... TRÁI với QUY ƯỚC thông thường "tham số ĐẦU
LÀ khoảng LỚN, tham số SAU LÀ khoảng NHỎ" — TypeScript kiểm tra thứ
tự THAM SỐ số học lúc BIÊN dịch
::why
Gần đúng ở việc bạn nghĩ tới VIỆC thứ tự tham số CÓ THỂ MANG ý nghĩa
QUAN TRỌNG — MỘT trực giác hợp lý khi ĐỌC MỘT hàm CÓ NHIỀU tham số
CÙNG kiểu.

Chỗ lệch: TypeScript **KHÔNG** kiểm tra MỐI QUAN HỆ SỐ HỌC GIỮA các
tham số CÙNG kiểu `number` — nó CHỈ kiểm KIỂU (`number`), KHÔNG kiểm
GIÁ TRỊ cụ thể hay THỨ TỰ lớn/nhỏ. Việc `min` PHẢI nhỏ HƠN `max` LÀ
MỘT RÀNG BUỘC NGHIỆP VỤ (giống MỌI ràng buộc KHÁC đã gặp — kiểm LÚC
CHẠY nếu CẦN, KHÔNG PHẢI Ở TẦNG kiểu). Biên dịch sạch — VÀ Ở LỜI GỌI
NÀY, `min=10 < max=20` VỐN ĐÃ hợp lý.
::
:::
::::

::::code{#viet_dieuchinhkhoanggiatri}
Tự viết `dieuChinhKhoangGiaTri` — HÀM clamp.

```typescript title=starter
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function dieuChinhKhoangGiaTri(gia: number, min: number, max: number): number {
  if (gia < min) return ___;
  if (gia > max) return ___;
  return ___;
}

kiemTraTinhChat(soNguyen(-1000, 1000), (gia) => {
  const ket = dieuChinhKhoangGiaTri(gia, 10, 20);
  return ket >= 10 && ket <= 20;
}, 100);
```

```typescript title=solution
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function dieuChinhKhoangGiaTri(gia: number, min: number, max: number): number {
  if (gia < min) return min;
  if (gia > max) return max;
  return gia;
}

kiemTraTinhChat(soNguyen(-1000, 1000), (gia) => {
  const ket = dieuChinhKhoangGiaTri(gia, 10, 20);
  return ket >= 10 && ket <= 20;
}, 100);
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

// gia tri PHAN BIET duoc cho tung nhanh
assertEqual(dieuChinhKhoangGiaTri(5, 10, 20), 10, "gia duoi min phai tra ve DUNG min, khong phai gia");
assertEqual(dieuChinhKhoangGiaTri(25, 10, 20), 20, "gia tren max phai tra ve DUNG max, khong phai gia");
assertEqual(dieuChinhKhoangGiaTri(15, 10, 20), 15, "gia da trong khoang phai tra ve DUNG gia, khong doi");

// bien: dung nguong
assertEqual(dieuChinhKhoangGiaTri(10, 10, 20), 10, "dung bang min phai giu nguyen (khong roi vao nhanh duoi min)");
assertEqual(dieuChinhKhoangGiaTri(20, 10, 20), 20, "dung bang max phai giu nguyen (khong roi vao nhanh tren max)");
```

:::hints
- kind: attention
  body: "Nhánh 1 (gia < min): trả về min, KHÔNG PHẢI gia. Nhánh 2 (gia > max): trả về max. Nhánh cuối: gia đã hợp lệ, trả về chính nó."
- kind: strategy
  body: 'min : max : gia — mỗi nhánh trả về đúng biên tương ứng, nhánh cuối trả về giá trị gốc.'
- kind: one-line
  body: '___ (1) = min\n___ (2) = max\n___ (3) = gia'
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
Mẫu Bất Biến: điều luôn đúng, không phụ thuộc input cụ thể. Mẫu tiếp
theo: Idempotent — áp dụng hai lần không khác áp dụng một lần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`chuanHoa` (trim + lowercase + gộp khoảng trắng) — chuẩn hoá dữ liệu
ĐÃ chuẩn hoá PHẢI ra CHÍNH NÓ, KHÔNG BAO GIỜ đổi THÊM. Mẫu tính chất
NÀY trông như thế nào?
::::

::::checkpoint{mastery=0.8}
::::
