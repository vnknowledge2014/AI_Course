---
id: ky-nghe-phan-mem.mau-tai-cau-truc.observer-la-gi
title: "Observer pattern OOP — Subject giữ danh sách Observer"
summary: "OOP Observer: Subject giữ mảng Observer[] (mỗi Observer là một object có update()), khi state đổi, Subject GỌI update() trên TỪNG Observer. FP: danh sách quan sát CHỈ LÀ một mảng HÀM — đăng ký = push, thông báo = lặp gọi từng hàm. Chưa đóng gói (bài 6 sẽ đóng gói qua factory)."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [mau.observer-oop-shape]
requires: [mau.gate-boss-strategy]
concepts: [mau.observer-oop-shape]
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
Cụm mới. Giá cổ phiếu đổi — NHIỀU màn hình CÙNG muốn biết NGAY. OOP
gọi đây LÀ Observer pattern — hình dạng của nó LÀ gì?
::::

::::explain{#subject-giu-danh-sach}
OOP Observer: một `Subject` (nguồn phát) giữ MỘT mảng
`Observer[]` — MỖI `Observer` LÀ một OBJECT có method `update()`.
Khi state đổi, Subject **LẶP** qua TOÀN BỘ mảng, GỌI `update()` trên
TỪNG cái. "Đăng ký quan tâm" nghĩa LÀ THÊM một Observer VÀO mảng đó.

Về BẢN CHẤT, việc DUY NHẤT một Observer LÀM LÀ "phản ứng khi có sự
kiện" — đó CHÍNH LÀ MỘT HÀM. FP bỏ QUA lớp OBJECT/interface, giữ
LẠI đúng PHẦN CỐT LÕI:

```typescript title=readonly
type NguoiQuanSat = (gia: number) => void;
const danhSachQuanSat: NguoiQuanSat[] = [];

function dangKy(quanSat: NguoiQuanSat): void {
  danhSachQuanSat.push(quanSat);
}

function thongBaoGiaMoi(gia: number): void {
  for (const quanSat of danhSachQuanSat) {
    quanSat(gia);
  }
}

dangKy((gia) => console.log("Nguoi quan sat A thay gia:", gia));
dangKy((gia) => console.log("Nguoi quan sat B thay gia:", gia));

thongBaoGiaMoi(100);
thongBaoGiaMoi(120);
```

```text title=readonly
Nguoi quan sat A thay gia: 100
Nguoi quan sat B thay gia: 100
Nguoi quan sat A thay gia: 120
Nguoi quan sat B thay gia: 120
```

`danhSachQuanSat` LÀ mảng `NguoiQuanSat[]` (mảng HÀM, KHÔNG PHẢI mảng
object `Observer`) — `dangKy` chỉ `push`, `thongBaoGiaMoi` chỉ LẶP VÀ
GỌI. KHÔNG `interface Observer { update() }`, KHÔNG `class`. Ở ĐÂY
`danhSachQuanSat` LỘ RA NGOÀI (bất kỳ đoạn code nào CŨNG sửa được
trực tiếp) — bài 6 sẽ ĐÓNG GÓI nó qua một nhà máy (factory), giống
kỹ thuật `taoKhoHang` đã học bài 3.
::::

::::example{#thu-tu-thong-bao-theo-thu-tu-dang-ky}
Thứ tự GỌI observer LUÔN khớp thứ tự **ĐĂNG KÝ** (mảng `push` vào
CUỐI, `for...of` duyệt TỪ ĐẦU) — KHÔNG có gì "đảo ngược":

```typescript title=readonly
type NguoiQuanSat = (gia: number) => void;
const danhSach: NguoiQuanSat[] = [];
function dangKy(q: NguoiQuanSat): void { danhSach.push(q); }
function thongBao(gia: number): void {
  for (const q of danhSach) q(gia);
}

const nhat: number[] = [];
dangKy((gia) => nhat.push(gia * 2));
dangKy((gia) => nhat.push(gia + 1));

thongBao(10);
console.log(nhat);
```

```text title=readonly
[20,11]
```

Observer ĐẦU TIÊN đăng ký (`gia * 2`) chạy TRƯỚC, ĐẨY `20` vào `nhat`;
observer THỨ HAI (`gia + 1`) chạy SAU, ĐẨY `11`. Kết quả `[20, 11]` —
ĐÚNG thứ tự đăng ký, KHÔNG PHẢI ngược lại.
::::

::::predict{#doan-thu-tu-dang-ky commitOnce}
```typescript
type NguoiQuanSat = (gia: number) => void;
const danhSach: NguoiQuanSat[] = [];
function dangKy(q: NguoiQuanSat): void { danhSach.push(q); }
function thongBao(gia: number): void {
  for (const q of danhSach) q(gia);
}

const ghi: string[] = [];
dangKy((gia) => ghi.push(`A:${gia}`));
dangKy((gia) => ghi.push(`B:${gia}`));
dangKy((gia) => ghi.push(`C:${gia}`));

thongBao(7);
console.log(ghi);
```

Dòng cuối in ra gì?

:::opt{correct}
`["A:7","B:7","C:7"]`
:::

:::opt
`["C:7","B:7","A:7"]` — vì `push` LUÔN thêm vào ĐẦU mảng (giống một
NGĂN XẾP — cái mới NHẤT lên TRƯỚC), nên quan sát viên đăng ký SAU
CÙNG (`C`) chạy TRƯỚC TIÊN
::why
Gần đúng ở việc bạn nhớ ĐÚNG có MỘT cấu trúc dữ liệu "mới nhất lên
trước" (ngăn xếp/stack) TỒN TẠI trong lập trình — quan sát ĐÓ không
sai VỀ MẶT KHÁI NIỆM.

Chỗ lệch: `Array.prototype.push` KHÔNG hoạt động NHƯ MỘT ngăn xếp
"thêm vào ĐẦU" — nó thêm vào **CUỐI** mảng, VÀ `for...of` duyệt mảng
**TỪ ĐẦU TỚI CUỐI** (thứ tự chỉ số `0, 1, 2, ...`). `A` đăng ký ĐẦU
TIÊN nằm Ở chỉ số `0` (đầu mảng), `C` đăng ký SAU CÙNG nằm Ở chỉ số
CUỐI — `for...of` gặp `A` TRƯỚC. Kết quả LUÔN khớp thứ tự **ĐĂNG KÝ**,
không đảo ngược.
::
:::

:::opt
Máy báo lỗi biên dịch — `dangKy` nhận tham số kiểu `NguoiQuanSat`
nhưng BA lời gọi TRUYỀN VÀO ba HÀM MŨI TÊN (arrow function) KHÁC
NHAU, TypeScript yêu cầu MỌI lời gọi `dangKy` phải truyền CÙNG MỘT
tham chiếu hàm
::why
Gần đúng ở việc bạn để ý BA hàm mũi tên `(gia) => ghi.push(...)` LÀ
BA GIÁ TRỊ hàm khác nhau (khác nội dung thân hàm) — một quan sát ĐÚNG
về mặt CÚ PHÁP.

Chỗ lệch: `dangKy` chỉ đòi tham số khớp KIỂU `NguoiQuanSat` (`(gia:
number) => void`) — KHÔNG đòi "CÙNG một tham chiếu". MỌI hàm khớp
CHỮ KÝ đó ĐỀU hợp lệ, dù nội dung THÂN hàm khác NHAU HOÀN TOÀN. Ba
lời gọi `dangKy` VỚI ba hàm KHÁC NHAU biên dịch SẠCH — đây CHÍNH LÀ
lý do NHIỀU bên ĐỘC LẬP đăng ký được VÀO CÙNG một danh sách.
::
:::
::::

::::code{#viet_dang_ky_thong_bao}
Viết `dangKy` (thêm observer vào danh sách) VÀ `thongBaoGiaMoi` (gọi
TỪNG observer đã đăng ký).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type NguoiQuanSat = (gia: number) => void;
const danhSachQuanSat: NguoiQuanSat[] = [];

function dangKy(quanSat: NguoiQuanSat): void {
  ___;
}

function thongBaoGiaMoi(gia: number): void {
  for (const quanSat of danhSachQuanSat) {
    ___;
  }
}

const nhanDuoc: number[] = [];
dangKy((gia) => nhanDuoc.push(gia));
thongBaoGiaMoi(42);
assertEqual(nhanDuoc.length, 1, "mot quan sat nhan duoc mot lan goi");
assertEqual(nhanDuoc[0], 42, "gia nhan duoc dung");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type NguoiQuanSat = (gia: number) => void;
const danhSachQuanSat: NguoiQuanSat[] = [];

function dangKy(quanSat: NguoiQuanSat): void {
  danhSachQuanSat.push(quanSat);
}

function thongBaoGiaMoi(gia: number): void {
  for (const quanSat of danhSachQuanSat) {
    quanSat(gia);
  }
}

const nhanDuoc: number[] = [];
dangKy((gia) => nhanDuoc.push(gia));
thongBaoGiaMoi(42);
assertEqual(nhanDuoc.length, 1, "mot quan sat nhan duoc mot lan goi");
assertEqual(nhanDuoc[0], 42, "gia nhan duoc dung");
```

```typescript title=test
const nhanDuoc2: number[] = [];
const nhanDuoc3: number[] = [];
dangKy((gia) => nhanDuoc2.push(gia * 10));
dangKy((gia) => nhanDuoc3.push(gia - 1));
thongBaoGiaMoi(5);
assertEqual(nhanDuoc2[nhanDuoc2.length - 1], 50, "quan sat thu hai nhan dung gia nhan 10");
assertEqual(nhanDuoc3[nhanDuoc3.length - 1], 4, "quan sat thu ba nhan dung gia tru 1");
assertEqual(danhSachQuanSat.length, 3, "tong so quan sat da dang ky la 3");
```

:::hints
- kind: attention
  body: "dangKy: THÊM quanSat vào danhSachQuanSat. thongBaoGiaMoi: GỌI quanSat (như một hàm bình thường) với gia, cho TỪNG phần tử trong vòng lặp."
- kind: strategy
  body: "danhSachQuanSat.push(quanSat) : quanSat(gia) — thêm vào mảng, và gọi hàm."
- kind: one-line
  body: '___ (dangKy) = danhSachQuanSat.push(quanSat)\n___ (thongBaoGiaMoi) = quanSat(gia)'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Observer = mảng hàm + push (đăng ký) + lặp gọi (thông báo). Danh sách
ĐANG LỘ RA NGOÀI — bài tiếp theo đóng gói nó vào một nhà máy sạch sẽ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`danhSachQuanSat` LÀ một biến module-level — BẤT KỲ đoạn code nào
CŨNG sửa được trực tiếp (kể cả xoá SẠCH mảng). `taoKhoHang` (bài 3)
đã giải quyết vấn đề TƯƠNG TỰ cho tồn kho bằng cách nào?
::::

::::checkpoint{mastery=0.8}
::::
