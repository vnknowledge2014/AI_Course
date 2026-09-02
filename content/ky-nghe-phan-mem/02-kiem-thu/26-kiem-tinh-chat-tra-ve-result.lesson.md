---
id: ky-nghe-phan-mem.kiem-thu.kiem-tinh-chat-tra-ve-result
title: "Capstone: Bộ kiểm tính chất TRẢ VỀ Result thay vì throw"
summary: "Bài chốt cụm 5, code có chấm điểm sống: viết LẠI kiemTraTinhChat để trả Result<{soLanDung}, {input, viTri}> THAY VÌ console.log/throw — nối phong cách track (Result đã học T4.5) — áp dụng vào tính chất đi-về cho một cặp serialize/parse số nguyên."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 26
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kt.gate-boss-pbt-patterns]
requires: [kt.hand-rolled-shrinking]
concepts: [kt.gate-boss-pbt-patterns]
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
Bài chốt cụm 5. `kiemTraTinhChat` TỪ TRƯỚC tới GIỜ throw/console.log
— viết LẠI để trả `Result` (đã học T4.5), NỐI phong cách track?
::::

::::explain{#ket-qua-tinh-chat-la-result}
Viết LẠI `kiemTraTinhChat` để TRẢ `Result<{soLanDung}, {input, viTri}>`
**THAY VÌ** `throw`/`console.log` — NGƯỜI GỌI **TỰ QUYẾT ĐỊNH** làm
GÌ VỚI kết quả (in RA, LƯU log, THU nhỏ ca lỗi — bài 25), THAY VÌ bị
`throw` NGẮT ngang:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

type BoSinh<T> = { generate: () => T };
type KetQuaTinhChat<A> = Result<{ soLanDung: number }, { input: A; viTri: number }>;

function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): KetQuaTinhChat<A> {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) {
      return loi({ input, viTri: lan });
    }
  }
  return ok({ soLanDung: soLan });
}
```

`Result<{soLanDung}, {input, viTri}>` — nhánh **ĐÚNG** MANG SỐ lần
đã thử THÀNH CÔNG (`soLanDung`); nhánh **LỖI** MANG **CẢ HAI** thông
tin CẦN để debug: `input` GÂY thất bại VÀ `viTri` (lần thứ MẤY).
KHÔNG mất thông tin NÀO so VỚI cách `throw` cũ — CHỈ đóng gói LẠI
theo phong cách Result ĐÃ dùng xuyên suốt dự án.
::::

::::example{#ap-dung-cho-serialize-parse}
Áp dụng VÀO tính chất Đi-Về (bài 23) CHO MỘT cặp `serialize`/`parse`
số nguyên — VÀ MỘT phiên bản CỐ Ý CÓ BUG để THẤY nhánh `loi` hoạt
động:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type BoSinh<T> = { generate: () => T };
type KetQuaTinhChat<A> = Result<{ soLanDung: number }, { input: A; viTri: number }>;
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): KetQuaTinhChat<A> {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) return loi({ input, viTri: lan });
  }
  return ok({ soLanDung: soLan });
}

function serialize(n: number): string {
  return `N:${n}`;
}
function parse(s: string): number {
  return Number(s.slice(2)); // bo qua "N:" (hai ky tu)
}
function parseCoBug(s: string): number {
  return Number(s.slice(1)); // BUG: chi bo MOT ky tu, con sot lai ":"
}

const boSinhCoDinh: BoSinh<number> = { generate: () => 42 };

console.log(JSON.stringify(kiemTraTinhChat(boSinhCoDinh, (n) => parse(serialize(n)) === n, 100)));
console.log(JSON.stringify(kiemTraTinhChat(boSinhCoDinh, (n) => parseCoBug(serialize(n)) === n, 100)));
```

```text title=readonly
{"kind":"ok","giaTri":{"soLanDung":100}}
{"kind":"loi","loi":{"input":42,"viTri":0}}
```

