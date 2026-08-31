---
id: lap-trinh-ham.cong-typescript.undefined-khong-tu-nhien-bien-mat
title: "undefined không tự nhiên biến mất chỉ vì bạn không viết ra"
summary: "`const v: number[] = [1, 2]; const a: number = v[0];` — TSC TỪ CHỐI, vì `v[0]` CÓ THỂ không tồn tại nếu mảng rỗng lúc chạy. Khác Python — không có cảnh báo trước, chỉ có IndexError lúc động tới ô đó."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.possibly-undefined]
requires: [ts.full-signature]
concepts: [ts.possibly-undefined]
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
Bài trước hỏi: đọc phần tử đầu tiên của một mảng `number[]` có LUÔN
chắc chắn ra một `number` không? Câu trả lời ngắn: không luôn — và
TypeScript biết trước điều đó, ngay cả khi bạn viết `[1, 2]` rành rành
trước mắt.
::::

::::explain{#o-trong-khong-tu-bien-mat}
`number[]` là một lời hứa về TỪNG PHẦN TỬ — "mỗi ô trong mảng này, nếu
có, là một `number`". Nó KHÔNG hứa mảng có bao nhiêu phần tử, cũng
không hứa ô bạn định đọc thật sự tồn tại.

Trong JavaScript thuần (và Python), đọc một ô mảng không tồn tại xử lý
rất khác nhau, nhưng đều lặng lẽ theo cách riêng của nó. Python ném ra
`IndexError` NGAY LÚC dòng đó chạy tới — ồn ào, dừng chương trình,
nhưng ít nhất bạn BIẾT ngay. JavaScript thì im lặng hơn: đọc một ô
ngoài phạm vi trả về `undefined`, không lỗi gì cả, và `undefined` đó
âm thầm trôi tiếp vào phép tính sau — cho tới khi nó gây ra một lỗi
khác, ở một chỗ xa hoàn toàn khác trong chương trình.

Đó là lý do TypeScript bật cờ `noUncheckedIndexedAccess`: mỗi lần bạn
đọc `mang[i]`, kiểu trả về không phải `T`, mà là **`T | undefined`** —
"có thể là T, có thể là không có gì cả". Cờ này buộc điều JavaScript
lặng lẽ bỏ qua phải hiện ra NGAY TRONG KIỂU, trước khi chương trình
được phép chạy — dù bạn viết `mang[0]` (nhìn có vẻ chắc chắn tồn tại)
hay `mang[999]` (rõ ràng liều lĩnh), TSC không phân biệt: nó không
biết mảng có bao nhiêu phần tử LÚC CHẠY, nên nó gắn `| undefined` cho
MỌI phép đọc chỉ số, không ngoại lệ.
::::

::::example{#doc-mot-o-mang}
Một mảng hai phần tử, đọc phần tử đầu, gán thẳng vào biến hứa kiểu
`number`:

```typescript title=readonly
const v: number[] = [1, 2];
const a: number = v[0];
```

```text title=readonly
(không có mã nào được sinh ra)

TS2322 (dòng 2, cột 7): Type 'number | undefined' is not assignable
to type 'number'.   Type 'undefined' is not assignable to type
'number'.
```

Đã chạy thật đoạn này: TSC từ chối, đúng mã `TS2322` — mã lỗi bạn đã
gặp từ bài 3, giờ xuất hiện từ một nguồn khác hẳn. Không phải bạn gõ
nhầm chuỗi cho biến `number` — bạn đọc một Ô MẢNG, và TypeScript biết
ô đó CÓ THỂ trống, dù `[1, 2]` ngay trước mắt bạn rõ ràng có hai phần
tử. TSC không "nhìn" vào giá trị `[1, 2]` để suy ra độ dài — nó chỉ
biết kiểu là `number[]`, và với `number[]`, MỌI chỉ số đều có thể
trượt ra ngoài.

Muốn dòng này qua được, cách đơn giản nhất là khai đúng kiểu mà `v[0]`
thật sự có — `number | undefined` — thay vì giả vờ nó luôn là
`number`:

```typescript title=readonly
const v: number[] = [1, 2];
const a: number | undefined = v[0];
console.log(a);
```

```text title=readonly
1
```

Đã chạy thật: dòng này biên dịch, chạy, in ra `1`. Không phải TypeScript
"bỏ qua" khả năng trống — bạn vừa THỪA NHẬN nó bằng chính kiểu bạn viết.
::::

::::predict{#dong-nao-bi-chan commitOnce}
Byte viết bốn dòng, KHÔNG chạy thử:

```typescript
const gia: number[] = [15000, 22000];

const dong1: number = gia.length;
const dong2: number = gia[0];
const dong3: number | undefined = gia[5];
console.log(dong1, dong2, dong3);
```

**Trước khi đọc đáp án**, dòng nào bị TypeScript từ chối, và vì sao?

:::opt{correct}
Dòng `const dong2: number = gia[0];` — `gia[0]` có kiểu
`number | undefined` (vì đọc chỉ số luôn có thể trống), còn `dong2` đã
hứa tường minh là `number` — gán một giá trị có thể-undefined vào một
biến không-chấp-nhận-undefined phá lời hứa, dù `0` là chỉ số hợp lệ
nhìn bằng mắt thường
:::

:::opt
Dòng `const dong3: number | undefined = gia[5];` bị từ chối — vì
`gia` chỉ có 2 phần tử, chỉ số 5 vượt ra ngoài mảng
::why
Gần đúng ở việc bạn để ý đúng: chỉ số 5 THẬT SỰ vượt ngoài mảng chỉ có
2 phần tử — về mặt CHẠY THẬT, `gia[5]` sẽ là `undefined`.

Chỗ lệch: TypeScript không biết mảng có bao nhiêu phần tử LÚC BIÊN
DỊCH — kiểu `number[]` không mang thông tin độ dài. Nó gắn
`number | undefined` cho MỌI phép đọc chỉ số như nhau, bất kể chỉ số
là 0 hay 5. Dòng này khai đúng kiểu `number | undefined` mà `gia[5]`
thật sự có, nên hoàn toàn hợp lệ — đã thử thật, không bị từ chối.
::
:::

:::opt
Dòng `const dong1: number = gia.length;` bị từ chối — vì `.length`
của một mảng có thể trống nếu mảng rỗng
::why
Gần đúng ở việc bạn cẩn thận với TRƯỜNG HỢP mảng rỗng — đúng là một
điều đáng lo với mảng.

Chỗ lệch: `.length` không phải một phép ĐỌC CHỈ SỐ — nó luôn trả về
đúng MỘT con số, kể cả khi mảng rỗng (`[].length` là `0`, không phải
`undefined`). Cờ `noUncheckedIndexedAccess` chỉ ảnh hưởng tới việc đọc
`mang[i]`, không ảnh hưởng `.length`. Dòng này hoàn toàn hợp lệ.
::
:::

:::opt
Không dòng nào bị từ chối — cả ba biến đều đọc từ cùng một mảng `gia`,
nên chúng chia sẻ chung độ an toàn
::why
Gần đúng ở việc cả ba đúng là cùng đọc từ `gia` — nhưng "đọc từ cùng
một mảng" không phải điều TSC quan tâm.

Chỗ lệch: TSC xét TỪNG DÒNG riêng — kiểu của vế phải so với kiểu đã
hứa của vế trái. `dong2` hứa `number` nhưng nhận `number | undefined`
— phá lời hứa, bị từ chối thật. Việc `dong1` và `dong3` hợp lệ không
"lây" sang `dong2`.
::
:::
::::

::::code{#khai-dung-kieu-cho-o-mang}
Ba học sinh vừa nộp điểm. Đọc điểm đầu tiên trong danh sách — điền
đúng cách đọc, khai đúng kiểu mà phép đọc đó THẬT SỰ trả về.

```typescript title=starter
const diem: number[] = [8, 9, 10];
const dau: number | undefined = ___;    // đọc phần tử đầu tiên

console.log("Điểm đầu:", dau);
```

```typescript title=solution
const diem: number[] = [8, 9, 10];
const dau: number | undefined = diem[0];

console.log("Điểm đầu:", dau);
```

```typescript title=test
if (dau !== 8) throw new Error("dau phải là 8 (phần tử đầu của diem) — đang là " + dau);
```

:::hints
- kind: attention
  body: Chỗ trống là một phép ĐỌC CHỈ SỐ trên mảng diem — không phải một con số viết tay. Kiểu number | undefined đã khai sẵn ở vế trái, đúng kiểu mà việc đọc chỉ số trả về.
- kind: strategy
  body: 'Điểm đầu tiên của một mảng nằm ở chỉ số 0. Đọc nó bằng diem[0] — kết quả có kiểu number | undefined, khớp đúng kiểu dau đã khai.'
- kind: one-line
  body: 'Chỗ trống là: diem[0]'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Điểm đầu: 8"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`[8, 9, 10]` rõ ràng có phần tử đầu — nhưng TypeScript vẫn không cho
bạn giả vờ chắc chắn. Bạn phải khai đúng: kết quả CÓ THỂ là undefined.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa học cách khai ĐÚNG kiểu cho một giá trị có thể vắng mặt — nhưng
khai đúng kiểu mới là bước đầu. Thử cộng `dau + 1` ngay sau khi khai —
TypeScript vẫn từ chối, một mã lỗi khác hẳn `TS2322`. Biết `dau` CÓ THỂ
là undefined chưa đủ để DÙNG được nó như một số bình thường.

Có cách nào để, tại một chỗ cụ thể trong code, khiến TypeScript tin
rằng giá trị chắc chắn ĐÃ CÓ, không còn undefined nữa không? Bài sau
trả lời.
::::

::::checkpoint{mastery=0.8}
::::
