---
id: lap-trinh-ham.adt-pattern-matching.string-cho-moi-thu-de-truyen-nham
title: "`string` cho MỌI THỨ — dễ truyền NHẦM tham số"
summary: "function guiEmail(email: string, tenNguoiDung: string) — cả hai tham số ĐỀU là string. Gọi guiEmail(tenNguoiDung, email) (đảo NHẦM thứ tự) biên dịch BÌNH THƯỜNG, TypeScript không phát hiện gì — vì với trình biên dịch, string LÀ string, không phân biệt 'chuỗi này LÀ MỘT email' hay 'chuỗi kia LÀ MỘT tên'."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 26
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ts.string-everything-swap-risk]
requires: [ts.review-illegal-states]
concepts: [ts.string-everything-swap-risk]
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
Bài trước bạn đo xong: cờ boolean/optional che giấu sum type, và
discriminated union vá được lỗ đó. Nhưng có một lỗ HOÀN TOÀN khác —
không nằm trong union nào cả. Nó nằm ngay trong chữ ký của một hàm rất
bình thường.
::::

::::explain{#string-vo-nghia-voi-compiler}
```typescript
function guiEmail(email: string, tenNguoiDung: string): void {
  console.log("Gửi tới " + email + ", chào " + tenNguoiDung);
}

const email = "an@vidu.com";
const ten = "An";

guiEmail(email, ten);
guiEmail(ten, email);
```

```text
Gửi tới an@vidu.com, chào An
Gửi tới An, chào an@vidu.com
```

`guiEmail` nhận hai tham số — cả hai ĐỀU khai kiểu `string`. Với trình
biên dịch, kiểu của tham số thứ nhất và tham số thứ hai là CÙNG MỘT
kiểu: `string` LÀ `string`, không có gì phân biệt "chuỗi này LÀ MỘT
địa chỉ email" với "chuỗi kia LÀ MỘT tên người dùng".

Gọi `guiEmail(email, ten)` (đúng thứ tự) rồi `guiEmail(ten, email)`
(đảo NHẦM thứ tự) — CẢ HAI dòng biên dịch được, không một cảnh báo
nào. Lý do: cả hai lời gọi đều truyền đúng HAI GIÁ TRỊ kiểu `string`
vào đúng HAI VỊ TRÍ đòi `string`. TypeScript đã làm xong việc của nó —
kiểm kiểu khớp — nhưng nó không có cách nào biết tham số thứ nhất
"phải" chứa email, tham số thứ hai "phải" chứa tên, vì thông tin đó
KHÔNG NẰM trong kiểu `string` ở chữ ký hàm. Dòng thứ hai in ra một câu
sai hoàn toàn ý nghĩa — "Gửi tới An" (gửi tới một cái TÊN, không phải
địa chỉ) — nhưng không có tín hiệu lỗi nào cả.
::::

::::example{#danh-mat-du-lieu-khi-doi-cho}
Lỗi này không chỉ xảy ra khi GỌI hàm — nó cũng lặng lẽ chui vào DỮ LIỆU
được tạo ra, nếu hàm tạo cũng nhận hai `string` cùng kiểu:

```typescript title=readonly
interface NguoiDung {
  hoTen: string;
  email: string;
}

function taoNguoiDung(hoTen: string, email: string): NguoiDung {
  return { hoTen: hoTen, email: email };
}

const nd1 = taoNguoiDung("Bình", "binh@vidu.com");
const nd2 = taoNguoiDung("binh@vidu.com", "Bình");

console.log(nd1);
console.log(nd2);
```

```text title=readonly
{"hoTen":"Bình","email":"binh@vidu.com"}
{"hoTen":"binh@vidu.com","email":"Bình"}
```

`nd1` đúng thứ tự — `hoTen` giữ tên, `email` giữ địa chỉ. `nd2` bị đảo
NHẦM khi gọi — nhưng TypeScript vẫn biên dịch, chương trình vẫn chạy,
và `nd2` được tạo ra với `hoTen` chứa một địa chỉ email, `email` chứa
một cái tên. Không có ngoại lệ nào bị ném ra, không có cảnh báo nào ở
dòng khai `nd2` — dữ liệu SAI được lưu vào biến `nd2` y như dữ liệu
ĐÚNG được lưu vào `nd1`, hoàn toàn cùng một con đường, không phân biệt
được từ bên ngoài.
::::

::::predict{#doan-dat-phong commitOnce}
```typescript
function datPhong(maPhong: string, tenKhach: string): string {
  return "Đặt phòng " + maPhong + " cho " + tenKhach;
}

const maPhong = "P101";
const tenKhach = "Lan";

console.log(datPhong(maPhong, tenKhach));
console.log(datPhong(tenKhach, maPhong));
```

Dòng cuối (`datPhong(tenKhach, maPhong)` — hai tham số bị đảo NHẦM chỗ)
xảy ra chuyện gì?

:::opt{correct}
Biên dịch và chạy BÌNH THƯỜNG, in ra `Đặt phòng Lan cho P101` —
TypeScript không phát hiện gì sai
:::

:::opt
Máy báo lỗi biên dịch — mã TS2345, vì `"P101"` không phải một tên
khách hợp lệ để truyền vào tham số `tenKhach`
::why
Gần đúng ở việc bạn nhận ra hai giá trị đã bị đảo SAI Ý NGHĨA —
`"P101"` đúng là một mã phòng, không phải một cái tên, quan sát đó
đúng.

Chỗ lệch: TypeScript chỉ kiểm KIỂU (cả `"P101"` lẫn `"Lan"` đều là
`string`), nó không kiểm Ý NGHĨA của chuỗi. Tham số `tenKhach` khai
kiểu `string` — bất kỳ chuỗi nào cũng khớp kiểu đó, kể cả một chuỗi
trông giống mã phòng. Không có mã lỗi TS2345 nào ở đây, vì không có gì
VI PHẠM kiểu cả.
::
:::

:::opt
Biên dịch được, nhưng chương trình CRASH lúc chạy vì đối số truyền sai
thứ tự so với ý định ban đầu của hàm
::why
Gần đúng ở việc bạn tin có gì đó BẤT THƯỜNG thật sự đang xảy ra khi
đối số bị đảo — đúng là kết quả cuối cùng SAI nghĩa, "Đặt phòng Lan cho
P101" là một câu vô lý về nghiệp vụ.

Chỗ lệch: không có crash nào. Lúc chạy, `datPhong` chỉ nối ba chuỗi lại
với nhau bằng `+` — nó không biết và không kiểm tham số nào "phải" là
mã phòng, tham số nào "phải" là tên khách. Hàm chạy y hệt với hai chuỗi
bất kỳ, không `throw`, không ngoại lệ, in ra chuỗi kết quả (dù sai
nghĩa) một cách hoàn toàn bình thường.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chốt lại: khi HAI tham số cùng khai `string`, thứ tự đúng chỉ còn nằm
ở TRÍ NHỚ người viết code — không nằm ở đâu trình biên dịch có thể
kiểm được.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vấn đề không phải `string` "tệ" — nó vẫn là kiểu đúng cho cả email lẫn
tên người dùng, xét về HÌNH DẠNG dữ liệu (một chuỗi ký tự). Vấn đề là:
kiểu `string` không MANG theo thông tin "chuỗi này dùng để LÀM GÌ" —
mọi `string` đều bình đẳng với trình biên dịch, dù ý nghĩa nghiệp vụ
của chúng khác hẳn nhau.

Nếu có cách GẮN một cái nhãn vào kiểu `string` — để `Email` và
`TenNguoiDung` trở thành hai kiểu KHÁC NHAU trong mắt trình biên dịch,
dù lúc chạy cả hai vẫn chỉ là chuỗi — thì sao?
::::

::::checkpoint{mastery=0.8}
::::
