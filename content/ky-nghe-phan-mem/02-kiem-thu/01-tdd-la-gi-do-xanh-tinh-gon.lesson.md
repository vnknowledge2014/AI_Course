---
id: ky-nghe-phan-mem.kiem-thu.tdd-la-gi-do-xanh-tinh-gon
title: "TDD là gì — Đỏ, Xanh, Tinh gọn"
summary: "Test-Driven Development: viết \"kịch bản\" (test) TRƯỚC \"buổi diễn\" (code) — Đỏ (test thất bại, chứng minh nó thật sự kiểm được gì) → Xanh (code TỐI THIỂU để qua) → Tinh gọn (cải thiện cấu trúc, test làm lưới an toàn). TDD là phương pháp THIẾT KẾ, không phải viết test sau cho có."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [kt.tdd-cycle]
requires: [ts.adt-gate-boss]
concepts: [kt.tdd-cycle]
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
Track mới. Cách viết TỰ NHIÊN NHẤT: code TRƯỚC, test SAU (nếu có thời
gian). TDD làm NGƯỢC LẠI — vì sao?
::::

::::explain{#do-xanh-tinh-gon}
**Test-Driven Development (TDD)**: viết "KỊCH BẢN" (test) **TRƯỚC**
"BUỔI DIỄN" (code) — MỘT chu trình BA bước:

**Đỏ**: viết test CHO hành vi **CHƯA tồn tại** — test **THẤT BẠI**
(chứng minh test ĐÓ THẬT SỰ kiểm được điều gì đó, KHÔNG PHẢI luôn
"xanh" một cách VÔ NGHĨA). **Xanh**: viết code **TỐI THIỂU** để test
qua — KHÔNG HƠN. **Tinh gọn**: cải thiện CẤU TRÚC code, test LÀM LƯỚI
AN TOÀN (nếu code VẪN đúng, test VẪN xanh):

```typescript
// ĐỎ: hàm CHƯA cài đặt, test THẤT BẠI
function tinhGiaSauGiam(gia: number, phanTramGiam: number): number {
  throw new Error("chua cai dat");
}

try {
  console.log(tinhGiaSauGiam(100, 10));
} catch (e) {
  console.log("DO:", (e as Error).message);
}
```

```text
DO: chua cai dat
```

`tinhGiaSauGiam` **CHỦ ĐỘNG** `throw` — đây LÀ bước **ĐỎ** ĐÚNG NGHĨA:
KHÔNG PHẢI lỗi cần SỬA NGAY, mà LÀ bước ĐẦU TIÊN, CỐ Ý, của quy trình.
Nó CHỨNG MINH test/lời gọi ĐANG chạy TỚI đúng chỗ, VÀ hàm THẬT SỰ
CHƯA làm gì cả — nếu BỎ QUA bước NÀY (viết code XONG rồi mới nghĩ tới
test), KHÔNG BAO GIỜ biết chắc test CÓ THẬT SỰ kiểm được gì hay không.
::::

::::example{#xanh-toi-thieu}
Bước **Xanh**: viết code **TỐI THIỂU** để hành vi MONG MUỐN hoạt
động — KHÔNG thêm tính năng "phòng khi cần sau":

```typescript title=readonly
function tinhGiaSauGiam(gia: number, phanTramGiam: number): number {
  return gia - (gia * phanTramGiam) / 100;
}

console.log(tinhGiaSauGiam(100, 10));
console.log(tinhGiaSauGiam(200, 25));
```

```text title=readonly
90
150
```

`tinhGiaSauGiam(100, 10)`: `100 - (100 * 10) / 100 = 100 - 10 = 90`.
Hàm NÀY **CHỈ** làm ĐÚNG điều CẦN (tính giá sau giảm) — KHÔNG thêm
kiểm tra "giảm giá âm", KHÔNG thêm làm tròn, KHÔNG thêm GÌ chưa được
YÊU CẦU. TDD KHÔNG khuyến khích "viết code THÔNG MINH trước" — nó
khuyến khích viết ĐỦ để test ĐANG CÓ qua, RỒI **THÊM** test MỚI (bài
sau) nếu CẦN hành vi THÊM.
::::

::::predict{#doan-do-la-co-y commitOnce}
```typescript
function tinhThue(thuNhap: number): number {
  throw new Error("chua cai dat");
}

console.log("truoc khi goi ham");
try {
  tinhThue(1000);
  console.log("day la dong KHONG BAO GIO chay toi");
} catch (e) {
  console.log("bat duoc loi:", (e as Error).message);
}
console.log("sau khoi try/catch");
```

Ba dòng `console.log` NÀO thực sự IN RA (theo ĐÚNG thứ tự)?

:::opt{correct}
`"truoc khi goi ham"`, `"bat duoc loi: chua cai dat"`, `"sau khoi try/catch"`
:::

:::opt
CHỈ `"truoc khi goi ham"` — vì `tinhThue` `throw` một lỗi, VÀ MỘT lỗi
KHÔNG BỊ BẮT sẽ dừng TOÀN BỘ chương trình NGAY LẬP TỨC, kể cả những
dòng NẰM TRONG khối `catch`/SAU khối `try`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `tinhThue` **THẬT SỰ** `throw` một lỗi
— quan sát ĐÓ đúng, VÀ đúng LÀ một lỗi KHÔNG BỊ BẮT sẽ dừng chương
trình.

Chỗ lệch: lỗi Ở ĐÂY **CÓ BỊ BẮT** — dòng `tinhThue(1000)` nằm **BÊN
TRONG** khối `try { ... }`, VÀ có MỘT khối `catch (e) { ... }` THEO
SAU. Khi `tinhThue` ném lỗi, luồng chương trình **NHẢY THẲNG** vào
khối `catch` (bỏ qua PHẦN CÒN LẠI của `try`, đúng LÀ dòng
`"day la dong KHONG BAO GIO chay toi"` KHÔNG BAO GIỜ in ra) — NHƯNG
`catch` **BẮT ĐƯỢC** lỗi đó, in `"bat duoc loi: chua cai dat"`, RỒI
chương trình **TIẾP TỤC BÌNH THƯỜNG** SAU khối `try/catch` — dòng
`"sau khoi try/catch"` VẪN chạy TỚI, VẪN in ra.
::
:::

:::opt
Máy báo lỗi biên dịch — `tinhThue` khai kiểu trả về `number`, nhưng
thân hàm CHỈ có `throw`, KHÔNG có `return` nào, TypeScript đòi HÀM
PHẢI trả về ĐÚNG kiểu đã khai
::why
Gần đúng ở việc bạn để ý `tinhThue: (thuNhap: number) => number`
khai RÕ kiểu trả về LÀ `number`, NHƯNG thân hàm KHÔNG có `return`
nào cả — một quan sát ĐÚNG về mặt CẤU TRÚC code.

Chỗ lệch: TypeScript **CÔNG NHẬN** `throw` NHƯ MỘT CÁCH HỢP LỆ để
"kết thúc" một hàm — một hàm CHỈ `throw` (KHÔNG BAO GIỜ trả về BÌNH
THƯỜNG) vẫn THOẢ MÃN chữ ký trả `number`, vì nó ĐƠN GIẢN KHÔNG BAO
GIỜ "trả về sai kiểu" (nó KHÔNG BAO GIỜ trả về CÁI GÌ CẢ theo NHÁNH
đó). Đây CHÍNH LÀ lý do bước ĐỎ dùng được `throw` LÀM "cài đặt tạm"
— biên dịch SẠCH, dù hàm CHƯA làm ĐÚNG việc nó CẦN làm.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đỏ (test thất bại, có chủ đích) → Xanh (code tối thiểu) → Tinh gọn
(cải thiện, test làm lưới an toàn). Bước tiếp theo: sandbox không có
thư viện test nào — tự viết công cụ kiểm tra.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Track này KHÔNG có `node:assert`, jest, hay vitest — vậy "viết test"
Ở ĐÂY nghĩa LÀ gì? Cần CÔNG CỤ gì trước tiên?
::::

::::checkpoint{mastery=0.8}
::::
