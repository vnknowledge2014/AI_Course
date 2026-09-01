---
id: lap-trinh-ham.adt-pattern-matching.refactor-mot-vi-du-that-loading-ui
title: "Refactor một ví dụ thật: trạng thái tải dữ liệu UI"
summary: "Một hàm hienThi(t: TrangThai) xử lý cả ba biến thể qua switch trên kind, đối chiếu TRỰC TIẾP với phiên bản cũ dùng cờ boolean phải viết if (dangTai) ... else if (loi !== null) ... else if (duLieu !== null) ... else — dài, dễ quên nhánh, và TypeScript không kiểm được tính đủ nhánh vì đó không phải discriminated union."
locale: vi
track: lap-trinh-ham
module: adt-pattern-matching
order: 22
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [ts.refactor-real-loading-state]
requires: [ts.union-makes-illegal-states-unrepresentable]
concepts: [ts.refactor-real-loading-state]
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
Bài trước bạn đã gộp một tập cờ boolean/optional rời rạc thành MỘT
union — trạng thái vô nghĩa không còn viết ra được nữa. Nhưng đó là ví
dụ nhỏ. Hôm nay: một ví dụ THẬT — trạng thái tải dữ liệu của một UI —
đối chiếu TRỰC TIẾP hai cách viết trên CÙNG một bài toán.
::::

