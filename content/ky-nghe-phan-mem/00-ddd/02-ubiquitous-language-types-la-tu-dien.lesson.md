---
id: ky-nghe-phan-mem.ddd.ubiquitous-language-types-la-tu-dien
title: "Ubiquitous Language — Types là từ điển domain"
summary: "Đặt tên type/field bằng ĐÚNG từ domain expert dùng biến type definition thành một cuốn từ điển sống. Litmus test: Product Owner đọc type mà nói \"đúng, đây là cách nghiệp vụ hoạt động\" thì đạt. `type Hang = 0|1|2|3` (fail) vs `type HangKhachHang = \"thuong\"|\"bac\"|\"vang\"|\"kim_cuong\"` (pass)."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ddd.ubiquitous-language]
requires: [ddd.what-and-why]
concepts: [ddd.ubiquitous-language]
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
Bài trước: DDD muốn code NÓI ngôn ngữ nghiệp vụ. Hôm nay: một BÀI KIỂM
TRA cụ thể để biết một tên có ĐẠT hay không.
::::

::::explain{#litmus-test}
**Ubiquitous Language**: dev và domain expert dùng CHUNG một từ vựng —
type definition trở thành một cuốn TỪ ĐIỂN sống, không chỉ là chú thích
kiểu dữ liệu.

**Litmus test**: đưa type definition cho Product Owner (người hiểu
nghiệp vụ) đọc — nếu họ nói "đúng, đây LÀ cách nghiệp vụ hoạt động" thì
ĐẠT.

```typescript
// FAIL — số vô nghĩa, PO đọc không hiểu "1" là hạng gì
type HangCu = 0 | 1 | 2 | 3;

// PASS — PO đọc hiểu NGAY, đúng thuật ngữ họ dùng hàng ngày
type HangKhachHang = "thuong" | "bac" | "vang" | "kim_cuong";
```

Đây KHÔNG phải chỉ là "đặt tên đẹp hơn" — nó là NGUỒN THẬT của thông
tin: `HangKhachHang` chỉ có ĐÚNG bốn giá trị hợp lệ (TypeScript CHẶN
`"platinum"` nếu ai đó gõ nhầm), còn `HangCu` chấp nhận BẤT KỲ số nào
(kể cả `99`, một hạng KHÔNG hề tồn tại) — Ubiquitous Language không chỉ
dễ đọc hơn, nó CHẶT hơn về mặt kiểu.
::::

::::example{#tac-dong-thuc-te-cua-ten}
Tên KỸ THUẬT ẩn đi Ý NGHĨA — dẫn tới lỗi khó phát hiện. So sánh hai cách
viết cùng MỘT phép tính (tỷ lệ giảm giá theo hạng):

```typescript title=readonly
type HangCu = 0 | 1 | 2 | 3;
function tyLeGiamCu(h: HangCu): number {
  return h * 0.05;
}

type HangKhachHang = "thuong" | "bac" | "vang" | "kim_cuong";
function tyLeGiam(hang: HangKhachHang): number {
  switch (hang) {
    case "thuong": return 0;
    case "bac": return 0.05;
    case "vang": return 0.1;
    case "kim_cuong": return 0.15;
  }
}

console.log(tyLeGiamCu(2));
console.log(tyLeGiam("vang"));
```

```text title=readonly
0.1
0.1
```

CÙNG kết quả (`0.1`) — NHƯNG `tyLeGiamCu(2)` đòi người gọi PHẢI NHỚ "2
nghĩa là vàng" (kiến thức đó nằm NGOÀI code, dễ quên/gõ nhầm thành `3`);
`tyLeGiam("vang")` tự nó ĐÃ nói rõ đang tính cho hạng nào — không cần
tra cứu gì thêm. Litmus test không chỉ là thẩm mỹ — nó LOẠI BỎ một lớp
lỗi thật (nhầm số hạng).
::::

::::predict{#doan-hang-sai-chinh-ta commitOnce}
```typescript
type HangKhachHang = "thuong" | "bac" | "vang" | "kim_cuong";
function tyLeGiam(hang: HangKhachHang): number {
  switch (hang) {
    case "thuong": return 0;
    case "bac": return 0.05;
    case "vang": return 0.1;
    case "kim_cuong": return 0.15;
  }
}

const hangKhach: HangKhachHang = "vang";
console.log(tyLeGiam(hangKhach));
```

Dòng cuối in ra gì?

:::opt{correct}
`0.1`
:::

:::opt
Máy báo lỗi biên dịch — thiếu nhánh `default` trong `switch`, TypeScript
không chắc hàm LUÔN trả về `number`
::why
Gần đúng ở việc bạn để ý `switch` không có `default` — quan sát về CẤU
TRÚC đó đúng.

Chỗ lệch: `HangKhachHang` là union CHỈ CÓ ĐÚNG bốn giá trị — `switch`
với BỐN case (`thuong`/`bac`/`vang`/`kim_cuong`) BAO PHỦ HẾT mọi giá trị
có thể của `hang`, TypeScript TỰ nhận ra switch đã EXHAUSTIVE (đã học ở
T4.3) — không cần `default`, không có lỗi thiếu return nào.
::
:::

:::opt
`0` — vì `hangKhach` là một biến MỚI khai báo, TypeScript coi nó khác
với literal `"vang"` dùng trực tiếp trong switch
::why
Gần đúng ở việc bạn để ý CÓ một bước trung gian (`const hangKhach =
"vang"` rồi mới gọi `tyLeGiam(hangKhach)`) — quan sát về CÓ biến trung
gian đó đúng.

Chỗ lệch: biến `hangKhach: HangKhachHang = "vang"` GIỮ giá trị `"vang"`
Y HỆT — gán qua một biến không hề "đổi" giá trị bên trong. `tyLeGiam(hangKhach)`
chạy CHÍNH XÁC như `tyLeGiam("vang")`, khớp nhánh `case "vang"`, trả về
`0.1`.
::
:::
::::

::::code{#viet_ty_le_giam}
Tự viết `tyLeGiam(hang: HangKhachHang): number` — mỗi hạng một tỷ lệ
giảm giá RIÊNG.

```typescript title=starter
type HangKhachHang = "thuong" | "bac" | "vang" | "kim_cuong";

function tyLeGiam(hang: HangKhachHang): number {
  switch (hang) {
    case "thuong": return ___;
    case "bac": return ___;
    case "vang": return ___;
    case "kim_cuong": return ___;
  }
}

console.log(tyLeGiam("vang"));
```

```typescript title=solution
type HangKhachHang = "thuong" | "bac" | "vang" | "kim_cuong";

function tyLeGiam(hang: HangKhachHang): number {
  switch (hang) {
    case "thuong": return 0;
    case "bac": return 0.05;
    case "vang": return 0.1;
    case "kim_cuong": return 0.15;
  }
}

console.log(tyLeGiam("vang"));
```

```typescript title=test
if (tyLeGiam("thuong") !== 0) throw new Error("hạng thường không giảm giá, tỷ lệ phải là 0");
if (tyLeGiam("bac") !== 0.05) throw new Error("hạng bạc phải giảm 0.05 (5%)");
if (tyLeGiam("vang") !== 0.1) throw new Error("hạng vàng phải giảm 0.1 (10%)");
if (tyLeGiam("kim_cuong") !== 0.15) throw new Error("hạng kim cương phải giảm 0.15 (15%)");
```

:::hints
- kind: attention
  body: "Bốn nhánh, BỐN tỷ lệ KHÁC NHAU — không phải cùng một số cho tất cả. Hạng càng cao, tỷ lệ giảm càng lớn: 0, 0.05, 0.1, 0.15."
- kind: strategy
  body: 'case "thuong": return 0; case "bac": return 0.05; case "vang": return 0.1; case "kim_cuong": return 0.15;'
- kind: one-line
  body: "return 0;\nreturn 0.05;\nreturn 0.1;\nreturn 0.15;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "0.1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ubiquitous Language — type definition trở thành từ điển sống, không chỉ
chú thích kiểu dữ liệu. Litmus test: PO đọc mà hiểu ngay thì đạt.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

CÙNG một từ (ví dụ "Sản phẩm") có thể mang NGHĨA khác nhau ở các phần
KHÁC nhau của hệ thống lớn — làm sao giữ được sự nhất quán khi đó?
::::

::::checkpoint{mastery=0.8}
::::
