---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.collect-all-gom-het-loi-khong-dung-som
title: "Collect-all — gom HẾT lỗi, không dừng sớm"
summary: "map2GomLoi(loiMot(\"thiếu tên\"), loiMot(\"thiếu tuổi\"), f) ra { kind: \"loi\", loi: [\"thiếu tên\", \"thiếu tuổi\"] } — CẢ HAI lỗi, không mất cái nào. Khác biệt cốt lõi applicative có mà chain/monad KHÔNG có."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 24
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.collect-all-semantics]
requires: [alg.fail-fast-semantics]
concepts: [alg.collect-all-semantics]
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
`map2` mất lỗi thứ hai. Hôm nay: một biến thể GOM ĐỦ mọi lỗi, không bỏ
sót cái nào.
::::

::::explain{#gom-loi-khong-mat}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }

function loiMot<T, E>(l: E): Result<T, E[]> {
  return { kind: "loi", loi: [l] };
}

function map2GomLoi<A, B, C, E>(
  ra: Result<A, E[]>,
  rb: Result<B, E[]>,
  f: (a: A, b: B) => C
): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

console.log(map2GomLoi(loiMot<number, string>("thiếu tên"), loiMot<number, string>("thiếu tuổi"), (a, b) => a + b));
```

```text
{"kind":"loi","loi":["thiếu tên","thiếu tuổi"]}
```

Khác biệt CỐT LÕI so với `map2` (bài 22-23): biến thể `"loi"` giờ
MANG một MẢNG lỗi (`loi: E[]`), không phải MỘT lỗi đơn. Khi CẢ `ra`
LẪN `rb` đều lỗi — `map2GomLoi` GHÉP CẢ HAI mảng lỗi lại
(`[...ra.loi, ...rb.loi]`), KHÔNG mất cái nào. Kết quả mang ĐỦ cả
`"thiếu tên"` VÀ `"thiếu tuổi"`.

Đây là điểm KHÁC BIỆT CỐT LÕI mà `map2GomLoi` (thuộc họ **Applicative**)
có mà `chain`/**Monad** (cụm sau) KHÔNG có: `chain` (bản chất PHỤ
THUỘC tuần tự — bước sau CẦN giá trị THẬT của bước trước) BẮT BUỘC
dừng ở lỗi đầu, không có cách nào "chạy song song rồi gom lỗi". `map2`/
`map2GomLoi` xử lý hai giá trị ĐỘC LẬP — không bên nào CẦN giá trị của
bên kia — nên GOM ĐƯỢC lỗi từ CẢ HAI phía, không cần dừng sớm.
::::

::::example{#so-sanh-ba-truong-hop}
Ba trường hợp, đặt cạnh nhau — chỉ MỘT lỗi, CHỈ lỗi kia, và CẢ HAI:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

const okA: Result<number, string[]> = ok(3);
const okB: Result<number, string[]> = ok(4);
const loiA = loiMot<number, string>("lỗi a");
const loiB = loiMot<number, string>("lỗi b");

console.log(map2GomLoi(loiA, okB, (a, b) => a + b));
console.log(map2GomLoi(okA, loiB, (a, b) => a + b));
console.log(map2GomLoi(loiA, loiB, (a, b) => a + b));
```

```text title=readonly
{"kind":"loi","loi":["lỗi a"]}
{"kind":"loi","loi":["lỗi b"]}
{"kind":"loi","loi":["lỗi a","lỗi b"]}
```

Chỉ `ra` lỗi → mảng lỗi CÓ MỘT phần tử (`["lỗi a"]`). Chỉ `rb` lỗi →
mảng lỗi có ĐÚNG lỗi của `rb`. CẢ HAI lỗi → mảng lỗi CÓ HAI phần tử,
ĐỦ CẢ hai. Số lượng lỗi trong mảng PHẢN ÁNH ĐÚNG số lượng phía THẬT
SỰ lỗi — không thiếu, không thừa.
::::

::::predict{#doan-gom-loi-hai-cai commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loiMot<T, E>(l: E): Result<T, E[]> { return { kind: "loi", loi: [l] }; }
function map2GomLoi<A, B, C, E>(ra: Result<A, E[]>, rb: Result<B, E[]>, f: (a: A, b: B) => C): Result<C, E[]> {
  if (ra.kind === "loi" && rb.kind === "loi") return { kind: "loi", loi: [...ra.loi, ...rb.loi] };
  if (ra.kind === "loi") return { kind: "loi", loi: ra.loi };
  if (rb.kind === "loi") return { kind: "loi", loi: rb.loi };
  return ok(f(ra.giaTri, rb.giaTri));
}

const ra: Result<number, string[]> = ok(10);
const rb: Result<number, string[]> = ok(20);

console.log(map2GomLoi(ra, rb, (a, b) => a + b));
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"ok","giaTri":30}`
:::

:::opt
`{"kind":"loi","loi":[]}` — vì `map2GomLoi` LUÔN trả về biến thể
`"loi"` với một mảng (rỗng nếu không có lỗi nào)
::why
Gần đúng ở việc bạn nghĩ tới việc mảng lỗi CÓ THỂ tồn tại ở dạng RỖNG
— một cấu trúc dữ liệu hợp lý về mặt lý thuyết.

Chỗ lệch: nhìn lại thân `map2GomLoi` — CẢ BA nhánh `if` đều kiểm CÓ
lỗi (`ra.kind === "loi"` hoặc `rb.kind === "loi"`) mới trả về `"loi"`.
Khi CẢ HAI `ra` VÀ `rb` đều `"ok"` (không nhánh `if` nào khớp), hàm
CHẠY TỚI dòng CUỐI: `return ok(f(ra.giaTri, rb.giaTri))` — trả về
`"ok"`, không phải `"loi"` rỗng.
::
:::

:::opt
Máy báo lỗi biên dịch — kiểu `E[]` (mảng lỗi) không tương thích với
`Result<C, E[]>` khi không có lỗi nào xảy ra
::why
Gần đúng ở việc bạn cân nhắc kiểu `E[]` có thể gây phức tạp trong một
số trường hợp — một mối lo hợp lý khi mới gặp kiểu lỗi dạng mảng.

Chỗ lệch: `E[]` CHỈ là kiểu của FIELD `loi` trong biến thể `"loi"` —
biến thể `"ok"` (`{ kind: "ok"; giaTri: C }`) KHÔNG hề có field `loi`
nào, không liên quan gì tới `E[]`. Khi cả hai đầu vào đều `ok`, kết
quả là biến thể `"ok"`, biên dịch và chạy hoàn toàn bình thường.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Collect-all không "tốt hơn" fail-fast một cách tuyệt đối — nó GIẢI
QUYẾT một vấn đề khác: cần biết TẤT CẢ lỗi, không chỉ lỗi đầu tiên.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã THẤY `map2GomLoi` hoạt động — giờ tự viết nó, từ đầu.
::::

::::checkpoint{mastery=0.8}
::::
