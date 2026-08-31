---
id: lap-trinh-ham.cong-typescript.du-doan-dong-nao-bi-chan-truoc-khi-chay
title: "Dự đoán dòng nào bị chặn TRƯỚC KHI CHẠY"
summary: "Bài chốt cụm: cho vài dòng khai báo TypeScript — có dòng đúng, có dòng phá lời hứa kiểu — dự đoán dòng nào TSC từ chối và VÌ SAO, mà KHÔNG bấm nút Chạy trước. Chỉ đọc kiểu."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.predict-compile-error]
requires: [ts.type-inference]
concepts: [ts.predict-compile-error]
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
Bốn bài, ba mảnh ghép: lời hứa, chặn trước khi chạy, suy luận vẫn giữ
lời hứa. Bài chốt cụm này không dạy mảnh nào mới — chỉ đòi bạn ĐỌC cả
ba, không bấm Chạy.
::::

::::explain{#doc-truoc-chay-sau}
Nhắc lại ba mảnh của cụm bài này:

1. Dấu hai chấm sau tên là một LỜI HỨA về kiểu (bài 2).
2. Lời hứa bị KIỂM TRƯỚC KHI CHẠY — sai một dòng, cả chương trình
   không được phép chạy dòng nào, kể cả dòng đứng trước (bài 3).
3. Lời hứa vẫn có hiệu lực dù không viết tường minh — TypeScript tự
   SUY LUẬN từ giá trị khởi tạo (bài 4).

Ghép cả ba lại, bạn có một khả năng mới: nhìn một khối khai báo mình
CHƯA TỪNG chạy, và biết trước dòng nào sẽ bị từ chối — không đoán mò,
đọc ra được. Việc cần làm với mỗi dòng: hỏi "biến này đã hứa kiểu gì —
tường minh hay suy luận — và giá trị mới có khớp không?"

Cái bẫy thường gặp nhất: một biến được GÁN từ MỘT BIẾN KHÁC (không
phải từ một giá trị viết thẳng), như `let c = b;`. Kiểu của `c` vẫn
suy luận được bình thường — TypeScript nhìn vào KIỂU của `b`, không
quan tâm `b` là một cái tên hay một giá trị viết sẵn.
::::

::::example{#doc-nam-dong}
Năm dòng, đọc từng dòng một, KHÔNG chạy thử trước:

```typescript
let ten: string = "Byte";
let tuoi = 5;
let diem = 9.5;
diem = 10;
tuoi = "nam";
```

Dòng 1: `ten` hứa tường minh `string`, nhận `"Byte"` — khớp. Dòng 2:
`tuoi` không khai kiểu, TypeScript suy luận `number` từ `5`. Dòng 3:
`diem` suy luận `number` từ `9.5`. Dòng 4: `diem = 10` — `10` là
`number`, khớp kiểu đã suy luận ở dòng 3 — vẫn ổn. Dòng 5: `tuoi =
"nam"` — `tuoi` đã suy luận là `number` từ dòng 2, còn `"nam"` là một
`string` — PHÁ lời hứa.

```text title=readonly
(không in ra gì cả)

TS2322 (dòng 5, cột 1): Type 'string' is not assignable to type 'number'.
```

Đã chạy thật: đúng dòng 5 bị từ chối, không dòng nào khác — và vì cả
chương trình bị từ chối cùng lúc (bài 3), không có gì được in ra, kể
cả bốn dòng đầu hoàn toàn đúng.
::::

::::predict{#chuoi-suy-luan-qua-bien commitOnce}
Byte viết năm dòng, KHÔNG chạy thử:

```typescript
let a: number = 10;
let b = "meo";
let c = b;
c = "cho";
a = c;
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng 5 (`a = c;`) — `c` được suy luận là `string` (vì `c = b`, và `b`
suy luận là `string`), còn `a` đã hứa tường minh là `number`. Gán một
`string` cho một biến `number` phá lời hứa, dù `c` chưa từng được viết
`: string` tường minh
:::

:::opt
Dòng 3 (`let c = b;`) — gán trực tiếp một biến sang một biến khác là
không hợp lệ, chỉ được phép gán từ một giá trị viết trực tiếp (như
`"meo"`), không phải từ một biến khác
::why
Gần đúng ở việc bạn cảm thấy "khai báo từ một biến khác" có gì đó khác
biệt so với khai từ một giá trị viết sẵn.

Chỗ lệch: TypeScript suy luận kiểu HỆT NHAU dù giá trị khởi tạo là một
giá trị viết thẳng (`"meo"`) hay một biến khác (`b`) — nó chỉ cần biết
KIỂU của vế phải. `let c = b;` suy ra `c` là `string`, hợp lệ hoàn
toàn — đã thử thật, không dòng nào bị từ chối ở đây.
::
:::

:::opt
Dòng 4 (`c = "cho";`) — `c` đã được gán một lần ở dòng 3 rồi, không
được gán lại lần thứ hai
::why
Gần đúng ở việc bạn nhớ đúng luật của `const` — biến khai bằng `const`
chỉ gán được đúng một lần, ngay lúc khai báo (bài 1).

Chỗ lệch: `c` được khai bằng `let`, không phải `const`. `let` cho phép
gán lại nhiều lần, miễn giá trị mới ĐÚNG KIỂU đã suy luận. `"cho"` là
`string`, khớp đúng kiểu `string` đã suy ra cho `c` ở dòng 3 — dòng
này hợp lệ, đã thử thật.
::
:::

:::opt
Không dòng nào bị từ chối — suy luận kiểu chỉ có tác dụng CẢNH BÁO,
không có dòng nào thực sự CHẶN được việc biên dịch
::why
Gần đúng ở cảm giác thận trọng — nhưng đây chính là điều bài 3 đã
chứng minh ngược lại.

Chỗ lệch: khi lỗi kiểu xảy ra, TypeScript từ chối SINH RA mã
JavaScript, không có ngoại lệ nào dành riêng cho kiểu suy luận. Dòng 5
của đoạn này bị từ chối thật — đã kiểm bằng trình biên dịch thật, đúng
mã lỗi `TS2322`.
::
:::
::::

::::code{#sua-mot-cho-sai}
Bảng điểm một môn học. Một dòng đang chờ giá trị — đọc kiểu của
`xep_loai` trước khi điền, để cả năm dòng đều giữ đúng lời hứa.

```typescript title=starter
let ten_mon_hoc: string = "Toán";
let diem_so = 8;
let xep_loai = "Khá";

diem_so = 9;
xep_loai = ___;                      // Byte đổi xếp loại thành "Giỏi"

console.log(ten_mon_hoc, diem_so, xep_loai);
```

```typescript title=solution
let ten_mon_hoc: string = "Toán";
let diem_so = 8;
let xep_loai = "Khá";

diem_so = 9;
xep_loai = "Giỏi";

console.log(ten_mon_hoc, diem_so, xep_loai);
```

```typescript title=test
if (ten_mon_hoc !== "Toán") throw new Error("ten_mon_hoc không được đổi — phải vẫn là \"Toán\"");
if (diem_so !== 9) throw new Error("diem_so phải là 9 — đang là " + diem_so);
if (xep_loai !== "Giỏi") throw new Error("xep_loai phải là \"Giỏi\" — đang là " + xep_loai);
```

:::hints
- kind: attention
  body: xep_loai chưa khai kiểu tường minh — nó suy luận từ "Khá" ở dòng khai báo, nên vẫn là kiểu string. Chỗ trống phải là một chuỗi.
- kind: strategy
  body: 'xep_loai suy luận là string (từ giá trị khởi tạo "Khá"). Byte đổi xếp loại thành "Giỏi" — điền đúng chuỗi "Giỏi", giữ đúng kiểu string đã suy luận.'
- kind: one-line
  body: 'Chỗ trống là: "Giỏi"'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Toán 9 Giỏi"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không bấm nút Chạy một lần nào — bạn vẫn biết chính xác dòng nào sẽ bị
từ chối, và vì sao. Đó là sức mạnh của việc ĐỌC kiểu, không phải đoán.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp, khép lại cụm bài này.

Tới giờ, mọi lời hứa kiểu bạn đọc được đều gắn với một BIẾN đứng riêng
lẻ. Nhưng chương trình thật không chỉ có biến — nó có HÀM, và hàm có
THAM SỐ. R1 từng dạy tham số bằng cách đặt cho nó một cái TÊN.

Tham số của một hàm TypeScript có cần một lời hứa kiểu, giống hệt biến
vừa học suốt cụm này không? Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
