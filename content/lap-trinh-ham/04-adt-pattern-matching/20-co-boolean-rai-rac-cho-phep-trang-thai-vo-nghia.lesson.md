---
id: lap-trinh-ham.adt-pattern-matching.co-boolean-rai-rac-cho-phep-trang-thai-vo-nghia
title: "Cờ boolean/optional rời rạc — cho phép trạng thái VÔ NGHĨA"
summary: "interface TrangThai { dangTai: boolean; loi: string | null; duLieu: string | null } — TypeScript biên dịch được { dangTai: true, loi: \"lỗi\", duLieu: \"có dữ liệu\" } dù tổ hợp đó VÔ NGHĨA về nghiệp vụ: không thể VỪA đang tải VỪA có lỗi VỪA có dữ liệu, nhưng không lỗi biên dịch nào ngăn được tổ hợp phi lý này."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 20
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ts.boolean-flags-allow-illegal-states]
requires: [ts.review-sum-product]
concepts: [ts.boolean-flags-allow-illegal-states]
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
Cụm trước khép lại: bạn tự ghép sum-of-products cho một bài toán MỚI,
không cần khuôn có sẵn. Cụm này lật ngược câu hỏi — nếu KHÔNG dùng sum
type, chỉ ghép vài cờ boolean/optional RỜI RẠC vào một product type,
chuyện gì xảy ra?
::::

