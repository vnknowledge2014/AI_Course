---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.khi-map-khong-du-ham-tra-ve-chinh-option
title: "Khi `map` KHÔNG ĐỦ — hàm áp dụng CHÍNH NÓ trả về `Option`"
summary: "chia(a,b) trả Option<number>. mapOption(co(10), x => chia(x, 2)) ra Option<Option<number>> — MỘT Option LỒNG bên trong Option, không phải Option<number> phẳng như mong muốn."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 28
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.map-insufficient-nested]
requires: [alg.review-applicative]
concepts: [alg.map-insufficient-nested]
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
Cụm 4 chốt lại Applicative — ghép giá trị ĐỘC LẬP. Cụm 5 hỏi câu
khác: nếu một BƯỚC cần giá trị THẬT của bước TRƯỚC, `map` còn đủ
không?
::::

::::explain{#map-tao-ra-long-nhau}
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

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));

const ketQua = mapOption(co(10), (x) => chia(x, 2));
console.log(JSON.stringify(ketQua));
```

```text
{"kind":"co","giaTri":{"kind":"co","giaTri":5}}
```

`chia(a, b)` — một phép chia CÓ THỂ THẤT BẠI (chia cho `0`) — TỰ NÓ đã
trả về `Option<number>`, không phải `number` trần. Gọi `mapOption(co(10),
x => chia(x, 2))` — `mapOption` LUÔN "gói" kết quả của hàm áp dụng vào
MỘT lớp `co(...)` MỚI (đúng khuôn bài 14 đã viết). Nhưng `chia(10, 2)`
ĐÃ LÀ một `Option` (`co(5)`) rồi — `mapOption` gói NÓ vào một `co(...)`
KHÁC, tạo ra `co(co(5))` — một `Option<Option<number>>` LỒNG HAI LỚP,
không phải `Option<number>` PHẲNG như mong muốn.
::::

::::example{#long-hai-lop-kho-doc}
`Option` lồng hai lớp buộc phải kiểm `kind` HAI LẦN mới đọc được giá
trị thật:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function mapOption<T, U>(o: Option<T>, f: (x: T) => U): Option<U> {
  switch (o.kind) {
    case "co": return co(f(o.giaTri));
    case "khong": return khong();
  }
}

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));

const long: Option<Option<number>> = mapOption(co(10), (x) => chia(x, 2));

if (long.kind === "co") {
  const trong = long.giaTri;
  if (trong.kind === "co") {
    console.log("giá trị thật: " + trong.giaTri);
  } else {
    console.log("lớp trong không có gì");
  }
} else {
  console.log("lớp ngoài không có gì");
}
```

```text title=readonly
giá trị thật: 5
```

Đọc `5` (giá trị THẬT bên trong) đòi phải kiểm `long.kind === "co"`
RỒI kiểm `trong.kind === "co"` — HAI TẦNG `switch`/`if` lồng nhau,
càng nhiều bước "chia có thể lỗi" nối tiếp thì càng lồng SÂU hơn. Đây
không phải LỖI CÚ PHÁP — code này CHẠY ĐÚNG, biên dịch sạch — chỉ là
CỒNG KỀNH, không phải hình dạng ta MUỐN.
::::

::::predict{#doan-long-hai-lop commitOnce}
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

const chia = (a: number, b: number): Option<number> => (b === 0 ? khong() : co(a / b));

const ketQua = mapOption(co(10), (x) => chia(x, 0));
console.log(ketQua.kind);
```

Dòng cuối in ra gì?

:::opt{correct}
`co`
:::

:::opt
`khong` — vì `chia(10, 0)` thất bại (chia cho 0), nên KẾT QUẢ CUỐI
CÙNG của `mapOption` cũng phải là "khong"
::why
Gần đúng ở việc bạn xác định ĐÚNG `chia(10, 0)` THẤT BẠI (chia cho `0`
không hợp lệ) — quan sát đó đúng, `chia(10, 0)` THẬT SỰ ra `khong()`.

Chỗ lệch: `mapOption(co(10), f)` với `o = co(10)` (LỚP NGOÀI "có" giá
trị `10`) LUÔN đóng gói kết quả của `f` vào MỘT lớp `co(...)` MỚI —
BẤT KỂ `f(10)` (tức `chia(10, 0)`) trả về gì. Kết quả là
`co(khong())` — LỚP NGOÀI VẪN LÀ `"co"` (chứa `khong()` BÊN TRONG),
không phải `"khong"` ở LỚP NGOÀI. `ketQua.kind` là `"co"`.
::
:::

:::opt
Máy báo lỗi biên dịch — không thể gọi `mapOption` khi hàm áp dụng `f`
tự nó trả về một `Option`
::why
Gần đúng ở việc bạn cảnh giác có gì đó "không ổn" khi `f` trả về
`Option` — bài học ĐÚNG là có vấn đề THẬT (kết quả lồng), nhưng không
phải lỗi biên dịch.

Chỗ lệch: `mapOption<T, U>` là GENERIC — `U` có thể là BẤT KỲ kiểu
nào, kể cả CHÍNH `Option<number>`. Không có ràng buộc kiểu nào ngăn
`f` trả về `Option`. Biên dịch và chạy HOÀN TOÀN bình thường — vấn đề
CHỈ là kết quả LỒNG, khó đọc, không phải lỗi.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`map` "gói lại" kết quả của hàm áp dụng — hữu ích khi hàm đó trả về
giá trị TRẦN, nhưng tạo ra lồng nhau khi hàm đó TỰ NÓ đã trả về
`Option`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Có công cụ nào KHÔNG "gói lại" kết quả của `f`, mà TRẢ THẲNG kết quả
đó (đã LÀ một `Option` rồi) — tránh được lồng nhau?

Bài sau viết đúng công cụ đó.
::::

::::checkpoint{mastery=0.8}
::::
