---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.tu-viet-mot-monoid-moi
title: "Tự viết một `Monoid` MỚI"
summary: "monoidNhan: Monoid<number> — kết hợp bằng phép NHÂN, không phải cộng. Điểm khó: rong phải là 1, KHÔNG phải 0 (nhân với 0 luôn ra 0, phá mọi phép nhân). Giá trị trung tính phải ĐÚNG với phép kết hợp cụ thể."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.write-monoid]
requires: [alg.empty-array-needs-identity]
concepts: [alg.write-monoid]
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
Bạn đã ĐỌC nhiều `Monoid` có sẵn — cộng, nối chuỗi, max, `&&`. Hôm nay
tự tay VIẾT một cái mới, chưa từng thấy trong bài học.
::::

::::explain{#tu-tim-monoid-nhan}
Muốn `Monoid<number>` kết hợp bằng phép NHÂN. Bước đầu: tìm hàm
`ketHop` — dễ, `(a, b) => a * b`. Bước hai (dễ SAI): tìm `rong` — giá
trị trung tính của phép NHÂN.

Bài 2 đã dạy: giá trị trung tính là giá trị mà kết hợp với NÓ không
đổi gì cả — với phép NHÂN, đó là số nào NHÂN với BẤT KỲ số nào cũng
GIỮ NGUYÊN số đó? Không phải `0` (`5 * 0 = 0`, làm MẤT giá trị `5`
hoàn toàn — SAI hoàn toàn, đây là "hố tử thần" thường gặp khi mới học
Monoid). Số ĐÚNG là `1` — `5 * 1 = 5`, giữ nguyên.

```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidNhan: Monoid<number> = {
  ketHop: (a, b) => a * b,
  rong: 1,
};

console.log(monoidNhan.ketHop(5, monoidNhan.rong));
console.log(monoidNhan.ketHop(monoidNhan.rong, 5));
```

```text
5
5
```

Kết hợp `5` với `rong` (dù bên trái hay bên phải) — vẫn ra `5`, không
đổi. Đây mới ĐÚNG định nghĩa "trung tính" — không phải "một số nào đó
trông hợp lý".
::::

::::example{#0-lam-hong-ca-phep-nhan}
So sánh TRỰC TIẾP: dùng SAI `rong = 0` phá HỎNG mọi kết quả:

```typescript title=readonly
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidNhanDung: Monoid<number> = { ketHop: (a, b) => a * b, rong: 1 };
const monoidNhanSai: Monoid<number> = { ketHop: (a, b) => a * b, rong: 0 };

console.log(gopTatCa(monoidNhanDung, [2, 3, 4]));
console.log(gopTatCa(monoidNhanSai, [2, 3, 4]));
```

```text title=readonly
24
0
```

`monoidNhanDung` (`rong: 1`) ra ĐÚNG `2 * 3 * 4 = 24`. `monoidNhanSai`
(`rong: 0`) ra `0` — SAI hoàn toàn, vì `reduce` BẮT ĐẦU từ `0`, rồi
`0 * 2 = 0`, `0 * 3 = 0`, `0 * 4 = 0` — MỌI phép nhân sau đó đều bị
`0` "nuốt chửng". Không phải lỗi cú pháp hay lỗi kiểu — TypeScript
KHÔNG hề báo gì, vì `0` VẪN LÀ một `number` hợp lệ. Sai LOGIC, không
sai CÚ PHÁP — nguy hiểm hơn nhiều vì không có tín hiệu báo động.
::::

::::predict{#doan-monoid-nhan-mang-rong commitOnce}
```typescript
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

function gopTatCa<T>(m: Monoid<T>, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

const monoidNhan: Monoid<number> = { ketHop: (a, b) => a * b, rong: 1 };

console.log(gopTatCa(monoidNhan, []));
console.log(gopTatCa(monoidNhan, [5]));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`1` rồi `5`
:::

:::opt
`0` rồi `5` — vì mảng rỗng luôn phải ra `0`, giống mọi Monoid khác
::why
Gần đúng ở việc bạn tính đúng dòng THỨ HAI (`gopTatCa(monoidNhan, [5])
= 5`, một phần tử duy nhất giữ nguyên) — kết quả đó đúng.

Chỗ lệch: KHÔNG có "giá trị mặc định `0` cho mảng rỗng" áp dụng cho
MỌI `Monoid` — mỗi `Monoid` có `rong` RIÊNG (bài 2 đã dạy). Với
`monoidNhan`, `rong = 1` (không phải `0`) — mảng rỗng trả về ĐÚNG
`rong` của NÓ, tức `1`.
::
:::

:::opt
Máy báo lỗi ở dòng `gopTatCa(monoidNhan, [])` — `Monoid` kết hợp bằng
phép NHÂN không định nghĩa được cho mảng rỗng
::why
Gần đúng ở việc bạn nghi ngờ mảng rỗng có thể là trường hợp ĐẶC BIỆT
khó xử lý — một mối lo hợp lý nếu chưa chắc `rong` hoạt động thế nào.

Chỗ lệch: `Monoid<T>` LUÔN định nghĩa được cho mảng rỗng — CHÍNH LÀ lý
do `rong` tồn tại (bài 5 đã đo). `monoidNhan.rong = 1` là câu trả lời
SẴN CÓ, không cần xử lý ĐẶC BIỆT gì, không lỗi.
::
:::
::::

::::code{#monoid_nhan}
Viết `monoidNhan: Monoid<number>` — kết hợp bằng phép NHÂN.

```typescript title=starter
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidNhan: Monoid<number> = {
  ketHop: (a, b) => ___,
  rong: ___,
};

console.log(monoidNhan.ketHop(3, 4));
```

```typescript title=solution
interface Monoid<T> {
  ketHop: (a: T, b: T) => T;
  rong: T;
}

const monoidNhan: Monoid<number> = {
  ketHop: (a, b) => a * b,
  rong: 1,
};

console.log(monoidNhan.ketHop(3, 4));
```

```typescript title=test
function gopTatCa<T>(m: { ketHop: (a: T, b: T) => T; rong: T }, ds: T[]): T {
  return ds.reduce(m.ketHop, m.rong);
}

if (monoidNhan.ketHop(3, 4) !== 12) throw new Error("ketHop phải nhân hai số lại — 3 * 4 = 12");
if (monoidNhan.rong !== 1) throw new Error("rong phải là 1, KHÔNG phải 0 — nhân với 0 phá mọi phép nhân");
if (gopTatCa(monoidNhan, [2, 3, 4]) !== 24) throw new Error("gopTatCa với monoidNhan trên [2,3,4] phải ra 24");
if (gopTatCa(monoidNhan, []) !== 1) throw new Error("mảng rỗng phải ra đúng rong, tức 1");
```

:::hints
- kind: attention
  body: "ketHop phải THẬT SỰ nhân hai số (dùng *). rong phải là 1, KHÔNG phải 0 — 0 sẽ làm MỌI phép nhân sau đó ra 0."
- kind: strategy
  body: 'ketHop: (a, b) => a * b — toán tử * là phép nhân. rong: 1 — giá trị trung tính của phép nhân (x * 1 === x với mọi x).'
- kind: one-line
  body: "ketHop: a * b, rong: 1"
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
Không có công thức chung "trung tính luôn là 0 hay 1" — phải hiểu
ĐÚNG phép kết hợp mới tìm đúng giá trị trung tính của nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã tự viết được MỘT `Monoid` mới. Nếu cho một phép kết hợp HOÀN
TOÀN khác (chưa từng thấy) — làm sao BIẾT nó CÓ PHẢI một `Monoid` hợp
lệ không, và tìm ĐÚNG giá trị trung tính của nó?

Bài sau chốt cụm — tự làm điều đó với một phép kết hợp MỚI.
::::

::::checkpoint{mastery=0.8}
::::
