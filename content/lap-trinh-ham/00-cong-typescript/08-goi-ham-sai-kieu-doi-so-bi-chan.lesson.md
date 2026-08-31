---
id: lap-trinh-ham.cong-typescript.goi-ham-sai-kieu-doi-so-bi-chan
title: "Gọi hàm sai kiểu đối số bị chặn trước khi chạy"
summary: "gapDoi(\"ba\") — TSC từ chối (TS2345) NGAY LÚC BIÊN DỊCH, trước khi có cơ hội chạy x * 2. Lời hứa của tham số được kiểm ở phía GỌI, không chỉ phía định nghĩa."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.arg-type-check]
requires: [ts.return-type]
concepts: [ts.arg-type-check]
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
Câu hỏi cuối bài trước: phía GỌI hàm có bị kiểm theo đúng lời hứa
không? Có — và nó bị chặn ở đúng dòng gọi, trước khi thân hàm chạy dù
chỉ một bước.
::::

::::explain{#doc-goi-sai-kieu}
Bài 6 dạy tham số hứa một kiểu:

```typescript
function gapDoi(x: number): number {
  return x * 2;
}
```

`x: number` không chỉ là một ghi chú cho người đọc thân hàm. Nó còn là
một điều kiện áp lên MỌI chỗ hàm `gapDoi` được GỌI, trong toàn bộ
chương trình. Gọi `gapDoi("ba")` — đưa một `string` vào chỗ hứa nhận
`number` — bị TypeScript từ chối biên dịch, dù bản thân hàm `gapDoi`
hoàn toàn không có gì sai.

Đây khác hẳn Python. Python không có lời hứa kiểu, nên `gap_doi("ba")`
vẫn CHẠY được — và tuỳ vào thân hàm, kết quả có thể là nối chuỗi
(`"ba" * 2` cho ra `"baba"`, không lỗi gì cả) hoặc một `TypeError` chỉ
lộ ra LÚC DÒNG ĐÓ THỰC SỰ CHẠY. TypeScript không chờ tới lúc đó — nó
đọc kiểu tại dòng gọi, thấy sai, và từ chối sinh mã trước khi có dòng
nào được thực thi.
::::

::::example{#doc-vi-du-goi-sai-kieu}
```typescript
function gapDoi(x: number): number {
  return x * 2;
}

console.log(gapDoi("ba"));
```

```text title=readonly
(không in ra gì cả)

TS2345 (dòng 5, cột 20): Argument of type 'string' is not assignable
to parameter of type 'number'.
```

Gọi đúng kiểu, không có gì để chặn:

```typescript
function gapDoi(x: number): number {
  return x * 2;
}

console.log(gapDoi(5));
```

```text title=readonly
10
```

Đã chạy thật cả hai. Để ý mã lỗi: TS2345, khác với TS2322 bài 7 vừa
gặp (`gán` sai kiểu cho biến hoặc cho giá trị trả về). TS2345 dành
riêng cho ĐỐI SỐ sai kiểu lúc gọi hàm — hai mã khác nhau cho hai chỗ
khác nhau mà một lời hứa kiểu có thể bị phá.
::::

::::predict{#chuoi-goi-sai-kieu commitOnce}
Byte viết ba dòng gọi cùng một hàm, KHÔNG chạy thử:

```typescript
function gapDoi(x: number): number {
  return x * 2;
}

console.log(gapDoi(4));
console.log(gapDoi(10));
console.log(gapDoi("4"));
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng cuối — `gapDoi("4")` — `"4"` là một `string` (dù trông giống một
con số khi đọc bằng mắt), còn `gapDoi` hứa nhận `number`. TypeScript
không tự đổi một chuỗi số thành số lúc biên dịch — kiểu vẫn là
`string`: TS2345
:::

:::opt
`gapDoi(4)` — gọi hàm với một số VIẾT THẲNG (không qua biến trung
gian) là không hợp lệ, phải gán `4` vào một biến trước rồi mới gọi
::why
Gần đúng ở việc bạn nhớ tới bài 4, nơi biến được suy luận kiểu từ giá
trị khởi tạo — và nghi ngờ đối số cũng cần một bước trung gian tương
tự.

Chỗ lệch: đối số của một lời gọi hàm có thể là bất kỳ biểu thức nào
cho ra đúng kiểu — kể cả một giá trị viết thẳng như `4`. Không cần
biến trung gian. Đã thử thật: `gapDoi(4)` biên dịch sạch.
::
:::

:::opt
`gapDoi(10)` — hàm `gapDoi` đã được gọi ở dòng trước (`gapDoi(4)`),
không được phép gọi lại một hàm lần thứ hai trong cùng chương trình
::why
Gần đúng ở việc bạn để ý `gapDoi` xuất hiện nhiều lần liên tiếp.

Chỗ lệch: không có luật nào giới hạn số lần gọi một hàm — một hàm
được định nghĩa một lần, gọi được bao nhiêu lần tuỳ ý, miễn mỗi lần
gọi đúng lời hứa kiểu. Đã thử thật: `gapDoi(10)` biên dịch sạch.
::
:::

:::opt
Không dòng nào bị từ chối — `"4"` là một chuỗi CHỨA một con số, nên
TypeScript tự đổi nó thành `number` trước khi so kiểu
::why
Gần đúng ở trực giác rằng `"4"` "trông giống" một số nên máy sẽ hiểu
ý.

Chỗ lệch: TypeScript không tự chuyển đổi kiểu giữa `string` và
`number` lúc kiểm kiểu — nó chỉ so KIỂU KHAI BÁO, và kiểu của `"4"`
luôn là `string`, bất kể nội dung bên trong dấu nháy. Đã thử thật:
dòng `gapDoi("4")` bị từ chối, đúng mã `TS2345`.
::
:::
::::

::::code{#tinh-tien-ve}
Byte bán vé xem phim, mỗi vé 75.000đ. Hàm `tinhTienVe` đã hứa nhận
`number` — Byte có 3 vé, nhưng dòng gọi bên dưới đang đưa vào một chỗ
trống. Điền đúng để không bị TypeScript chặn.

```typescript title=starter
function tinhTienVe(soVe: number): number {
  return soVe * 75000;
}

console.log(tinhTienVe(___));
```

```typescript title=solution
function tinhTienVe(soVe: number): number {
  return soVe * 75000;
}

console.log(tinhTienVe(3));
```

```typescript title=test
if (tinhTienVe(3) !== 225000) throw new Error("tinhTienVe(3) phải là 225000 — đang là " + tinhTienVe(3));
if (tinhTienVe(2) !== 150000) throw new Error("tinhTienVe(2) phải là 150000 — đang là " + tinhTienVe(2));
```

:::hints
- kind: attention
  body: soVe hứa nhận number. Byte có 3 vé — điền một con số, không phải một chuỗi ("3" vẫn là string, không khớp lời hứa).
- kind: strategy
  body: 'Byte có 3 vé. Điền đúng con số 3 (không có dấu nháy) vào chỗ trống.'
- kind: one-line
  body: 'Chỗ trống là: 3'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "225000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lời hứa kiểu của tham số không chỉ canh gác BÊN TRONG hàm — nó canh
gác cả MỌI CHỖ hàm được gọi, dù chỗ đó cách xa dòng định nghĩa bao xa
đi nữa.
::::

::::reflect{#nghi-lai}
Tới giờ bạn thấy sai KIỂU đối số bị chặn lúc gọi hàm. Nhưng kiểu chỉ
là một nửa lời hứa của một tham số — nửa còn lại là chính SỰ TỒN TẠI
của nó.

Gọi hàm mà THIẾU hẳn một đối số, hay THỪA một đối số không hàm nào
cần — TypeScript có chặn được kiểu chuyện đó không, hay chỉ kiểm được
kiểu của những đối số đã có mặt? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
