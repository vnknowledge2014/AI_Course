---
id: lap-trinh-ham.adt-pattern-matching.viet-mot-smart-constructor
title: "Tự viết một smart constructor"
summary: "Bài code có chấm điểm sống: viết `taoTuoiHopLe(n: number): TuoiHopLe | null` — một branded type cho số tuổi hợp lệ (`0 <= n <= 150`), trả `null` nếu ngoài khoảng. Test gọi với cả giá trị hợp lệ VÀ không hợp lệ, kiểm cả hai nhánh."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 30
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: ["ts.write-smart-constructor"]
requires: ["ts.smart-constructor-pattern"]
concepts: ["ts.write-smart-constructor"]
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
Bài trước: `taoEmail(s)` — hàm validate, CHỈ trả về `Email` khi chuỗi
hợp lệ, `null` khi không. Giờ tới lượt bạn: viết smart constructor của
RIÊNG mình — cho một brand khác hẳn, không phải chuỗi.
::::

::::explain{#smart-constructor-so}
```typescript
type TuoiHopLe = number & { readonly __brand: "TuoiHopLe" };

function taoTuoiHopLe(n: number): TuoiHopLe | null {
  if (n < 0 || n > 150) return null;
  return n as TuoiHopLe;
}

const t1 = taoTuoiHopLe(25);
const t2 = taoTuoiHopLe(200);

console.log(t1);
console.log(t2);
```

```text
25
null
```

`taoEmail` (bài trước) gắn brand lên `string`. `TuoiHopLe` gắn brand y
hệt cơ chế đó lên `number`: `number & { readonly __brand: "TuoiHopLe"
}`. Cơ chế brand không đổi — chỉ KIỂU NỀN đổi.

`taoTuoiHopLe` là CÁCH DUY NHẤT tạo ra một giá trị kiểu `TuoiHopLe`: nó
tự kiểm `n` có nằm trong khoảng hợp lệ không, CHỈ trả về brand (`as
TuoiHopLe`) nếu qua kiểm tra, còn không thì trả `null`. Số tuổi hợp lệ
là từ 0 đến 150 — điều kiện LOẠI viết là `n < 0 || n > 150` (loại
những giá trị NẰM NGOÀI hai đầu đó). `as TuoiHopLe` — ép kiểu — CHỈ
xuất hiện ĐÚNG một chỗ trong cả hàm, y hệt kỷ luật `taoEmail` đã lập.
::::

::::example{#bien-cua-khoang-hop-le}
Hai đầu ĐÚNG biên (0 và 150) và hai giá trị NGAY NGOÀI biên (-1 và
151) — cùng một hàm, bốn lời gọi:

```typescript title=readonly
type TuoiHopLe = number & { readonly __brand: "TuoiHopLe" };

function taoTuoiHopLe(n: number): TuoiHopLe | null {
  if (n < 0 || n > 150) return null;
  return n as TuoiHopLe;
}

console.log(taoTuoiHopLe(0));
console.log(taoTuoiHopLe(150));
console.log(taoTuoiHopLe(-1));
console.log(taoTuoiHopLe(151));
```

```text title=readonly
0
150
null
null
```

0 và 150 đều là số tuổi HỢP LỆ — điều kiện loại dùng `<` và `>` (chứ
không phải `<=`/`>=`), nên cả hai đầu biên ĐỀU lọt qua, hàm trả về
đúng giá trị đó. -1 và 151 đều vừa lọt RA NGOÀI — hàm trả `null`,
không `throw`, không `NaN`. Kết quả luôn LÀ một trong hai: một
`TuoiHopLe` hợp lệ, hoặc `null` — không có khả năng thứ ba.
::::

::::predict{#doan-bien-tren commitOnce}
```typescript
type TuoiHopLe = number & { readonly __brand: "TuoiHopLe" };

function taoTuoiHopLe(n: number): TuoiHopLe | null {
  if (n < 0 || n > 150) return null;
  return n as TuoiHopLe;
}

console.log(taoTuoiHopLe(150));
console.log(taoTuoiHopLe(151));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`150` rồi `null`
:::

:::opt
`null` rồi `150`
::why
Gần đúng ở việc bạn nhận ra CÓ một trong hai lời gọi phải bị từ chối —
quan sát rằng một giá trị vượt khoảng hợp lệ là đúng.

Chỗ lệch: nhầm CHIỀU — `taoTuoiHopLe(150)` gọi TRƯỚC, và 150 THOẢ điều
kiện (biên trên được chấp nhận, vì điều kiện loại dùng `>`, không phải
`>=`), nên trả về `150`. `taoTuoiHopLe(151)` mới là lời gọi vượt
khoảng, trả `null`.
::
:::

:::opt
Máy báo lỗi biên dịch ở dòng `taoTuoiHopLe(151)` — vì 151 vượt quá
150, TypeScript kiểm tra được giá trị cụ thể ngay lúc biên dịch
::why
Gần đúng ở việc bạn nhớ track này có nhiều ví dụ TypeScript CHẶN giá
trị sai ngay lúc biên dịch (ví dụ kiểu literal ở bài 2) — trực giác
"TypeScript kiểm khá chặt" không sai chung chung.

Chỗ lệch: tham số của `taoTuoiHopLe` khai `n: number` — MỌI số, kể cả
151, ĐỀU khớp kiểu `number`. TypeScript không thể biết riêng giá trị
151 "vượt khoảng [0, 150]" chỉ bằng cách nhìn KIỂU — đó là một ràng
buộc NGHIỆP VỤ, chỉ kiểm được LÚC CHẠY bên trong thân hàm (chính lý do
cần smart constructor). Chương trình biên dịch bình thường,
`taoTuoiHopLe(151)` chạy và trả `null`, không có lỗi biên dịch nào.
::
:::
::::

::::code{#tao_tuoi_hop_le}
Viết nốt điều kiện LOẠI trong `taoTuoiHopLe` — biên TRÊN của khoảng số
tuổi hợp lệ.

```typescript title=starter
type TuoiHopLe = number & { readonly __brand: "TuoiHopLe" };

function taoTuoiHopLe(n: number): TuoiHopLe | null {
  if (n < 0 || n > ___) return null;
  return n as TuoiHopLe;
}

console.log(taoTuoiHopLe(30));
console.log(taoTuoiHopLe(200));
```

```typescript title=solution
type TuoiHopLe = number & { readonly __brand: "TuoiHopLe" };

function taoTuoiHopLe(n: number): TuoiHopLe | null {
  if (n < 0 || n > 150) return null;
  return n as TuoiHopLe;
}

console.log(taoTuoiHopLe(30));
console.log(taoTuoiHopLe(200));
```

```typescript title=test
const t1 = taoTuoiHopLe(30);
if (t1 !== 30) throw new Error("taoTuoiHopLe(30) phải trả về 30 — đang là " + t1);
const t2 = taoTuoiHopLe(0);
if (t2 !== 0) throw new Error("taoTuoiHopLe(0) phải trả về 0 (biên dưới hợp lệ) — đang là " + t2);
const t3 = taoTuoiHopLe(150);
if (t3 !== 150) throw new Error("taoTuoiHopLe(150) phải trả về 150 (biên trên hợp lệ) — đang là " + t3);
const t4 = taoTuoiHopLe(-1);
if (t4 !== null) throw new Error("taoTuoiHopLe(-1) phải trả về null — đang là " + t4);
const t5 = taoTuoiHopLe(151);
if (t5 !== null) throw new Error("taoTuoiHopLe(151) phải trả về null — đang là " + t5);
const t6 = taoTuoiHopLe(200);
if (t6 !== null) throw new Error("taoTuoiHopLe(200) phải trả về null — đang là " + t6);
```

:::hints
- kind: attention
  body: "Chỗ trống là biên TRÊN của khoảng số tuổi hợp lệ — điều kiện `n > ___` phải LOẠI đúng những giá trị vượt biên trên đó."
- kind: strategy
  body: "Số tuổi hợp lệ là từ 0 đến bao nhiêu? Test gọi taoTuoiHopLe(150) mong đợi 150 hợp lệ, taoTuoiHopLe(151) mong đợi null — chỗ trống chính là ranh giới giữa hai kết quả đó."
- kind: one-line
  body: "Chỗ trống là: 150"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "30"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn vừa tự viết một smart constructor NGUYÊN VẸN — không chỉ đọc hiểu
khuôn có sẵn, mà tự đặt đúng điều kiện LOẠI theo khoảng hợp lệ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoEmail` gắn brand lên `string`. `taoTuoiHopLe` gắn brand lên
`number`. Brand không kén KIỂU NỀN — nó gắn được lên bất kỳ kiểu nào,
miễn có một hàm validate đi kèm, kiểm đúng và chỉ trả brand khi hợp lệ.

Bài sau: tự thiết kế MỘT brand HOÀN TOÀN MỚI (không phải `Email`,
không phải `TuoiHopLe`) VÀ smart constructor của nó — rồi dùng thẳng
kết quả trong một hàm khác chỉ chấp nhận ĐÚNG kiểu đã được xác nhận.
::::

::::checkpoint{mastery=0.8}
::::
