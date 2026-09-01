---
id: lap-trinh-ham.adt-pattern-matching.sum-type-mot-trong-nhieu-kha-nang
title: "Sum type — MỘT trong NHIỀU khả năng"
summary: "type Hinh = Vuong | Tron | TamGiac là 'sum type' (kiểu TỔNG) — một giá trị CHỈ thuộc ĐÚNG MỘT biến thể tại một thời điểm, không bao giờ 'vừa Vuong vừa Tron'. Đối lập trực tiếp product type (bài trước): product ĐÒI đủ mọi trường CÙNG LÚC, sum CHỈ CHO đúng MỘT biến thể."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 15
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ts.sum-type-concept]
requires: [ts.product-type-concept]
concepts: [ts.sum-type-concept]
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
Bài trước: `NguoiDung { ten; tuoi; email }` phải có ĐỦ CẢ BA trường CÙNG
LÚC — kiểu TÍCH. Hôm nay quay lại `Hinh` — một kiểu dữ liệu hoạt động
NGƯỢC LẠI hoàn toàn.
::::

::::explain{#mot-gia-tri-mot-bien-the}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiac {
  kind: "tamGiac";
  day: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

const h1: Hinh = { kind: "vuong", canh: 4 };
const h2: Hinh = { kind: "tron", banKinh: 3 };

console.log(h1.kind);
console.log(h2.kind);
```

```text
vuong
tron
```

Bài trước, `NguoiDung` là 'product type' (kiểu TÍCH) — một giá trị PHẢI
có ĐỦ `ten` VÀ `tuoi` VÀ `email` CÙNG LÚC, thiếu một trường là không
biên dịch được. `Hinh` ở đây hoạt động NGƯỢC LẠI: `h1` là một `Vuong` —
nó KHÔNG PHẢI, và không thể ĐỒNG THỜI là, một `Tron` hay một `TamGiac`.
Không có khoảnh khắc nào `h1` "vừa là Vuong vừa là Tron" — nó luôn luôn
ĐÚNG MỘT trong ba biến thể.

Một discriminated union như `Hinh` được gọi là 'sum type' (kiểu TỔNG).
Trong khi product ĐÒI đủ mọi trường CÙNG LÚC, sum CHỈ CHO đúng MỘT biến
thể tại một thời điểm — bạn được CHỌN `Vuong` HOẶC `Tron` HOẶC
`TamGiac`, không phải kết hợp cả ba.
::::

::::example{#khong-mang-field-hai-bien-the}
Không phần tử `Hinh` nào mang field của HAI biến thể cùng lúc — đếm số
field THẬT SỰ có trên từng phần tử để thấy rõ:

```typescript title=readonly
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

interface TamGiac {
  kind: "tamGiac";
  day: number;
  chieuCao: number;
}

type Hinh = Vuong | Tron | TamGiac;

const dsHinh: Hinh[] = [
  { kind: "vuong", canh: 4 },
  { kind: "tron", banKinh: 3 },
  { kind: "tamGiac", day: 6, chieuCao: 5 },
];

for (const h of dsHinh) {
  console.log(h.kind, Object.keys(h).length);
}
```

```text title=readonly
vuong 2
tron 2
tamGiac 3
```

`Vuong` và `Tron` mỗi cái chỉ có 2 field (`kind` cộng MỘT field riêng),
`TamGiac` có 3 field (`kind` cộng HAI field riêng). Không phần tử nào
có cả `canh` LẪN `banKinh` LẪN `day` — mỗi giá trị chỉ SỐNG trong đúng
MỘT biến thể tại một thời điểm, không nhiều hơn.
::::

::::predict{#doan-loi-field-hai-bien-the commitOnce}
```typescript
interface Vuong {
  kind: "vuong";
  canh: number;
}

interface Tron {
  kind: "tron";
  banKinh: number;
}

type Hinh = Vuong | Tron;

const h: Hinh = { kind: "vuong", canh: 4, banKinh: 3 };

console.log(h);
```

Dòng khai `h` biên dịch và chạy thế nào?

:::opt{correct}
Máy báo lỗi biên dịch — object literal có field `banKinh` mà `Vuong`
không hề khai, mã TS2353, không dòng nào chạy
:::

:::opt
In ra `{ kind: 'vuong', canh: 4, banKinh: 3 }` — object vẫn được TẠO RA
với đủ ba field, TypeScript chỉ kiểm KIỂU chứ không cấm field thừa
::why
Gần đúng ở việc bạn tin JavaScript LÚC CHẠY không hề cấm object có
field thừa — điều đó đúng, object JS thuần có thể mang bao nhiêu field
tuỳ ý.

Chỗ lệch: `h` được khai TƯỜNG MINH là kiểu `Hinh` (tức khớp `Vuong` HOẶC
`Tron`), và TypeScript kiểm object literal gán TRỰC TIẾP cho một kiểu đã
biết bằng "excess property check" — field `banKinh` không thuộc `Vuong`
(biến thể mà `kind: "vuong"` khớp) bị CHẶN ngay lúc biên dịch, không bao
giờ tới lúc chạy để in ra bất cứ gì.
::
:::

:::opt
In ra `{ kind: 'vuong', canh: 4 }` — TypeScript tự LOẠI BỎ field
`banKinh` vì nó không thuộc `Vuong`, chỉ giữ lại field hợp lệ
::why
Gần đúng ở việc bạn nhận ra `banKinh` không THUỘC VỀ `Vuong` — quan sát
đó đúng, `Vuong` chỉ khai `kind` và `canh`.

Chỗ lệch: TypeScript KHÔNG BAO GIỜ tự "loại bỏ" field khỏi mã bạn viết —
nó chỉ CHẶN nếu có field thừa, buộc BẠN xoá field đó khỏi mã nguồn. Mã
này BIÊN DỊCH THẤT BẠI hoàn toàn (mã TS2353), không có object nào được
tạo ra, không có gì để in.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sum type CHỈ CHO một biến thể tại một thời điểm — đối lập trực tiếp với
product type CHỈ CHO đủ mọi trường cùng lúc. Hai cái tên này giờ là
NGÔN NGỮ chung để nói về ADT.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`NguoiDung` có 3 trường, mỗi trường nhận vô số giá trị khả dĩ — có bao
nhiêu `NguoiDung` khác nhau có thể tồn tại? `Hinh` có 3 biến thể — có
bao nhiêu `Hinh` khác nhau có thể tồn tại? Hai câu hỏi này có ĐẾM theo
cùng một cách không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