`parse` ĐÚNG → `ok({soLanDung: 100})` (chạy HẾT `100` lần, KHÔNG lần
NÀO fail). `parseCoBug` (SÓT LẠI ký tự `":"`, khiến `Number(...)` ra
`NaN`) → `loi({input: 42, viTri: 0})` NGAY LẦN ĐẦU (`viTri: 0`) —
KHÔNG throw, KHÔNG dừng chương trình — CHỈ trả VỀ MỘT giá trị người
GỌI CÓ THỂ kiểm tra, LOG, HOẶC đưa VÀO `thuNhoSoNguyen` (bài 25) để
thu nhỏ TIẾP nếu `input` LÀ MỘT SỐ.
::::

::::predict{#doan-solan-tuy-chinh commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
type KetQuaTinhChat<A> = Result<{ soLanDung: number }, { input: A; viTri: number }>;
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): KetQuaTinhChat<A> {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) return loi({ input, viTri: lan });
  }
  return ok({ soLanDung: soLan });
}

const kq = kiemTraTinhChat(soNguyen(1, 1), (n) => n === 1, 7);
console.log(JSON.stringify(kq));
```

Dòng cuối in ra gì?

:::opt{correct}
`{"kind":"ok","giaTri":{"soLanDung":7}}`
:::

:::opt
`{"kind":"ok","giaTri":{"soLanDung":100}}` — vì `kiemTraTinhChat`
LUÔN dùng giá trị MẶC ĐỊNH `soLan = 100` để TÍNH `soLanDung` TRONG
nhánh `ok`, BẤT KỂ đối số THỨ BA (`7`) được TRUYỀN vào HAY KHÔNG
::why
Gần đúng ở việc bạn nhớ ĐÚNG `kiemTraTinhChat` khai GIÁ TRỊ mặc định
`soLan: number = 100` — một quan sát ĐÚNG về CHỮ KÝ hàm.

Chỗ lệch: giá trị MẶC ĐỊNH CHỈ áp dụng khi đối số **KHÔNG được
TRUYỀN** — Ở lời gọi NÀY, `7` **ĐƯỢC truyền TƯỜNG MINH** làm đối số
THỨ BA, nên `soLan` NHẬN `7` (KHÔNG PHẢI `100`). Vòng lặp chạy ĐÚNG
`7` lần (`n === 1` LUÔN đúng, vì `soNguyen(1, 1)` LUÔN sinh `1`),
KHÔNG lần NÀO fail — trả `ok({ soLanDung: 7 })`.
::
:::

:::opt
Máy báo lỗi biên dịch — `KetQuaTinhChat<A>` khai `Result<{soLanDung:
number}, {input: A; viTri: number}>`, NHƯNG `ok({ soLanDung: soLan
})` truyền MỘT object CHỈ CÓ field `soLanDung`, THIẾU field `input`
VÀ `viTri` MÀ kiểu `Result` đòi HỎI CẢ HAI nhánh PHẢI CÓ ĐỦ
::why
Gần đúng ở việc bạn để ý `KetQuaTinhChat<A>` LÀ MỘT kiểu `Result`
KHÁ "nặng" (HAI field Ở NHÁNH lỗi) — một quan sát ĐÚNG về ĐỘ PHỨC
TẠP của kiểu.

Chỗ lệch: `Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi";
loi: E }` LÀ MỘT **UNION** — HAI NHÁNH **HOÀN TOÀN TÁCH BIỆT**, MỖI
GIÁ TRỊ CỤ THỂ CHỈ THUỘC **MỘT** NHÁNH TẠI MỘT THỜI ĐIỂM (KHÔNG PHẢI
"PHẢI CÓ ĐỦ CẢ HAI field CÙNG LÚC" — đó SẼ LÀ MỘT kiểu GIAO `&`,
KHÔNG PHẢI union `|`). `ok({ soLanDung: soLan })` tạo NHÁNH `"ok"`
(CHỈ CẦN field `giaTri: {soLanDung}`), HOÀN TOÀN KHÔNG liên QUAN tới
field `input`/`viTri` (CHỈ THUỘC NHÁNH `"loi"`). Biên dịch sạch.
::
:::
::::

::::code{#viet_kiemtratinhchat_result}
Viết LẠI `kiemTraTinhChat` để TRẢ `Result` thay vì throw.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type BoSinh<T> = { generate: () => T };
type KetQuaTinhChat<A> = Result<{ soLanDung: number }, { input: A; viTri: number }>;

function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): KetQuaTinhChat<A> {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) {
      return ___;
    }
  }
  return ___;
}

const boSinhCoDinh: BoSinh<number> = { generate: () => 5 };
const ketQuaThu = kiemTraTinhChat(boSinhCoDinh, (n) => n === 5, 3);
console.log(JSON.stringify(ketQuaThu) === JSON.stringify(ok({ soLanDung: 3 })) ? "[PASS] tinh chat dung phai tra ok" : "[FAIL] tinh chat dung phai tra ok");
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
type BoSinh<T> = { generate: () => T };
type KetQuaTinhChat<A> = Result<{ soLanDung: number }, { input: A; viTri: number }>;

function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): KetQuaTinhChat<A> {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) {
      return loi({ input, viTri: lan });
    }
  }
  return ok({ soLanDung: soLan });
}

const boSinhCoDinh: BoSinh<number> = { generate: () => 5 };
const ketQuaThu = kiemTraTinhChat(boSinhCoDinh, (n) => n === 5, 3);
console.log(JSON.stringify(ketQuaThu) === JSON.stringify(ok({ soLanDung: 3 })) ? "[PASS] tinh chat dung phai tra ok" : "[FAIL] tinh chat dung phai tra ok");
```

