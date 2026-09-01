---
id: lap-trinh-ham.adt-pattern-matching.branded-type-loi-hua-chi-ton-tai-luc-bien-dich
title: "Branded type — lời hứa CHỈ TỒN TẠI lúc biên dịch"
summary: "`type Email = string & { readonly __brand: \"Email\" }` — một `string` gắn thêm một nhãn chỉ tồn tại trong THẾ GIỚI KIỂU. Đã đo thật hai điều đối lập: `typeof email === \"string\"` lúc chạy (nhãn không tồn tại runtime), nhưng truyền một `string` thường vào chỗ đòi `Email` bị TS2345 chặn lúc biên dịch (nhãn có hiệu lực compile-time)."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 27
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ts.branded-type-basics]
requires: [ts.string-everything-swap-risk]
concepts: [ts.branded-type-basics]
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
Bài trước: `guiEmail(email, tenNguoiDung)` — cả hai tham số ĐỀU là
`string`. Đảo thứ tự lúc gọi, TypeScript không hề phát hiện — với
trình biên dịch, một `string` LÀ một `string`, không phân biệt string
nào "là" email, string nào "là" tên. Hôm nay: một cách để hai `string`
đó KHÔNG còn giống nhau nữa, trong con mắt của trình biên dịch.
::::

::::explain{#branded-type-la-gi}
`Email` và `TenNguoiDung`, nếu khai đơn giản là `string`, là CÙNG một
kiểu — TypeScript không có cách nào phân biệt chúng dựa trên kiểu đã
khai. Một kỹ thuật gọi là "branded type" (kiểu GẮN NHÃN) giải quyết
đúng chỗ đó: gắn thêm một "dấu" chỉ tồn tại trong THẾ GIỚI KIỂU vào
một `string` bình thường.

```typescript
type Email = string & { readonly __brand: "Email" };

function guiEmail(email: Email): void {
  console.log("đã gửi tới " + email);
}

const email = "an@vidu.com" as Email;
guiEmail(email);
console.log(typeof email);
```

```text
đã gửi tới an@vidu.com
string
```

`Email` là kiểu GIAO (`&`, "intersection type") giữa `string` và một
object type chỉ có đúng một field `__brand` kiểu literal `"Email"`.
Đọc là: một giá trị kiểu `Email` phải VỪA là `string`, VỪA có field
`__brand` mang đúng giá trị `"Email"`.

Nhưng KHÔNG object thật nào trong bộ nhớ có field `__brand` cả.
`"an@vidu.com" as Email` không GẮN thêm bất cứ thứ gì vào chuỗi — `as
Email` (một PHÉP ÉP KIỂU, "type assertion") chỉ nói với TypeScript:
"tin tôi, coi chuỗi này là kiểu `Email`". Chuỗi đó, LÚC CHẠY, vẫn y
hệt một chuỗi JavaScript bình thường — không field nào cả. Dòng cuối
chứng minh: `typeof email` trả về đúng `"string"`, không phải một cái
gì lạ lẫm liên quan tới `__brand`. Nhãn KHÔNG tồn tại lúc chạy.

Nhưng nhãn đó lại có hiệu lực THẬT SỰ lúc BIÊN DỊCH — trình biên dịch
coi `Email` và `string` (hay một kiểu gắn nhãn khác) là hai kiểu KHÁC
NHAU, dù cả hai cùng là chuỗi bên dưới. Xem điều đó xảy ra thế nào.
::::

::::example{#branded-chan-truyen-nham}
Gắn hai nhãn KHÁC NHAU cho hai tham số của `guiEmail` — đúng vấn đề
bài trước nêu ra, giờ thử lại với branded type:

```typescript title=readonly
type Email = string & { readonly __brand: "Email" };
type TenNguoiDung = string & { readonly __brand: "TenNguoiDung" };

function guiEmail(email: Email, tenNguoiDung: TenNguoiDung): void {
  console.log("gửi tới " + email + ", tên " + tenNguoiDung);
}

const email = "an@vidu.com" as Email;
const ten = "An" as TenNguoiDung;

guiEmail(email, ten);
```

```text title=readonly
gửi tới an@vidu.com, tên An
```

Đúng thứ tự — biên dịch qua, chạy đúng. Giờ đảo thứ tự — đúng cú pháp
gọi hàm mà bài trước đã đo là "không bị chặn gì" khi cả hai tham số
cùng là `string` thường:

```typescript title=readonly
type Email = string & { readonly __brand: "Email" };
type TenNguoiDung = string & { readonly __brand: "TenNguoiDung" };

function guiEmail(email: Email, tenNguoiDung: TenNguoiDung): void {
  console.log("gửi tới " + email + ", tên " + tenNguoiDung);
}

const email = "an@vidu.com" as Email;
const ten = "An" as TenNguoiDung;

guiEmail(ten, email);
```

```text title=readonly
(không dòng nào chạy)

TS2345 (dòng 11, cột 10): Argument of type 'TenNguoiDung' is not
assignable to parameter of type 'Email'.   Type 'TenNguoiDung' is not
assignable to type '{ readonly __brand: "Email"; }'.     Types of property
'__brand' are incompatible.       Type '"TenNguoiDung"' is not assignable
to type '"Email"'.
```

Lần này TypeScript CHẶN NGAY — mã `TS2345`, ngay lúc biên dịch, không
dòng nào chạy. Chỗ khác biệt CỐT LÕI so với bài trước: `TenNguoiDung`
và `Email` giờ có field `__brand` mang HAI giá trị literal khác nhau
(`"TenNguoiDung"` với `"Email"`) — thông điệp lỗi nói đúng chỗ đó:
"Types of property `__brand` are incompatible". Hai kiểu này CÙNG là
`string` bên dưới, nhưng KHÁC nhau trong con mắt trình biên dịch, chỉ
vì nhãn khác nhau. Đây chính là điều bài trước còn thiếu.
::::

::::predict{#doan-truyen-string-thuong commitOnce}
Một biến khai kiểu `string` THƯỜNG (chưa từng ép sang `Email`), dù nội
dung trông đúng định dạng một email:

```typescript
type Email = string & { readonly __brand: "Email" };

function xacNhanEmail(email: Email): string {
  return "đã xác nhận: " + email;
}

const emailTho: string = "an@vidu.com";
console.log(xacNhanEmail(emailTho));
```

Dòng cuối chạy ra sao?

:::opt{correct}
Máy báo lỗi biên dịch — mã TS2345 tại dòng `xacNhanEmail(emailTho)`,
vì `emailTho` mang kiểu `string` THƯỜNG, không phải `Email`
:::

:::opt
In ra `đã xác nhận: an@vidu.com` — vì nội dung chuỗi ĐÚNG định dạng
một địa chỉ email, TypeScript tự nhận ra và chấp nhận
::why
Gần đúng ở việc bạn để ý đúng: `"an@vidu.com"` trông ĐÚNG như một địa
chỉ email hợp lệ thật — quan sát về NỘI DUNG chuỗi đó đúng.

Chỗ lệch: TypeScript không đọc GIÁ TRỊ chuỗi lúc kiểm kiểu — nó chỉ
nhìn KIỂU đã khai. `emailTho: string` là một `string` THƯỜNG cho trình
biên dịch, bất kể nội dung trông thế nào, vì nó thiếu field `__brand`
trong THẾ GIỚI KIỂU (không có `as Email` nào cả). TS2345 chặn ngay, đã
đo thật.
::
:::

:::opt
In ra `đã xác nhận: an@vidu.com`, biên dịch qua bình thường, rồi lúc
CHẠY mới ném lỗi vì thiếu field `__brand` trên object thật
::why
Gần đúng ở việc bạn tin `__brand` đóng vai trò TRUNG TÂM trong cơ chế
này — đúng, field đó chính là thứ TypeScript so sánh.

Chỗ lệch: không có "kiểm tra lúc chạy" nào liên quan tới `__brand` —
field đó KHÔNG tồn tại lúc chạy (đã đo ở phần trước: `typeof` một
`Email` trả đúng `"string"`, không field nào cả). Toàn bộ việc kiểm
brand diễn ra Ở BIÊN DỊCH: hoặc TS2345 chặn từ đầu — không dòng nào
chạy — hoặc biên dịch qua và chạy y hệt một chuỗi thường. Không có
bước kiểm nào ở GIỮA hai khả năng đó.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một `string` được TypeScript coi là `Email` hay không tuỳ NHÃN nó
mang trong thế giới kiểu — không tuỳ nội dung, không tuỳ bất cứ điều
gì lúc chạy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã thấy `typeof email` trả về `"string"` — nhãn không hiện ra Ở
ĐÓ. Nhưng còn cách khác để "nhìn vào" một giá trị lúc chạy — ví dụ
chuyển nó thành JSON để gửi qua mạng. Nhãn có lẫn vào đâu đó trong kết
quả đó không, hay biến mất HOÀN TOÀN? Bài sau kiểm tra trực giác đó.
::::

::::checkpoint{mastery=0.8}
::::