::::explain{#co-roi-rac-cho-phep-vo-nghia}
```typescript
interface TrangThai {
  dangTai: boolean;
  loi: string | null;
  duLieu: string | null;
}

const bat: TrangThai = { dangTai: true, loi: "lỗi", duLieu: "có dữ liệu" };
console.log(bat.dangTai);
console.log(bat.loi);
console.log(bat.duLieu);
```

```text
true
lỗi
có dữ liệu
```

`TrangThai` là một `interface` — một PRODUCT TYPE, khái niệm cụm trước
đã đặt tên — CÓ CẢ BA trường CÙNG LÚC: `dangTai` (boolean), `loi`
(string hoặc null), `duLieu` (string hoặc null). Ba cờ này được nghĩ ra
để mô tả BA trạng thái tải dữ liệu tách biệt trong đầu người viết:
"đang tải" (chỉ `dangTai` true), "lỗi" (chỉ `loi` có giá trị), "thành
công" (chỉ `duLieu` có giá trị).

Nhưng TypeScript không biết Ý ĐỊNH đó — nó chỉ kiểm TỪNG trường ĐÚNG
kiểu đã khai, ĐỘC LẬP với nhau. Gán `{ dangTai: true, loi: "lỗi",
duLieu: "có dữ liệu" }` — cả ba trường đúng kiểu riêng của nó (boolean
đúng, string đúng, string đúng) — biên dịch TRÔI CHẢY, không một cảnh
báo. Nhưng về NGHIỆP VỤ, tổ hợp này VÔ NGHĨA: không thể VỪA đang tải
(`dangTai` true) VỪA đã có lỗi (`loi` khác null) VỪA đã có dữ liệu
(`duLieu` khác null) — ba điều đó đáng lẽ LOẠI TRỪ NHAU. Không lỗi
biên dịch nào, không cảnh báo nào, không CHẶN nào ngăn tổ hợp phi lý
này — chỉ vì mỗi trường riêng lẻ đúng kiểu của nó, TypeScript coi cả
object là HỢP LỆ.
::::

::::example{#hien-thi-lang-tham}
Nguy hiểm không chỉ nằm ở việc BIÊN DỊCH được — mà ở việc chương trình
CHẠY BÌNH THƯỜNG, không crash gì, chỉ ÂM THẦM bỏ qua thông tin:

```typescript title=readonly
interface TrangThai {
  dangTai: boolean;
  loi: string | null;
  duLieu: string | null;
}

function hienThi(t: TrangThai): string {
  if (t.dangTai) {
    return "Đang tải...";
  }
  if (t.loi !== null) {
    return "Lỗi: " + t.loi;
  }
  return "Dữ liệu: " + t.duLieu;
}

const bat: TrangThai = { dangTai: true, loi: "lỗi mạng", duLieu: "kết quả cũ" };
console.log(hienThi(bat));
```

```text title=readonly
Đang tải...
```

`bat` mang CẢ hai tín hiệu khác `dangTai`: một thông điệp lỗi THẬT
(`"lỗi mạng"`) và dữ liệu THẬT (`"kết quả cũ"`, có lẽ cache từ lần tải
trước). Nhưng `hienThi` kiểm `dangTai` TRƯỚC TIÊN, thấy `true`, trả về
ngay `"Đang tải..."` — thông điệp lỗi và dữ liệu cache bị BỎ QUA HOÀN
TOÀN, không một dòng log, không một ngoại lệ. Người dùng nhìn màn hình
thấy "Đang tải..." mãi, không biết có lỗi mạng đang chờ, cũng không
biết có dữ liệu cũ có thể hiển thị tạm.
::::

::::predict{#doan-hien-thi commitOnce}
Đổi tổ hợp trường — `dangTai` giờ `false`, còn `loi` VÀ `duLieu` đều
khác `null`:

```typescript
interface TrangThai {
  dangTai: boolean;
  loi: string | null;
  duLieu: string | null;
}

function hienThi(t: TrangThai): string {
  if (t.dangTai) {
    return "Đang tải...";
  }
  if (t.loi !== null) {
    return "Lỗi: " + t.loi;
  }
  return "Dữ liệu: " + t.duLieu;
}

const vo_nghia: TrangThai = { dangTai: false, loi: "hết hạn phiên", duLieu: "dữ liệu cache" };
console.log(hienThi(vo_nghia));
```

Dòng cuối in ra gì?

:::opt{correct}
`Lỗi: hết hạn phiên`
:::

:::opt
Máy báo lỗi biên dịch — `vo_nghia` "vừa có lỗi vừa có dữ liệu" dù
không đang tải, TypeScript chặn tổ hợp phi lý này
::why
Gần đúng ở việc bạn nhận ra tổ hợp NÀY vô lý về nghiệp vụ (khác
`dangTai`, cả `loi` lẫn `duLieu` đều khác `null` cùng lúc) — quan sát
về TÍNH VÔ LÝ đúng thật.

Chỗ lệch: TypeScript CHỈ kiểm KIỂU từng trường (`dangTai` đúng
boolean, `loi`/`duLieu` đúng `string | null`) — nó KHÔNG kiểm NGHIỆP
VỤ giữa các trường. `vo_nghia` khớp đủ cả ba trường đúng kiểu, biên
dịch bình thường, không lỗi gì — đúng chính vấn đề bài học đang chỉ
ra: không có gì NGĂN tổ hợp phi lý này.
::
:::

:::opt
`Dữ liệu: dữ liệu cache` — vì `duLieu` khác `null` mới là tín hiệu
"thật" nhất trong ba trường
::why
Gần đúng ở việc bạn để ý object CÓ trường `duLieu` khác `null`
(`"dữ liệu cache"`) — quan sát đó đúng, trường đó thực sự mang giá
trị.

Chỗ lệch: `hienThi` kiểm THEO THỨ TỰ VIẾT TRONG MÃ, không theo trường
nào "thật nhất". `t.dangTai` (`false`) bị bỏ qua, rồi `t.loi !== null`
(`true`, vì `loi` là `"hết hạn phiên"`) — nhánh THỨ HAI trả về NGAY,
dòng kiểm `duLieu` phía dưới KHÔNG BAO GIỜ chạy tới. Nhánh nào chạy
TRƯỚC thắng, không phải nhánh nào "đúng nhất" về nghiệp vụ.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cờ boolean/optional rời rạc biên dịch được MỌI tổ hợp giá trị hợp lệ
theo KIỂU — kể cả những tổ hợp không một trạng thái nghiệp vụ THẬT nào
khớp. TypeScript kiểm KIỂU từng trường, không kiểm NGHIỆP VỤ giữa các
trường.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Vấn đề không phải BOOLEAN hay OPTIONAL "sai" — chúng là công cụ hợp
lệ. Vấn đề là: `TrangThai` cho phép BA trường đó biến thiên ĐỘC LẬP,
trong khi nghiệp vụ THẬT chỉ cho phép ĐÚNG MỘT trong ba khả năng tại
một thời điểm — đúng hình dạng một SUM TYPE, không phải PRODUCT TYPE.

Nếu gộp ba khả năng đó lại thành MỘT UNION — mỗi biến thể chỉ MANG
ĐÚNG field nó cần, không thừa field nào để mâu thuẫn — thì tổ hợp vô
lý còn VIẾT RA ĐƯỢC không?
::::

::::checkpoint{mastery=0.8}
::::