```typescript title=test
function assertDeepEqual<T>(actual: T, expected: T, label: string): void {
  const a = JSON.stringify(actual);
  const e = JSON.stringify(expected);
  if (a !== e) throw new Error(`[FAIL] ${label}: mong ${e}, nhan ${a}`);
  console.log(`[PASS] ${label}`);
}

const bsLuonDung: BoSinh<number> = { generate: () => 5 };
assertDeepEqual(kiemTraTinhChat(bsLuonDung, (n) => n === 5, 3), ok({ soLanDung: 3 }), "tinh chat dung het thi phai tra ok voi DUNG soLanDung");

const bsLuonSai: BoSinh<number> = { generate: () => 9 };
assertDeepEqual(kiemTraTinhChat(bsLuonSai, (n) => n === 5, 10), loi({ input: 9, viTri: 0 }), "tinh chat sai NGAY lan dau phai bao viTri 0 va DUNG input");

let dem = 0;
const bsDem: BoSinh<number> = { generate: () => dem++ };
assertDeepEqual(kiemTraTinhChat(bsDem, (n) => n < 3, 10), loi({ input: 3, viTri: 3 }), "phai bao DUNG lan va DUNG input gay that bai, khong phai lan dau");

function serialize(n: number): string { return `N:${n}`; }
function parse(s: string): number { return Number(s.slice(2)); }
const bsCoDinh: BoSinh<number> = { generate: () => 100 };
assertDeepEqual(kiemTraTinhChat(bsCoDinh, (n) => parse(serialize(n)) === n, 5), ok({ soLanDung: 5 }), "cap serialize/parse dung phai qua tinh chat di-ve");
```

:::hints
- kind: attention
  body: "Nhánh lỗi: gói input GÂY thất bại và viTri (lần thứ mấy) vào loi({...}). Nhánh cuối (chạy hết vòng lặp, không lần nào fail): gói soLan vào ok({...})."
- kind: strategy
  body: 'loi({ input, viTri: lan }) : ok({ soLanDung: soLan })'
- kind: one-line
  body: '___ (nhanh loi) = loi({ input, viTri: lan })\n___ (nhanh ok) = ok({ soLanDung: soLan })'
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
Cụm 5 hoàn tất: bốn mẫu tính chất, shrinking, kiemTraTinhChat trả
Result. Cụm chốt track: kiểm thử đột biến — chính kỷ luật dự án đã
dùng xuyên suốt track này.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt track này, "đột biến" (mutation) đã được nhắc tới nhiều lần
trong các đoạn code có bug ẩn. Đột biến LÀ GÌ, chính xác — và tại sao
nó lại là một CÁCH KIỂM TRA test, không phải kiểm tra code?
::::

::::checkpoint{mastery=0.8}
::::
