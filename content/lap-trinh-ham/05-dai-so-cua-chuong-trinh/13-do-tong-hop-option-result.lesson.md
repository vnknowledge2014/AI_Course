---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.do-tong-hop-option-result
title: "Đo tổng hợp: `Option` & `Result` tự viết"
summary: "Viết một hàm dùng Option (tìm số CHẴN đầu tiên, không có LÝ DO khi không tìm thấy) VÀ một hàm dùng Result (chuyển chuỗi thành số nguyên, LỖI ghi rõ chuỗi gốc). Không khái niệm mới — đo khả năng CHỌN ĐÚNG kiểu ADT theo ngữ cảnh."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 13
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [alg.review-option-result]
requires: [alg.option-vs-result]
concepts: [alg.review-option-result]
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
Cụm 2 chốt lại: `Option`, `Result`, và biết CHỌN đúng cái nào. Hôm nay
ghép cả hai trong một bài.
::::

::::explain{#ghep-option-va-result}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function timSoChanDauTien(ds: number[]): Option<number> {
  const x = ds.find((n) => n % 2 === 0);
  return x === undefined ? khong() : co(x);
}

function chuyenThanhSoNguyen(vb: string): Result<number, string> {
  const n = Number(vb);
  if (Number.isNaN(n) || !Number.isInteger(n)) {
    return loi("chuỗi \"" + vb + "\" không phải số nguyên");
  }
  return ok(n);
}

console.log(timSoChanDauTien([5, 3, 8, 2]));
console.log(chuyenThanhSoNguyen("42"));
```

```text
{"kind":"co","giaTri":8}
{"kind":"ok","giaTri":42}
```

`timSoChanDauTien` dùng `Option` — "không tìm thấy số chẵn" là kết quả
BÌNH THƯỜNG, không cần lý do (bài 12 đã dạy tiêu chí này). `chuyenThanhSoNguyen`
dùng `Result` — chuyển thất bại thì LỖI ghi rõ CHUỖI NÀO gây ra, đúng
Ý "Result mang lý do". Hai hàm, hai kiểu ADT khác nhau, mỗi cái ĐÚNG
với bản chất việc nó làm.
::::

::::example{#dung-ca-hai-trong-mot-chuong-trinh}
Một chương trình THẬT thường cần CẢ hai, cho hai việc khác nhau:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function timSoChanDauTien(ds: number[]): Option<number> {
  const x = ds.find((n) => n % 2 === 0);
  return x === undefined ? khong() : co(x);
}

function chuyenThanhSoNguyen(vb: string): Result<number, string> {
  const n = Number(vb);
  if (Number.isNaN(n) || !Number.isInteger(n)) {
    return loi("chuỗi \"" + vb + "\" không phải số nguyên");
  }
  return ok(n);
}

const khongCoChan = timSoChanDauTien([1, 3, 5]);
const loiChuyenSo = chuyenThanhSoNguyen("ba");

console.log(khongCoChan.kind === "co" ? "có" : "không có số chẵn");
console.log(loiChuyenSo.kind === "loi" ? loiChuyenSo.loi : "chuyển thành công");
```

```text title=readonly
không có số chẵn
chuỗi "ba" không phải số nguyên
```

`[1, 3, 5]` không có số chẵn — `Option` báo "không có", KHÔNG kèm lý
do (đơn giản không có gì để nói thêm). `"ba"` không chuyển được thành
số — `Result` báo lỗi kèm CHUỖI GỐC gây ra, đủ thông tin để người gọi
biết SỬA GÌ.
::::

::::predict{#doan-ghep-option-result commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function timSoChanDauTien(ds: number[]): Option<number> {
  const x = ds.find((n) => n % 2 === 0);
  return x === undefined ? khong() : co(x);
}

