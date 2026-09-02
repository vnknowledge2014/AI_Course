---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.tu-viet-result-thanh-cong-hoac-loi
title: "Tự viết `Result<T, E>` — thành công hoặc lỗi CÓ LÝ DO"
summary: "type Result<T, E> = { kind: \"ok\"; giaTri: T } | { kind: \"loi\"; loi: E } cùng ok<T,E>()/loi<T,E>() — khác Option (chỉ 'có/không'), Result MANG THEO lý do lỗi trong biến thể \"loi\"."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [alg.result-type]
requires: [alg.pattern-match-option]
concepts: [alg.result-type]
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
`Option` nói "có" hoặc "không" — nhưng không nói TẠI SAO "không". Hôm
nay xây một kiểu MỚI, mang theo LÝ DO.
::::

::::explain{#tu-viet-result}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function ok<T, E>(giaTri: T): Result<T, E> {
  return { kind: "ok", giaTri };
}
function loi<T, E>(l: E): Result<T, E> {
  return { kind: "loi", loi: l };
}

function chuyenSo(s: string): Result<number, string> {
  const n = Number(s);
  return Number.isNaN(n) ? loi("chuỗi \"" + s + "\" không phải số") : ok(n);
}

console.log(chuyenSo("42"));
console.log(chuyenSo("abc"));
```

```text
{"kind":"ok","giaTri":42}
{"kind":"loi","loi":"chuỗi \"abc\" không phải số"}
```

`Result<T, E>` — CÙNG hình dạng `Option<T>` (một discriminated union
hai biến thể), nhưng biến thể "không thành công" (`"loi"`) MANG THEO
một GIÁ TRỊ (`loi: E`) — GIẢI THÍCH lý do thất bại. `chuyenSo("abc")`
không chỉ nói "thất bại" — nó nói RÕ: `"chuỗi \"abc\" không phải số"`.

`E` là kiểu của LÝ DO lỗi — có thể là `string` (một thông điệp), hoặc
bất kỳ kiểu nào khác (một mã lỗi, một object có nhiều field) — `Result`
KHÔNG bắt buộc `E` phải là `string`, hoàn toàn tự chọn theo nhu cầu.
::::

::::example{#doc-ket-qua-result}
Đọc `Result<T, E>` — buộc kiểm `kind` trước khi đọc field TƯƠNG ỨNG,
giống `Option`:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function ok<T, E>(giaTri: T): Result<T, E> {
  return { kind: "ok", giaTri };
}
function loi<T, E>(l: E): Result<T, E> {
  return { kind: "loi", loi: l };
}

function chuyenSo(s: string): Result<number, string> {
  const n = Number(s);
  return Number.isNaN(n) ? loi("không phải số: " + s) : ok(n);
}

for (const vb of ["10", "xyz", "3.5"]) {
  const kq = chuyenSo(vb);
  if (kq.kind === "ok") {
    console.log("thành công: " + kq.giaTri);
  } else {
    console.log("lỗi: " + kq.loi);
  }
}
```

```text title=readonly
thành công: 10
lỗi: không phải số: xyz
thành công: 3.5
```

Mỗi phần tử được xử lý ĐỘC LẬP — `"10"` và `"3.5"` (cả hai chuyển được
thành số hợp lệ, kể cả số thập phân) ra `"ok"`; `"xyz"` ra `"loi"`,
kèm THÔNG ĐIỆP RÕ chuỗi nào gây lỗi. Field `kq.loi` CHỈ đọc được SAU
khi đã kiểm `kq.kind === "loi"` — TypeScript không cho đọc TUỲ TIỆN.
::::

::::predict{#doan-result-hai-loai-e commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function ok<T, E>(giaTri: T): Result<T, E> {
  return { kind: "ok", giaTri };
}
function loi<T, E>(l: E): Result<T, E> {
  return { kind: "loi", loi: l };
}

function kiemTuoi(n: number): Result<number, { thongDiep: string; giaTriSai: number }> {
  if (n < 0 || n > 150) {
    return loi({ thongDiep: "tuổi phải trong khoảng 0-150", giaTriSai: n });
  }
  return ok(n);
}

