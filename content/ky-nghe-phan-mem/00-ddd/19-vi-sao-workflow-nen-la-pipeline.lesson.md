---
id: ky-nghe-phan-mem.ddd.vi-sao-workflow-nen-la-pipeline
title: "Vì sao workflow nên là Pipeline — đối lập imperative dài dòng"
summary: "Đối lập MỘT hàm workflow dài (if/else lồng nhau, khó test vì phải mock TẤT CẢ cùng lúc) với ý tưởng PIPELINE: mỗi bước MỘT hàm thuần, dữ liệu \"chảy\" qua từng trạm, test RIÊNG từng trạm không cần mock."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 19
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [ddd.why-pipeline]
requires: [ddd.aggregate-root]
concepts: [ddd.why-pipeline]
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
Cụm mới. Bạn có Domain Model hoàn chỉnh (cụm 3). Giờ cần GHÉP nhiều
bước xử lý thành MỘT workflow — nhưng ghép KIỂU GÌ?
::::

::::explain{#imperative-vs-pipeline}
Cách VIẾT TỰ NHIÊN NHẤT: MỘT hàm dài, `if`/`else` LỒNG NHAU:

```typescript
function xuLyDonHangImperative(maKhachHang: string, tongTien: number): string {
  if (maKhachHang.trim() === "") {
    return "lỗi: thiếu mã khách hàng";
  } else {
    if (tongTien <= 0) {
      return "lỗi: tổng tiền không hợp lệ";
    } else {
      const thue = tongTien * 0.1;
      const tongCoThue = tongTien + thue;
      return `đơn hợp lệ, tổng cộng ${tongCoThue}`;
    }
  }
}

console.log(xuLyDonHangImperative("KH-01", 100000));
console.log(xuLyDonHangImperative("", 100000));
```

```text
đơn hợp lệ, tổng cộng 110000
lỗi: thiếu mã khách hàng
```

Hàm này LÀM ĐÚNG — NHƯNG muốn TEST RIÊNG "quy tắc tính thuế" mà KHÔNG
kèm hai bước validate TRƯỚC nó là KHÔNG THỂ — MỌI logic đều LỒNG chung
vào MỘT thân hàm, càng nhiều bước càng LỒNG SÂU (`if` trong `if` trong
`if`...).

Ý TƯỞNG **Pipeline**: mỗi bước là MỘT hàm THUẦN, dữ liệu "CHẢY" qua
từng trạm — test được TỪNG trạm RIÊNG, độc lập với các trạm khác:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }

function validateDonHang(maKhachHang: string, tongTien: number): Result<{ maKhachHang: string; tongTien: number }, string> {
  if (maKhachHang.trim() === "") return loi("thiếu mã khách hàng");
  if (tongTien <= 0) return loi("tổng tiền không hợp lệ");
  return ok({ maKhachHang, tongTien });
}

function tinhGia(dh: { maKhachHang: string; tongTien: number }): number {
  return dh.tongTien + dh.tongTien * 0.1;
}

const kq = validateDonHang("KH-01", 100000);
if (kq.kind === "ok") {
  console.log(tinhGia(kq.giaTri));
}
```

```text
110000
```

`tinhGia` TEST ĐƯỢC RIÊNG (chỉ cần một object `{maKhachHang, tongTien}`
bất kỳ, KHÔNG cần chạy `validateDonHang` trước) — `validateDonHang`
CŨNG test được RIÊNG. Hai TRẠM ĐỘC LẬP, ghép lại bằng cách LẤY kết quả
trạm TRƯỚC làm đầu vào trạm SAU.
::::

::::example{#kho-mock-vs-de-test}
Điểm KHÁC BIỆT thực tế: muốn kiểm "tổng tiền `100000` cộng thuế ra
`110000`" TRÊN CÁCH imperative, PHẢI đi qua CẢ HÀM (bao gồm cả hai
bước validate) — trên cách pipeline, CHỈ CẦN gọi `tinhGia` MỘT MÌNH:

```typescript title=readonly
function tinhGia(dh: { maKhachHang: string; tongTien: number }): number {
  return dh.tongTien + dh.tongTien * 0.1;
}

// Test CHỈ tinhGia -- không cần validate, không cần dữ liệu "hợp lệ toàn phần"
console.log(tinhGia({ maKhachHang: "bất kỳ", tongTien: 200000 }));
console.log(tinhGia({ maKhachHang: "", tongTien: 50000 })); // maKhachHang rỗng -- KHÔNG quan trọng với tinhGia
```

```text title=readonly
220000
55000
```

`tinhGia` KHÔNG QUAN TÂM `maKhachHang` có hợp lệ hay không — nó CHỈ
cần MỘT object có `tongTien`. Test được NGAY LẬP TỨC, không phải dựng
"một đơn hàng ĐẦY ĐỦ hợp lệ" trước. Đây LÀ lợi ích thực tế của tách
pipeline: mỗi trạm chỉ cần ĐÚNG dữ liệu NÓ CẦN, không hơn.
::::

::::predict{#doan-tinhgia-doc-lap commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function validateDonHang(maKhachHang: string, tongTien: number): Result<{ maKhachHang: string; tongTien: number }, string> {
  if (maKhachHang.trim() === "") return loi("thiếu mã khách hàng");
  if (tongTien <= 0) return loi("tổng tiền không hợp lệ");
  return ok({ maKhachHang, tongTien });
}

const ketQua = validateDonHang("KH-05", -1000);
console.log(ketQua.kind);
```

`tongTien` truyền vào là `-1000` (số ÂM). Dòng cuối in ra gì?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `validateDonHang` chỉ kiểm tra `maKhachHang` có rỗng hay
không, KHÔNG hề kiểm tra dấu của `tongTien`
::why
Gần đúng ở việc bạn nhớ ĐÚNG BƯỚC ĐẦU TIÊN của `validateDonHang` là
kiểm `maKhachHang` — quan sát về THỨ TỰ kiểm tra đó đúng.

Chỗ lệch: `validateDonHang` có HAI kiểm tra LIÊN TIẾP — SAU khi kiểm
`maKhachHang` (ở đây "KH-05" hợp lệ, KHÔNG rỗng), hàm TIẾP TỤC kiểm
`if (tongTien <= 0) return loi(...)`. `-1000 <= 0` là `true` — hàm trả
về `loi("tổng tiền không hợp lệ")` Ở BƯỚC KIỂM TRA THỨ HAI.
::
:::

:::opt
Máy báo lỗi biên dịch — tham số `tongTien: number` không cho phép
truyền giá trị ÂM
::why
Gần đúng ở việc bạn nghĩ tới ràng buộc "tiền không được âm" — MỘT quy
tắc NGHIỆP VỤ hợp lý (và ĐÚNG là điều `validateDonHang` kiểm tra).

Chỗ lệch: kiểu `number` KHÔNG có khái niệm "âm/dương" ở TẦNG KIỂU —
nó nhận MỌI số thực. Ràng buộc "không âm" là quy tắc NGHIỆP VỤ, kiểm
LÚC CHẠY bằng `if`, không phải giới hạn của kiểu `number` — biên dịch
sạch, `-1000` truyền vào hoàn toàn hợp lệ về mặt kiểu.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Pipeline: mỗi trạm một hàm thuần, test riêng được, không cần dựng dữ
liệu "hợp lệ toàn phần" trước. Bước tiếp theo: công cụ ghép các trạm
lại thành một chuỗi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Ghép nhiều hàm thành một chuỗi — bạn đã học `pipe()` ở đâu đó trước
đây (T4.2). Áp dụng nó vào domain thật trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
