---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.dinh-nghia-monoid-ket-hop-va-trung-tinh
title: "Định nghĩa `Monoid` — một cặp: cách KẾT HỢP, và giá trị TRUNG TÍNH"
summary: "interface Monoid<T> { ketHop: (a: T, b: T) => T; rong: T } — một object TypeScript ghi lại ĐÚNG hai thứ đã thấy: hàm kết hợp, và giá trị trung tính. Không phải khái niệm mới — chỉ đặt tên hình thức cho khuôn đã lặp lại."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.monoid-interface]
requires: [alg.identity-element]
concepts: [alg.monoid-interface]
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
Bạn đã thấy: một phép kết hợp, luôn đi cùng một giá trị trung tính
riêng. Hôm nay gói cả hai vào MỘT khai báo TypeScript.
::::

::::explain{#dinh-nghia-monoid}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidCong: Monoid<number> = {
  ketHop: (a, b) => a + b,
  rong: 0,
};

console.log(monoidCong.ketHop(3, 4));
console.log(monoidCong.rong);
console.log(monoidCong.ketHop(5, monoidCong.rong));
```

```text
7
0
5
```

`interface Monoid<T>` ghi lại ĐÚNG hai thứ hai bài trước đã thấy:
`ketHop` (hàm nhận HAI giá trị kiểu `T`, trả về MỘT giá trị kiểu `T`)
và `rong` (giá trị trung tính, cũng kiểu `T`). `monoidCong` là một
GIÁ TRỊ CỤ THỂ của `Monoid<number>` — không phải khái niệm trừu tượng,
mà một OBJECT TypeScript THẬT, gọi được, đọc được như bất kỳ object
nào khác.

`monoidCong.ketHop(5, monoidCong.rong)` ra `5` — đúng tính chất "kết
hợp với trung tính thì không đổi" (bài 2), giờ VIẾT ĐƯỢC thành một
biểu thức TypeScript thật, không chỉ nói bằng lời.
::::

::::example{#nhieu-monoid-cung-mot-dinh-nghia}
CÙNG một `interface Monoid<T>`, nhiều GIÁ TRỊ khác nhau — mỗi giá trị
là một cách kết hợp riêng:

```typescript title=readonly
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidMax: Monoid<number> = {
  ketHop: (a, b) => Math.max(a, b),
  rong: -Infinity,
};

const monoidVaLogic: Monoid<boolean> = {
  ketHop: (a, b) => a && b,
  rong: true,
};

