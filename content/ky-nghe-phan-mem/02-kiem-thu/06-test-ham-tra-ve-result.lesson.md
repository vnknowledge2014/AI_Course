---
id: ky-nghe-phan-mem.kiem-thu.test-ham-tra-ve-result
title: "Test hàm trả về Result — cả nhánh THÀNH CÔNG lẫn nhánh LỖI"
summary: "Hàm trả Result<T,E> (đã học T4.5) cần test CẢ HAI PHÍA: nhánh ok (giá trị ĐÚNG) VÀ TỪNG nhánh loi (MỖI mã lỗi RIÊNG, không chỉ MỘT trường hợp lỗi đại diện). assertDeepEqual so Result OBJECT toàn bộ, không chỉ field kind."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.test-result-branches]
requires: [kt.refactor-safety-net]
concepts: [kt.test-result-branches]
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
Cụm mới. `parsePrice` trả `number | null` — một hàm trả `Result<T,E>`
(đã học T4.5) — test hàm ĐÓ có gì KHÁC?
::::

::::explain{#test-result-ca-hai-phia}
Hàm trả `Result<T,E>` cần test **CẢ HAI PHÍA**: nhánh `ok` (giá trị
ĐÚNG) VÀ **TỪNG** nhánh `loi` **RIÊNG** (KHÔNG CHỈ một trường hợp lỗi
ĐẠI DIỆN — MỖI mã lỗi LÀ một assertion RIÊNG):

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

type LoiEmail = "rong" | "thieu_at" | "thieu_domain";

function xacThucEmail(input: string): Result<string, LoiEmail> {
  const s = input.trim().toLowerCase();
  if (s.length === 0) return loi("rong");
  if (!s.includes("@")) return loi("thieu_at");
  const [, domain] = s.split("@");
  if (!domain) return loi("thieu_domain");
  return ok(s);
}

assertDeepEqual(xacThucEmail(" An@Mail.COM "), ok("an@mail.com"), "trim + lower email hop le");
assertDeepEqual(xacThucEmail(""), loi("rong"), "email rong");
assertDeepEqual(xacThucEmail("khongco"), loi("thieu_at"), "thieu ky tu @");
```

```text
[PASS] trim + lower email hop le
[PASS] thieu ky tu @
[PASS] email rong
```

BỐN assertion, **BỐN** kịch bản KHÁC NHAU (MỘT `ok`, BA `loi` KHÁC
NHAU) — KHÔNG PHẢI MỘT test "chung chung" kiểm `xacThucEmail(...).kind
=== "loi"`. `assertDeepEqual` so sánh **TOÀN BỘ** object `Result`
(`{kind, giaTri}` HOẶC `{kind, loi}`) — KHÔNG CHỈ field `kind`, nên
BẮT được cả trường hợp `kind` ĐÚNG NHƯNG `loi`/`giaTri` bên trong SAI.
::::

::::example{#khong-kiem-du-cac-nhanh-bo-sot-bug}
Nếu CHỈ test MỘT nhánh `loi` "đại diện", BUG Ở nhánh `loi` KHÁC có
THỂ **KHÔNG BỊ PHÁT HIỆN**:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type LoiEmail = "rong" | "thieu_at" | "thieu_domain";

// BUG: nhánh "thieu_domain" bị viết NHẦM thành "thieu_at"
function xacThucEmailCoBug(input: string): Result<string, LoiEmail> {
  const s = input.trim().toLowerCase();
  if (s.length === 0) return loi("rong");
  if (!s.includes("@")) return loi("thieu_at");
  const [, domain] = s.split("@");
  if (!domain) return loi("thieu_at"); // SAI: phải là "thieu_domain"
  return ok(s);
}

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

// Nếu CHỈ test "rong" và "thieu_at" -- BUG này KHÔNG BỊ phát hiện
assertDeepEqual(xacThucEmailCoBug(""), loi("rong"), "email rong");
assertDeepEqual(xacThucEmailCoBug("khongco"), loi("thieu_at"), "thieu at");
// CHỈ khi test RIÊNG "thieu_domain" mới lộ ra:
try {
  assertDeepEqual(xacThucEmailCoBug("an@"), loi("thieu_domain"), "thieu domain");
} catch (e) {
  console.log((e as Error).message);
}
```

```text title=readonly
[PASS] email rong
[PASS] thieu at
[FAIL] thieu domain: mong {"kind":"loi","loi":"thieu_domain"}, nhan {"kind":"loi","loi":"thieu_at"}
```

HAI assertion ĐẦU (`"rong"`, `"thieu_at"`) ĐỀU `[PASS]` — bug Ở nhánh
`"thieu_domain"` **VẪN ẨN** cho tới khi CÓ assertion RIÊNG kiểm ĐÚNG
nhánh ĐÓ. Đây LÀ lý do "test TỪNG nhánh lỗi RIÊNG" KHÔNG PHẢI thừa
thãi — MỖI nhánh LÀ một CON ĐƯỜNG code RIÊNG, CÓ THỂ chứa bug RIÊNG.
::::

::::predict{#doan-assertdeepequal-so-sanh-toan-bo commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

const ketQua1: Result<string, string> = ok("an@mail.com");
const ketQua2: Result<string, string> = ok("binh@mail.com");

try {
  // Hai kết quả CÙNG kind ("ok"), NHƯNG giá trị bên trong KHÁC nhau
  assertDeepEqual(ketQua1, ketQua2, "so sanh hai ket qua ok khac gia tri");
} catch (e) {
  console.log((e as Error).message);
}
```

Dòng cuối in ra gì?

:::opt{correct}
`[FAIL] so sanh hai ket qua ok khac gia tri: mong {"kind":"ok","giaTri":"binh@mail.com"}, nhan {"kind":"ok","giaTri":"an@mail.com"}`
:::

:::opt
Không in gì cả — assertion ĐÃ QUA, vì CẢ HAI `Result` ĐỀU có
`kind: "ok"` (CÙNG "loại" kết quả — thành công), VÀ `assertDeepEqual`
CHỈ kiểm `kind` khớp NHAU LÀ đủ, không đi sâu vào GIÁ TRỊ bên trong
::why
Gần đúng ở việc bạn nhớ ĐÚNG `ketQua1.kind === ketQua2.kind` (CẢ HAI
đều `"ok"`) — một quan sát ĐÚNG về field `kind`.

Chỗ lệch: `assertDeepEqual` so sánh qua `JSON.stringify` **TOÀN BỘ**
object — `JSON.stringify(ketQua1)` = `'{"kind":"ok","giaTri":"an@mail.com"}'`,
`JSON.stringify(ketQua2)` = `'{"kind":"ok","giaTri":"binh@mail.com"}'` —
HAI chuỗi NÀY **KHÁC NHAU** (khác Ở PHẦN `giaTri`), nên `a !== e` là
`true`, assertion **THẤT BẠI**. `assertDeepEqual` KHÔNG dừng lại Ở
`kind` — nó so TOÀN BỘ CẤU TRÚC, đúng LÀ điểm mạnh của nó (bài 6's
explain: "bắt được cả trường hợp kind đúng nhưng nội dung sai").
::
:::

:::opt
Máy báo lỗi biên dịch — `Result<string, string>` không hợp lệ vì `T`
VÀ `E` cùng LÀ `string`, TypeScript đòi hai tham số kiểu generic
PHẢI khác nhau
::why
Gần đúng ở việc bạn để ý `T` VÀ `E` CÙNG được truyền `string` — một
quan sát ĐÚNG về việc HAI tham số kiểu generic NHẬN giá trị GIỐNG
NHAU.

Chỗ lệch: KHÔNG có ràng buộc nào trong TypeScript BUỘC hai tham số
kiểu generic KHÁC NHAU của MỘT type (`Result<T, E>`) phải nhận kiểu
CỤ THỂ khác nhau — `T = string` VÀ `E = string` HOÀN TOÀN hợp lệ
(chỉ đơn giản NGHĨA LÀ: giá trị THÀNH CÔNG VÀ giá trị LỖI ĐỀU LÀ
chuỗi, một tình huống HỢP LÝ, ví dụ CẢ HAI đều LÀ thông điệp văn
bản). Biên dịch sạch.
::
:::
::::

::::code{#viet_xacthucemail}
Tự viết BỐN nhánh của `xacThucEmail`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

type LoiEmail = "rong" | "thieu_at" | "thieu_domain";

function xacThucEmail(input: string): Result<string, LoiEmail> {
  const s = input.trim().toLowerCase();
  if (s.length === 0) return ___;
  if (!s.includes("@")) return ___;
  const [, domain] = s.split("@");
  if (!domain) return ___;
  return ___;
}

assertDeepEqual(xacThucEmail("An@Mail.com"), ok("an@mail.com"), "email hop le");
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

type LoiEmail = "rong" | "thieu_at" | "thieu_domain";

function xacThucEmail(input: string): Result<string, LoiEmail> {
  const s = input.trim().toLowerCase();
  if (s.length === 0) return loi("rong");
  if (!s.includes("@")) return loi("thieu_at");
  const [, domain] = s.split("@");
  if (!domain) return loi("thieu_domain");
  return ok(s);
}

assertDeepEqual(xacThucEmail("An@Mail.com"), ok("an@mail.com"), "email hop le");
```

```typescript title=test
assertDeepEqual(xacThucEmail(""), loi("rong"), "email rong phai bao loi rong");
assertDeepEqual(xacThucEmail("   "), loi("rong"), "email toan khoang trang phai bao loi rong");
assertDeepEqual(xacThucEmail("khongcoat"), loi("thieu_at"), "thieu @ phai bao loi thieu_at");
assertDeepEqual(xacThucEmail("an@"), loi("thieu_domain"), "thieu domain phai bao loi thieu_domain, KHONG PHAI thieu_at");
assertDeepEqual(xacThucEmail(" An@Mail.COM "), ok("an@mail.com"), "phai trim va lowercase");
```

:::hints
- kind: attention
  body: "Ba nhánh loi ứng với BA lý do KHÁC NHAU (rỗng, thiếu @, thiếu domain sau @) — mỗi nhánh loi() với ĐÚNG mã lỗi tương ứng, KHÔNG dùng lẫn. Nhánh cuối bọc ok(s)."
- kind: strategy
  body: 'loi("rong") : loi("thieu_at") : loi("thieu_domain") : ok(s) — bốn nhánh, đúng thứ tự bốn dòng if.'
- kind: one-line
  body: '___ (rỗng) = loi("rong")\n___ (thiếu @) = loi("thieu_at")\n___ (thiếu domain) = loi("thieu_domain")\n___ (hợp lệ) = ok(s)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Test hàm trả Result: test TỪNG nhánh lỗi RIÊNG, so sánh TOÀN BỘ object.
Bước tiếp theo: test một máy trạng thái — vẫn là hàm thuần.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một máy trạng thái (state machine, ví dụ đơn hàng: nhập → chờ thanh
toán → đã thanh toán) là MỘT hàm THUẦN `transition(trangThai, suKien)`
— test nó có gì KHÁC test một hàm bình thường?
::::

::::checkpoint{mastery=0.8}
::::
