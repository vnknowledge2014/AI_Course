---
id: ky-nghe-phan-mem.ddd.data-mapper-dich-du-lieu
title: "Data Mapper — dịch dữ liệu giữa định dạng lưu trữ và định dạng domain"
summary: "Khi \"kho\" thật dùng shape KHÁC domain (snake_case, kiểu thô), cần cặp hàm mapper hai chiều để CÔ LẬP domain khỏi cấu trúc lưu trữ — nối thẳng bài 31. hangThoSangMien/mienSangHangTho — viết repository THỨ HAI (khác in-memory bài 38) dùng client giả lập + mapper này, vẫn khớp CÙNG interface KhoDonHang."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 39
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.data-mapper]
requires: [ddd.in-memory-repo-test]
concepts: [ddd.data-mapper]
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
`taoKhoDonHangBoNho` (bài trước) lưu domain object TRỰC TIẾP. "Kho"
THẬT (database) thường KHÔNG lưu ĐÚNG hình dạng domain — vì sao?
::::

::::explain{#hang-tho-va-mapper}
Database THẬT thường dùng **snake_case**, **kiểu thô** (không brand,
không `Date` object) — hình dạng "hàng" trong bảng KHÁC HẲN domain
object. Cần **cặp hàm mapper HAI CHIỀU** để CÔ LẬP domain khỏi cấu
trúc lưu trữ (nối thẳng bài 31):

```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };

// "Hàng thô" -- hình dạng LƯU TRỮ, snake_case, không brand
type HangThoDonHang = { ma_don_hang: string; ma_khach_hang: string; tong_tien: number };

function hangThoSangMien(h: HangThoDonHang): DonHang {
  return { ma: h.ma_don_hang as MaDonHang, maKhachHang: h.ma_khach_hang, tongTien: h.tong_tien };
}
function mienSangHangTho(dh: DonHang): HangThoDonHang {
  return { ma_don_hang: dh.ma, ma_khach_hang: dh.maKhachHang, tong_tien: dh.tongTien };
}

const hangTho: HangThoDonHang = { ma_don_hang: "DH-01", ma_khach_hang: "KH-01", tong_tien: 100000 };
const domain = hangThoSangMien(hangTho);
console.log(JSON.stringify(domain));
console.log(JSON.stringify(mienSangHangTho(domain)));
```

```text
{"ma":"DH-01","maKhachHang":"KH-01","tongTien":100000}
{"ma_don_hang":"DH-01","ma_khach_hang":"KH-01","tong_tien":100000}
```

`hangThoSangMien` (INBOUND — nối bài 34) đổi TÊN field
(`ma_don_hang`→`ma`), THÊM brand (`as MaDonHang`). `mienSangHangTho`
(OUTBOUND) làm NGƯỢC LẠI — bỏ brand, đổi tên field VỀ snake_case.
::::

::::example{#repository-thu-hai}
Viết **repository THỨ HAI** — dùng "client giả lập" bên ngoài (KHÔNG
phải Prisma/ORM thật, CHỈ mô phỏng — tránh ngộ nhận API giả LÀ API
thật) + cặp mapper — VẪN khớp **CÙNG** interface `KhoDonHang` (bài
37), domain function DÙNG ĐƯỢC NGAY, không sửa gì:

```typescript title=readonly
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = { timTheoId: (ma: MaDonHang) => DonHang | null; timTheoKhachHang: (maKhachHang: string) => DonHang[]; luu: (dh: DonHang) => void; xoa: (ma: MaDonHang) => void };
type HangThoDonHang = { ma_don_hang: string; ma_khach_hang: string; tong_tien: number };

function taoClientGiaLap() {
  const bang: HangThoDonHang[] = [];
  return {
    truyVan: (maDonHang: string): HangThoDonHang | undefined => bang.find((h) => h.ma_don_hang === maDonHang),
    truyVanTheoKhachHang: (maKhachHang: string): HangThoDonHang[] => bang.filter((h) => h.ma_khach_hang === maKhachHang),
    ghi: (h: HangThoDonHang): void => { bang.push(h); },
    xoaHang: (maDonHang: string): void => {
      const idx = bang.findIndex((h) => h.ma_don_hang === maDonHang);
      if (idx !== -1) bang.splice(idx, 1);
    },
  };
}

function hangThoSangMien(h: HangThoDonHang): DonHang {
  return { ma: h.ma_don_hang as MaDonHang, maKhachHang: h.ma_khach_hang, tongTien: h.tong_tien };
}
function mienSangHangTho(dh: DonHang): HangThoDonHang {
  return { ma_don_hang: dh.ma, ma_khach_hang: dh.maKhachHang, tong_tien: dh.tongTien };
}

function taoKhoDonHangGiaLap(): KhoDonHang {
  const client = taoClientGiaLap();
  return {
    timTheoId: (ma) => {
      const h = client.truyVan(ma);
      return h ? hangThoSangMien(h) : null;
    },
    timTheoKhachHang: (mkh) => client.truyVanTheoKhachHang(mkh).map(hangThoSangMien),
    luu: (dh) => client.ghi(mienSangHangTho(dh)),
    xoa: (ma) => client.xoaHang(ma),
  };
}

function timDonHangTheoId(kho: KhoDonHang, ma: MaDonHang): DonHang | null {
  return kho.timTheoId(ma);
}

const khoGiaLap = taoKhoDonHangGiaLap();
khoGiaLap.luu({ ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 });
console.log(JSON.stringify(timDonHangTheoId(khoGiaLap, "DH-01" as MaDonHang)));
```

```text title=readonly
{"ma":"DH-01","maKhachHang":"KH-01","tongTien":100000}
```

`timDonHangTheoId` (hàm domain TỪ bài 37) gọi `khoGiaLap` MÀ **KHÔNG
BIẾT** bên trong nó có `client`, "bảng" snake_case, hay mapper NÀO cả
— nó CHỈ thấy `KhoDonHang`. "Rối rắm" của việc dịch dữ liệu bị NHỐT
HOÀN TOÀN bên trong `taoKhoDonHangGiaLap`, ở BIÊN.
::::

::::predict{#doan-mapper-khong-doi-y-nghia commitOnce}
```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type HangThoDonHang = { ma_don_hang: string; ma_khach_hang: string; tong_tien: number };

function hangThoSangMien(h: HangThoDonHang): DonHang {
  return { ma: h.ma_don_hang as MaDonHang, maKhachHang: h.ma_khach_hang, tongTien: h.tong_tien };
}
function mienSangHangTho(dh: DonHang): HangThoDonHang {
  return { ma_don_hang: dh.ma, ma_khach_hang: dh.maKhachHang, tong_tien: dh.tongTien };
}

const hangGoc: HangThoDonHang = { ma_don_hang: "DH-42", ma_khach_hang: "KH-09", tong_tien: 250000 };
// Đi VÒNG: hàng thô -> domain -> hàng thô
const diVong = mienSangHangTho(hangThoSangMien(hangGoc));
console.log(diVong.ma_don_hang === hangGoc.ma_don_hang && diVong.tong_tien === hangGoc.tong_tien);
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì `hangThoSangMien` THÊM brand (`as MaDonHang`) vào `ma`,
và khi `mienSangHangTho` chuyển NGƯỢC LẠI, giá trị mang brand đó
KHÔNG còn "bằng" (`===`) giá trị chuỗi GỐC (không brand) nữa
::why
Gần đúng ở việc bạn nhớ ĐÚNG `hangThoSangMien` THÊM brand vào `ma` —
một quan sát chính xác về BƯỚC ĐI (hàng thô → domain).

Chỗ lệch: brand (`& { readonly __brand: "MaDonHang" }`) là một khái
niệm CHỈ tồn tại Ở TẦNG KIỂU (lúc BIÊN DỊCH) — nó KHÔNG hề THAY ĐỔI
giá trị THẬT lúc chạy. `h.ma_don_hang as MaDonHang` vẫn LÀ **CÙNG MỘT
CHUỖI** (`"DH-42"`), chỉ được TypeScript "GẮN NHÃN" thêm; khi
`mienSangHangTho` đọc lại `dh.ma`, nó nhận ĐÚNG chuỗi `"DH-42"` đó,
gán vào `ma_don_hang`. So sánh `===` (so sánh GIÁ TRỊ chuỗi lúc chạy,
không quan tâm brand) cho ra `true`.
::
:::

:::opt
Máy báo lỗi biên dịch — `diVong.ma_don_hang === hangGoc.ma_don_hang`
so sánh HAI kiểu KHÔNG tương thích, vì `diVong` đến từ MỘT vòng
mapper, còn `hangGoc` là dữ liệu GỐC CHƯA qua mapper nào
::why
Gần đúng ở việc bạn để ý `diVong` và `hangGoc` đến từ HAI "nguồn gốc"
KHÁC NHAU trong luồng code — một quan sát về LỊCH SỬ tạo ra chúng.

Chỗ lệch: TypeScript so sánh KIỂU, không quan tâm "lịch sử" một giá
trị được TẠO RA thế nào. CẢ HAI biến (`diVong`, `hangGoc`) đều có
CÙNG kiểu `HangThoDonHang` — `ma_don_hang` CẢ HAI đều là `string`. So
sánh `===` giữa hai `string` LUÔN hợp lệ về mặt kiểu, bất kể GIÁ TRỊ
cụ thể VÀ bất kể chúng được TÍNH TOÁN thế nào.
::
:::
::::

::::code{#viet_mapper_hai_chieu}
Tự viết phần `maKhachHang`/`tongTien` và `ma_khach_hang`/`tong_tien`
trong cặp mapper.

```typescript title=starter
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type HangThoDonHang = { ma_don_hang: string; ma_khach_hang: string; tong_tien: number };

function hangThoSangMien(h: HangThoDonHang): DonHang {
  return { ma: h.ma_don_hang as MaDonHang, maKhachHang: ___, tongTien: ___ };
}
function mienSangHangTho(dh: DonHang): HangThoDonHang {
  return { ma_don_hang: dh.ma, ma_khach_hang: ___, tong_tien: ___ };
}

const hangTho: HangThoDonHang = { ma_don_hang: "DH-01", ma_khach_hang: "KH-01", tong_tien: 100000 };
console.log(JSON.stringify(hangThoSangMien(hangTho)));
```

```typescript title=solution
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type HangThoDonHang = { ma_don_hang: string; ma_khach_hang: string; tong_tien: number };

function hangThoSangMien(h: HangThoDonHang): DonHang {
  return { ma: h.ma_don_hang as MaDonHang, maKhachHang: h.ma_khach_hang, tongTien: h.tong_tien };
}
function mienSangHangTho(dh: DonHang): HangThoDonHang {
  return { ma_don_hang: dh.ma, ma_khach_hang: dh.maKhachHang, tong_tien: dh.tongTien };
}

const hangTho: HangThoDonHang = { ma_don_hang: "DH-01", ma_khach_hang: "KH-01", tong_tien: 100000 };
console.log(JSON.stringify(hangThoSangMien(hangTho)));
```

```typescript title=test
const hangTest: HangThoDonHang = { ma_don_hang: "DH-77", ma_khach_hang: "KH-33", tong_tien: 250000 };
const domainTest = hangThoSangMien(hangTest);
if (domainTest.maKhachHang !== "KH-33") throw new Error("hangThoSangMien: maKhachHang phải khớp đúng ma_khach_hang, không lẫn field khác");
if (domainTest.tongTien !== 250000) throw new Error("hangThoSangMien: tongTien phải khớp đúng tong_tien, không lẫn field khác");

const dhTest: DonHang = { ma: "DH-88" as MaDonHang, maKhachHang: "KH-44", tongTien: 999000 };
const hangNguoc = mienSangHangTho(dhTest);
if (hangNguoc.ma_khach_hang !== "KH-44") throw new Error("mienSangHangTho: ma_khach_hang phải khớp đúng maKhachHang, không lẫn field khác");
if (hangNguoc.tong_tien !== 999000) throw new Error("mienSangHangTho: tong_tien phải khớp đúng tongTien, không lẫn field khác");

// Đi vòng phải giữ nguyên giá trị
const diVongTest = mienSangHangTho(hangThoSangMien(hangTest));
if (diVongTest.ma_don_hang !== hangTest.ma_don_hang || diVongTest.ma_khach_hang !== hangTest.ma_khach_hang || diVongTest.tong_tien !== hangTest.tong_tien) {
  throw new Error("đi vòng hangThoSangMien rồi mienSangHangTho phải giữ NGUYÊN mọi field");
}
```

:::hints
- kind: attention
  body: "hangThoSangMien: đọc từ h (snake_case) gán vào field camelCase. mienSangHangTho: đọc từ dh (camelCase) gán vào field snake_case — NGƯỢC HƯỚNG với hàm kia, đừng nhầm lẫn field NÀO map sang field NÀO."
- kind: strategy
  body: "h.ma_khach_hang : h.tong_tien — hangThoSangMien. dh.maKhachHang : dh.tongTien — mienSangHangTho."
- kind: one-line
  body: "___ (hangThoSangMien.maKhachHang) = h.ma_khach_hang\n___ (hangThoSangMien.tongTien) = h.tong_tien\n___ (mienSangHangTho.ma_khach_hang) = dh.maKhachHang\n___ (mienSangHangTho.tong_tien) = dh.tongTien"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "DH-01"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Data Mapper: cách ly domain khỏi cấu trúc lưu trữ, nhiều repository
khớp CÙNG interface. Bước tiếp theo: trừu tượng hoá interface để dùng
chung cho MỌI entity, không viết lại cho từng loại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`KhoDonHang` và một `KhoSanPham` (giả sử tồn tại) sẽ có `timTheoId`/
`luu`/`xoa` GẦN NHƯ GIỐNG HỆT nhau. Viết lại interface này cho MỖI
entity có lãng phí không? Có cách nào TRỪU TƯỢNG HOÁ chung không?
::::

::::checkpoint{mastery=0.8}
::::