console.log(monoidMax.ketHop(3, 7));
console.log(monoidVaLogic.ketHop(true, false));
console.log(monoidVaLogic.ketHop(monoidVaLogic.rong, false));
```

```text title=readonly
7
false
false
```

`monoidMax` dùng ĐÚNG giá trị trung tính đã đo ở bài 2 (`-Infinity`,
không phải `0`). `monoidVaLogic` (phép `&&`) có `rong: true` — kết hợp
`true` với BẤT KỲ giá trị nào (`monoidVaLogic.ketHop(true, x)`) LUÔN
ra ĐÚNG `x`, không đổi (`true && false === false`, giữ nguyên `false`).
Mỗi `Monoid<T>` là một "gói" TRỌN VẸN, tự chứa đủ mọi thứ cần để dùng
đúng — không cần nhớ RIÊNG "trung tính của cái này là gì" ở đâu khác.
::::

::::predict{#doan-monoid-va-logic commitOnce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidHoacLogic: Monoid<boolean> = {
  ketHop: (a, b) => a || b,
  rong: false,
};

console.log(monoidHoacLogic.ketHop(true, monoidHoacLogic.rong));
console.log(monoidHoacLogic.ketHop(false, monoidHoacLogic.rong));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`true` rồi `false`
:::

:::opt
`true` rồi `true` — vì `rong` (giá trị trung tính) của một Monoid LUÔN
LÀ giá trị "mạnh hơn", áp đảo giá trị kia trong phép kết hợp
::why
Gần đúng ở việc bạn tính đúng dòng ĐẦU (`true || false === true`) —
phép tính đó đúng.

Chỗ lệch: giá trị trung tính KHÔNG "áp đảo" — nó CHÍNH XÁC là giá trị
KHÔNG THAY ĐỔI kết quả. Với phép `||` (HOẶC), `false` là trung tính
(`x || false === x`, LUÔN giữ nguyên `x`, dù `x` là `true` hay
`false`). `monoidHoacLogic.ketHop(false, false)` (dòng 2) ra `false`,
không phải `true` — không có giá trị nào "áp đảo" cả, `rong` chỉ đơn
giản KHÔNG đổi gì.
::
:::

:::opt
Máy báo lỗi biên dịch — `Monoid<boolean>` không hợp lệ vì `boolean`
không phải kiểu SỐ, không thể có phép "kết hợp"
::why
Gần đúng ở việc bạn nghĩ tới việc `Monoid` thường gắn với các kiểu SỐ
(cộng, nhân) — nhiều ví dụ TRƯỚC bài này đúng là dùng `number`.

Chỗ lệch: `Monoid<T>` là GENERIC — `T` có thể là BẤT KỲ kiểu nào, miễn
có một phép kết hợp hợp lý và một giá trị trung tính (đã đo `Monoid
<string>`, giờ `Monoid<boolean>` — cả hai đều biên dịch và chạy đúng,
không có ràng buộc "chỉ dùng được với số").
::
:::
::::

::::code{#monoid_noi_chuoi}
Viết `monoidNoiChuoi: Monoid<string>` — kết hợp bằng cách NỐI hai
chuỗi, giá trị trung tính là chuỗi RỖNG.

```typescript title=starter
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidNoiChuoi: Monoid<string> = {
  ketHop: (a, b) => ___,
  rong: ___,
};

console.log(monoidNoiChuoi.ketHop("chao", " ban"));
```

```typescript title=solution
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidNoiChuoi: Monoid<string> = {
  ketHop: (a, b) => a + b,
  rong: "",
};

console.log(monoidNoiChuoi.ketHop("chao", " ban"));
```

```typescript title=test
if (monoidNoiChuoi.ketHop("a", "b") !== "ab") throw new Error("ketHop phải nối hai chuỗi lại — a rồi b, không đảo thứ tự");
if (monoidNoiChuoi.rong !== "") throw new Error("rong phải là chuỗi rỗng");
if (monoidNoiChuoi.ketHop("x", monoidNoiChuoi.rong) !== "x") throw new Error("kết hợp với rong phải giữ nguyên giá trị kia");
if (monoidNoiChuoi.ketHop(monoidNoiChuoi.rong, "y") !== "y") throw new Error("rong kết hợp ở BÊN TRÁI cũng phải giữ nguyên giá trị kia");
```

:::hints
- kind: attention
  body: "ketHop phải THẬT SỰ nối a với b (dùng +, không phải trả về một chuỗi cố định). rong phải là chuỗi rỗng \"\", không phải chuỗi nào khác."
- kind: strategy
  body: 'ketHop: (a, b) => a + b — toán tử + trên string là phép NỐI, không phải cộng số. rong: "" — chuỗi rỗng, viết bằng hai dấu ngoặc kép liền nhau.'
- kind: one-line
  body: "ketHop: a + b, rong: \"\""
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "chao ban"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một `interface`, nhiều giá trị: cộng, nối chuỗi, `max`, `||`. Mỗi cái
là một `Monoid` riêng — cùng hình dạng, khác cách kết hợp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có `Monoid` cho HAI giá trị (`ketHop(a, b)`). Nhưng thường cần
gộp một MẢNG DÀI giá trị — không chỉ hai. Có cách nào dùng một
`Monoid<T>` để gộp CẢ MỘT MẢNG `T[]` thành một `T` duy nhất không?

Bài sau trả lời — và bạn đã có sẵn công cụ từ T4.2.
::::

::::checkpoint{mastery=0.8}
::::
