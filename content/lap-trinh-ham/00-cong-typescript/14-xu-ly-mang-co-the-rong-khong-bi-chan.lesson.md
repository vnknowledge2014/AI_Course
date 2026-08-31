---
id: lap-trinh-ham.cong-typescript.xu-ly-mang-co-the-rong-khong-bi-chan
title: "Xử lý mảng có thể rỗng, không bị TS chặn"
summary: "Bài chốt cụm: viết một hàm nhận một mảng CÓ THỂ RỖNG, kiểm tra đúng cách trước khi đọc phần tử, để TSC không chặn — ghép đúng bài 11-13 thành một chương trình chạy được."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.handle-optional-safely]
requires: [ts.null-vs-undefined]
concepts: [ts.handle-optional-safely]
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
Ba bài, ba mảnh: đọc mảng có thể trống, kiểm tra trước khi dùng, chọn
đúng loại "không có gì". Bài chốt cụm này không dạy mảnh nào mới — chỉ
đòi bạn ghép cả ba vào MỘT hàm.
::::

::::explain{#ghep-ba-manh}
Nhắc lại ba mảnh của cụm bài này, theo đúng thứ tự chúng thường xuất
hiện trong một hàm thật:

1. Đọc một phần tử của mảng cho ra kiểu `T | undefined`, không phải
   `T` — vì TypeScript không biết mảng có bao nhiêu phần tử LÚC BIÊN
   DỊCH (bài 11).
2. Kiểm tra `!== undefined` trước khi dùng, và CHỈ dùng được trong
   đúng nhánh `if` đã kiểm tra — thu hẹp kiểu không "lan" ra ngoài
   (bài 12).
3. Khi hàm cần TRẢ VỀ một tín hiệu "không có gì" cho người GỌI nó,
   `null` là lựa chọn RÕ RÀNG hơn — nó nói "tôi đã kiểm tra, và không
   có gì cả", khác với `undefined` (chưa từng có) (bài 13).

Một hàm nhận mảng có thể rỗng, muốn AN TOÀN và RÕ RÀNG cùng lúc, cần cả
ba: đọc chỉ số ra `T | undefined` (mảnh 1), kiểm tra bằng `if` trước
khi dùng (mảnh 2), rồi CHỦ Ý trả `null` cho người gọi khi mảng thật sự
rỗng, thay vì để `undefined` tự nhiên rò ra ngoài hàm (mảnh 3). Ba
mảnh không đối lập nhau — chúng là ba bước LIÊN TIẾP của cùng một
hàm.
::::

::::example{#doc-phan-tu-dau-an-toan}
Một hàm lấy điểm đầu tiên của một mảng CÓ THỂ RỖNG, dùng đủ cả ba
mảnh:

```typescript title=readonly
function diemDauTien(mangDiem: number[]): number | null {
  const dau: number | undefined = mangDiem[0];
  if (dau !== undefined) {
    return dau;
  }
  return null;
}

console.log(diemDauTien([7, 8, 9]));
console.log(diemDauTien([]));
```

```text title=readonly
7
null
```

Đã chạy thật, hai lời gọi, hai kết quả khác nhau. Đọc từng bước: `dau`
mang kiểu `number | undefined` vì `mangDiem[0]` là một phép đọc chỉ số
(mảnh 1). `if (dau !== undefined)` thu hẹp `dau` xuống `number` — chỉ
TRONG nhánh đó, `return dau;` mới hợp lệ (mảnh 2). Khi mảng rỗng,
`mangDiem[0]` là `undefined`, nhánh `if` không chạy, hàm CHỦ Ý
`return null;` — một tín hiệu rõ ràng "không có phần tử nào", không
phải `undefined` tự nhiên rò ra (mảnh 3). Kiểu trả về của cả hàm,
`number | null`, phản ánh đúng thứ hàm THẬT SỰ có thể trả — không hơn,
không kém.
::::

::::predict{#kiem-nham-thu-khac commitOnce}
Byte viết một biến thể khác của `diemDauTien`, KHÔNG chạy thử. Lần
này, thay vì kiểm tra `dau`, Byte kiểm tra độ dài của mảng:

```typescript
function diemDauTien(mangDiem: number[]): number | null {
  const dau: number | undefined = mangDiem[0];
  if (mangDiem.length > 0) {
    return dau;
  }
  return null;
}
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng `return dau;` — điều kiện `if (mangDiem.length > 0)` kiểm tra một
BIỂU THỨC khác (`mangDiem.length`), không phải chính `dau`. TSC chỉ
thu hẹp kiểu của biến ĐANG ĐƯỢC KIỂM TRA trực tiếp trong điều kiện —
dù về mặt LOGIC hai điều đó tương đương, TSC không tự suy luận ra mối
liên hệ ấy, nên `dau` bên trong nhánh này vẫn giữ nguyên kiểu
`number | undefined`
:::

:::opt
Dòng `return null;` — hàm không bao giờ chạy tới đó khi `mangDiem` có
ít nhất một phần tử, nên nhánh này thừa và bị TSC từ chối
::why
Gần đúng ở việc bạn suy luận đúng: khi `mangDiem.length > 0`, nhánh
`return null;` (nằm ngoài `if`) đúng là không chạy tới với một mảng có
phần tử.

Chỗ lệch: TSC không loại bỏ nhánh code "trên lý thuyết không chạy tới"
— nó vẫn kiểm KIỂU cho mọi dòng code có mặt trong hàm, bất kể lúc chạy
có ghé qua hay không. `return null;` khớp đúng kiểu trả về đã hứa
(`number | null`), hoàn toàn hợp lệ, đã thử thật.
::
:::

:::opt
Dòng khai `const dau: number | undefined = mangDiem[0];` — vì
`mangDiem[0]` chỉ hợp lệ SAU khi đã có một câu kiểm tra độ dài mảng
đứng trước nó
::why
Gần đúng ở việc bạn cẩn thận với THỨ TỰ đọc mảng trước/sau kiểm tra —
đúng là một mối lo hợp lý.

Chỗ lệch: TypeScript không quan tâm việc kiểm tra độ dài có đứng
TRƯỚC hay SAU dòng đọc chỉ số — `mangDiem[0]` LUÔN cho kiểu
`number | undefined`, bất kể ngữ cảnh xung quanh, vì cờ
`noUncheckedIndexedAccess` áp dụng cho MỌI phép đọc chỉ số (bài 11).
Dòng khai này hợp lệ hoàn toàn, đã thử thật, không bị từ chối.
::
:::

:::opt
Không dòng nào bị từ chối — `if (mangDiem.length > 0)` đã đủ để đảm
bảo phần tử đầu tồn tại, nên `dau` chắc chắn là `number` ở đó
::why
Gần đúng ở việc LOGIC của bạn đúng theo nghĩa CON NGƯỜI — nếu mảng có
ít nhất một phần tử, phần tử đầu tiên chắc chắn tồn tại thật.

Chỗ lệch: TSC không lý luận theo kiểu đó. Nó chỉ thu hẹp kiểu của
CHÍNH biến xuất hiện trong điều kiện `if` — ở đây là `mangDiem.length`,
không phải `dau`. Hai biểu thức tương đương về Ý NGHĨA không tương
đương về KIỂU dưới mắt TSC. Dòng `return dau;` bị từ chối thật, mã
`TS2322` — "kiểm tra đúng biến" (bài 12) không phải một gợi ý, mà là
điều kiện bắt buộc để thu hẹp có tác dụng.
::
:::
::::

::::code{#ham-day-du-ba-manh}
Viết lại hàm lấy điểm đầu tiên — điền đúng giá trị trả về khi mảng có
phần tử, dùng đủ cả ba mảnh của cụm bài này.

```typescript title=starter
function diemDauTien(mangDiem: number[]): number | null {
  const dau: number | undefined = mangDiem[0];
  if (dau !== undefined) {
    return ___;
  }
  return null;
}

console.log(diemDauTien([7, 8, 9]));
console.log(diemDauTien([]));
```

```typescript title=solution
function diemDauTien(mangDiem: number[]): number | null {
  const dau: number | undefined = mangDiem[0];
  if (dau !== undefined) {
    return dau;
  }
  return null;
}

console.log(diemDauTien([7, 8, 9]));
console.log(diemDauTien([]));
```

```typescript title=test
if (diemDauTien([7, 8, 9]) !== 7) throw new Error("diemDauTien([7,8,9]) phải trả về 7 — đang trả về " + diemDauTien([7, 8, 9]));
if (diemDauTien([]) !== null) throw new Error("diemDauTien([]) phải trả về null khi mảng rỗng — đang trả về " + diemDauTien([]));
```

:::hints
- kind: attention
  body: Chỗ trống nằm bên trong nhánh if (dau !== undefined) — đúng nơi TSC đã thu hẹp dau xuống kiểu number. Trả về chính giá trị đã kiểm tra, không phải một con số viết tay.
- kind: strategy
  body: 'Bên trong nhánh if (dau !== undefined), dau đã được thu hẹp xuống number — trả về đúng dau, không cần chuyển đổi hay tính toán gì thêm.'
- kind: one-line
  body: 'Chỗ trống là: dau'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "7"
- tier: output
  match: contains
  expect: "null"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc chỉ số ra kiểu có thể-vắng-mặt, kiểm tra đúng biến trong đúng
nhánh, rồi CHỦ Ý trả `null` khi thật sự rỗng — ba mảnh, một hàm, không
chỗ nào bị TSC chặn.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả cụm bài này.

Tới giờ, mọi "không có gì" bạn xử lý đều là một trong HAI khả năng —
`number | undefined` hoặc `number | null` — và luôn có một khả năng là
"trống". Nhưng nếu một giá trị có thể là MỘT TRONG NHIỀU KIỂU THẬT SỰ
khác nhau — không phải "có số hay không có gì", mà "là một con số HAY
là một đoạn chữ", cả hai đều là giá trị THẬT, không cái nào là "trống"
— TypeScript viết lời hứa đó bằng cách nào? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
