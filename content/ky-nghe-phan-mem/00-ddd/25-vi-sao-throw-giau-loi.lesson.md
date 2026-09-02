---
id: ky-nghe-phan-mem.ddd.vi-sao-throw-giau-loi
title: "Vì sao throw giấu khả năng lỗi khỏi kiểu — throw là goto hiện đại"
summary: "chia(a, b): number KHÔNG nói gì về khả năng throw trong chữ ký — TypeScript không cảnh báo, quên try/catch = crash. catch(e) có kiểu unknown — phải instanceof mới đọc được. try/catch phá vỡ composition: một bước throw làm vỡ TOÀN BỘ pipe(), không sửa được từng bước riêng."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 25
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ddd.throw-hides-errors]
requires: [ddd.workflow-capstone]
concepts: [ddd.throw-hides-errors]
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
Cụm mới. Trước khi học thêm combinator, một câu hỏi ngược: `throw`
(cách JavaScript vốn có để báo lỗi) SAI ở đâu mà cả track này tránh nó?
::::

::::explain{#throw-giau-kieu}
Chữ ký `chia(a: number, b: number): number` KHÔNG nói GÌ về khả năng
NÉM lỗi — TypeScript KHÔNG cảnh báo nếu caller QUÊN `try/catch`:

```typescript
function chia(a: number, b: number): number {
  if (b === 0) throw new Error("chia cho 0");
  return a / b;
}

console.log("trước khi gọi");
console.log(chia(10, 0)); // quên try/catch
console.log("sau khi gọi -- có in được không?");
```

```text
trước khi gọi
Lỗi runtime: chia cho 0
```

Dòng `"sau khi gọi..."` KHÔNG BAO GIỜ in ra — chương trình CRASH ngay
tại `chia(10, 0)`. TypeScript biên dịch SẠCH (không một cảnh báo nào)
— chữ ký `(a: number, b: number): number` nói dối: nó HỨA một `number`
nhưng CÓ THỂ không trả gì cả, chỉ NÉM một exception. Đây LÀ lý do
`throw` được gọi là "goto hiện đại" — nó NHẢY khỏi luồng chương trình
BÌNH THƯỜNG, ĐI NGƯỢC với mọi thứ kiểu tĩnh (`Result<T,E>`, DU) mà
track này đã xây — kiểu KHÔNG còn phản ánh ĐÚNG mọi đường đi của
chương trình.
::::

::::example{#catch-la-unknown}
NGAY CẢ khi BẮT được lỗi, `catch(e)` có kiểu `unknown` — TypeScript
KHÔNG biết `e` là gì, PHẢI thu hẹp bằng `instanceof` trước khi đọc bất
kỳ trường nào:

```typescript title=readonly
function chia(a: number, b: number): number {
  if (b === 0) throw new Error("chia cho 0");
  return a / b;
}

try {
  console.log(chia(10, 0));
} catch (e) {
  if (e instanceof Error) {
    console.log(`bắt được lỗi: ${e.message}`);
  } else {
    console.log("lỗi không rõ dạng");
  }
}
console.log(chia(10, 2));
```

```text title=readonly
bắt được lỗi: chia cho 0
5
```

Thử BỎ `instanceof` (viết thẳng `e.message`) → TypeScript báo
`TS18046: 'e' is of type 'unknown'` — biên dịch LỖI NGAY, không chạy
được. So sánh với `Result<T,E>`: `r.loi` LUÔN có kiểu `E` CHÍNH XÁC
(vd `LoiWorkflow[]`, bài 24) — KHÔNG BAO GIỜ cần thu hẹp kiểu thủ công
để đọc trường lỗi.
::::

::::predict{#doan-pipe-vo-vi-throw commitOnce}
```typescript
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe<A, B, C>(a: A, f: (a: A) => B, g: (b: B) => C): C;
function pipe<A, B, C, D>(a: A, f: (a: A) => B, g: (b: B) => C, h: (c: C) => D): D;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}

function nhanDoi(x: number): number { return x * 2; }
function chiaChoKhong(x: number): number {
  if (x === 0) throw new Error("chia cho 0");
  return 100 / x;
}
let daChayCongMuoi = false;
function congMuoi(x: number): number {
  daChayCongMuoi = true;
  return x + 10;
}

try {
  const ketQua = pipe(0, nhanDoi, chiaChoKhong, congMuoi);
  console.log(ketQua);
} catch (e) {
  console.log("bị bắt ở NGOÀI pipe");
}
console.log(daChayCongMuoi);
```

`pipe(0, nhanDoi, chiaChoKhong, congMuoi)`: `0` qua `nhanDoi` vẫn là
`0`, rồi `chiaChoKhong(0)` NÉM lỗi. Hai dòng cuối in ra gì?

:::opt{correct}
`bị bắt ở NGOÀI pipe` rồi `false`
:::

:::opt
`bị bắt ở NGOÀI pipe` rồi `true` — vì `pipe` vẫn CHẠY XONG toàn bộ các
hàm còn lại (kể cả `congMuoi`) TRƯỚC khi ném lỗi ra ngoài, giống như
`chainResult` VẪN chạy hết chuỗi trước khi trả kết quả cuối
::why
Gần đúng ở việc bạn liên tưởng tới `chainResult` (đã học từ bài 23) —
một PHÉP SO SÁNH hợp lý, vì cả hai đều là công cụ "ghép chuỗi hàm".

Chỗ lệch: đây CHÍNH LÀ điểm KHÁC BIỆT cốt lõi giữa `throw` và
`Result`. `chainResult` short-circuit bằng cách TRẢ VỀ SỚM một GIÁ TRỊ
(`loi(...)`) — luồng chương trình vẫn chạy BÌNH THƯỜNG, `reduce` NHÌN
THẤY giá trị đó và tự quyết định KHÔNG gọi bước sau. `throw` thì KHÁC
HẲN: nó NGẮT luồng NGAY LẬP TỨC, "nhảy" thẳng ra `catch` GẦN NHẤT, bỏ
qua MỌI code còn lại — kể cả `reduce`'s các lần lặp CÒN LẠI bên trong
`pipe`. `congMuoi` (bước SAU `chiaChoKhong` trong chuỗi) KHÔNG BAO GIỜ
được gọi — `daChayCongMuoi` giữ nguyên `false`.
::
:::

:::opt
Máy báo lỗi biên dịch — `pipe(0, nhanDoi, chiaChoKhong, congMuoi)`
không hợp lệ vì `chiaChoKhong` CÓ THỂ throw, TypeScript không cho phép
một hàm throw xuất hiện trong danh sách tham số của `pipe`
::why
Gần đúng ở việc bạn nhận ra `chiaChoKhong` CÓ NGUY CƠ throw — đúng LÀ
NGUYÊN NHÂN của cả bài học này.

Chỗ lệch: đây CHÍNH XÁC là vấn đề mà bài học vừa nêu — TypeScript
KHÔNG THEO DÕI "khả năng throw" ở tầng kiểu. Chữ ký `(x: number) =>
number` của `chiaChoKhong` giống HỆT `nhanDoi`/`congMuoi` về mặt KIỂU,
dù nó CÓ THỂ ném lỗi còn hai hàm kia thì KHÔNG — trình biên dịch không
có cách nào phân biệt. Biên dịch HOÀN TOÀN sạch.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`throw` ngắt luồng NGAY, bỏ qua mọi code còn lại, kiểu không phản ánh
khả năng lỗi. `Result` biến lỗi thành GIÁ TRỊ — kiểm tra được, composed
được, kiểu KHAI BÁO trung thực. Track tiếp theo: thêm công cụ trích
giá trị an toàn từ `Result`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

| | `throw`/`catch` | `Result<T,E>` |
|---|---|---|
| Khả năng lỗi | Ẩn trong chữ ký | Hiện RÕ trong kiểu trả về |
| Đọc lỗi | `catch(e: unknown)`, phải `instanceof` | `r.loi` có kiểu `E` chính xác |
| Composition | Vỡ `pipe()`/`chainResult` giữa chừng | `chainResult` short-circuit AN TOÀN |

Khi CHỈ cần MỘT giá trị cụ thể từ `Result` (không quan tâm lý do lỗi,
chỉ cần một mặc định) — có công cụ nào ngắn gọn hơn `switch(r.kind)`
mỗi lần không?
::::

::::checkpoint{mastery=0.8}
::::
