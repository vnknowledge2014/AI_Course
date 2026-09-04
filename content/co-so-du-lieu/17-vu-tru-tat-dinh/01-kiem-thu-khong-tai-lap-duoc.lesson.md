---
id: co-so-du-lieu.vu-tru-tat-dinh.kiem-thu-khong-tai-lap-duoc
title: "Kiểm thử không tái lập được"
summary: "haiDayGiongHet so sánh hai dãy số ĐỂ kiểm tra chúng có giống hệt nhau không. Gọi Math.random() năm lần cho 'lần 1', RỒI năm lần nữa cho 'lần 2' — haiDayGiongHet(lan1, lan2) LUÔN false, dù cùng chạy trong CÙNG một script. Math.random() KHÔNG nhận seed — không có cách nào yêu cầu JS 'lặp lại đúng dãy số lần trước' để chạy LẠI một kịch bản đã từng thấy lỗi."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.kiem-thu-khong-tai-lap-duoc]
requires: [db.id-trung-lap-la-idempotent]
concepts: [db.kiem-thu-khong-tai-lap-duoc]
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
q16 xây một sổ cái ĐÚNG đắn. Nhưng "đúng" phải được KIỂM chứng — VÀ
kiểm thử một hệ THỐNG phân tán THẬT sự khó, vì lỗi thường CHỈ xuất
hiện Ở đúng một THỨ tự sự kiện hiếm gặp. Tìm được lỗi đó MỘT lần —
làm sao THẤY lại nó lần NỮA?
::::

