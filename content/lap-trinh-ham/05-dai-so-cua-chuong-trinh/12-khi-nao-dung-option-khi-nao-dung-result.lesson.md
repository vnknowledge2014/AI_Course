---
id: lap-trinh-ham.dai-so-cua-chuong-trinh.khi-nao-dung-option-khi-nao-dung-result
title: "Khi nào dùng `Option`, khi nào dùng `Result`"
summary: "Tìm một phần tử trong mảng theo điều kiện → Option (không cần LÝ DO khi không tìm thấy). Chuyển chuỗi thành số → Result (NGƯỜI DÙNG cần biết TẠI SAO thất bại). Phụ thuộc: lỗi có cần GIẢI THÍCH được không."
locale: vi
track: lap-trinh-ham
module: dai-so-cua-chuong-trinh
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [alg.option-vs-result]
requires: [alg.result-type]
concepts: [alg.option-vs-result]
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
Bạn có cả `Option<T>` LẪN `Result<T, E>` — hai kiểu, cùng khuôn, khác
nhau ĐÚNG một chỗ. Khi nào dùng cái nào?
::::

::::explain{#tieu-chi-chon}
```typescript
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };

function co<T>(giaTri: T): Option<T> {
  return { kind: "co", giaTri };
}
function khong<T>(): Option<T> {
  return { kind: "khong" };
}

function timPhanTuAm(ds: number[]): Option<number> {
  const x = ds.find((n) => n < 0);
  return x === undefined ? khong() : co(x);
}

console.log(timPhanTuAm([3, 5, 8]));
console.log(timPhanTuAm([3, -5, 8]));
```

```text
{"kind":"khong"}
{"kind":"co","giaTri":-5}
```

`timPhanTuAm([3, 5, 8])` KHÔNG tìm thấy số âm — dùng `Option`, không
`Result`, vì KHÔNG có "lý do lỗi" nào để nói. Mảng `[3, 5, 8]` đơn giản
KHÔNG chứa số âm — đó KHÔNG PHẢI một sự cố, không có gì "sai" cần giải
thích. `khong()` diễn đạt đúng ý: "không có, và điều đó bình thường".

Tiêu chí chọn: **`Option` khi việc "không có" là một kết quả BÌNH
THƯỜNG, không cần giải thích. `Result` khi việc thất bại là một sự
kiện cần NGƯỜI DÙNG (hoặc lập trình viên đọc log) biết TẠI SAO.**
::::

::::example{#doi-chieu-hai-tinh-huong}
Hai tình huống TRÔNG giống nhau (cả hai đều "có thể không có kết quả")
nhưng chọn kiểu KHÁC nhau:

```typescript title=readonly
type Option<T> = { kind: "co"; giaTri: T } | { kind: "khong" };
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

function timDonHang(ds: string[], ma: string): Option<string> {
  const found = ds.find((d) => d === ma);
  return found === undefined ? { kind: "khong" } : { kind: "co", giaTri: found };
}

function chuyenSo(vb: string): Result<number, string> {
  const n = Number(vb);
  return Number.isNaN(n) ? { kind: "loi", loi: "\"" + vb + "\" không phải số" } : { kind: "ok", giaTri: n };
}

console.log(timDonHang(["DH-1", "DH-2"], "DH-9"));
console.log(chuyenSo("chin"));
```

```text title=readonly
{"kind":"khong"}
{"kind":"loi","loi":"\"chin\" không phải số"}
```

`timDonHang` KHÔNG tìm thấy `"DH-9"` — đây là kết quả BÌNH THƯỜNG của
việc TÌM KIẾM (mã đơn hàng đó đơn giản không có trong danh sách), dùng
`Option`. `chuyenSo("chin")` thất bại vì CHUỖI ĐẦU VÀO không đúng định
dạng — đây là một VẤN ĐỀ CẦN GIẢI THÍCH cho người gọi hàm (họ cần biết
CHUỖI NÀO gây lỗi để sửa), dùng `Result`.
::::

::::predict{#doan-chon-option-hay-result commitOnce}
Một hàm `layTuoiNguoiDung(id: number)` — tra một bảng dữ liệu người
dùng theo `id`, trả về TUỔI của người đó nếu `id` tồn tại.

Nếu `id` KHÔNG tồn tại trong bảng dữ liệu — đây là tình huống BÌNH
THƯỜNG (ai đó gõ nhầm `id`, hoặc người dùng đó đã bị xoá) hay là một
LỖI CẦN GIẢI THÍCH?

:::opt{correct}
Tình huống thường gặp — nên dùng `Option<number>`, không cần `Result`
:::

:::opt
Luôn luôn là `Result<number, string>` — MỌI hàm CÓ THỂ "không trả về
gì" đều nên dùng `Result`, không có ngoại lệ
::why
Gần đúng ở việc bạn nghĩ tới việc chuẩn hoá MỘT quy tắc áp dụng cho
mọi hàm — một mong muốn hợp lý để tránh phải QUYẾT ĐỊNH mỗi lần.

Chỗ lệch: không có quy tắc "luôn dùng Result" — bài học CHÍNH của bài
này là PHỤ THUỘC NGỮ CẢNH. `id` không tồn tại trong bảng dữ liệu THƯỜNG
là chuyện BÌNH THƯỜNG (không phải lỗi hệ thống, không phải input sai
định dạng) — dùng `Result` ở đây tạo ra MỘT trường `loi` không có nội
dung Ý NGHĨA gì để viết ("không tìm thấy" không phải một "LÝ DO" theo
nghĩa cần giải thích, nó CHÍNH LÀ kết quả).
::
:::

:::opt
Phụ thuộc vào việc `id` là `number` hay `string` — kiểu dữ liệu của
tham số quyết định dùng `Option` hay `Result`
::why
Gần đúng ở việc bạn cân nhắc tới THUỘC TÍNH của tham số đầu vào — một
hướng suy nghĩ hợp lý khi tìm quy tắc chọn.

Chỗ lệch: KIỂU DỮ LIỆU của tham số (`number`, `string`, hay bất kỳ gì)
KHÔNG liên quan gì tới việc chọn `Option` hay `Result` — quyết định chỉ
dựa trên Ý NGHĨA NGHIỆP VỤ của việc "không có kết quả" (bình thường
hay cần giải thích), không phụ thuộc kiểu dữ liệu đầu vào là gì.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không có công thức máy móc — chỉ có một câu hỏi: "không có kết quả" ở
đây có cần một LỜI GIẢI THÍCH hay không?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn giờ có đủ hai công cụ — `Option` và `Result`, cùng biết CHỌN đúng
lúc nào. Bài sau chốt cụm: tự viết CẢ HAI trong một chương trình nhỏ.
::::

::::checkpoint{mastery=0.8}
::::
