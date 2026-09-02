---
id: ky-nghe-phan-mem.ddd.capstone-compose-workflow-day-du
title: "Capstone: Compose Domain Workflow hoàn chỉnh"
summary: "Bài chốt cụm 4: kết hợp mọi kỹ thuật đã học (chainResult, DU lỗi giàu ngữ cảnh, short-circuit) thành MỘT workflow nghiệp vụ thật — xuLyDonHangWorkflow(khachHang, matHang, gioHienTai): Result<DonHang, LoiWorkflow[]>, đọc như quy trình thật: kiểm khách hàng, kiểm mặt hàng, dựng đơn đã kiểm, tính giá (có giảm giá hạng vàng), sinh mã đơn."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 24
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.workflow-capstone]
requires: [ddd.rop-domain-application]
concepts: [ddd.workflow-capstone]
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
Bài chốt cụm 4. Ghép TẤT CẢ kỹ thuật đã học (`chainResult`, lỗi giàu
ngữ cảnh, short-circuit) thành MỘT workflow đọc như quy trình thật.
::::

::::explain{#workflow-hoan-chinh}
Nhiều hàm THUẦN, mỗi hàm MỘT trách nhiệm, nối bằng `chainResult` —
đọc TỪ TRÊN XUỐNG đúng như quy trình nghiệp vụ: kiểm khách hàng, kiểm
mặt hàng, dựng đơn đã kiểm, TÍNH GIÁ (có giảm giá cho khách hàng
HẠNG VÀNG), sinh mã đơn:

```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

type LoiWorkflow =
  | { tag: "loi_khach_hang"; thongDiep: string }
  | { tag: "loi_mat_hang"; thongDiep: string };

type KhachHang = { maKhachHang: string; email: string; hangKhachHang: "thuong" | "vang" };
type MatHang = { maSanPham: string; soLuong: number; donGia: number };
type DonHangDaKiem = { khachHang: KhachHang; matHang: MatHang };
type DonHang = { maDonHang: string; maKhachHang: string; maSanPham: string; soLuong: number; tongTien: number; thoiGianTao: number };

function kiemTraKhachHang(kh: KhachHang): Result<KhachHang, LoiWorkflow[]> {
  if (!kh.email.includes("@")) return loi([{ tag: "loi_khach_hang", thongDiep: "email không hợp lệ" }]);
  return ok(kh);
}
function kiemTraMatHang(mh: MatHang): Result<MatHang, LoiWorkflow[]> {
  if (mh.soLuong <= 0) return loi([{ tag: "loi_mat_hang", thongDiep: "số lượng phải lớn hơn 0" }]);
  return ok(mh);
}
function dungDonHangDaKiem(kh: KhachHang, mh: MatHang): Result<DonHangDaKiem, LoiWorkflow[]> {
  return ok({ khachHang: kh, matHang: mh });
}
function tinhGia(dhDaKiem: DonHangDaKiem): number {
  const tongGoc = dhDaKiem.matHang.soLuong * dhDaKiem.matHang.donGia;
  const giamGia = dhDaKiem.khachHang.hangKhachHang === "vang" ? 0.1 : 0;
  return tongGoc * (1 - giamGia);
}
function sinhMaDonHang(kh: KhachHang, gioHienTai: number): string {
  return `DH-${kh.maKhachHang}-${gioHienTai}`;
}

function xuLyDonHangWorkflow(khachHang: KhachHang, matHang: MatHang, gioHienTai: number): Result<DonHang, LoiWorkflow[]> {
  return chainResult(
    chainResult(kiemTraKhachHang(khachHang), (kh) =>
      chainResult(kiemTraMatHang(matHang), (mh) => dungDonHangDaKiem(kh, mh))
    ),
    (dhDaKiem) => {
      const tongTien = tinhGia(dhDaKiem);
      const maDonHang = sinhMaDonHang(dhDaKiem.khachHang, gioHienTai);
      return ok({
        maDonHang,
        maKhachHang: dhDaKiem.khachHang.maKhachHang,
        maSanPham: dhDaKiem.matHang.maSanPham,
        soLuong: dhDaKiem.matHang.soLuong,
        tongTien,
        thoiGianTao: gioHienTai,
      });
    },
  );
}

// Khách hàng HẠNG VÀNG — giảm 10%
const khVang: KhachHang = { maKhachHang: "KH-01", email: "vang@shop.vn", hangKhachHang: "vang" };
const matHang: MatHang = { maSanPham: "SP-01", soLuong: 2, donGia: 100000 };
console.log(JSON.stringify(xuLyDonHangWorkflow(khVang, matHang, 1000)));
```

```text
{"kind":"ok","giaTri":{"maDonHang":"DH-KH-01-1000","maKhachHang":"KH-01","maSanPham":"SP-01","soLuong":2,"tongTien":180000,"thoiGianTao":1000}}
```

`2 * 100000 = 200000` (giá gốc), khách hàng HẠNG VÀNG giảm `10%` →
`200000 * 0.9 = 180000`. `xuLyDonHangWorkflow` KHÔNG chứa `if`/`else`
lồng nhau NÀO — CHỈ là hai lời gọi `chainResult` NỐI năm hàm THUẦN,
mỗi hàm MỘT trách nhiệm, TEST ĐƯỢC riêng từng hàm (đã học ở bài 19).
`gioHienTai` được TRUYỀN VÀO như DỮ LIỆU (không gọi `Date.now()` bên
trong) — workflow VẪN thuần, tất định, test được với BẤT KỲ thời điểm
giả lập nào.
::::

::::example{#loi-email-short-circuit}
Đơn hàng có email KHÔNG hợp lệ — cả HAI bước `kiemTraMatHang` và
`tinhGia` bị SKIP hoàn toàn, dù mặt hàng có tồn tại và hợp lệ:

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}
type LoiWorkflow = { tag: "loi_khach_hang"; thongDiep: string } | { tag: "loi_mat_hang"; thongDiep: string };
type KhachHang = { maKhachHang: string; email: string; hangKhachHang: "thuong" | "vang" };
type MatHang = { maSanPham: string; soLuong: number; donGia: number };
type DonHangDaKiem = { khachHang: KhachHang; matHang: MatHang };
type DonHang = { maDonHang: string; maKhachHang: string; maSanPham: string; soLuong: number; tongTien: number; thoiGianTao: number };
function kiemTraKhachHang(kh: KhachHang): Result<KhachHang, LoiWorkflow[]> {
  if (!kh.email.includes("@")) return loi([{ tag: "loi_khach_hang", thongDiep: "email không hợp lệ" }]);
  return ok(kh);
}
let demGoiKiemMatHang = 0;
function kiemTraMatHang(mh: MatHang): Result<MatHang, LoiWorkflow[]> {
  demGoiKiemMatHang = demGoiKiemMatHang + 1;
  if (mh.soLuong <= 0) return loi([{ tag: "loi_mat_hang", thongDiep: "số lượng phải lớn hơn 0" }]);
  return ok(mh);
}
function dungDonHangDaKiem(kh: KhachHang, mh: MatHang): Result<DonHangDaKiem, LoiWorkflow[]> {
  return ok({ khachHang: kh, matHang: mh });
}
function tinhGia(dhDaKiem: DonHangDaKiem): number {
  const tongGoc = dhDaKiem.matHang.soLuong * dhDaKiem.matHang.donGia;
  const giamGia = dhDaKiem.khachHang.hangKhachHang === "vang" ? 0.1 : 0;
  return tongGoc * (1 - giamGia);
}
function sinhMaDonHang(kh: KhachHang, gioHienTai: number): string {
  return `DH-${kh.maKhachHang}-${gioHienTai}`;
}
function xuLyDonHangWorkflow(khachHang: KhachHang, matHang: MatHang, gioHienTai: number): Result<DonHang, LoiWorkflow[]> {
  return chainResult(
    chainResult(kiemTraKhachHang(khachHang), (kh) =>
      chainResult(kiemTraMatHang(matHang), (mh) => dungDonHangDaKiem(kh, mh))
    ),
    (dhDaKiem) => {
      const tongTien = tinhGia(dhDaKiem);
      const maDonHang = sinhMaDonHang(dhDaKiem.khachHang, gioHienTai);
      return ok({ maDonHang, maKhachHang: dhDaKiem.khachHang.maKhachHang, maSanPham: dhDaKiem.matHang.maSanPham, soLuong: dhDaKiem.matHang.soLuong, tongTien, thoiGianTao: gioHienTai });
    },
  );
}

const khSai: KhachHang = { maKhachHang: "KH-03", email: "khong-hop-le", hangKhachHang: "vang" };
const matHangHopLe: MatHang = { maSanPham: "SP-01", soLuong: 2, donGia: 100000 };
const ketQua = xuLyDonHangWorkflow(khSai, matHangHopLe, 3000);
console.log(JSON.stringify(ketQua));
console.log(demGoiKiemMatHang);
```

```text title=readonly
{"kind":"loi","loi":[{"tag":"loi_khach_hang","thongDiep":"email không hợp lệ"}]}
0
```

`demGoiKiemMatHang` vẫn LÀ `0` — dù `matHangHopLe` HOÀN TOÀN hợp lệ
(`soLuong: 2`), `kiemTraMatHang` KHÔNG BAO GIỜ được gọi, vì
`kiemTraKhachHang` (trạm ĐẦU TIÊN trong chuỗi `chainResult`) đã lỗi
TRƯỚC. Composition = ĐÚNG THỨ TỰ hàm được LỒNG trong `chainResult`.
::::

::::predict{#doan-hai-loi-workflow commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}
type LoiWorkflow = { tag: "loi_khach_hang"; thongDiep: string } | { tag: "loi_mat_hang"; thongDiep: string };
type KhachHang = { maKhachHang: string; email: string; hangKhachHang: "thuong" | "vang" };
type MatHang = { maSanPham: string; soLuong: number; donGia: number };
type DonHangDaKiem = { khachHang: KhachHang; matHang: MatHang };
type DonHang = { maDonHang: string; maKhachHang: string; maSanPham: string; soLuong: number; tongTien: number; thoiGianTao: number };
function kiemTraKhachHang(kh: KhachHang): Result<KhachHang, LoiWorkflow[]> {
  if (!kh.email.includes("@")) return loi([{ tag: "loi_khach_hang", thongDiep: "email không hợp lệ" }]);
  return ok(kh);
}
function kiemTraMatHang(mh: MatHang): Result<MatHang, LoiWorkflow[]> {
  if (mh.soLuong <= 0) return loi([{ tag: "loi_mat_hang", thongDiep: "số lượng phải lớn hơn 0" }]);
  return ok(mh);
}
function dungDonHangDaKiem(kh: KhachHang, mh: MatHang): Result<DonHangDaKiem, LoiWorkflow[]> {
  return ok({ khachHang: kh, matHang: mh });
}
function tinhGia(dhDaKiem: DonHangDaKiem): number {
  const tongGoc = dhDaKiem.matHang.soLuong * dhDaKiem.matHang.donGia;
  const giamGia = dhDaKiem.khachHang.hangKhachHang === "vang" ? 0.1 : 0;
  return tongGoc * (1 - giamGia);
}
function sinhMaDonHang(kh: KhachHang, gioHienTai: number): string {
  return `DH-${kh.maKhachHang}-${gioHienTai}`;
}
function xuLyDonHangWorkflow(khachHang: KhachHang, matHang: MatHang, gioHienTai: number): Result<DonHang, LoiWorkflow[]> {
  return chainResult(
    chainResult(kiemTraKhachHang(khachHang), (kh) =>
      chainResult(kiemTraMatHang(matHang), (mh) => dungDonHangDaKiem(kh, mh))
    ),
    (dhDaKiem) => {
      const tongTien = tinhGia(dhDaKiem);
      const maDonHang = sinhMaDonHang(dhDaKiem.khachHang, gioHienTai);
      return ok({ maDonHang, maKhachHang: dhDaKiem.khachHang.maKhachHang, maSanPham: dhDaKiem.matHang.maSanPham, soLuong: dhDaKiem.matHang.soLuong, tongTien, thoiGianTao: gioHienTai });
    },
  );
}

// CẢ HAI đều sai: email không hợp lệ VÀ số lượng bằng 0
const khSai: KhachHang = { maKhachHang: "KH-09", email: "sai-dinh-dang", hangKhachHang: "vang" };
const matHangSai: MatHang = { maSanPham: "SP-02", soLuong: 0, donGia: 50000 };
const ketQua = xuLyDonHangWorkflow(khSai, matHangSai, 4000);
console.log(ketQua.kind === "loi" ? (ketQua.loi[0] ? ketQua.loi[0].tag : "?") : "ok");
```

Dòng cuối in ra gì?

:::opt{correct}
`loi_khach_hang`
:::

:::opt
`loi_mat_hang` — vì bên trong `xuLyDonHangWorkflow`, `kiemTraMatHang`
nằm Ở CHUỖI `chainResult` LỒNG BÊN TRONG (gần hàm `dungDonHangDaKiem`
hơn), nên nó CHẠY TRƯỚC `kiemTraKhachHang` (ở chuỗi NGOÀI)
::why
Gần đúng ở việc bạn để ý CÓ hai `chainResult` LỒNG NHAU trong
`xuLyDonHangWorkflow` — quan sát về cấu trúc LỒNG đó đúng.

Chỗ lệch: "lồng bên trong" KHÔNG có nghĩa là "chạy TRƯỚC". `chainResult`
NGOÀI CÙNG nhận `kiemTraKhachHang(khachHang)` làm THAM SỐ ĐẦU — theo
ngữ nghĩa của `chainResult` (bài 23: `case "ok": return f(r.giaTri)`),
`kiemTraKhachHang(khachHang)` phải được ĐÁNH GIÁ TRƯỚC để `chainResult`
NGOÀI có `r` mà kiểm tra `r.kind`. Chỉ SAU KHI nó ra `ok`, hàm lồng bên
trong (chứa `kiemTraMatHang`) mới được GỌI. Thứ tự CHẠY đi từ tham số
ĐẦU của lời gọi `chainResult` NGOÀI CÙNG, không phải theo độ LỒNG sâu
của code.
::
:::

:::opt
Máy báo lỗi biên dịch — `matHangSai.soLuong` bằng `0` không hợp lệ với
kiểu `number` của trường `soLuong`
::why
Gần đúng ở việc bạn nhận ra `soLuong: 0` LÀ trường hợp NGHIỆP VỤ không
hợp lệ (đúng — `kiemTraMatHang` từ chối nó) — quan sát về việc `0` có
vấn đề đó đúng.

Chỗ lệch: kiểu `number` chấp nhận MỌI số hữu hạn, KỂ CẢ `0` — không có
ràng buộc "phải dương" Ở TẦNG KIỂU. Việc từ chối `soLuong <= 0` là quy
tắc NGHIỆP VỤ kiểm LÚC CHẠY (`if` bên trong `kiemTraMatHang`), không
phải giới hạn của kiểu — biên dịch sạch, `0` truyền vào hoàn toàn hợp
lệ về mặt kiểu.
::
:::
::::

::::code{#viet_kiemtrakhachhang_va_tinhgia}
Tự viết `kiemTraKhachHang` và phần tính giảm giá trong `tinhGia`.

```typescript title=starter
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

type LoiWorkflow =
  | { tag: "loi_khach_hang"; thongDiep: string }
  | { tag: "loi_mat_hang"; thongDiep: string };
type KhachHang = { maKhachHang: string; email: string; hangKhachHang: "thuong" | "vang" };
type MatHang = { maSanPham: string; soLuong: number; donGia: number };
type DonHangDaKiem = { khachHang: KhachHang; matHang: MatHang };
type DonHang = { maDonHang: string; maKhachHang: string; maSanPham: string; soLuong: number; tongTien: number; thoiGianTao: number };

function kiemTraKhachHang(kh: KhachHang): Result<KhachHang, LoiWorkflow[]> {
  if (!kh.email.includes("@")) return ___;
  return ___;
}

function kiemTraMatHang(mh: MatHang): Result<MatHang, LoiWorkflow[]> {
  if (mh.soLuong <= 0) return loi([{ tag: "loi_mat_hang", thongDiep: "số lượng phải lớn hơn 0" }]);
  return ok(mh);
}

function dungDonHangDaKiem(kh: KhachHang, mh: MatHang): Result<DonHangDaKiem, LoiWorkflow[]> {
  return ok({ khachHang: kh, matHang: mh });
}

function tinhGia(dhDaKiem: DonHangDaKiem): number {
  const tongGoc = dhDaKiem.matHang.soLuong * dhDaKiem.matHang.donGia;
  const giamGia = dhDaKiem.khachHang.hangKhachHang === "vang" ? ___ : 0;
  return tongGoc * (1 - giamGia);
}

function sinhMaDonHang(kh: KhachHang, gioHienTai: number): string {
  return `DH-${kh.maKhachHang}-${gioHienTai}`;
}

function xuLyDonHangWorkflow(khachHang: KhachHang, matHang: MatHang, gioHienTai: number): Result<DonHang, LoiWorkflow[]> {
  return chainResult(
    chainResult(kiemTraKhachHang(khachHang), (kh) =>
      chainResult(kiemTraMatHang(matHang), (mh) => dungDonHangDaKiem(kh, mh))
    ),
    (dhDaKiem) => {
      const tongTien = tinhGia(dhDaKiem);
      const maDonHang = sinhMaDonHang(dhDaKiem.khachHang, gioHienTai);
      return ok({ maDonHang, maKhachHang: dhDaKiem.khachHang.maKhachHang, maSanPham: dhDaKiem.matHang.maSanPham, soLuong: dhDaKiem.matHang.soLuong, tongTien, thoiGianTao: gioHienTai });
    },
  );
}

const khVang: KhachHang = { maKhachHang: "KH-01", email: "vang@shop.vn", hangKhachHang: "vang" };
const matHangA: MatHang = { maSanPham: "SP-01", soLuong: 2, donGia: 100000 };
console.log(JSON.stringify(xuLyDonHangWorkflow(khVang, matHangA, 1000)));
```

```typescript title=solution
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
function ok<T, E>(giaTri: T): Result<T, E> { return { kind: "ok", giaTri }; }
function loi<T, E>(l: E): Result<T, E> { return { kind: "loi", loi: l }; }
function chainResult<T, U, E>(r: Result<T, E>, f: (x: T) => Result<U, E>): Result<U, E> {
  switch (r.kind) {
    case "ok": return f(r.giaTri);
    case "loi": return loi(r.loi);
  }
}

type LoiWorkflow =
  | { tag: "loi_khach_hang"; thongDiep: string }
  | { tag: "loi_mat_hang"; thongDiep: string };
type KhachHang = { maKhachHang: string; email: string; hangKhachHang: "thuong" | "vang" };
type MatHang = { maSanPham: string; soLuong: number; donGia: number };
type DonHangDaKiem = { khachHang: KhachHang; matHang: MatHang };
type DonHang = { maDonHang: string; maKhachHang: string; maSanPham: string; soLuong: number; tongTien: number; thoiGianTao: number };

function kiemTraKhachHang(kh: KhachHang): Result<KhachHang, LoiWorkflow[]> {
  if (!kh.email.includes("@")) return loi([{ tag: "loi_khach_hang", thongDiep: "email không hợp lệ" }]);
  return ok(kh);
}

function kiemTraMatHang(mh: MatHang): Result<MatHang, LoiWorkflow[]> {
  if (mh.soLuong <= 0) return loi([{ tag: "loi_mat_hang", thongDiep: "số lượng phải lớn hơn 0" }]);
  return ok(mh);
}

function dungDonHangDaKiem(kh: KhachHang, mh: MatHang): Result<DonHangDaKiem, LoiWorkflow[]> {
  return ok({ khachHang: kh, matHang: mh });
}

function tinhGia(dhDaKiem: DonHangDaKiem): number {
  const tongGoc = dhDaKiem.matHang.soLuong * dhDaKiem.matHang.donGia;
  const giamGia = dhDaKiem.khachHang.hangKhachHang === "vang" ? 0.1 : 0;
  return tongGoc * (1 - giamGia);
}

function sinhMaDonHang(kh: KhachHang, gioHienTai: number): string {
  return `DH-${kh.maKhachHang}-${gioHienTai}`;
}

function xuLyDonHangWorkflow(khachHang: KhachHang, matHang: MatHang, gioHienTai: number): Result<DonHang, LoiWorkflow[]> {
  return chainResult(
    chainResult(kiemTraKhachHang(khachHang), (kh) =>
      chainResult(kiemTraMatHang(matHang), (mh) => dungDonHangDaKiem(kh, mh))
    ),
    (dhDaKiem) => {
      const tongTien = tinhGia(dhDaKiem);
      const maDonHang = sinhMaDonHang(dhDaKiem.khachHang, gioHienTai);
      return ok({ maDonHang, maKhachHang: dhDaKiem.khachHang.maKhachHang, maSanPham: dhDaKiem.matHang.maSanPham, soLuong: dhDaKiem.matHang.soLuong, tongTien, thoiGianTao: gioHienTai });
    },
  );
}

const khVang: KhachHang = { maKhachHang: "KH-01", email: "vang@shop.vn", hangKhachHang: "vang" };
const matHangA: MatHang = { maSanPham: "SP-01", soLuong: 2, donGia: 100000 };
console.log(JSON.stringify(xuLyDonHangWorkflow(khVang, matHangA, 1000)));
```

```typescript title=test
// Khách hàng hạng VÀNG — phải giảm 10% (200000 * 0.9 = 180000)
const khVangTest: KhachHang = { maKhachHang: "KH-10", email: "vang10@shop.vn", hangKhachHang: "vang" };
const mhTest: MatHang = { maSanPham: "SP-10", soLuong: 2, donGia: 100000 };
const kqVang = xuLyDonHangWorkflow(khVangTest, mhTest, 9000);
if (kqVang.kind !== "ok") throw new Error("đơn hàng hợp lệ, hạng vàng phải ra ok");
if (kqVang.kind === "ok" && kqVang.giaTri.tongTien !== 180000) throw new Error("khách hàng hạng vàng phải được giảm 10% (200000 -> 180000)");

// Khách hàng hạng THƯỜNG — không giảm giá
const khThuongTest: KhachHang = { maKhachHang: "KH-11", email: "thuong11@shop.vn", hangKhachHang: "thuong" };
const kqThuong = xuLyDonHangWorkflow(khThuongTest, mhTest, 9001);
if (kqThuong.kind !== "ok") throw new Error("đơn hàng hợp lệ, hạng thường phải ra ok");
if (kqThuong.kind === "ok" && kqThuong.giaTri.tongTien !== 200000) throw new Error("khách hàng hạng thường KHÔNG được giảm giá (phải ra đúng 200000)");

// Email KHÔNG hợp lệ — phải báo loi_khach_hang
const khSaiTest: KhachHang = { maKhachHang: "KH-12", email: "khong-co-ky-tu-a-cong", hangKhachHang: "vang" };
const kqSai = xuLyDonHangWorkflow(khSaiTest, mhTest, 9002);
if (kqSai.kind !== "loi") throw new Error("email không hợp lệ phải ra loi");
if (kqSai.kind === "loi" && (!kqSai.loi[0] || kqSai.loi[0].tag !== "loi_khach_hang")) throw new Error("lỗi phải đúng tag loi_khach_hang");

// Biên: soLuong đúng NGƯỠNG 0 phải bị từ chối, soLuong = 1 phải hợp lệ
const mhBienZero: MatHang = { maSanPham: "SP-11", soLuong: 0, donGia: 10000 };
const kqBienZero = xuLyDonHangWorkflow(khVangTest, mhBienZero, 9003);
if (kqBienZero.kind !== "loi") throw new Error("số lượng đúng 0 (ngưỡng) phải bị từ chối");
if (kqBienZero.kind === "loi" && (!kqBienZero.loi[0] || kqBienZero.loi[0].tag !== "loi_mat_hang")) throw new Error("lỗi ở ngưỡng soLuong=0 phải đúng tag loi_mat_hang");

const mhBienMot: MatHang = { maSanPham: "SP-11", soLuong: 1, donGia: 10000 };
const kqBienMot = xuLyDonHangWorkflow(khVangTest, mhBienMot, 9004);
if (kqBienMot.kind !== "ok") throw new Error("số lượng = 1 (ngưỡng + 1) phải hợp lệ");
```

:::hints
- kind: attention
  body: "kiemTraKhachHang: nhánh lỗi bọc loi([{tag: \"loi_khach_hang\", thongDiep: ...}]) (CHÚ Ý: mảng có MỘT phần tử, LoiWorkflow[] chứ không phải LoiWorkflow đơn); nhánh hợp lệ trả ok(kh). tinhGia: khách hàng hạng vàng được giảm 10%, viết dưới dạng số thập phân dùng trong phép trừ (1 - giamGia)."
- kind: strategy
  body: 'loi([{ tag: "loi_khach_hang", thongDiep: "email không hợp lệ" }]) : ok(kh) — kiemTraKhachHang. "vang" ? 0.1 : 0 — tinhGia (10% viết là 0.1).'
- kind: one-line
  body: '___ (kiemTraKhachHang lỗi) = loi([{ tag: "loi_khach_hang", thongDiep: "email không hợp lệ" }])\n___ (kiemTraKhachHang ok) = ok(kh)\n___ (tinhGia, hạng vàng) = 0.1'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "ok"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cụm 4 hoàn tất: pipe, flow, tap, chainResult (ROP) — ghép lại thành
MỘT workflow nghiệp vụ thật, đọc từ trên xuống, mỗi trạm test riêng
được. Cụm tiếp theo: xử lý lỗi SÂU HƠN.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`chainResult` DỪNG ngay ở lỗi ĐẦU TIÊN — nhưng nếu muốn kiểm TẤT CẢ
các trường của một FORM, báo HẾT lỗi cùng lúc (không dừng sớm), thì
cần công cụ khác. Công cụ đó là gì?
::::

::::checkpoint{mastery=0.8}
::::
