---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.fail-fast-dung-o-loi-dau-tien
title: "Fail-fast — dừng Ở LỖI ĐẦU TIÊN, không xem tiếp"
summary: "map2(loi(\"lỗi a\"), ok(4), (a,b)=>a+b) phải ra loi(\"lỗi a\"). rb (ok(4) hợp lệ) KHÔNG hề được đọc tới. Bài predict: nếu CẢ HAI ra và rb đều lỗi, map2 fail-fast báo lỗi CỦA AI (chỉ ra, mất thông tin lỗi của rb)."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 23
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.fail-fast-semantics]
requires: [alg.map2-handwritten]
concepts: [alg.fail-fast-semantics]
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
`map2` dừng NGAY ở lỗi đầu tiên. Nhưng nếu CẢ HAI `ra` VÀ `rb` đều
lỗi — chuyện gì xảy ra với lỗi THỨ HAI?
::::

::::explain{#fail-fast-mat-loi-thu-hai}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2<A, B, C, E>(ra: Result<A, E>, rb: Result<B, E>, f: (a: A, b: B) => C): Result<C, E> {
  if (ra.kind === "loi") return loi(ra.loi);
  if (rb.kind === "loi") return loi(rb.loi);
  return ok(f(ra.giaTri, rb.giaTri));
}

const ketQua = map2(loi<number, string>("lỗi a"), loi<number, string>("lỗi b"), (a, b) => a + b);
console.log(ketQua);
```

```text
{"kind":"loi","loi":"lỗi a"}
```

CẢ `ra` (`loi("lỗi a")`) VÀ `rb` (`loi("lỗi b")`) ĐỀU lỗi — nhưng kết
quả CHỈ mang `"lỗi a"`. `"lỗi b"` biến mất HOÀN TOÀN, không xuất hiện
ở đâu trong kết quả. Lý do: `if (ra.kind === "loi") return loi(ra.loi);`
là dòng ĐẦU TIÊN trong thân `map2` — hàm TRẢ VỀ NGAY khi thấy `ra`
lỗi, KHÔNG BAO GIỜ chạm tới dòng kiểm `rb`.

Đây CHÍNH LÀ Ý NGHĨA của "fail-fast": dừng lại NGAY tại lỗi ĐẦU TIÊN
THEO THỨ TỰ KIỂM (ở đây là `ra` trước `rb`), không đọc tiếp để tìm
THÊM lỗi khác, dù có thể có.
::::

::::example{#thu-tu-tham-so-quyet-dinh}
Đổi THỨ TỰ hai đối số truyền vào — LỖI ĐƯỢC BÁO cũng đổi theo:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2<A, B, C, E>(ra: Result<A, E>, rb: Result<B, E>, f: (a: A, b: B) => C): Result<C, E> {
  if (ra.kind === "loi") return loi(ra.loi);
  if (rb.kind === "loi") return loi(rb.loi);
  return ok(f(ra.giaTri, rb.giaTri));
}

const loiA: Result<number, string> = loi("lỗi a");
const loiB: Result<number, string> = loi("lỗi b");

console.log(map2(loiA, loiB, (a, b) => a + b));
console.log(map2(loiB, loiA, (a, b) => a + b));
```

```text title=readonly
{"kind":"loi","loi":"lỗi a"}
{"kind":"loi","loi":"lỗi b"}
```

CÙNG hai `Result` (`loiA`, `loiB`), CHỈ đổi THỨ TỰ truyền vào — lỗi
ĐƯỢC BÁO cũng đổi theo (lỗi của tham số ĐẦU TIÊN luôn "thắng"). Đây
KHÔNG phải một chi tiết phụ — nó cho THẤY fail-fast phụ thuộc THỨ TỰ,
không phải "lỗi nào quan trọng hơn".
::::

::::predict{#doan-mat-loi-thu-hai commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function map2<A, B, C, E>(ra: Result<A, E>, rb: Result<B, E>, f: (a: A, b: B) => C): Result<C, E> {
  if (ra.kind === "loi") return loi(ra.loi);
  if (rb.kind === "loi") return loi(rb.loi);
  return ok(f(ra.giaTri, rb.giaTri));
}

function kiemTuoi(tuoi: number): Result<number, string> {
  return tuoi >= 0 && tuoi <= 150 ? ok(tuoi) : loi("tuổi không hợp lệ: " + tuoi);
}
function kiemDiem(diem: number): Result<number, string> {
  return diem >= 0 && diem <= 10 ? ok(diem) : loi("điểm không hợp lệ: " + diem);
}

const ketQua = map2(kiemTuoi(200), kiemDiem(15), (t, d) => t + d);
console.log(ketQua);
```

Cả `kiemTuoi(200)` VÀ `kiemDiem(15)` đều KHÔNG hợp lệ. Dòng cuối in
ra gì?

:::opt{correct}
`{"kind":"loi","loi":"tuổi không hợp lệ: 200"}`
:::

:::opt
`{"kind":"loi","loi":"điểm không hợp lệ: 15"}` — vì `kiemDiem` được
GỌI SAU, nên lỗi của nó "ghi đè" lên lỗi của `kiemTuoi`
::why
Gần đúng ở việc bạn xác định ĐÚNG `kiemDiem(15)` cũng thất bại
(`15 > 10`, ngoài khoảng hợp lệ) — quan sát đó đúng.

Chỗ lệch: không có khái niệm "gọi sau thì ghi đè" — `map2` kiểm `ra`
(tham số ĐẦU, ở đây là `kiemTuoi(200)`) TRƯỚC, và `if (ra.kind ===
"loi") return loi(ra.loi);` TRẢ VỀ NGAY tại đó. `kiemDiem(15)` CÓ được
tính TRƯỚC khi gọi `map2` (vì đối số hàm luôn tính trước khi hàm chạy)
— nhưng lỗi CỦA NÓ không hề được ĐỌC tới bên trong `map2`, vì hàm đã
`return` từ dòng kiểm `ra`.
::
:::

:::opt
Cả hai lỗi được gộp — kết quả phải là một danh sách CẢ HAI thông điệp
lỗi
::why
Gần đúng ở việc bạn mong muốn GOM ĐỦ cả hai lỗi — một mong muốn hợp lý
(và CHÍNH LÀ điều bài SAU sẽ giải quyết).

Chỗ lệch: `map2` (bài này đang học) là fail-fast — `loi: E` (không
phải `loi: E[]`) chỉ MANG được MỘT lỗi tại một thời điểm, không có
"danh sách lỗi" nào ở đây. Gộp ĐỦ nhiều lỗi cần một hàm KHÁC (collect-
all), chưa viết ở bài này.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Fail-fast dừng ở lỗi ĐẦU, mất thông tin lỗi CÒN LẠI. Hữu ích khi chỉ
cần biết "có lỗi hay không" — nhưng chưa đủ khi cần biết TẤT CẢ lỗi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Với một FORM đăng ký nhiều trường, người dùng THƯỜNG muốn biết TẤT CẢ
lỗi trong MỘT lần — không phải sửa từng lỗi một, gửi lại, thấy lỗi
tiếp theo. Có cách nào GOM ĐỦ mọi lỗi, không chỉ lỗi đầu tiên?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
