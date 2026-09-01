---
id: lap-trinh-ham.adt-pattern-matching.doi-chieu-python-dataclass-match-va-ts-union-switch
title: "Đối chiếu: Python's `@dataclass`+`match` (T4.1) và TS's union+`switch`"
summary: "Python's state machine (T4.1) dùng NHIỀU `@dataclass(frozen=True)` riêng, `match`/`case` khớp theo KIỂU CLASS lúc chạy; TypeScript dùng MỘT discriminated union, `switch` khớp theo GIÁ TRỊ field `kind`, và chữ ký hàm được kiểm ngay LÚC BIÊN DỊCH — trước khi có dòng nào chạy. Cùng một Ý (đủ mọi hình dạng), khác cơ chế: kiểm kiểu ĐỘNG so với TĨNH."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 24
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: ["ts.compare-python-dataclass-vs-ts-union"]
requires: ["ts.write-refactored-adt"]
concepts: ["ts.compare-python-dataclass-vs-ts-union"]
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
Bài trước: bạn tự tay refactor một tập cờ rời rạc thành một
discriminated union thật — bằng TypeScript. Dừng lại một nhịp: đây
CHÍNH LÀ việc bạn từng làm, nhiều track trước, bằng Python (T4.1). Cùng
Ý — khác ngôn ngữ, khác cơ chế.
::::