const ketQua = timSoChanDauTien([7, 9, 0, 3]);
console.log(ketQua.kind === "co" ? ketQua.giaTri : "không có");
```

Dòng cuối in ra gì?

:::opt{correct}
`0`
:::

:::opt
`không có` — vì `0` không phải một số CHẴN "thật sự", nó là giá trị
đặc biệt (falsy), không được `n % 2 === 0` coi là chẵn
::why
Gần đúng ở việc bạn cảnh giác `0` có thể bị đối xử khác thường (đúng
bài học bẫy falsy-value từ bài 8-9) — cảnh giác đó có căn cứ nói chung.

Chỗ lệch: về mặt TOÁN HỌC, `0` LÀ một số chẵn (`0 % 2 === 0` tính ra
`true`, không có ngoại lệ). VÀ vì `timSoChanDauTien` dùng `Option`
(kiểm `kind`, không kiểm truthy/falsy — CHÍNH LÀ bài học của bài 9),
`0` được nhận diện ĐÚNG là "tìm thấy", không bị bỏ sót.
::
:::

:::opt
Máy báo lỗi biên dịch — `ds.find(...)` không hoạt động đúng khi mảng
chứa `0`
::why
Gần đúng ở việc bạn cân nhắc `0` có thể gây vấn đề gì đó cho `find` —
một mối lo hợp lý nếu chưa chắc `find` xử lý giá trị "đặc biệt" ra sao.

Chỗ lệch: `Array.prototype.find` hoạt động HOÀN TOÀN BÌNH THƯỜNG với
`0` — nó chỉ đơn giản kiểm ĐIỀU KIỆN (`n % 2 === 0`) trên TỪNG phần
tử, không quan tâm giá trị đó là gì. Không có lỗi biên dịch hay lỗi
chạy nào.
::
:::
::::

::::code{#option_result_review}
Viết `timSoChanDauTien(ds: number[]): Option<number>` (không lý do khi
không tìm thấy) VÀ `chuyenThanhSoNguyen(vb: string): Result<number,
string>` (lỗi ghi rõ chuỗi gốc).

```typescript title=starter
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function timSoChanDauTien(ds: number[]): Option<number> {
  const x = ds.find((n) => n % 2 === 0);
  return ___;
}

function chuyenThanhSoNguyen(vb: string): Result<number, string> {
  const n = Number(vb);
  if (Number.isNaN(n) || !Number.isInteger(n)) {
    return ___;
  }
  return ok(n);
}

console.log(timSoChanDauTien([5, 3, 8, 2]));
```

```typescript title=solution
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function timSoChanDauTien(ds: number[]): Option<number> {
  const x = ds.find((n) => n % 2 === 0);
  return x === undefined ? khong() : co(x);
}

function chuyenThanhSoNguyen(vb: string): Result<number, string> {
  const n = Number(vb);
  if (Number.isNaN(n) || !Number.isInteger(n)) {
    return loi("chuỗi \"" + vb + "\" không phải số nguyên");
  }
  return ok(n);
}

console.log(timSoChanDauTien([5, 3, 8, 2]));
```

```typescript title=test
const a = timSoChanDauTien([5, 3, 8, 2]);
if (a.kind !== "co") throw new Error("phải tìm thấy số chẵn trong [5,3,8,2]");
if (a.kind === "co" && a.giaTri !== 8) throw new Error("số chẵn đầu tiên phải là 8");
const b = timSoChanDauTien([5, 3, 7]);
if (b.kind !== "khong") throw new Error("không có số chẵn thì phải là \"khong\"");

const c = chuyenThanhSoNguyen("42");
if (c.kind !== "ok") throw new Error("\"42\" phải chuyển thành công");
if (c.kind === "ok" && c.giaTri !== 42) throw new Error("giá trị phải là 42");
const d = chuyenThanhSoNguyen("abc");
if (d.kind !== "loi") throw new Error("\"abc\" phải là loi");
if (d.kind === "loi" && d.loi !== "chuỗi \"abc\" không phải số nguyên") throw new Error("lỗi phải ghi rõ chuỗi gốc");
```

:::hints
- kind: attention
  body: "timSoChanDauTien: nếu x là undefined thì trả khong(), ngược lại co(x). chuyenThanhSoNguyen: nếu không phải số nguyên hợp lệ thì trả loi(...) ghi rõ chuỗi vb gốc."
- kind: strategy
  body: 'return x === undefined ? khong() : co(x). return loi("chuỗi \\"" + vb + "\\" không phải số nguyên") — nối chuỗi vb NGUYÊN VĂN vào thông điệp lỗi.'
- kind: one-line
  body: "timSoChanDauTien: return x === undefined ? khong() : co(x);  chuyenThanhSoNguyen: return loi(\"chuỗi \\\"\" + vb + \"\\\" không phải số nguyên\");"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "co"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Option` và `Result` — cùng khuôn ADT, mỗi cái đúng chỗ của nó. Bạn giờ
tự viết VÀ tự chọn được cả hai.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã viết `mapOption`/`mapResult` cho MỘT bài chưa? Chưa — nhưng bạn
ĐÃ dùng `.map()` trên mảng suốt từ T4.2. Có cách nào áp dụng "biến đổi
giá trị bên trong, giữ nguyên vỏ" cho `Option`/`Result`, giống hệt
`Array.map` không?

Cụm sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
