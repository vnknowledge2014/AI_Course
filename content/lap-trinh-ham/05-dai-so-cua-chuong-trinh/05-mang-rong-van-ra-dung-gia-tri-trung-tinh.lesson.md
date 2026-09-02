---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.mang-rong-van-ra-dung-gia-tri-trung-tinh
title: "Mảng RỖNG vẫn ra đúng — nhờ giá trị trung tính"
summary: "gopTatCa(monoidCong, []) phải ra 0, không phải lỗi hay undefined. rong phải có mặt trong Monoid vì reduce với mảng rỗng KHÔNG có phần tử nào để gộp, phải bắt đầu từ ĐÂU ĐÓ."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.empty-array-needs-identity]
requires: [alg.concat-all-via-reduce]
concepts: [alg.empty-array-needs-identity]
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
`gopTatCa` với mảng RỖNG — không có phần tử nào để gộp. Kết quả là gì?
Hôm nay trả lời, và giải thích TẠI SAO `rong` không phải field
"phòng hờ".
::::

::::explain{#mang-rong-ra-dung-rong}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidCong: Monoid<number> = { ketHop: (a, b) => a + b, rong: 0 };

console.log(gopTatCa(monoidCong, []));
console.log(typeof gopTatCa(monoidCong, []));
```

```text
0
number
```

`gopTatCa(monoidCong, [])` ra ĐÚNG `0` — một CON SỐ THẬT, không phải
lỗi, không phải `undefined`. Lý do: `ds.reduce(m.ketHop, m.rong)` với
`ds` rỗng KHÔNG lặp qua phần tử nào cả (không có gì để lặp) — nhưng vì
`reduce` ĐÃ ĐƯỢC CHO `m.rong` làm điểm khởi đầu, nó chỉ đơn giản TRẢ
LẠI `m.rong` NGUYÊN VẸN, không cần chạy vòng lặp nào.

Đây CHÍNH LÀ lý do `Monoid<T>` PHẢI có field `rong` (không chỉ
`ketHop`) — thiếu nó, `gopTatCa([])` không có gì để trả về.
::::

::::example{#khong-co-rong-thi-loi-that}
So sánh: `Array.prototype.reduce` KHÔNG BẮT BUỘC phải truyền giá trị
khởi đầu — nhưng nếu KHÔNG truyền, và mảng RỖNG, chuyện gì xảy ra?

```typescript title=readonly
const mangRong: number[] = [];
try {
  const ketQua = mangRong.reduce((a, b) => a + b);
  console.log(ketQua);
} catch (e) {
  console.log("lỗi: " + (e as Error).message);
}
```

```text title=readonly
lỗi: Reduce of empty array with no initial value
```

KHÔNG truyền giá trị khởi đầu cho `reduce`, và mảng RỖNG — JavaScript
NÉM một lỗi THẬT lúc chạy: "Reduce of empty array with no initial
value" (không có phần tử nào để BẮT ĐẦU tích luỹ, và cũng không có
điểm khởi đầu nào được cho). `gopTatCa` TRÁNH được lỗi này HOÀN TOÀN —
vì `Monoid<T>` BẮT BUỘC luôn có `rong`, `reduce` bên trong `gopTatCa`
LUÔN được truyền điểm khởi đầu, không bao giờ rơi vào tình huống này.
::::

::::predict{#doan-mang-rong-truyen-rong commitOnce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidNoiChuoi: Monoid<string> = { ketHop: (a, b) => a + b, rong: "" };
const monoidMax: Monoid<number> = { ketHop: (a, b) => Math.max(a, b), rong: -Infinity };

console.log(gopTatCa(monoidNoiChuoi, []));
console.log(gopTatCa(monoidMax, []));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`` (chuỗi rỗng) rồi `-Infinity`
:::

:::opt
Máy báo lỗi ở CẢ HAI dòng — mảng rỗng luôn gây lỗi cho `reduce`, bất kể
có truyền `rong` hay không
::why
Gần đúng ở việc bạn nhớ ĐÚNG rằng mảng rỗng CÓ THỂ gây lỗi cho `reduce`
(bài trước vừa đo điều đó thật) — quan sát đó có căn cứ.

Chỗ lệch: lỗi đó CHỈ xảy ra khi `reduce` KHÔNG được truyền giá trị
khởi đầu. `gopTatCa` LUÔN truyền `m.rong` (bài học cốt lõi của bài
này) — với `Monoid` ĐÚNG cách (luôn có `rong`), mảng rỗng KHÔNG BAO GIỜ
gây lỗi, chỉ đơn giản trả về `rong` nguyên vẹn.
::
:::

:::opt
`undefined` rồi `undefined` — vì `Monoid<string>`/`Monoid<number>`
không định nghĩa RÕ giá trị trả về cho trường hợp mảng rỗng
::why
Gần đúng ở việc bạn nghĩ tới trường hợp "chưa định nghĩa rõ" cho mảng
rỗng — một mối lo hợp lý nếu `Monoid` KHÔNG có field `rong`.

Chỗ lệch: `Monoid<T>` CÓ định nghĩa rõ — field `rong` CHÍNH LÀ giá trị
dành riêng cho trường hợp "không có phần tử nào". `monoidNoiChuoi.rong
= ""`, `monoidMax.rong = -Infinity` — cả hai đều được TRẢ VỀ NGUYÊN VẸN
khi mảng rỗng, không phải `undefined`.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`rong` không phải field "cho có" — nó là câu trả lời CHO SẴN cho câu
hỏi "mảng rỗng thì sao?", tránh một lỗi runtime thật mà `reduce` trần
có thể gặp.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có `Monoid<number>` (cộng, nhân, max) và `Monoid<string>` (nối
chuỗi). Có cách nào tự TÌM RA một `Monoid` MỚI — không chỉ đọc ví dụ có
sẵn?

Bài sau: tự tay viết một `Monoid` chưa từng thấy.
::::

::::checkpoint{mastery=0.8}
::::
