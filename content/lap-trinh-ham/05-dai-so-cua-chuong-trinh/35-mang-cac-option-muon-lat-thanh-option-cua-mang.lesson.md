---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.mang-cac-option-muon-lat-thanh-option-cua-mang
title: "`Array<Option<T>>` — muốn LẬT thành `Option<Array<T>>`"
summary: "[\"1\",\"2\",\"3\"].map(chuyenSo) ra Option<number>[] — MỖI phần tử có Option RIÊNG. Thường muốn NGƯỢC LẠI: một Option<number[]> DUY NHẤT — có TOÀN BỘ mảng nếu mọi phần tử chuyển được, không có gì nếu MỘT phần tử thất bại."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 35
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [alg.traverse-motivation]
requires: [alg.review-monad]
concepts: [alg.traverse-motivation]
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
Có `chuyenSo: (s: string) => Option<number>`. Áp nó lên MỘT MẢNG chuỗi
bằng `.map` — kết quả trông thế nào?
::::

::::explain{#map-ra-mang-cac-option}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }

function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

const ketQua = ["1", "2", "3"].map(chuyenSo);
console.log(JSON.stringify(ketQua));
```

```text
[{"kind":"co","giaTri":1},{"kind":"co","giaTri":2},{"kind":"co","giaTri":3}]
```

`.map(chuyenSo)` (Array.map — CÙNG `.map` bài 16 đã dạy, KHÔNG phải
`mapOption`) áp `chuyenSo` lên TỪNG phần tử, ra kiểu `Option<number>[]`
— một MẢNG, MỖI Ô trong mảng là MỘT `Option` RIÊNG, độc lập với các ô
khác. Nếu MỘT chuỗi không parse được, CHỈ ô đó thành `"khong"`, các ô
khác VẪN giữ kết quả của chúng — mảng VẪN đủ độ dài, chỉ MỘT ô khác:

```typescript
const conLoi = ["1", "abc", "3"].map(chuyenSo);
console.log(JSON.stringify(conLoi));
```

```text
[{"kind":"co","giaTri":1},{"kind":"khong"},{"kind":"co","giaTri":3}]
```

Nhưng THƯỜNG câu hỏi thật sự KHÁC: "mảng CHUỖI NÀY có chuyển được
THÀNH MỘT mảng số HOÀN CHỈNH không?" — câu trả lời MUỐN là MỘT
`Option<number[]>` DUY NHẤT: `co([1, 2, 3])` nếu MỌI phần tử chuyển
được, `khong()` nếu DÙ CHỈ MỘT phần tử thất bại (không quan tâm mảng
"gần đúng" chứa lẫn lộn `"co"`/`"khong"` như trên). `.map` KHÔNG cho
câu trả lời đó trực tiếp — nó dừng ở `Option<number>[]`, một mảng CÁC
`Option`, không phải MỘT `Option` của mảng.
::::

::::example{#hinh-dung-hai-huong-lat}
Đặt cạnh nhau để thấy rõ HAI HÌNH DẠNG khác nhau của cùng dữ liệu:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

// hình dạng CÓ SẴN qua .map: Array<Option<number>>
const mangCacOption: Option<number>[] = ["1", "2", "3"].map(chuyenSo);

// hình dạng MUỐN CÓ: Option<Array<number>> — CHƯA có hàm nào tạo ra được nó
const giaSuCoTraverse = "một Option<number[]> DUY NHẤT — cần một công cụ MỚI";

console.log(mangCacOption.length);
console.log(giaSuCoTraverse);
```

```text title=readonly
3
một Option<number[]> DUY NHẤT — cần một công cụ MỚI
```

`Array<Option<T>>` (mảng CÁC Option, ĐỘ DÀI luôn khớp mảng gốc, mỗi ô
tự chứa trạng thái riêng) và `Option<Array<T>>` (MỘT Option DUY NHẤT
bọc quanh cả mảng, ĐỘ DÀI của mảng bên trong chỉ có Ý NGHĨA khi Option
đó là `"co"`) là HAI HÌNH DẠNG khác nhau chứa CÙNG loại thông tin, chỉ
LẬT vị trí "trong" và "ngoài". `.map` cho hình dạng THỨ NHẤT — cần một
công cụ KHÁC để LẬT sang hình dạng THỨ HAI.
::::

::::predict{#doan-array-map-option-do-dai commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

const ketQua = ["1", "xyz", "3", "abc", "5"].map(chuyenSo);
console.log(ketQua.length);
console.log(ketQua.filter((o) => o.kind === "khong").length);
```

Hai dòng cuối in ra gì?

:::opt{correct}
`5` rồi `2`
:::

:::opt
`3` rồi `2` — vì `.map` chỉ giữ lại các phần tử chuyển được, bỏ qua
những phần tử gây lỗi, giống `.filter`
::why
Gần đúng ở việc bạn đếm ĐÚNG có bao nhiêu phần tử KHÔNG chuyển được
(`"xyz"`, `"abc"` — hai phần tử) — đếm đó đúng.

Chỗ lệch: `.map` (Array.map, bài 16) KHÔNG BAO GIỜ bỏ bớt phần tử —
nó LUÔN trả về một mảng CÙNG ĐỘ DÀI với mảng gốc, áp hàm lên TỪNG
phần tử một cách ĐỘC LẬP. Mảng gốc có 5 phần tử, `ketQua` CŨNG có
đúng 5 phần tử — 3 trong số đó là `co(...)`, 2 là `khong()`, nhưng
CẢ NĂM đều tồn tại trong mảng. `.map` không phải `.filter`.
::
:::

:::opt
`5` rồi `0` — vì `.map` chỉ THAY ĐỔI kiểu của từng phần tử, không thể
tự phát hiện lỗi bên trong hàm `chuyenSo` được áp dụng
::why
Gần đúng ở việc bạn tính ĐÚNG độ dài (`5`, mảng gốc có 5 phần tử,
`.map` giữ nguyên độ dài) — phần đó đúng.

Chỗ lệch: `chuyenSo` LÀ hàm áp dụng — nó TỰ trả về `Option<number>`
(`khong()` khi `Number.isNaN`), và `.map` chỉ đơn giản GỌI `chuyenSo`
trên TỪNG phần tử rồi GOM kết quả vào một mảng mới. `"xyz"` và
`"abc"` đều khiến `chuyenSo` trả về `khong()` — hai phần tử ĐÓ trong
`ketQua` LÀ `khong()` thật sự, không phải `0` con lại "im lặng".
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`.map` cho `Array<Option<T>>` — đủ độ dài, mỗi ô tự chứa trạng thái
riêng. Muốn `Option<Array<T>>` — một Option DUY NHẤT bọc cả mảng —
cần một công cụ mới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Công cụ đó tên là gì, và viết nó thế nào?
::::

::::checkpoint{mastery=0.8}
::::
