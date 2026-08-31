---
id: lap-trinh-ham.cong-typescript.sai-kieu-bi-chan-truoc-khi-chay
title: "Sai kiểu bị CHẶN TRƯỚC KHI CHẠY, không phải lúc chạy"
summary: "`let n: number = \"ba\"` — TypeScript TỪ CHỐI biên dịch (TS2322), và không một dòng mã nào được sinh ra để chạy — kể cả những dòng đứng TRƯỚC dòng sai. Khác hẳn Python: một TypeError chỉ lộ ra LÚC DÒNG ĐÓ THỰC SỰ CHẠY TỚI."
locale: vi
track: lap-trinh-ham
module: cong-typescript
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ts.compile-time-check]
requires: [ts.type-annotation]
concepts: [ts.compile-time-check]
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
Bài trước hỏi: phá lời hứa thì sao? Không phải "chạy rồi lỗi nổ ra".
Là điều khác hẳn — và khác xa Python hơn bạn tưởng.
::::

::::explain{#khong-mot-dong-nao-chay}
Với Python, một `TypeError` chỉ lộ ra LÚC TRÌNH THÔNG DỊCH CHẠY TỚI
đúng dòng gây lỗi. Mọi dòng ĐỨNG TRƯỚC dòng đó đã chạy xong xuôi rồi —
`print` đã in, file đã ghi, tiền đã trừ — TRƯỚC KHI chương trình dừng
lại giữa chừng.

TypeScript không hoạt động như vậy. Trước khi cho phép bất cứ dòng nào
chạy, nó đọc TOÀN BỘ chương trình một lượt, kiểm mọi lời hứa kiểu. Tìm
thấy MỘT chỗ phá lời hứa — dù chỉ một dòng, ở bất cứ đâu trong file —
nó TỪ CHỐI sinh ra bản mã chạy được. Không có bản mã nào cả. Không
dòng nào chạy, kể cả những dòng hoàn toàn đúng đứng NGAY TRƯỚC dòng sai
đó.

Nói cách khác: Python dừng GIỮA CHỪNG. TypeScript không cho BẮT ĐẦU.

Mã lỗi cho việc phá một lời hứa kiểu là `TS2322` — bạn đã gặp thoáng
qua ở bài trước, giờ nhìn nó hoạt động thật.
::::

::::example{#hai-cach-dung-lai}
Cùng một câu chuyện — in một dòng, rồi phạm lỗi kiểu, rồi in dòng nữa —
ở hai ngôn ngữ:

```python title=readonly
print("Bắt đầu chương trình")
diem = "chín mươi" + 5
print("Không bao giờ tới đây")
```

```text title=readonly
Bắt đầu chương trình
Traceback (most recent call last):
  ...
TypeError: can only concatenate str (not "int") to str
```

Python in "Bắt đầu chương trình" TRƯỚC, rồi mới gặp dòng cộng chuỗi với
số và dừng lại — dòng in thứ ba không bao giờ chạy tới, nhưng dòng in
đầu đã chạy XONG rồi.

```typescript title=readonly
console.log("Bắt đầu chương trình");
let diem: number = "chín mươi";
console.log("Không bao giờ tới đây");
```

```text title=readonly
(không in ra gì cả)

TS2322 (dòng 2, cột 5): Type 'string' is not assignable to type 'number'.
```

Đã chạy thật đoạn TypeScript này: output rỗng — không một chữ nào,
không cả "Bắt đầu chương trình". TypeScript phát hiện `diem` phá lời
hứa `number` ở dòng 2, từ chối sinh mã cho CẢ CHƯƠNG TRÌNH, và dòng in
đầu tiên — dù tự nó hoàn toàn đúng — không bao giờ được phép chạy.
::::

::::predict{#doan-cai-gi-duoc-in commitOnce}
Đoạn TypeScript này có một dòng phá lời hứa kiểu (`tuoi` hứa là
`number` nhưng nhận một chuỗi):

```typescript
console.log("Dòng 1");
console.log("Dòng 2");
let tuoi: number = "hai mươi";
console.log("Dòng 3");
```

**Trước khi đọc đáp án**, đoạn này sẽ in ra những gì?

:::opt{correct}
Không in ra dòng nào cả — TypeScript từ chối biên dịch ngay khi phát
hiện lỗi kiểu, nên không có mã JavaScript nào được sinh ra để chạy,
kể cả hai dòng `console.log` đứng TRƯỚC dòng lỗi
:::

:::opt
In ra "Dòng 1" và "Dòng 2", rồi dừng lại — giống hệt cách Python dừng
khi gặp `TypeError`
::why
Gần đúng ở việc bạn nhớ đúng CÁCH PYTHON hoạt động — Python chạy tuần
tự, in xong "Dòng 1", "Dòng 2" rồi mới gặp dòng lỗi và dừng lại giữa
chừng.

Chỗ lệch: TypeScript không chạy từng dòng rồi dừng khi gặp lỗi. Nó
kiểm kiểu cho TOÀN BỘ chương trình TRƯỚC, và nếu thấy bất kỳ chỗ nào
phá lời hứa, nó từ chối sinh mã JavaScript cho CẢ CHƯƠNG TRÌNH — kể cả
những dòng đứng trước dòng lỗi, dù bản thân chúng không sai gì. Đã thử
thật: output rỗng hoàn toàn.
::
:::

:::opt
In ra cả ba dòng — vì `console.log` không liên quan gì tới kiểu dữ
liệu của `tuoi`
::why
Gần đúng ở việc `console.log` tự nó không khai kiểu — ba lệnh in đó
đúng là không sai gì cả, tách riêng ra.

Chỗ lệch: TypeScript kiểm kiểu cho TOÀN BỘ tệp trước khi cho phép bất
cứ dòng nào chạy. Một lỗi kiểu ở dòng 3 chặn đứng cả chương trình, dù
các dòng in không hề đụng tới `tuoi`. "Không liên quan" không đủ để
cứu một dòng khỏi bị chặn — cả file bị từ chối cùng lúc.
::
:::

:::opt
Máy báo lỗi cú pháp, vì `"hai mươi"` không phải một con số hợp lệ để
viết trực tiếp
::why
Gần đúng ở việc bạn để ý đúng: `"hai mươi"` có vấn đề thật.

Chỗ lệch: đây không phải lỗi CÚ PHÁP. Dòng `let tuoi: number = "hai
mươi";` viết đúng ngữ pháp TypeScript hoàn toàn — một chuỗi hợp lệ,
gán cho một biến, đúng cấu trúc. Vấn đề là lỗi KIỂU: chuỗi đó không
khớp lời hứa `number`. Mã lỗi thật (`TS2322`, một lỗi NGỮ NGHĨA) khác
hẳn nhóm mã báo lỗi cú pháp (`TS1xxx`).
::
:::
::::

::::code{#sua-de-duoc-chay}
Bài kiểm tra điểm của một học sinh. Dòng khai `diem` đang chờ một giá
trị — điền đúng để CẢ CHƯƠNG TRÌNH được phép chạy, kể cả dòng in đứng
TRƯỚC nó.

```typescript title=starter
console.log("Kiểm tra học sinh");

let diem: number = ___;              // học sinh được 9 điểm
let dat: boolean = diem >= 5;

console.log("Điểm:", diem);
console.log("Đạt:", dat);
```

```typescript title=solution
console.log("Kiểm tra học sinh");

let diem: number = 9;
let dat: boolean = diem >= 5;

console.log("Điểm:", diem);
console.log("Đạt:", dat);
```

```typescript title=test
if (diem !== 9) throw new Error("diem phải là 9 — đang là " + diem);
if (dat !== true) throw new Error("dat phải là true khi diem >= 5 — đang là " + dat);
```

:::hints
- kind: attention
  body: Một chỗ trống — giá trị number của diem. Điền sai kiểu (một chuỗi hay boolean), CẢ chương trình sẽ không được phép chạy, kể cả dòng "Kiểm tra học sinh" đứng trên.
- kind: strategy
  body: 'diem đã hứa là number, và đề bài nói rõ học sinh được 9 điểm. Điền đúng con số 9 — không phải chuỗi "9", không phải bất cứ giá trị nào khác kiểu.'
- kind: one-line
  body: 'Chỗ trống là: 9'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "Kiểm tra học sinh"
- tier: output
  match: contains
  expect: "Điểm: 9"
- tier: output
  match: contains
  expect: "Đạt: true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Điền đúng kiểu, CẢ chương trình được phép chạy — kể cả dòng đứng trên
đầu, dòng chẳng liên quan gì tới `diem`. Đúng-hay-sai được quyết định
TRƯỚC, không phải giữa chừng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Giờ bạn biết: một lời hứa kiểu (bài 2), bị kiểm TRƯỚC KHI CHẠY, không
phải lúc chạy (bài này). Cả chương trình chạy được hay không được
quyết định trong MỘT lượt đọc, trước khi có dòng nào chạy.

Nhưng mọi ví dụ tới giờ đều VIẾT RA tường minh dấu hai chấm và tên
kiểu — `: number`, `: string`. Nếu bạn LƯỜI, không viết kiểu nào cả,
TypeScript có còn kiểm được gì không, hay lúc đó nó bó tay? Bài sau trả
lời.
::::

::::checkpoint{mastery=0.8}
::::
