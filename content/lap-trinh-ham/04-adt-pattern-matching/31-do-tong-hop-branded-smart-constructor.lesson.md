---
id: lap-trinh-ham.adt-pattern-matching.do-tong-hop-branded-smart-constructor
title: "Đo tổng hợp: branded type & smart constructor"
summary: "Bài chốt cụm 5, code có chấm điểm sống: tự thiết kế MỘT branded type mới cùng smart constructor của nó, rồi dùng kết quả trong một hàm khác đòi ĐÚNG kiểu đã brand — không khái niệm mới, đo khả năng tự dựng cặp brand+constructor từ đầu, không chỉ điền khung có sẵn."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 31
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ts.review-branded-smart-constructor]
requires: [ts.write-smart-constructor]
concepts: [ts.review-branded-smart-constructor]
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
Bài trước, bạn tự viết `taoTuoiHopLe` — cặp brand + smart constructor
ĐẦU TIÊN bạn tự tay dựng, cho số tuổi. Cụm 5 sắp khép: hôm nay tự
thiết kế một cặp HOÀN TOÀN MỚI, rồi DÙNG nó ở một hàm khác đòi đúng
kiểu đã brand — không phải `string`, không phải `number` trần.
::::

::::explain{#on-lai-brand-smart-constructor}
```typescript
type Email = string & { readonly __brand: "Email" };

function taoEmail(s: string): Email | null {
  if (!s.includes("@")) return null;
  return s as Email;
}

function guiEmail(email: Email): string {
  return "đã gửi tới " + email;
}

const e = taoEmail("an@vidu.com");
if (e !== null) {
  console.log(guiEmail(e));
}
```

```text
đã gửi tới an@vidu.com
```

Ba mảnh ghép của cụm 5, ghép lại thành MỘT chuỗi thật: `Email` là một
`string` gắn thêm nhãn chỉ tồn tại lúc biên dịch (bài 27), `taoEmail`
là CỬA DUY NHẤT để có một giá trị kiểu `Email` — tự kiểm `s`, chỉ ép
kiểu (`as Email`) nếu hợp lệ (bài 29–30). Nhưng điểm MẤU CHỐT nằm ở
`guiEmail`: tham số của nó khai `email: Email`, KHÔNG phải `email:
string`.

Nhớ lại bài 26 — `guiEmail(email: string, tenNguoiDung: string)` với
CẢ HAI tham số đều `string`, đảo thứ tự biên dịch được bình thường, vì
trình biên dịch chỉ thấy "một string". Giờ `guiEmail` đòi đúng `Email`
— gọi `guiEmail("an@vidu.com")` (một `string` trần, chưa qua
`taoEmail`) bị TypeScript CHẶN NGAY, mã TS2345, dòng báo lỗi ĐÃ ĐO
THẬT: `Argument of type 'string' is not assignable to parameter of
type 'Email'`. Rủi ro truyền nhầm ở bài 26 giờ bị chặn KHÔNG PHẢI vì
bạn cẩn thận đặt tên biến, mà vì CẤU TRÚC KIỂU không cho phép viết ra
lời gọi sai.
::::

::::example{#brand-cho-ca-number}
Brand không chỉ gắn lên `string` — gắn lên `number` y hệt, và một hàm
tiêu thụ vẫn đòi đúng kiểu đã brand:

```typescript title=readonly
type SoLuongDatHang = number & { readonly __brand: "SoLuongDatHang" };

function taoSoLuong(n: number): SoLuongDatHang | null {
  if (!Number.isInteger(n) || n <= 0) return null;
  return n as SoLuongDatHang;
}

function tinhTongTien(soLuong: SoLuongDatHang, donGia: number): number {
  return soLuong * donGia;
}

const sl1 = taoSoLuong(3);
const sl2 = taoSoLuong(-1);

if (sl1 !== null) {
  console.log(tinhTongTien(sl1, 50000));
}
console.log(sl2);
```

```text title=readonly
150000
null
```

`taoSoLuong(3)` hợp lệ (số nguyên dương) nên `sl1` mang giá trị `3`,
`tinhTongTien(sl1, 50000)` trả về `150000`. `taoSoLuong(-1)` KHÔNG hợp
lệ (âm) nên `sl2` là `null` — đúng khuôn smart constructor đã học,
chỉ khác miền dữ liệu là `number` thay vì `string`. `tinhTongTien` vẫn
đòi đúng `SoLuongDatHang`, không nhận `number` trần nào khác.
::::

::::predict{#doan-thieu-kiem-tra-null commitOnce}
```typescript
type MaSoNhanVien = string & { readonly __brand: "MaSoNhanVien" };

function taoMaSoNhanVien(s: string): MaSoNhanVien | null {
  if (s.length !== 5) return null;
  return s as MaSoNhanVien;
}

function chamCong(ma: MaSoNhanVien): string {
  return "đã chấm công cho " + ma;
}

const ma = taoMaSoNhanVien("NV001");
console.log(chamCong(ma));
```

Dòng cuối cùng xảy ra chuyện gì?

:::opt{correct}
Máy báo lỗi biên dịch tại dòng `console.log(chamCong(ma))` — `ma` có
kiểu `MaSoNhanVien | null` (vì `taoMaSoNhanVien` có thể trả `null`),
không khớp tham số `MaSoNhanVien` mà `chamCong` đòi (thiếu kiểm tra
`null` trước khi dùng)
:::

:::opt
In ra `đã chấm công cho NV001` — vì `"NV001"` hợp lệ (đúng 5 ký tự)
nên `ma` chắc chắn không phải `null`
::why
Gần đúng ở việc bạn kiểm tra đúng `"NV001"` có 5 ký tự, nên LÚC CHẠY
`taoMaSoNhanVien` thật sự sẽ trả về một `MaSoNhanVien` hợp lệ, không
phải `null` — quan sát về mặt DỮ LIỆU đó đúng.

Chỗ lệch: TypeScript không "biết" giá trị CỤ THỂ của chuỗi
`"NV001"` lúc biên dịch — nó chỉ nhìn vào KIỂU TRẢ VỀ đã khai của
`taoMaSoNhanVien`, là `MaSoNhanVien | null`. Kiểu đó KHÔNG khớp tham
số `MaSoNhanVien` (không `null`) mà `chamCong` đòi, nên trình biên
dịch CHẶN NGAY, bất kể giá trị runtime thực tế có hợp lệ hay không.
::
:::

:::opt
Biên dịch được bình thường, nhưng CHẠY thì ném lỗi runtime vì thiếu
kiểm tra `null`
::why
Gần đúng ở việc bạn nhớ đúng: thiếu kiểm tra `null` LÀ một rủi ro
thật — nếu `taoMaSoNhanVien` trả `null` mà mã cứ dùng tiếp như một
`MaSoNhanVien` bình thường, hậu quả đúng kiểu lỗi runtime.

Chỗ lệch: TypeScript không đợi tới lúc CHẠY để lỗi đó hiện ra — kiểm
tra kiểu TĨNH (đã học từ T4.0a) phát hiện `MaSoNhanVien | null` không
khớp `MaSoNhanVien` NGAY lúc biên dịch, chặn đứng trước khi chương
trình chạy dòng nào. Không có phiên bản "chạy rồi mới lỗi" ở đây.
::
:::
::::

::::code{#tao_so_dien_thoai}
Thiết kế branded type `SoDienThoaiHopLe` (chuỗi đúng 10 ký tự, bắt
đầu bằng `"0"`) và smart constructor của nó — `taoSoDienThoai` đã có
đủ logic kiểm tra, chỉ thiếu phép ép kiểu sang ĐÚNG brand đã khai ở
dòng `type`.

```typescript title=starter
type SoDienThoaiHopLe = string & { readonly __brand: "SoDienThoaiHopLe" };

function taoSoDienThoai(s: string): SoDienThoaiHopLe | null {
  if (s.length !== 10 || s[0] !== "0") return null;
  return s as ___;
}

function goiDien(sdt: SoDienThoaiHopLe): string {
  return "đang gọi " + sdt;
}

const sdt = taoSoDienThoai("0912345678");
if (sdt !== null) {
  console.log(goiDien(sdt));
}
```

```typescript title=solution
type SoDienThoaiHopLe = string & { readonly __brand: "SoDienThoaiHopLe" };

function taoSoDienThoai(s: string): SoDienThoaiHopLe | null {
  if (s.length !== 10 || s[0] !== "0") return null;
  return s as SoDienThoaiHopLe;
}

function goiDien(sdt: SoDienThoaiHopLe): string {
  return "đang gọi " + sdt;
}

const sdt = taoSoDienThoai("0912345678");
if (sdt !== null) {
  console.log(goiDien(sdt));
}
```

```typescript title=test
const sdt1 = taoSoDienThoai("0912345678");
if (sdt1 === null) throw new Error("taoSoDienThoai(\"0912345678\") phải hợp lệ — đang trả về null");
const kq = goiDien(sdt1);
if (kq !== "đang gọi 0912345678") throw new Error("goiDien phải trả về \"đang gọi 0912345678\" — đang là " + kq);

const sdt2 = taoSoDienThoai("12345");
if (sdt2 !== null) throw new Error("taoSoDienThoai(\"12345\") phải trả về null vì không đủ 10 ký tự — đang là " + JSON.stringify(sdt2));

const sdt3 = taoSoDienThoai("1912345678");
if (sdt3 !== null) throw new Error("taoSoDienThoai(\"1912345678\") phải trả về null vì không bắt đầu bằng 0 — đang là " + JSON.stringify(sdt3));
```

:::hints
- kind: attention
  body: "Chỗ trống nằm ở phép ép kiểu `as` bên trong taoSoDienThoai — đây là ĐÚNG MỘT chỗ được phép ép sang brand, và nó phải ép sang đúng tên kiểu đã khai ở dòng type, không phải một kiểu nào khác."
- kind: strategy
  body: "taoSoDienThoai khai trả về SoDienThoaiHopLe | null. Sau khi s đã qua kiểm tra length và ký tự đầu, dòng return phải ép s sang ĐÚNG kiểu SoDienThoaiHopLe đã khai ở dòng type đầu file."
- kind: one-line
  body: "Chỗ trống là: SoDienThoaiHopLe"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "đang gọi 0912345678"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một brand mới, một smart constructor mới, một hàm tiêu thụ đòi đúng
kiểu — bạn vừa tự dựng trọn cặp, không chỉ điền vào khung có sẵn. Cụm
5 khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm này khép lại: `string`/`number` cho mọi thứ dễ truyền nhầm (bài
26), branded type gắn nhãn chỉ tồn tại lúc biên dịch (bài 27–28),
smart constructor là CỬA DUY NHẤT để có một giá trị đã brand (bài
29–30), và hôm nay bạn ghép trọn cặp đó vào một hàm tiêu thụ thật.

Nhưng mọi kiểu dữ liệu học từ đầu track — `Hinh`, `TrangThaiDonHang`,
`Email` — đều "phẳng": field của một biến thể chỉ chứa giá trị cơ bản
(`string`, `number`, một literal). Nếu một biến thể lại có field mang
CHÍNH kiểu đó — một biểu thức số học chứa một biểu thức số học khác
bên trong nó — TypeScript có cho khai kiểu như vậy không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
