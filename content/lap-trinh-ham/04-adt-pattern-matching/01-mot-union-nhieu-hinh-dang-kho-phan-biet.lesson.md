---
id: lap-trinh-ham.adt-pattern-matching.mot-union-nhieu-hinh-dang-kho-phan-biet
title: "Một union nhiều hình dạng — khó phân biệt LÚC CHẠY"
summary: "type Hinh = Vuong | Tron — có một Hinh, làm sao BIẾT nó là Vuong hay Tron? 'canh' in h chạy được, nhưng cồng kềnh khi union có NHIỀU hơn hai biến thể, và có thể phân loại SAI mà không lỗi gì."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ts.union-ambiguity]
requires: [ts.interface]
concepts: [ts.union-ambiguity]
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
Bạn vừa khép một track ghép hàm bằng Python. Giờ quay lại TypeScript —
với một câu hỏi mới: một giá trị có thể mang NHIỀU HÌNH DẠNG khác nhau
không?
::::

::::explain{#union-hai-hinh-dang}
```typescript
interface Vuong {
  canh: number;
}

interface Tron {
  banKinh: number;
}

type Hinh = Vuong | Tron;

function moTa(h: Hinh): string {
  if ("canh" in h) {
    return "hình vuông cạnh " + h.canh;
  }
  return "hình tròn bán kính " + h.banKinh;
}

console.log(moTa({ canh: 4 }));
console.log(moTa({ banKinh: 3 }));
```

```text
hình vuông cạnh 4
hình tròn bán kính 3
```

`Hinh` là một `Vuong` HOẶC một `Tron` — bạn đã biết `union` này từ
T4.0a. Nhưng có một `h: Hinh` trong tay, làm sao BIẾT nó thuộc biến thể
nào? `"canh" in h` (toán tử `in`, T4.0a's `ts.union-narrowing` đã dạy)
kiểm tra field `canh` CÓ MẶT không — nếu có, TypeScript tự NARROW `h`
thành `Vuong` bên trong nhánh đó, cho phép đọc `h.canh` an toàn.

Cách này CHẠY ĐƯỢC — nhưng chỉ với ĐÚNG hai biến thể, và chỉ vì
`Vuong`/`Tron` không CÓ field trùng tên nào. Track này sẽ hỏi: cách đó
có SỐNG SÓT khi số biến thể tăng lên không?
::::

::::example{#phan-loai-sai-am-tham}
Thêm biến thể THỨ BA (`TamGiacDeu`) vào `Hinh` — nhưng hàm `moTa` VẪN
CHỈ kiểm tra `"canh" in h`, coi MỌI THỨ KHÁC là `Tron`:

```typescript title=readonly
interface Vuong {
  canh: number;
}

interface Tron {
  banKinh: number;
}

interface TamGiacDeu {
  canhTamGiac: number;
}

type Hinh = Vuong | Tron | TamGiacDeu;

function moTa(h: Hinh): string {
  if ("canh" in h) {
    return "hình vuông";
  } else {
    return "hình tròn";
  }
}

console.log(moTa({ canhTamGiac: 5 }));
```

```text title=readonly
hình tròn
```

Truyền một `TamGiacDeu` — TypeScript BIÊN DỊCH bình thường (không lỗi
gì!), chương trình CHẠY bình thường (không crash gì!) — nhưng kết quả
SAI: một tam giác bị gọi là "hình tròn". Không có TÍN HIỆU nào báo cho
bạn biết có gì đó không ổn — lỗi ẩn HOÀN TOÀN trong logic, chỉ lộ ra
khi ai đó ĐỌC KỸ và nhận ra kết quả vô lý.
::::

::::predict{#doan-phan-loai-sai commitOnce}
```typescript
interface DonChoXuLy {
  ma: string;
}

interface DonDaHuy {
  ma: string;
  lyDoHuy: string;
}

type Don = DonChoXuLy | DonDaHuy;

function trangThai(d: Don): string {
  if ("lyDoHuy" in d) {
    return "đã huỷ";
  } else {
    return "chờ xử lý";
  }
}

console.log(trangThai({ ma: "DH-1", lyDoHuy: "hết hàng" }));
console.log(trangThai({ ma: "DH-2" }));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`đã huỷ` rồi `chờ xử lý`
:::

:::opt
Máy báo lỗi biên dịch ở dòng `trangThai({ ma: "DH-2" })` — thiếu field
`lyDoHuy` so với `DonDaHuy`
::why
Gần đúng ở việc bạn để ý `DonDaHuy` CÓ field `lyDoHuy` mà object này
thiếu — quan sát về CẤU TRÚC field đó đúng.

Chỗ lệch: `{ ma: "DH-2" }` khớp CHÍNH XÁC hình dạng `DonChoXuLy` (chỉ
cần field `ma`) — nó là một `Don` HỢP LỆ, không phải lỗi. `Don` là
UNION của hai interface, một giá trị chỉ cần khớp MỘT trong hai, không
cần khớp CẢ HAI.
::
:::

:::opt
`đã huỷ` rồi `đã huỷ` — vì `"lyDoHuy" in d` luôn trả `true` một khi
TypeScript đã xác nhận `d` thuộc kiểu `Don`
::why
Gần đúng ở việc bạn tin kết quả DÒNG ĐẦU đúng (`"đã huỷ"`) — dòng đó
đúng thật, object đó CÓ `lyDoHuy`.

Chỗ lệch: `in` là một PHÉP KIỂM TRA THẬT LÚC CHẠY, không phải một hằng
số cố định — nó kiểm TỪNG object RIÊNG. `{ ma: "DH-2" }` (dòng 2) KHÔNG
có field `lyDoHuy`, nên `"lyDoHuy" in d` trả `false` cho object đó,
rơi vào nhánh `else` ("chờ xử lý").
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`in` phân biệt được — nhưng chỉ khi bạn NHỚ kiểm ĐỦ mọi biến thể, và
kiểm ĐÚNG field. Không có gì buộc bạn phải làm đúng cả hai điều đó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vấn đề không phải `in` "sai" — nó là một công cụ hợp lệ. Vấn đề là:
không có gì trong `Vuong`/`Tron`/`TamGiacDeu` NÓI RÕ "tôi là biến thể
nào" — bạn phải ĐOÁN qua việc field nào có mặt.

Nếu MỖI biến thể tự MANG một cái NHÃN rõ ràng — một field nói thẳng
"tôi là Vuong", "tôi là Tron" — thì sao?
::::

::::checkpoint{mastery=0.8}
::::