::::explain{#khong-tai-lap-duoc}
`haiDayGiongHet` so sánh hai dãy số ĐỂ kiểm tra chúng CÓ giống hệt
nhau không. Gọi `Math.random()` năm lần CHO "lần 1", RỒI năm lần
NỮA cho "lần 2" — NGAY trong CÙNG một script:

```typescript title=readonly
function haiDayGiongHet(a: number[], b: number[]): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

const lan1: number[] = [];
const lan2: number[] = [];
for (let i = 0; i < 5; i++) lan1.push(Math.random());
for (let i = 0; i < 5; i++) lan2.push(Math.random());

console.log("lan 1 va lan 2 giong het:", haiDayGiongHet(lan1, lan2));
console.log("lan 1 tu giong chinh no:", haiDayGiongHet(lan1, lan1));
```

```text title=readonly
lan 1 va lan 2 giong het: false
lan 1 tu giong chinh no: true
```

`lan1` VÀ `lan2` KHÁC nhau — `Math.random()` không hề "nhớ" gì GIỮA
hai lần gọi, mỗi con SỐ đều mới VÀ không đoán trước được. `lan1` DĨ
nhiên giống CHÍNH nó (so sánh một mảng VỚI chính nó LUÔN `true`) —
nhưng KHÔNG có cách nào tạo RA một mảng THỨ hai khớp `lan1`, kể cả
khi MUỐN, vì `Math.random()` không nhận MỘT tham số nào để "yêu cầu
lặp lại".
::::

::::example{#loi-hiem-khong-thay-lai}
Đây LÀ vấn đề THẬT khi kiểm thử một hệ phân TÁN: một lỗi CHỈ xảy ra
khi ba sự kiện tới ĐÚNG một thứ tự hiếm (VÍ dụ "node A sập NGAY giữa
lúc node B đang gửi hint") — nếu thứ tự đó được quyết ĐỊNH bởi
`Math.random()`/`Date.now()` THẬT, chạy LẠI CÙNG một bài kiểm thử
KHÔNG có gì đảm bảo tái tạo được đúng thứ tự ĐÓ lần nữa. Bug "biến
mất" khi thử BẮT lại nó LÀ một trong những LOẠI lỗi khó chịu nhất
trong kỹ nghệ phần mềm.
::::

::::predict{#doan-math-random-co-tham-so commitOnce}
Gọi `Math.random(42)` (truyền MỘT đối số, giống như "seed"). JavaScript
KHÔNG báo lỗi (hàm chấp nhận CUỘC gọi CÓ đối số thừa). Kết QUẢ trả về
CÓ phụ thuộc VÀO `42` không — nghĩa LÀ gọi `Math.random(42)` hai LẦN
CÓ cho ra CÙNG một số không?
:::opt{correct}
KHÔNG — `Math.random()` KHÔNG hề định nghĩa tham SỐ nào; đối số `42`
bị JS ÂM thầm bỏ qua (không NÉM lỗi, nhưng cũng KHÔNG dùng tới), kết
quả VẪN ngẫu nhiên VÀ khác nhau mỗi lần gọi, y HỆT như gọi
`Math.random()` KHÔNG đối số
:::
:::opt
CÓ — `42` đóng vai TRÒ một seed, nên `Math.random(42)` LUÔN trả về
cùng một SỐ mỗi lần gọi
::why
Trực giác NÀY hợp LÝ nếu so SÁNH với các thư viện PRNG CÓ hỗ trợ
seed (giống ĐIỀU bài SAU sẽ tự xây) — nhưng `Math.random()` chuẩn
CỦA JavaScript hoàn TOÀN không có tham số NÀO trong đặc tả.

Chỗ lệch: gọi MỘT hàm JavaScript VỚI nhiều đối số HƠN nó khai BÁO
không phải lỗi CÚ pháp — các đối số THỪA đơn giản bị bỏ QUA. `Math.
random(42)` chạy CHÍNH XÁC như `Math.random()`, VÀ hai lần gọi liên
tiếp cho RA hai số KHÁC nhau, không liên quan GÌ tới `42`.
::
:::
::::

::::code{#viet_hai_day_giong_het}
Hoàn thiện `haiDayGiongHet` — so sánh TỪNG phần tử của hai mảng,
trả VỀ `false` ngay khi gặp một CẶP khác nhau.

```typescript title=starter
function haiDayGiongHet(a: number[], b: number[]): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    ___
  }
  return true;
}

const lan1: number[] = [];
const lan2: number[] = [];
for (let i = 0; i < 5; i++) lan1.push(Math.random());
for (let i = 0; i < 5; i++) lan2.push(Math.random());
console.log(haiDayGiongHet(lan1, lan2));
```

```typescript title=solution
function haiDayGiongHet(a: number[], b: number[]): boolean {
  if (a.length !== b.length) return false;
  for (let i = 0; i < a.length; i++) {
    if (a[i] !== b[i]) return false;
  }
  return true;
}

const lan1: number[] = [];
const lan2: number[] = [];
for (let i = 0; i < 5; i++) lan1.push(Math.random());
for (let i = 0; i < 5; i++) lan2.push(Math.random());
console.log(haiDayGiongHet(lan1, lan2));
```

```typescript title=test
if (haiDayGiongHet([1, 2, 3], [1, 2, 3]) !== true) throw new Error("hai mang giong het nhau phai la true");
if (haiDayGiongHet([1, 2, 3], [1, 2, 4]) !== false) throw new Error("khac nhau O MOT phan tu cuoi cung phai la false");
if (haiDayGiongHet([1, 2, 3], [9, 2, 3]) !== false) throw new Error("khac nhau O phan tu DAU tien phai la false");
if (haiDayGiongHet([], []) !== true) throw new Error("hai mang rong phai duoc coi la giong het nhau (true)");
if (haiDayGiongHet([1, 2], [1, 2, 3]) !== false) throw new Error("do dai khac nhau phai la false, khong duoc so sanh phan chung roi bo qua phan thua");
if (haiDayGiongHet([1, 2, 3], [1, 2]) !== false) throw new Error("do dai khac nhau (mang thu hai NGAN hon) van phai la false");

const d = [5, 5, 5];
if (haiDayGiongHet(d, d) !== true) throw new Error("mot mang so sanh voi CHINH no phai la true");
```

:::hints
- kind: attention
  body: "Neu a[i] khac b[i] thi tra ve false ngay -- mot dong."
- kind: strategy
  body: "if (a[i] !== b[i]) return false;"
- kind: one-line
  body: "if (a[i] !== b[i]) return false;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`Math.random()` không tái lập được — nhưng KHÔNG có nghĩa LÀ "ngẫu
nhiên" và "tái lập được" LOẠI trừ nhau. Có cách NÀO vừa trông NGẪU
nhiên, vừa lặp LẠI được y hệt KHI muốn?
::::

::::reflect{#nghi-lai}
`haiDayGiongHet` LÀ công cụ đơn GIẢN nhất để trả LỜI câu hỏi "hai lần
chạy CÓ giống hệt nhau không" — VÀ câu trả lời CHO `Math.random()`
LUÔN LÀ "không", vĩnh viễn. Đây KHÔNG phải lỗi CỦA `Math.random()` —
nó được thiết kế ĐỂ không đoán trước được, đúng mục ĐÍCH của nó
(mật mã, trò CHƠI). Nhưng kiểm thử một hệ THỐNG phức tạp cần điều
NGƯỢC lại: "ngẫu nhiên" NHƯNG tái lập được KHI cần. Bước tiếp theo:
xây một MÁY sinh số "giả ngẫu nhiên" CÓ đúng tính chất đó.
::::

::::checkpoint{mastery=0.8}
::::
