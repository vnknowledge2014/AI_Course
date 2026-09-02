---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.gop-mot-mang-bang-monoid-va-reduce
title: "Gộp MỘT MẢNG giá trị bằng Monoid — nối thẳng `reduce`"
summary: "function gopTatCa<T>(m: Monoid<T>, ds: T[]): T { return ds.reduce(m.ketHop, m.rong); } — dùng CHUNG một hàm cho monoidCong ([1,2,3,4]→10) VÀ monoidNoiChuoi ([\"a\",\"b\",\"c\"]→\"abc\"). Nối thẳng T4.2's reduce."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.concat-all-via-reduce]
requires: [alg.monoid-interface]
concepts: [alg.concat-all-via-reduce]
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
`Monoid<T>` kết hợp ĐÚNG HAI giá trị. Nhưng thường cần gộp một MẢNG DÀI
— không chỉ hai. Hôm nay dùng một công cụ đã có sẵn từ T4.2.
::::

::::explain{#goptatca-bang-reduce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };
const monoidNoiChuoi: Monoid<string> = { ketHop: (a, b) => a + b, rong: "" };

console.log(gopTatCa(monoidCong, [1, 2, 3, 4]));
console.log(gopTatCa(monoidNoiChuoi, ["a", "b", "c"]));
```

```text
10
abc
```

`gopTatCa` KHÔNG phải công cụ mới — nó chỉ là `reduce` (T4.2's bài
9-10) gọi với ĐÚNG hai tham số của một `Monoid`: `m.ketHop` chính là
hàm gộp `reduce` cần, `m.rong` chính là `khoi_dau` `reduce` cần. Cùng
MỘT hàm `gopTatCa`, dùng ĐƯỢC cho `monoidCong` (ra `10`) LẪN
`monoidNoiChuoi` (ra `"abc"`) — vì cả hai đều CÙNG hình dạng
`Monoid<T>`, chỉ khác `T` là gì.

Đại số không thay thế `reduce` — nó cho `reduce` một BỘ THAM SỐ có
TÊN, đóng gói lại, dùng lại được nhiều nơi mà không phải viết lại
`ds.reduce(...)` với đúng hai tham số mỗi lần.
::::

::::example{#nhieu-monoid-cung-mot-goptatca}
CÙNG một `gopTatCa`, dùng cho BẤT KỲ `Monoid<T>` nào, kể cả những cái
mới ở bài trước:

```typescript title=readonly
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidMax: Monoid<number> = { ketHop: (a, b) => Math.max(a, b), rong: -Infinity };
const monoidVaLogic: Monoid<boolean> = { ketHop: (a, b) => a && b, rong: true };

console.log(gopTatCa(monoidMax, [3, 7, 2, 9, 1]));
console.log(gopTatCa(monoidVaLogic, [true, true, false, true]));
```

```text title=readonly
9
false
```

`gopTatCa(monoidMax, ...)` tìm SỐ LỚN NHẤT trong mảng — không cần viết
`Math.max(...mang)` hay vòng lặp riêng, CHỈ CẦN có một `Monoid<number>`
đúng. `gopTatCa(monoidVaLogic, ...)` kiểm TẤT CẢ phần tử có `true`
không — một `false` DUY NHẤT làm cả kết quả thành `false` (đúng bản
chất phép `&&`). Không viết hàm MỚI cho từng việc — CÙNG MỘT `gopTatCa`.
::::

::::predict{#doan-goptatca-mang-mot-phan-tu commitOnce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidNhan: Monoid<number> = { ketHop: (a, b) => a * b, rong: 1 };

console.log(gopTatCa(monoidNhan, [7]));
console.log(gopTatCa(monoidNhan, [2, 3, 4]));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`7` rồi `24`
:::

:::opt
`7` rồi `9` — vì `gopTatCa` với NHIỀU hơn một phần tử sẽ CỘNG chúng
lại với nhau trước, không nhân
::why
Gần đúng ở việc bạn tính đúng dòng ĐẦU (`gopTatCa(monoidNhan, [7])` ra
`7`, một phần tử duy nhất giữ nguyên giá trị của nó) — kết quả đó đúng.

Chỗ lệch: `monoidNhan.ketHop` là `(a, b) => a * b` — phép NHÂN, không
phải cộng. `gopTatCa(monoidNhan, [2, 3, 4])` tính `2 * 3 * 4 = 24`
(qua `reduce`, tích luỹ dần), không phải `2 + 3 + 4 = 9`.
::
:::

:::opt
Máy báo lỗi ở dòng `gopTatCa(monoidNhan, [7])` — mảng chỉ có MỘT phần
tử không đủ để `reduce` "kết hợp" (`ketHop` cần HAI giá trị, mảng chỉ
có một)
::why
Gần đúng ở việc bạn để ý `ketHop` cần ĐÚNG HAI tham số — quan sát về
CHỮ KÝ hàm đó đúng.

Chỗ lệch: `reduce` (T4.2 đã đo) không đòi mảng phải có từ hai phần tử
trở lên — với `rong` LÀM điểm khởi đầu, `reduce` gọi `ketHop(rong,
phanTuDauTien)` NGAY CẢ khi mảng chỉ có một phần tử. `gopTatCa(monoidNhan,
[7])` tính `ketHop(1, 7) = 1 * 7 = 7`, không lỗi gì.
::
:::
::::

::::code{#goptatca}
Tự viết `gopTatCa<T>(m: Monoid<T>, ds: T[]): T` — gộp toàn bộ `ds`
bằng `Monoid<T>` đã cho.

```typescript title=starter
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ___;
}

const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };
console.log(gopTatCa(monoidCong, [1, 2, 3, 4]));
```

```typescript title=solution
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };
console.log(gopTatCa(monoidCong, [1, 2, 3, 4]));
```

```typescript title=test
const monoidNoiChuoi: Monoid<string> = { ketHop: (a, b) => a + b, rong: "" };
if (gopTatCa(monoidCong, [1, 2, 3, 4]) !== 10) throw new Error("gopTatCa với monoidCong phải ra 10");
if (gopTatCa(monoidNoiChuoi, ["a", "b", "c"]) !== "abc") throw new Error("gopTatCa với monoidNoiChuoi phải ra abc");
if (gopTatCa(monoidCong, []) !== 0) throw new Error("mảng rỗng phải ra đúng rong");
if (gopTatCa(monoidCong, [5]) !== 5) throw new Error("một phần tử duy nhất phải ra đúng nó");
```

:::hints
- kind: attention
  body: "gopTatCa CHỈ cần gọi ds.reduce với đúng hai tham số của Monoid — không viết vòng lặp tay, không tính riêng cho từng kiểu T."
- kind: strategy
  body: 'return ds.reduce(m.ketHop, m.rong) — m.ketHop LÀ hàm gộp reduce cần, m.rong LÀ khoi_dau reduce cần. Không có gì khác phải viết.'
- kind: one-line
  body: "return ds.reduce(m.ketHop, m.rong);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "10"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một hàm, viết đúng MỘT lần — gộp được BẤT KỲ mảng nào, miễn có một
`Monoid<T>` đúng. `reduce` không đổi; chỉ đóng gói lại cho gọn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`gopTatCa(monoidCong, [])` (mảng RỖNG) — không có phần tử nào để cộng.
Kết quả có phải là lỗi, `undefined`, hay một con số cụ thể? Nếu là một
con số, số đó tới từ đâu?

Bài sau trả lời — và giải thích tại sao `rong` không phải một field
"phòng hờ", mà THẬT SỰ cần thiết.
::::

::::checkpoint{mastery=0.8}
::::
