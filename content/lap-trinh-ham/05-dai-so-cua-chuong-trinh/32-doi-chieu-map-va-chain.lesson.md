---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.doi-chieu-map-va-chain
title: "Đối chiếu `map` và `chain` — khi nào dùng cái nào"
summary: "Dùng map khi hàm áp dụng LUÔN trả một giá trị TRẦN. Dùng chain khi hàm áp dụng TỰ NÓ trả một Option/Result. Nhầm map khi cần chain tạo ra kết quả LỒNG."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 32
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.map-vs-chain]
requires: [alg.chain-sequential-steps]
concepts: [alg.map-vs-chain]
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
`map` và `chain` — cả hai đều "áp dụng một hàm lên giá trị bên trong".
Khi nào dùng cái nào?
::::

::::explain{#tieu-chi-chon-map-hay-chain}
Tiêu chí DUY NHẤT cần hỏi: **hàm áp dụng (`f`) trả về giá trị TRẦN,
hay TỰ NÓ đã trả về một `Option`/`Result`?**

- Hàm trả giá trị TRẦN (`x => x * 2`, `x => x.length`, `x => String(x)`)
  → dùng `map`.
- Hàm TỰ NÓ trả về `Option`/`Result` (một phép tính CÓ THỂ thất bại,
  như `chia`, `chuyenSo`, `canBac2`) → dùng `chain`.

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function mapResult<T, U, E>(r: Result<T, E>, f: (x: T) => U): Result<U, E> {
  switch (r.kind) {
    case "ok": return ok(f(r.giaTri));
    case "loi": return loi(r.loi);
  }
}

function chia(a: number, b: number): Result<number, string> {
  return b === 0 ? loi("chia cho 0") : ok(a / b);
}

const dungNham = mapResult(ok<number, string>(10), (x) => chia(x, 2));
console.log(JSON.stringify(dungNham));
```

```text
{"kind":"ok","giaTri":{"kind":"ok","giaTri":5}}
```

DÙNG NHẦM `map` khi CẦN `chain` (`chia` tự nó đã trả `Result`) —
KẾT QUẢ LỒNG (`ok(ok(5))`, một `Result<Result<number,string>,string>`)
— ĐÚNG lỗi bài 28 đã dạy cho `Option`, giờ lặp lại y hệt cho `Result`.
::::

::::example{#nham-chieu-nguoc-lai-typescript-bat-loi}
Dùng NHẦM `chain` khi CẦN `map` — TypeScript THƯỜNG báo lỗi KIỂU ngay
lúc biên dịch, KHÔNG chạy tới lúc lỗi ẩn như trường hợp ngược lại:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

// nhanDoi trả về number TRẦN, không phải Result — chainResult ĐÒI f trả Result
const nhanDoi = (x: number): number => x * 2;

// dòng dưới đây KHÔNG biên dịch được nếu bỏ comment:
// const sai = chainResult(ok<number, string>(5), nhanDoi);

console.log("nếu bỏ comment dòng trên, TypeScript báo lỗi kiểu ngay lập tức");
```

```text title=readonly
nếu bỏ comment dòng trên, TypeScript báo lỗi kiểu ngay lập tức
```

`chainResult` đòi `f: (x: T) => Result<U, E>` — `nhanDoi` trả về
`number` TRẦN, KHÔNG khớp chữ ký đó. TypeScript CHẶN ngay lúc biên
dịch (mã TS2345, "đối số sai kiểu" — đã học từ T4.0a). ĐÂY LÀ điểm
KHÁC BIỆT quan trọng: nhầm `map` thành `chain` khi cần `map` bị BẮT
NGAY LÚC BIÊN DỊCH; nhầm `chain` thành `map` khi cần `chain` (ví dụ
`explain` phía trên) KHÔNG bị bắt — biên dịch SẠCH, chỉ tạo ra kết
quả LỒNG khó đọc, không lỗi rõ ràng.
::::

::::predict{#doan-nham-map-khi-can-chain commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}

const timSoAm = (ds: number[]): Option<number> => {
  const x = ds.find((n) => n < 0);
  return x === undefined ? khong() : co(x);
};

const ketQua = mapOption(co([3, -5, 8]), timSoAm);
console.log(ketQua.kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`co`
:::

:::opt
`khong` — vì dùng NHẦM `map` khi cần `chain` khiến kết quả CUỐI luôn
là `khong`, đúng bản chất một lỗi thiết kế
::why
Gần đúng ở việc bạn nhận ra ĐÂY LÀ tình huống DÙNG NHẦM `map` khi cần
`chain` (`timSoAm` tự nó trả `Option`) — quan sát về LOẠI LỖI đó đúng.

Chỗ lệch: dùng nhầm `map` khi cần `chain` tạo ra kết quả LỒNG (`kind`
bên NGOÀI vẫn là `"co"`, chứa một `Option` KHÁC bên trong), KHÔNG
BIẾN kết quả ngoài cùng thành `"khong"`. `mapOption` luôn BỌC kết quả
của `f` vào MỘT lớp `co(...)` MỚI khi đầu vào là `"co"` — LỚP NGOÀI
CÙNG luôn là `"co"` (bọc quanh một `Option` khác), không đổi thành
`"khong"`.
::
:::

:::opt
Máy báo lỗi biên dịch — `timSoAm` trả về `Option<number>`, không khớp
chữ ký `f: (x) => U` mà `mapOption` đòi hỏi
::why
Gần đúng ở việc bạn nhớ ĐÚNG: dùng nhầm `chain` thành `map` (chiều
NGƯỢC LẠI) THẬT SỰ bị TypeScript chặn ngay — cơ chế đó có thật (đã học
ở khối `example`).

Chỗ lệch: đây là chiều NGƯỢC LẠI — dùng `map` khi CẦN `chain`. `U`
trong chữ ký `mapOption<T, U>(o, f: (x:T) => U)` là GENERIC, chấp
nhận BẤT KỲ kiểu trả về nào — kể cả CHÍNH `U = Option<number>`.
Không có ràng buộc nào ngăn `f` trả về một `Option`. Biên dịch sạch,
chỉ tạo ra kết quả LỒNG.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map` cho hàm trả giá trị trần. `chain` cho hàm tự nó trả `Option`/
`Result`. Nhầm chiều nào cũng có hậu quả — nhưng chỉ MỘT chiều được
TypeScript tự bắt.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã có đủ công cụ để ghép một chuỗi THẬT — nhiều bước, mỗi bước có
thể lỗi, ghép qua `chain`.
::::

::::checkpoint{mastery=0.8}
::::