const ketQua = kiemTuoi(200);
console.log(ketQua.kind === "loi" ? ketQua.loi.giaTriSai : "hợp lệ");
```

Dòng cuối in ra gì?

:::opt{correct}
`200`
:::

:::opt
Máy báo lỗi biên dịch — `E` (kiểu lỗi) trong `Result<T, E>` chỉ được
phép là `string`, không được là một `object` có nhiều field
::why
Gần đúng ở việc bạn nghĩ tới GIỚI HẠN có thể có cho kiểu `E` — hầu hết
ví dụ TRƯỚC bài này đúng là dùng `E = string`.

Chỗ lệch: `Result<T, E>` là GENERIC với `E` — `E` có thể là BẤT KỲ
kiểu nào, kể cả một `object` có nhiều field (`{ thongDiep: string;
giaTriSai: number }`, như ở đây). Không có giới hạn "chỉ dùng string
cho lỗi" — đây CHÍNH LÀ lợi ích của `Result` so với chỉ một thông điệp
chuỗi đơn: lỗi có thể mang NHIỀU thông tin có cấu trúc.
::
:::

:::opt
`hợp lệ` — vì `kiemTuoi(200)` biên dịch được nghĩa là `200` ĐÃ được coi
là một tuổi hợp lệ
::why
Gần đúng ở việc bạn suy luận "biên dịch được thì phải hợp lệ" — một
liên tưởng dễ hiểu nếu nhầm lẫn giữa kiểm tra KIỂU (lúc biên dịch) và
kiểm tra GIÁ TRỊ (lúc chạy).

Chỗ lệch: `kiemTuoi(200)` biên dịch được vì `200` là một `number` hợp
lệ (đúng KIỂU tham số) — nhưng hàm `kiemTuoi` sau đó TỰ kiểm GIÁ TRỊ
lúc CHẠY (`n < 0 || n > 150`), và `200 > 150` nên trả về `loi(...)`,
không phải `ok(...)`. `ketQua.kind` là `"loi"`, không phải `"co"`/`"ok"`
hợp lệ.
::
:::
::::

::::code{#result_constructors}
Tự viết `ok<T, E>(giaTri: T): Result<T, E>` và `loi<T, E>(l: E):
Result<T, E>`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function ok<T, E>(giaTri: T): Result<T, E> {
  return ___;
}
function loi<T, E>(l: E): Result<T, E> {
  return ___;
}

console.log(ok(42));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function ok<T, E>(giaTri: T): Result<T, E> {
  return { kind: "ok", giaTri };
}
function loi<T, E>(l: E): Result<T, E> {
  return { kind: "loi", loi: l };
}

console.log(ok(42));
```

```typescript title=test
const a = ok<number, string>(42);
if (a.kind !== "ok") throw new Error("ok() phải trả về kind là \"ok\"");
if (a.kind === "ok" && a.giaTri !== 42) throw new Error("ok(42) phải giữ giaTri là 42");
const b = loi<number, string>("thất bại");
if (b.kind !== "loi") throw new Error("loi() phải trả về kind là \"loi\"");
if (b.kind === "loi" && b.loi !== "thất bại") throw new Error("loi(\"thất bại\") phải giữ loi là \"thất bại\"");
const c = ok<number, string>(0);
if (c.kind !== "ok") throw new Error("ok(0) phải có kind là \"ok\", kể cả với giá trị falsy");
```

:::hints
- kind: attention
  body: "ok() phải trả về object có kind: \"ok\" VÀ giaTri. loi() phải trả về object có kind: \"loi\" VÀ loi (không phải giaTri)."
- kind: strategy
  body: 'return { kind: "ok", giaTri } — shorthand property. return { kind: "loi", loi: l } — field TÊN loi nhận GIÁ TRỊ tham số l (khác tên tham số, không viết gọn được).'
- kind: one-line
  body: "ok: return { kind: \"ok\", giaTri };  loi: return { kind: \"loi\", loi: l };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "ok"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Result<T, E>` — cùng khuôn `Option`, chỉ thêm một chỗ mang LÝ DO khi
thất bại. Chọn cái nào tuỳ bài toán có cần giải thích hay không.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn giờ có CẢ `Option<T>` LẪN `Result<T, E>`. Khi nào dùng cái nào?

Bài sau trả lời — bằng lý thuyết, không code.
::::

::::checkpoint{mastery=0.8}
::::
