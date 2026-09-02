---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-monoid
title: "Đo tổng hợp: Monoid"
summary: "Viết MỘT Monoid<boolean> kết hợp bằng &&, rong=true — VÀ dùng gopTatCa với nó. Không khái niệm mới — đo khả năng tự NHẬN RA một phép kết hợp có phải Monoid không, và tìm ĐÚNG giá trị trung tính của nó."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.review-monoid]
requires: [alg.write-monoid]
concepts: [alg.review-monoid]
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
Cộng, nhân, nối chuỗi, max — bốn `Monoid` đã gặp. Hôm nay: một phép
kết hợp HOÀN TOÀN khác — phép LOGIC `&&` (VÀ).
::::

::::explain{#monoid-cho-boolean}
Câu hỏi TRƯỚC KHI viết code: `&&` có phải một phép kết hợp hợp lệ cho
`Monoid<boolean>` không? Kiểm hai điều kiện (bài 1-2 đã dạy):

1. Nhận HAI `boolean`, trả về MỘT `boolean` — `true && false` ra
   `false` (một `boolean`). ĐÚNG.
2. Có giá trị trung tính không — kết hợp với `boolean` nào cũng
   ra ĐÚNG `boolean` đó? `x && true === x` với MỌI `x` (`true && true
   = true`, `false && true = false` — cả hai giữ nguyên `x`). `true`
   là giá trị trung tính của `&&`.

```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidVaLogic: Monoid<boolean> = {
  ketHop: (a, b) => a && b,
  rong: true,
};

console.log(gopTatCa(monoidVaLogic, [true, true, true]));
console.log(gopTatCa(monoidVaLogic, [true, false, true]));
```

```text
true
false
```

`[true, true, true]` — TẤT CẢ đều `true`, kết quả `true`. `[true,
false, true]` — CÓ một `false`, kết quả `false` (một `false` DUY NHẤT
"nhiễm" cả kết quả, đúng bản chất `&&`: CẢ HAI vế phải `true` mới ra
`true`). `gopTatCa` dùng CHO `Monoid<boolean>` y hệt mọi `Monoid` khác
— không cần sửa gì trong chính `gopTatCa`.
::::

::::example{#khong-phai-moi-phep-deu-la-monoid}
Không phải MỌI phép toán hai-vào-một-ra đều là `Monoid` — phép TRỪ là
một ví dụ KHÔNG khớp:

```typescript title=readonly
const ketHopTru = (a: number, b: number) => a - b;

console.log(ketHopTru(5, 3));
console.log(ketHopTru(3, 5));
console.log(ketHopTru(5, 0));
console.log(ketHopTru(0, 5));
```

```text title=readonly
2
-2
5
-5
```

`ketHopTru(5, 3)` KHÁC `ketHopTru(3, 5)` (`2` khác `-2`) — TRỪ không
"đối xứng" (thứ tự quan trọng, khác cộng/nhân). Quan trọng hơn: `0`
CÓ vẻ "trung tính" khi đứng BÊN PHẢI (`5 - 0 = 5`, giữ nguyên), nhưng
KHÔNG trung tính khi đứng BÊN TRÁI (`0 - 5 = -5`, KHÔNG giữ nguyên `5`)
— vi phạm chính định nghĩa "trung tính phải đúng ở CẢ HAI phía" (bài
3's ví dụ `0 + 5` và `5 + 0`). Phép TRỪ KHÔNG PHẢI một `Monoid` hợp lệ.
::::

::::predict{#doan-hoac-logic commitOnce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidHoacLogic: Monoid<boolean> = {
  ketHop: (a, b) => a || b,
  rong: false,
};

console.log(gopTatCa(monoidHoacLogic, [false, false, false]));
console.log(gopTatCa(monoidHoacLogic, [false, true, false]));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`false` rồi `true`
:::

:::opt
`true` rồi `true` — vì `rong: false` là một giá trị "yếu", phép `||`
LUÔN thiên về `true` khi có `rong` tham gia
::why
Gần đúng ở việc bạn tính đúng dòng THỨ HAI (`[false, true, false]` CÓ
một `true`, phép `||` cho `true`) — kết quả đó đúng.

Chỗ lệch: `rong` không "thiên về" gì cả — nó chỉ TRUNG TÍNH
(`x || false === x`, GIỮ NGUYÊN `x`, dù `x` là `true` hay `false`).
`[false, false, false]` — TẤT CẢ đều `false`, không có `true` nào để
"thắng" — kết quả PHẢI là `false`, không phải `true`.
::
:::

:::opt
Máy báo lỗi — `Monoid<boolean>` chỉ định nghĩa được MỘT phép kết hợp
hợp lệ (`&&`), không thể có `Monoid<boolean>` THỨ HAI với `||`
::why
Gần đúng ở việc bạn nhớ `&&` (bài explain) đúng là một `Monoid<boolean>`
hợp lệ — quan sát đó đúng.

Chỗ lệch: KHÔNG có giới hạn "mỗi kiểu chỉ có ĐÚNG MỘT Monoid" — CÙNG
một kiểu `T` (`boolean`) có thể có NHIỀU `Monoid` khác nhau, miễn MỖI
cái tự có `ketHop`+`rong` đúng luật riêng của nó. `&&`/`true` và
`||`/`false` là HAI `Monoid<boolean>` HOÀN TOÀN hợp lệ, độc lập nhau.
::
:::
::::

::::code{#monoid_hoac}
Viết `monoidHoacLogic: Monoid<boolean>` — kết hợp bằng phép `||`
(HOẶC).

```typescript title=starter
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidHoacLogic: Monoid<boolean> = {
  ketHop: (a, b) => ___,
  rong: ___,
};

console.log(gopTatCa(monoidHoacLogic, [false, true, false]));
```

```typescript title=solution
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidHoacLogic: Monoid<boolean> = {
  ketHop: (a, b) => a || b,
  rong: false,
};

console.log(gopTatCa(monoidHoacLogic, [false, true, false]));
```

```typescript title=test
if (monoidHoacLogic.ketHop(true, false) !== true) throw new Error("ketHop phải là phép HOẶC — true || false = true");
if (monoidHoacLogic.ketHop(false, false) !== false) throw new Error("false || false phải là false");
if (monoidHoacLogic.rong !== false) throw new Error("rong phải là false, không phải true");
if (gopTatCa(monoidHoacLogic, [false, false, false]) !== false) throw new Error("tất cả false phải ra false");
if (gopTatCa(monoidHoacLogic, [false, true, false]) !== true) throw new Error("có một true phải ra true");
if (gopTatCa(monoidHoacLogic, []) !== false) throw new Error("mảng rỗng phải ra đúng rong, tức false");
```

:::hints
- kind: attention
  body: "ketHop phải là phép || (HOẶC), không phải && (VÀ). rong phải là false — giá trị trung tính của ||, KHÔNG phải true."
- kind: strategy
  body: 'ketHop: (a, b) => a || b — toán tử ||. rong: false — x || false === x với mọi x (đối lập với monoidVaLogic của bài explain: && dùng rong=true, || dùng rong=false).'
- kind: one-line
  body: "ketHop: a || b, rong: false"
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
Cộng, nhân, nối chuỗi, max, `&&`, `||` — sáu `Monoid` khác nhau, CÙNG
một khuôn `interface`. Bạn giờ tự NHẬN RA và tự VIẾT được một cái mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`Monoid` giải quyết đúng MỘT việc: kết hợp nhiều giá trị CÙNG kiểu
thành một. Nhưng có một vấn đề CŨ hơn, T4.0a đã từng nêu: giá trị có
thể VẮNG MẶT (`undefined`) — làm sao mô hình hoá điều đó một cách AN
TOÀN, không dùng `null`?

Cụm sau quay lại vấn đề đó, dùng chính kỹ năng T4.3 đã dạy.
::::

::::checkpoint{mastery=0.8}
::::
