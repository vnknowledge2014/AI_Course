---
id: lap-trinh-ham.adt-pattern-matching.smart-constructor-ham-validate-tra-ve-brand
title: "Smart constructor — hàm validate, TRẢ VỀ kiểu đã gắn nhãn"
summary: "`function taoEmail(s: string): Email | null { if (!s.includes(\"@\")) return null; return s as Email; }` — đây là cách DUY NHẤT để có một giá trị kiểu `Email`: gọi qua `taoEmail()`, hàm TỰ KIỂM `s` hợp lệ rồi mới trả về brand. `as Email` chỉ được viết ở ĐÚNG MỘT chỗ này trong cả chương trình — mọi nơi khác chỉ NHẬN `Email` từ kết quả `taoEmail()`."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 29
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.smart-constructor-pattern]
requires: [ts.predict-brand-erasure]
concepts: [ts.smart-constructor-pattern]
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
Bài trước: brand biến mất HOÀN TOÀN lúc chạy — `typeof` không phân
biệt nổi `Email` với `string` thường. Vậy làm sao BIẾT một giá trị
`Email` trong tay THẬT SỰ đã được kiểm tra hợp lệ, chứ không phải ai đó
gõ bừa?
::::

::::explain{#smart-constructor}
```typescript
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email | null {
  if (!s.includes("@")) return null;
  return s as Email;
}

const e1 = taoEmail("an@vidu.com");
const e2 = taoEmail("khongcoat");
console.log(e1);
console.log(e2);
```

```text
an@vidu.com
null
```

`taoEmail` là một "smart constructor" — một hàm KHÔNG chỉ TẠO ra giá
trị, mà còn TỰ KIỂM nó có hợp lệ không TRƯỚC KHI tạo. Nhận vào `s:
string` (một chuỗi bình thường, chưa được tin tưởng), kiểm `s.includes
("@")` — nếu SAI, trả `null` ngay, KHÔNG có `Email` nào sinh ra. Chỉ
khi kiểm qua, dòng cuối `return s as Email` mới chạy: `as Email` (ép
kiểu) nói với TypeScript "tôi đã tự kiểm chuỗi này rồi, hãy tin và gắn
nhãn `Email` cho nó".

Vì sao cần `as Email` ở đây mà bài trước KHÔNG cần? TypeScript không
thể tự SUY ra một `string` có chứa `"@"` hay không chỉ từ CẤU TRÚC kiểu
— brand không phải phép màu, nó không tự kiểm tra nội dung chuỗi. Phải
có MỘT chỗ, do NGƯỜI viết mã tự đảm bảo bằng logic thật (`if
(!s.includes("@"))`), rồi MỚI ép kiểu. `taoEmail` chính là chỗ đó — và
là CÁCH DUY NHẤT hợp lệ để có một giá trị kiểu `Email`: gọi qua hàm
này, không có đường tắt nào khác được coi là đúng.

`e1` (chuỗi có `"@"`) được trả về nguyên vẹn kèm brand. `e2` (chuỗi
không có `"@"`) nhận `null` — không có `Email` giả mạo nào lọt qua.
::::

::::example{#dung-lai-brand-da-validate}
Một hàm KHÁC chỉ NHẬN `Email` — nó không tự kiểm tra gì thêm, vì nó TIN
TƯỞNG rằng bất kỳ giá trị nào có kiểu `Email` đều ĐÃ đi qua `taoEmail`:

```typescript title=readonly
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email | null {
  if (!s.includes("@")) return null;
  return s as Email;
}

function guiEmail(email: Email): string {
  return "Đã gửi tới " + email;
}

const e = taoEmail("lan@congty.vn");
if (e !== null) {
  console.log(guiEmail(e));
}

console.log(taoEmail("khong-hop-le"));
```

```text title=readonly
Đã gửi tới lan@congty.vn
null
```

`guiEmail(email: Email)` không viết `if (!email.includes("@"))` lần
nữa — KHÔNG CẦN, vì tham số đã có kiểu `Email`, và lời hứa của brand là
"mọi giá trị mang nhãn này đã qua cửa `taoEmail`". Việc kiểm tra chỉ
xảy ra ĐÚNG MỘT LẦN, ngay lúc tạo ra giá trị — mọi hàm sau đó ĐỌC brand
mà không phải kiểm lại.

Chú ý khối `if (e !== null)`: `taoEmail` trả về `Email | null`, nên
phải NARROW loại bỏ `null` trước khi truyền `e` vào `guiEmail` (đúng
luật union đã học từ T4.0a) — TypeScript sẽ không cho gọi `guiEmail(e)`
trực tiếp khi `e` còn mang khả năng `null`.
::::

::::predict{#doan-bypass-as-email commitOnce}
```typescript
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email | null {
  if (!s.includes("@")) return null;
  return s as Email;
}

function guiEmail(email: Email): string {
  return "Đã gửi tới " + email;
}

const gia: Email = "khong-co-at-nao" as Email;
console.log(guiEmail(gia));
```

Dòng cuối in ra gì?

:::opt{correct}
`Đã gửi tới khong-co-at-nao` — máy KHÔNG báo lỗi gì, dù chuỗi này chưa
từng đi qua `taoEmail`
:::

:::opt
Máy báo lỗi biên dịch — mã TS2352, vì `"khong-co-at-nao"` chưa được
`taoEmail` kiểm tra và xác nhận hợp lệ trước khi ép kiểu
::why
Gần đúng ở việc bạn nhận ra chuỗi này KHÔNG hợp lệ theo luật nghiệp vụ
(`taoEmail` sẽ trả `null` nếu nhận đúng chuỗi này) — quan sát về NỘI
DUNG chuỗi đúng.

Chỗ lệch: `as Email` là một PHÉP ÉP KIỂU tĩnh, nó không CHẠY hàm
`taoEmail` hay bất kỳ logic kiểm tra nào — nó chỉ nói với trình biên
dịch "coi chuỗi này như `Email`". TypeScript chỉ chặn `as` khi hai kiểu
KHÔNG hề chồng lấp cấu trúc (ví dụ ép `number` thành `Email` mới báo
TS2352); `string` và `Email` (vốn LÀ `string & {...}`) chồng lấp nhau,
nên phép ép này biên dịch được — dù về nghiệp vụ là SAI.
::
:::

:::opt
Máy ném lỗi lúc CHẠY (uncaught exception) — vì `gia` không đúng "hình
dạng" `Email` thật sự
::why
Gần đúng ở việc bạn tin có một CƠ CHẾ nào đó bảo vệ `Email` khỏi giá
trị giả mạo — đúng là ĐÓ LÀ Ý ĐỊNH của brand pattern.

Chỗ lệch: bài trước đã đo — brand `{ __brand: "Email" }` bị XOÁ HOÀN
TOÀN lúc chạy, KHÔNG còn field nào để kiểm tra runtime. `gia` lúc chạy
đơn thuần là chuỗi `"khong-co-at-nao"`, không có "hình dạng" nào khác
biệt để ném lỗi. `guiEmail` chỉ nối chuỗi, không throw gì.
::
:::
::::

::::code{#tao_email}
Hoàn thiện `taoEmail(s)` — trả về `Email` (kèm brand) nếu `s` CÓ chứa
`"@"`, trả `null` nếu không.

```typescript title=starter
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email | null {
  if (!s.includes("@")) {
    return null;
  }
  return ___ as Email;
}

console.log(taoEmail("an@vidu.com"));
console.log(taoEmail("khongcoat"));
```

```typescript title=solution
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email | null {
  if (!s.includes("@")) {
    return null;
  }
  return s as Email;
}

console.log(taoEmail("an@vidu.com"));
console.log(taoEmail("khongcoat"));
```

```typescript title=test
const e1 = taoEmail("an@vidu.com");
if (e1 === null) throw new Error("taoEmail(\"an@vidu.com\") phải trả về Email, không phải null — chuỗi này CÓ chứa \"@\"");
if (e1 !== "an@vidu.com") throw new Error("taoEmail(\"an@vidu.com\") phải trả về đúng chuỗi \"an@vidu.com\" — đang là " + e1);
const e2 = taoEmail("khongcoat");
if (e2 !== null) throw new Error("taoEmail(\"khongcoat\") phải trả về null vì chuỗi KHÔNG chứa \"@\" — đang là " + e2);
const e3 = taoEmail("b@c");
if (e3 === null) throw new Error("taoEmail(\"b@c\") phải trả về Email, không phải null — chuỗi này CÓ chứa \"@\"");
```

:::hints
- kind: attention
  body: "Chỗ trống là GIÁ TRỊ được ép kiểu thành Email — hàm đã kiểm s hợp lệ ở nhánh if phía trên, giờ chỉ cần ép ĐÚNG biến đã kiểm."
- kind: strategy
  body: "Tham số hàm tên là s — dòng return phải ép kiểu CHÍNH tham số đó (return s as Email), không phải một biến hay chuỗi nào khác."
- kind: one-line
  body: "Chỗ trống là: s"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "an@vidu.com"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`taoEmail` là cổng DUY NHẤT hợp lệ để có một `Email` — nhưng cổng đó
chỉ đứng vững nếu KHÔNG ai viết `as Email` ở nơi khác trong chương
trình. Kỷ luật đó nằm ở NGƯỜI viết mã, không phải trình biên dịch.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa THẤY một smart constructor hoàn chỉnh (`taoEmail`) — nhưng
`Email` và điều kiện kiểm `"@"` đã được viết sẵn, bạn chỉ điền một chỗ
trống. Nếu phải TỰ TAY thiết kế một branded type MỚI (không phải
`Email`) VÀ smart constructor của nó, từ một luật hợp lệ khác (ví dụ:
một số tuổi PHẢI nằm trong khoảng nào đó) — bạn có làm được không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
