---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.luat-functor-mot-map-id-khong-doi-gi
title: "Luật Functor 1: `map(id)` không đổi gì cả"
summary: "mapOption(co(5), x => x) phải ra ĐÚNG co(5). Hàm 'giữ nguyên' (id) áp dụng qua map thì KHÔNG được đổi gì — nếu một map tự chế phá luật này, nó KHÔNG PHẢI một Functor hợp lệ, dù cú pháp trông giống."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.functor-law-identity]
requires: [alg.functor-definition]
concepts: [alg.functor-law-identity]
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
Có `map` đúng CHỮ KÝ chưa chắc đã ĐÁNG TIN. Hôm nay: luật đầu tiên để
kiểm một `map` có hoạt động ĐÚNG hay không.
::::

::::explain{#luat-identity}
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

const id = (x: number) => x;
console.log(mapOption(co(5), id));
```

```text
{"kind":"co","giaTri":5}
```

`id` (viết tắt "identity") là hàm "giữ nguyên": nhận `x`, trả LẠI
ĐÚNG `x`, không đổi gì. **Luật Functor thứ nhất**: `map(hop, id)` PHẢI
ra ĐÚNG `hop` — áp dụng hàm "không làm gì" thì kết quả CŨNG "không đổi
gì". `mapOption(co(5), id)` ra ĐÚNG `co(5)` — luật này ĐÚNG với
`mapOption` (đã đo thật).

Đây KHÔNG phải một điều TỰ ĐỘNG ĐÚNG với mọi hàm gọi là `map` — nó là
một RÀNG BUỘC mà một `map` VIẾT SAI có thể VI PHẠM, dù CHỮ KÝ (kiểu
tham số, kiểu trả về) trông giống hệt một `map` đúng.
::::

::::example{#map-sai-vi-pham-luat}
Một hàm TRÔNG giống `mapOption` — cùng chữ ký, biên dịch được — nhưng
VI PHẠM luật identity:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOptionSai(o: Option<number>, f: (x: number) => number): Option<number> {
  switch (o.kind) {
    case "co":
      return co(f(o.giaTri) + 1);
    case "khong":
      return khong();
  }
}

const id = (x: number) => x;
console.log(mapOptionSai(co(5), id));
```

```text title=readonly
{"kind":"co","giaTri":6}
```

`mapOptionSai` biên dịch SẠCH — chữ ký `(Option<number>, (x:number)
=> number) => Option<number>` HOÀN TOÀN hợp lệ, TRÔNG như một `map`
bình thường. Nhưng `co(f(o.giaTri) + 1)` ÂM THẦM cộng thêm `1` VÀO
KẾT QUẢ, bất kể `f` là hàm gì. Gọi với `id` (hàm KHÔNG đổi gì) — kết
quả VẪN bị đổi (`5` → `6`). VI PHẠM luật identity — `mapOptionSai`
KHÔNG PHẢI một Functor `map` hợp lệ, dù trông giống hệt một cái đúng.
::::

::::predict{#doan-map-sai-vi-pham commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function mapOptionSai2(o: Option<number>, f: (x: number) => number): Option<number> {
  switch (o.kind) {
    case "co":
      return co(f(o.giaTri) * 2);
    case "khong":
      return khong();
  }
}

const id = (x: number) => x;
console.log(mapOptionSai2(co(7), id));
```

Đoạn mã này có VI PHẠM luật identity của Functor không, và in ra gì?

:::opt{correct}
CÓ vi phạm — in ra `{"kind":"co","giaTri":14}`, không phải `co(7)`
:::

:::opt
KHÔNG vi phạm — `f(o.giaTri) * 2` vẫn ÁP DỤNG `f` lên `o.giaTri`, chỉ
thêm một BƯỚC TÍNH TOÁN nữa, không phải bỏ qua `f`
::why
Gần đúng ở việc bạn để ý `f` VẪN được GỌI (`f(o.giaTri)` xuất hiện
trong biểu thức) — quan sát đó đúng, `f` không bị bỏ qua hoàn toàn.

Chỗ lệch: luật identity không chỉ đòi "có gọi `f`" — nó đòi kết quả
CUỐI CÙNG, khi `f = id`, phải ra ĐÚNG giá trị GỐC, không đổi gì thêm.
`f(o.giaTri) * 2` với `f = id`: `id(7) * 2 = 7 * 2 = 14` — KHÁC `7`
(giá trị gốc). Phép nhân `* 2` THÊM VÀO sau `f` LÀ một sự "đổi gì đó",
vi phạm luật, dù `f` có được gọi.
::
:::

:::opt
Máy báo lỗi biên dịch — TypeScript TỰ PHÁT HIỆN vi phạm luật Functor
và từ chối biên dịch những hàm `map` sai
::why
Gần đúng ở việc bạn hy vọng có một CÔNG CỤ TỰ ĐỘNG bắt được lỗi này —
một mong muốn hợp lý (giống `tier: static` cho Python đã học ở các
track trước).

Chỗ lệch: TypeScript KHÔNG kiểm tra được luật Functor — luật này nói
về HÀNH VI (kết quả khi chạy với `id`), không phải KIỂU (chữ ký hàm).
`mapOptionSai2` có chữ ký HOÀN TOÀN hợp lệ, biên dịch sạch — vi phạm
luật CHỈ phát hiện được bằng cách CHẠY THỬ với `id` và SO SÁNH kết
quả, đúng cách bài này đang làm.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map` đúng CHỮ KÝ chưa đủ — phải đúng LUẬT. Chạy thử với `id`, so
sánh kết quả, là cách kiểm tra thực tế nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Luật identity kiểm `map` với MỘT hàm (`id`). Có luật thứ hai không —
kiểm khi GHÉP NHIỀU hàm lại?

Bài sau trả lời, nối thẳng T4.2's `compose`.
::::

::::checkpoint{mastery=0.8}
::::