::::explain{#hien-thi-tren-union}
```typescript
type TrangThai =
  | { kind: "dangTai" }
  | { kind: "loi"; thongDiep: string }
  | { kind: "thanhCong"; duLieu: string };

function hienThi(t: TrangThai): string {
  switch (t.kind) {
    case "dangTai":
      return "Đang tải...";
    case "loi":
      return "Lỗi: " + t.thongDiep;
    case "thanhCong":
      return "Dữ liệu: " + t.duLieu;
  }
}

console.log(hienThi({ kind: "dangTai" }));
console.log(hienThi({ kind: "loi", thongDiep: "mất mạng" }));
console.log(hienThi({ kind: "thanhCong", duLieu: "Xin chào" }));
```

```text
Đang tải...
Lỗi: mất mạng
Dữ liệu: Xin chào
```

`TrangThai` là một union BA biến thể — đúng ba trạng thái một UI tải
dữ liệu THẬT SỰ có thể ở: đang tải, lỗi, hoặc đã có dữ liệu. Hàm
`hienThi` dùng `switch (t.kind)` (đã học ở bài 4) — mỗi `case` tự
NARROW `t` về đúng biến thể, cho phép đọc `t.thongDiep` hay `t.duLieu`
an toàn mà không cần ép kiểu.

Quên một `case` trong `switch` này — ví dụ bỏ hẳn `case "loi"` —
TypeScript báo lỗi NGAY LÚC BIÊN DỊCH (mã TS2366: hàm thiếu `return` ở
cuối), vì `t.kind` vẫn còn khả năng là `"loi"` mà không nhánh nào xử
lý. Bạn KHÔNG THỂ quên âm thầm một trạng thái.

Còn thử viết một `TrangThai` VỪA đang tải VỪA có lỗi — ví dụ
`{ kind: "dangTai", thongDiep: "mất mạng", duLieu: "Xin chào" }` — thì
sao? TypeScript CHẶN NGAY (mã TS2353): field `thongDiep` không thuộc
về biến thể `{ kind: "dangTai" }`. Trạng thái vô nghĩa đó KHÔNG VIẾT RA
ĐƯỢC — đúng điều bài trước đã hứa.
::::

::::example{#phien-ban-co-truoc}
So trực tiếp với cách viết CŨ — trước khi có bài 20/21 — cho ĐÚNG bài
toán này, dùng ba field rời rạc `dangTai`/`loi`/`duLieu`:

```typescript title=readonly
interface TrangThaiCu {
  dangTai: boolean;
  loi: string | null;
  duLieu: string | null;
}

function hienThiCu(t: TrangThaiCu): string {
  if (t.dangTai) {
    return "Đang tải...";
  } else if (t.loi !== null) {
    return "Lỗi: " + t.loi;
  } else if (t.duLieu !== null) {
    return "Dữ liệu: " + t.duLieu;
  } else {
    return "Không rõ trạng thái";
  }
}

console.log(hienThiCu({ dangTai: true, loi: null, duLieu: null }));
console.log(hienThiCu({ dangTai: false, loi: "mất mạng", duLieu: null }));
console.log(hienThiCu({ dangTai: false, loi: null, duLieu: "Xin chào" }));
```

```text title=readonly
Đang tải...
Lỗi: mất mạng
Dữ liệu: Xin chào
```

Cùng MỘT bài toán, cùng ba kết quả — nhưng `hienThiCu` DÀI hơn (bốn
nhánh `if`/`else if`/`else` so với ba `case` gọn gàng), và có một nhánh
`else` cuối KHÔNG thuộc bài toán chút nào ("Không rõ trạng thái").
Nhánh đó bắt buộc phải có — KHÔNG PHẢI vì TypeScript hiểu bạn đã xét đủ
ba trạng thái nghiệp vụ, mà chỉ vì hàm phải `return` một `string` ở
MỌI nhánh (thiếu nó, TypeScript báo TS2366 y hệt lỗi thiếu `return` ở
`hienThi` — một ràng buộc THUẦN CÚ PHÁP, không liên quan gì đến việc
bạn có quên xử lý `loi` hay `duLieu` hay không).

Xoá hẳn nhánh `else if (t.loi !== null)` — giữ nguyên `else` cuối —
`hienThiCu` VẪN BIÊN DỊCH bình thường, không lỗi gì. Gọi nó với một
đối tượng CÓ `loi` khác `null`, hàm rơi thẳng vào `else` cuối và in ra
"Không rõ trạng thái" thay vì báo lỗi thật — sai HOÀN TOÀN trong im
lặng. TypeScript không biết ba field `dangTai`/`loi`/`duLieu` đại diện
cho BA TRẠNG THÁI LOẠI TRỪ LẪN NHAU, nên không thể kiểm bạn đã xét đủ
chưa — đúng vấn đề `TrangThaiCu` mắc phải mà `TrangThai` (union) không
mắc.
::::

::::predict{#trang-thai-vo-nghia-van-chay commitOnce}
```typescript
interface TrangThaiCu {
  dangTai: boolean;
  loi: string | null;
  duLieu: string | null;
}

function hienThiCu(t: TrangThaiCu): string {
  if (t.dangTai) {
    return "Đang tải...";
  } else if (t.loi !== null) {
    return "Lỗi: " + t.loi;
  } else if (t.duLieu !== null) {
    return "Dữ liệu: " + t.duLieu;
  } else {
    return "Không rõ trạng thái";
  }
}

const voNghia: TrangThaiCu = { dangTai: true, loi: "mất mạng", duLieu: "Xin chào" };
console.log(hienThiCu(voNghia));
```

`voNghia` gán CẢ BA field cùng lúc — vừa đang tải, vừa có lỗi, vừa có
dữ liệu — một trạng thái VÔ LÝ không UI thật nào từng ở đó. Dòng cuối
in ra gì?

:::opt{correct}
`Đang tải...`
:::

:::opt
Máy báo lỗi biên dịch — vì `voNghia` gán cả ba field `dangTai`/`loi`/
`duLieu` cùng lúc, mâu thuẫn logic với nhau
::why
Gần đúng ở việc bạn nhận ra ba field này MÂU THUẪN nhau về Ý NGHĨA
nghiệp vụ (không UI nào vừa đang tải vừa có lỗi vừa có dữ liệu cùng
lúc) — quan sát đó đúng.

Chỗ lệch: TypeScript CHỈ kiểm KIỂU của từng field riêng lẻ (`dangTai`
là `boolean`, `loi` là `string | null`, `duLieu` là `string | null`) —
nó KHÔNG kiểm quan hệ NGỮ NGHĨA giữa các field độc lập với nhau.
`voNghia` gán đúng kiểu cho cả ba, biên dịch trót lọt, không lỗi gì —
đây chính là "trạng thái vô nghĩa VẪN VIẾT RA ĐƯỢC" mà bài 20/21 đã
cảnh báo, khác hẳn `TrangThai` (union) ở bài này, nơi TS2353 chặn ngay.
::
:::

:::opt
`Lỗi: mất mạng` — vì `if`/`else if` sẽ chọn nhánh có điều kiện CỤ THỂ
hơn (`loi !== null` cụ thể hơn `dangTai: true`) trước
::why
Gần đúng ở việc bạn để ý `loi` CÓ giá trị thật ("mất mạng", khác
`null`) — quan sát về DỮ LIỆU của field đó đúng.

Chỗ lệch: `if`/`else if` chạy TUẦN TỰ theo ĐÚNG THỨ TỰ VIẾT trong mã
nguồn, không theo độ "cụ thể" của điều kiện nào cả. Nhánh ĐẦU TIÊN
`t.dangTai` (đúng `true`) khớp NGAY, hàm `return` "Đang tải..." LẬP
TỨC — hai nhánh `else if` phía sau (kiểm `t.loi`, `t.duLieu`) không
bao giờ được xét tới cho lệnh gọi này.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng ba kết quả đúng, nhưng `hienThi` trên union NGẮN hơn, không nhánh
thừa, và CHẶN từ gốc mọi trạng thái vô nghĩa — thay vì âm thầm chạy
sai như `hienThiCu` vừa làm với `voNghia`.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn đã THẤY cuộc refactor này xảy ra — nhưng chỉ ĐỌC nó, chưa TỰ TAY
làm. Gặp một ví dụ MỚI, với những cờ rời rạc KHÁC, bạn có tự viết ra
được union thay thế không?

Bài sau: đến lượt bạn.
::::

::::checkpoint{mastery=0.8}
::::
