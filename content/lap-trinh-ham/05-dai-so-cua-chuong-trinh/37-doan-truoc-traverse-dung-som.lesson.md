---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.doan-truoc-traverse-dung-som
title: "Dự đoán: `traverse` có XỬ LÝ HẾT mảng trước khi báo `\"khong\"` không?"
summary: "traverse dừng NGAY khi f trả về khong() — các phần tử SAU phần tử lỗi KHÔNG BAO GIỜ được f gọi tới, dù chúng nằm giữa mảng."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 37
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [alg.predict-traverse-shortcircuit]
requires: [alg.traverse-handwritten]
concepts: [alg.predict-traverse-shortcircuit]
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
Bài trước ĐO short-circuit bằng một BỘ ĐẾM. Lần này: LỘ RÕ nó bằng
`console.log` — nhìn TRỰC TIẾP `traverse` dừng ở đâu.
::::

::::explain{#f-co-log}
Bọc `chuyenSo` bằng MỘT hàm CÓ `console.log` — mỗi lần `f` được gọi
trên một phần tử, in RÕ phần tử đó ra:

```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}

const chuyenSoCoLog = (vb: string): Option<number> => {
  console.log("kiểm tra: " + vb);
  return chuyenSo(vb);
};
```

Mảng đầu vào có MỘT phần tử lỗi Ở GIỮA — không phải đầu, không phải
cuối: `["10", "20", "abc", "30", "40"]` (phần tử thứ BA, `"abc"`, là
phần tử duy nhất không parse được).
::::

::::predict{#doan-so-dong-log commitOnce}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
function co<T>(giaTri: T): Option<T> { return { kind: "co", giaTri }; }
function khong<T>(): Option<T> { return { kind: "khong" }; }
function traverse<T, U>(ds: T[], f: (x: T) => Option<U>): Option<U[]> {
  const kq: U[] = [];
  for (const x of ds) {
    const o = f(x);
    if (o.kind === "khong") return khong();
    kq.push(o.giaTri);
  }
  return co(kq);
}
function chuyenSo(vb: string): Option<number> {
  const n = Number(vb);
  return Number.isNaN(n) ? khong() : co(n);
}
const chuyenSoCoLog = (vb: string): Option<number> => {
  console.log("kiểm tra: " + vb);
  return chuyenSo(vb);
};

traverse(["10", "20", "abc", "30", "40"], chuyenSoCoLog);
```

Có bao nhiêu dòng `"kiểm tra: ..."` được in ra, và dòng CUỐI CÙNG in
giá trị nào?

:::opt{correct}
Ba dòng — `"10"`, `"20"`, `"abc"` (dừng ngay tại phần tử lỗi)
:::

:::opt
Năm dòng — `"10"`, `"20"`, `"abc"`, `"30"`, `"40"` (mỗi phần tử của
mảng đều được kiểm ĐÚNG MỘT LẦN trước khi quyết định kết quả cuối)
::why
Gần đúng ở việc bạn nghĩ tới CƠ CHẾ "duyệt hết mảng rồi mới quyết
định" — một mô hình HỢP LÝ cho những hàm KHÔNG có fail-fast (ví dụ
`.map`, bài 35 — LUÔN áp `f` lên MỌI phần tử, không dừng sớm).

Chỗ lệch: `traverse` (bài 36) CÓ fail-fast — thân vòng lặp kiểm
`if (o.kind === "khong") return khong();` NGAY SAU MỖI lần gọi `f`.
Khi `f("abc")` (lần gọi THỨ BA) trả về `khong()`, `traverse` `return`
NGAY LẬP TỨC — vòng lặp `for` KHÔNG BAO GIỜ chạy tới `"30"` hay
`"40"`, nên `console.log` bên trong `f` không hề chạy cho hai phần tử
đó.
::
:::

:::opt
Ba dòng, nhưng dòng CUỐI CÙNG là `"kiểm tra: 30"` — vì `traverse` bỏ
qua phần tử LỖI rồi tiếp tục kiểm phần tử KẾ TIẾP để xem có sửa được
không
::why
Gần đúng ở việc bạn đếm ĐÚNG có BA lần gọi trước khi dừng — số lượng
đó đúng.

Chỗ lệch: `traverse` KHÔNG "bỏ qua rồi tiếp tục" — nó `return khong()`
NGAY khi gặp lỗi ĐẦU TIÊN, thoát khỏi CẢ hàm ngay lập tức (không chỉ
bỏ qua MỘT phần tử rồi lặp tiếp). Ba dòng log đúng là `"10"`, `"20"`,
`"abc"` — dòng CUỐI CÙNG là `"kiểm tra: abc"`, không phải `"kiểm tra:
30"` (phần tử `"30"` không hề được nhắc tới, vì vòng lặp đã dừng
TRƯỚC KHI tới lượt nó).
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`traverse` dừng NGAY tại phần tử lỗi đầu tiên nó gặp — các phần tử
SAU, dù ở đâu trong mảng, không hề được kiểm tới.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`traverse` viết bằng vòng lặp `for`. Bạn đã học `reduce` "đóng gói"
vòng lặp (T4.2) — viết `traverse` bằng `reduce` có cho CÙNG hành vi
short-circuit này không?
::::

::::checkpoint{mastery=0.8}
::::