::::explain{#doi-chieu-co-che-khop}
T4.1's bài 20 dựng một state machine bài viết bằng BA
`@dataclass(frozen=True)` RIÊNG BIỆT — `Draft`, `Published`,
`Archived` — không hề gộp chung vào một kiểu union nào. Hàm chuyển
trạng thái dùng `match`: `case Draft(tieu_de=t): ...`, rồi
`case _: raise ValueError(...)`. Cơ chế đó khớp theo KIỂU CLASS — Python
hỏi "object này THẬT SỰ được tạo bằng `Draft(...)` hay không?", một
phép kiểm kiểu `isinstance` NGẦM, diễn ra ĐÚNG lúc dòng `match` đó
CHẠY.

TypeScript dựng lại CÙNG Ý bằng cách khác hẳn: không nhiều kiểu riêng,
mà MỘT discriminated union — mỗi biến thể là một `interface` mang field
`kind` kiểu literal (track này đã dựng từ bài 2). Hàm chuyển trạng thái
không hỏi "đây có phải LỚP `Draft` không", mà hỏi "trường `kind` mang
GIÁ TRỊ gì":

```typescript
interface Draft {
  kind: "draft";
  tieuDe: string;
}

interface Published {
  kind: "published";
  tieuDe: string;
  ngayDang: string;
}

type Bai = Draft | Published;

function moTa(bai: Bai): string {
  switch (bai.kind) {
    case "draft":
      return "nháp: " + bai.tieuDe;
    case "published":
      return "đã đăng: " + bai.tieuDe + " (" + bai.ngayDang + ")";
  }
}

const nhap: Bai = { kind: "draft", tieuDe: "Bài đầu tiên" };
const daDang: Bai = { kind: "published", tieuDe: "Bài đầu tiên", ngayDang: "2026-09-01" };

console.log(moTa(nhap));
console.log(moTa(daDang));
```

```text
nháp: Bài đầu tiên
đã đăng: Bài đầu tiên (2026-09-01)
```

`case "draft"` và `case "published"` so khớp đúng CHUỖI literal của
`kind` — không có `isinstance` nào, không hỏi object này được tạo ra
bằng constructor nào. Kết quả CUỐI CÙNG giống Python: mỗi biến thể được
xử lý đúng nhánh của nó. Nhưng CÁCH khớp khác hẳn — GIÁ TRỊ một field,
không phải LỚP của cả object.
::::

::::example{#ham-chuyen-trang-thai-doi-dung-bien-the}
Chỗ khác biệt sâu hơn lộ ra khi một hàm chuyển trạng thái CHỈ được
phép gọi từ ĐÚNG một biến thể nguồn — y hệt `publish(bai)` ở T4.1
(chỉ publish được từ `Draft`, gọi sai ném `ValueError`):

```typescript title=readonly
interface Draft {
  kind: "draft";
  tieuDe: string;
}

interface Published {
  kind: "published";
  tieuDe: string;
  ngayDang: string;
}

function xuatBan(bai: Draft): Published {
  return { kind: "published", tieuDe: bai.tieuDe, ngayDang: "2026-09-01" };
}

const nhap: Draft = { kind: "draft", tieuDe: "Bài đầu tiên" };
const daDang = xuatBan(nhap);
console.log(daDang);
```

```text title=readonly
{"kind":"published","tieuDe":"Bài đầu tiên","ngayDang":"2026-09-01"}
```

Ở Python, `publish(bai)` KHÔNG khai kiểu tham số — nhận được BẤT KỲ
giá trị nào, rồi TỰ kiểm bằng `match` bên trong thân hàm, ném
`ValueError` nếu gọi sai biến thể. Phép kiểm đó là một HÀNH ĐỘNG xảy ra
LÚC CHẠY, đúng dòng `match` được thực thi — Python không có cách nào
biết TRƯỚC, chỉ khi chương trình chạy tới đó.

TypeScript viết `xuatBan(bai: Draft): Published` — tham số khai THẲNG
kiểu `Draft`, không phải `Bai` (union rộng). Không cần một dòng kiểm
tra nào bên trong thân hàm: chữ ký hàm TỰ LÀ lời hứa, và TypeScript
kiểm lời hứa đó với MỌI lời gọi, tại CHỖ GỌI, LÚC BIÊN DỊCH — trước khi
có dòng nào được phép chạy (T4.0a's bài 3 đã dạy chính cơ chế này).
::::

::::predict{#doan-goi-sai-bien-the-nguon commitOnce}
Cùng hàm `xuatBan` ở trên, nhưng lần này gọi nó với `tinCu` — một
`Published`, SAI biến thể nguồn:

```typescript
interface Draft {
  kind: "draft";
  tieuDe: string;
}

interface Published {
  kind: "published";
  tieuDe: string;
  ngayDang: string;
}

function xuatBan(bai: Draft): Published {
  return { kind: "published", tieuDe: bai.tieuDe, ngayDang: "2026-09-01" };
}

console.log("Trước khi gọi");

const tinCu: Published = { kind: "published", tieuDe: "Tin cũ", ngayDang: "2026-08-01" };
console.log(xuatBan(tinCu));
```

Chạy đoạn này in ra gì?

:::opt{correct}
Không in ra dòng nào cả — kể cả `"Trước khi gọi"` — TypeScript từ chối
biên dịch NGAY (TS2345: `Published` không gán được cho tham số kiểu
`Draft`), không có dòng nào được phép chạy
:::

:::opt
In ra `"Trước khi gọi"`, rồi máy NÉM lỗi lúc chạy tới `xuatBan(tinCu)`
— giống hệt cách Python's `archive()` (T4.1's bài 20) chạy xong các
dòng trước rồi mới `raise ValueError` khi gặp đúng lời gọi sai
::why
Gần đúng ở việc bạn nhớ ĐÚNG cách PYTHON hoạt động — `archive(draft2,
...)` ở T4.1's bài 20 đúng là chạy hết các dòng đứng trước, rồi mới
`raise ValueError` NGAY LÚC dòng gọi sai đó thực thi.

Chỗ lệch: TypeScript không kiểm "lúc dòng đó chạy tới" — nó kiểm KIỂU
cho TOÀN BỘ chương trình MỘT LƯỢT, TRƯỚC KHI có dòng nào chạy (T4.0a's
bài 3 đã dạy chính cơ chế này). `xuatBan(tinCu)` phá lời hứa `Draft`
đã đủ để chặn CẢ CHƯƠNG TRÌNH — `"Trước khi gọi"` không bao giờ được
in, dù dòng đó tự nó không sai gì. Đã thử thật: output rỗng hoàn toàn.
::
:::

:::opt
In ra `"Trước khi gọi"`, rồi in ra object mà `xuatBan(tinCu)` tạo được
— vì `Draft` và `Published` đều có field `tieuDe` chung, TypeScript
coi vậy là đủ khớp
::why
Gần đúng ở việc bạn để ý ĐÚNG: `Draft` và `Published` đều CÓ field
`tieuDe` — quan sát cấu trúc đó thật, cả hai interface đều mang field
này.

Chỗ lệch: TypeScript so khớp kiểu THEO CẤU TRÚC, nhưng field `kind`
của mỗi interface là một kiểu LITERAL RIÊNG — `"draft"` với `Draft`,
`"published"` với `Published` — không CHUNG. `tinCu.kind` mang giá trị
`"published"`, không khớp kiểu `"draft"` mà tham số của `xuatBan` đòi
hỏi. CÓ CHUNG field `tieuDe` không đủ CỨU — field `kind` lệch đã đủ để
TypeScript chặn: TS2345, đã thử thật.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng một Ý — mỗi trạng thái tự mang hình dạng riêng, hàm chuyển trạng
thái kiểm ĐỦ mọi hình dạng — nhưng Python biết LÚC CHẠY, TypeScript
biết LÚC BIÊN DỊCH. Hai ngôn ngữ, một bài học.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã thấy CẢ HAI ngôn ngữ đạt cùng một đích, bằng hai cơ chế khác
nhau. Nhưng câu hỏi lớn hơn từ đầu track vẫn còn treo lại: khi nào một
tập cờ boolean/optional rời rạc ĐANG CHE GIẤU một sum type thật sự, mà
mắt thường không thấy ngay?

Bài sau (bài chốt cụm) đưa bạn một mô tả nghiệp vụ MỚI — không khái
niệm mới, chỉ đo khả năng TỰ NHẬN RA.
::::

::::checkpoint{mastery=0.8}
::::
