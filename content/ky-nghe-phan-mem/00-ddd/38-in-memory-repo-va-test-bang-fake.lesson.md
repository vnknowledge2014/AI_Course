---
id: ky-nghe-phan-mem.ddd.in-memory-repo-va-test-bang-fake
title: "In-Memory Repository & Test bằng Fake — không cần mock library"
summary: "taoKhoDonHangBoNho() dùng Map trong closure — implementation ĐẦU TIÊN của KhoDonHang, KHÔNG cần database thật. Vì repository chỉ là object literal thực hiện interface, viết \"fake\" phục vụ test KHÔNG cần thư viện mocking — test chạy NHANH, không I/O, mỗi lần gọi có state ĐỘC LẬP."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 38
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.in-memory-repo-test]
requires: [ddd.repository-pattern]
concepts: [ddd.in-memory-repo-test]
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
`KhoDonHang` (bài trước) là một khế ước — CHƯA có implementation nào.
Implementation ĐƠN GIẢN NHẤT: không cần database thật, chỉ cần khớp
khế ước.
::::

::::explain{#kho-bo-nho}
`taoKhoDonHangBoNho()` — dùng `Map<MaDonHang, DonHang>` **BÊN TRONG
CLOSURE** (nối kỹ thuật closure đã học), TRẢ VỀ một object khớp
`KhoDonHang`. **KHÔNG database, KHÔNG file, KHÔNG mạng** — TOÀN BỘ dữ
liệu sống TRONG BỘ NHỚ, mất đi khi chương trình kết thúc:

```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = {
  timTheoId: (ma: MaDonHang) => DonHang | null;
  timTheoKhachHang: (maKhachHang: string) => DonHang[];
  luu: (dh: DonHang) => void;
  xoa: (ma: MaDonHang) => void;
};

function taoKhoDonHangBoNho(): KhoDonHang {
  const duLieu = new Map<MaDonHang, DonHang>();
  return {
    timTheoId: (ma) => duLieu.get(ma) ?? null,
    timTheoKhachHang: (mkh) => [...duLieu.values()].filter((dh) => dh.maKhachHang === mkh),
    luu: (dh) => { duLieu.set(dh.ma, dh); },
    xoa: (ma) => { duLieu.delete(ma); },
  };
}

const kho = taoKhoDonHangBoNho();
console.log(kho.timTheoId("DH-01" as MaDonHang));
kho.luu({ ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 });
console.log(kho.timTheoId("DH-01" as MaDonHang));
kho.xoa("DH-01" as MaDonHang);
console.log(kho.timTheoId("DH-01" as MaDonHang));
```

```text
null
{ ma: 'DH-01', maKhachHang: 'KH-01', tongTien: 100000 }
null
```

`duLieu` (biến `Map`) sống TRONG **CLOSURE** của `taoKhoDonHangBoNho`
— object TRẢ VỀ (bốn method) đều "NHỚ" `duLieu` ĐÓ, dù `duLieu` KHÔNG
hề được truyền RA ngoài hàm (private, không ai đọc/ghi trực tiếp
được, CHỈ qua bốn method). Vì `taoKhoDonHangBoNho` KHÔNG CẦN import gì
(không database driver, không thư viện) — nó dùng được NGAY TRONG
TEST, KHÔNG cần cài đặt gì thêm.
::::

::::example{#fake-khong-can-mock-library}
Vì repository CHỈ là MỘT OBJECT LITERAL thực hiện interface, viết
"fake" phục vụ TEST **KHÔNG cần thư viện mocking** nào — VÀ mỗi lần
gọi `taoKhoDonHangBoNho()` tạo ra state **ĐỘC LẬP HOÀN TOÀN**:

```typescript title=readonly
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = { timTheoId: (ma: MaDonHang) => DonHang | null; timTheoKhachHang: (maKhachHang: string) => DonHang[]; luu: (dh: DonHang) => void; xoa: (ma: MaDonHang) => void };
function taoKhoDonHangBoNho(): KhoDonHang {
  const duLieu = new Map<MaDonHang, DonHang>();
  return {
    timTheoId: (ma) => duLieu.get(ma) ?? null,
    timTheoKhachHang: (mkh) => [...duLieu.values()].filter((dh) => dh.maKhachHang === mkh),
    luu: (dh) => { duLieu.set(dh.ma, dh); },
    xoa: (ma) => { duLieu.delete(ma); },
  };
}

// Hai kho ĐỘC LẬP, KHÔNG chia sẻ state
const khoRieng1 = taoKhoDonHangBoNho();
const khoRieng2 = taoKhoDonHangBoNho();
khoRieng1.luu({ ma: "DH-05" as MaDonHang, maKhachHang: "KH-05", tongTien: 5000 });

console.log(khoRieng1.timTheoId("DH-05" as MaDonHang));
console.log(khoRieng2.timTheoId("DH-05" as MaDonHang));
```

```text title=readonly
{"ma":"DH-05","maKhachHang":"KH-05","tongTien":5000}
null
```

`khoRieng2` KHÔNG "thấy" dữ liệu `khoRieng1` vừa lưu — MỖI lần gọi
`taoKhoDonHangBoNho()` tạo MỘT `Map` MỚI HOÀN TOÀN (mỗi closure "nhớ"
`duLieu` RIÊNG của NÓ). Đây LÀ lý do fake AN TOÀN cho test: MỖI test
case gọi `taoKhoDonHangBoNho()` MỘT LẦN, đảm bảo state KHÔNG "rò rỉ"
giữa các test — KHÔNG CẦN `beforeEach`/reset thủ công, và chạy **CỰC
NHANH** (không I/O thật nào cả).
::::

::::predict{#doan-hai-kho-doc-lap commitOnce}
```typescript
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = { timTheoId: (ma: MaDonHang) => DonHang | null; timTheoKhachHang: (maKhachHang: string) => DonHang[]; luu: (dh: DonHang) => void; xoa: (ma: MaDonHang) => void };
function taoKhoDonHangBoNho(): KhoDonHang {
  const duLieu = new Map<MaDonHang, DonHang>();
  return {
    timTheoId: (ma) => duLieu.get(ma) ?? null,
    timTheoKhachHang: (mkh) => [...duLieu.values()].filter((dh) => dh.maKhachHang === mkh),
    luu: (dh) => { duLieu.set(dh.ma, dh); },
    xoa: (ma) => { duLieu.delete(ma); },
  };
}

const khoChung = taoKhoDonHangBoNho();
khoChung.luu({ ma: "DH-07" as MaDonHang, maKhachHang: "KH-07", tongTien: 70000 });

// GÁN LẠI biến -- CÙNG MỘT object kho, không gọi taoKhoDonHangBoNho() lần nữa
const thamChieuKhac = khoChung;
console.log(thamChieuKhac.timTheoId("DH-07" as MaDonHang));
```

Dòng cuối in ra gì?

:::opt{correct}
`{ ma: 'DH-07', maKhachHang: 'KH-07', tongTien: 70000 }`
:::

:::opt
`null` — vì `taoKhoDonHangBoNho()` LUÔN tạo `Map` MỚI mỗi lần được
GỌI (đã học ở ví dụ trước), và `thamChieuKhac = khoChung` được coi
như MỘT lần gọi MỚI, tạo state RIÊNG
::why
Gần đúng ở việc bạn nhớ ĐÚNG quy tắc "MỖI lần GỌI `taoKhoDonHangBoNho()`
tạo state RIÊNG" — quy tắc ĐÓ, từ ví dụ trước, hoàn toàn chính xác.

Chỗ lệch: `const thamChieuKhac = khoChung;` **KHÔNG GỌI**
`taoKhoDonHangBoNho()` lần nào cả — nó chỉ gán biến `thamChieuKhac`
TRỎ tới **CÙNG MỘT object** mà `khoChung` ĐANG trỏ tới (object đã
được TẠO SẴN từ lần gọi TRƯỚC đó). `thamChieuKhac` và `khoChung` là
HAI CÁI TÊN cho **MỘT** repository DUY NHẤT — dữ liệu đã `luu` qua
`khoChung` CHẮC CHẮN đọc được qua `thamChieuKhac`, vì bên trong CÙNG
là MỘT `duLieu` (Map) DUY NHẤT.
::
:::

:::opt
Máy báo lỗi biên dịch — không thể gán MỘT giá trị kiểu `KhoDonHang`
cho biến MỚI mà không gọi lại `taoKhoDonHangBoNho()`
::why
Gần đúng ở việc bạn nghĩ tới việc `KhoDonHang` là một kiểu KHÁ "đặc
biệt" (nhiều method khép trong closure) — một trực giác dễ hiểu khi
mới gặp closure lần đầu.

Chỗ lệch: `khoChung` (biến ĐÃ khai) có kiểu `KhoDonHang` — MỘT giá
trị BÌNH THƯỜNG như bất kỳ object nào khác. Gán `const thamChieuKhac
= khoChung;` chỉ đơn giản SAO CHÉP tham chiếu (KHÔNG sao chép NỘI
DUNG) sang một biến MỚI — hoàn toàn hợp lệ với MỌI kiểu object trong
TypeScript, không có ràng buộc đặc biệt nào cho `KhoDonHang`.
::
:::
::::

::::code{#viet_khodonhangbonho}
Tự viết BA method của `taoKhoDonHangBoNho`.

```typescript title=starter
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = {
  timTheoId: (ma: MaDonHang) => DonHang | null;
  timTheoKhachHang: (maKhachHang: string) => DonHang[];
  luu: (dh: DonHang) => void;
  xoa: (ma: MaDonHang) => void;
};

function taoKhoDonHangBoNho(): KhoDonHang {
  const duLieu = new Map<MaDonHang, DonHang>();
  return {
    timTheoId: (ma) => ___,
    timTheoKhachHang: (mkh) => [...duLieu.values()].filter((dh) => dh.maKhachHang === mkh),
    luu: (dh) => { ___; },
    xoa: (ma) => { ___; },
  };
}

const kho = taoKhoDonHangBoNho();
kho.luu({ ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 });
console.log(kho.timTheoId("DH-01" as MaDonHang));
```

```typescript title=solution
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { ma: MaDonHang; maKhachHang: string; tongTien: number };
type KhoDonHang = {
  timTheoId: (ma: MaDonHang) => DonHang | null;
  timTheoKhachHang: (maKhachHang: string) => DonHang[];
  luu: (dh: DonHang) => void;
  xoa: (ma: MaDonHang) => void;
};

function taoKhoDonHangBoNho(): KhoDonHang {
  const duLieu = new Map<MaDonHang, DonHang>();
  return {
    timTheoId: (ma) => duLieu.get(ma) ?? null,
    timTheoKhachHang: (mkh) => [...duLieu.values()].filter((dh) => dh.maKhachHang === mkh),
    luu: (dh) => { duLieu.set(dh.ma, dh); },
    xoa: (ma) => { duLieu.delete(ma); },
  };
}

const kho = taoKhoDonHangBoNho();
kho.luu({ ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 });
console.log(kho.timTheoId("DH-01" as MaDonHang));
```

```typescript title=test
const khoTest = taoKhoDonHangBoNho();
if (khoTest.timTheoId("DH-99" as MaDonHang) !== null) throw new Error("kho rỗng phải trả về null cho mã chưa tồn tại");

khoTest.luu({ ma: "DH-01" as MaDonHang, maKhachHang: "KH-01", tongTien: 100000 });
const timDuoc = khoTest.timTheoId("DH-01" as MaDonHang);
if (timDuoc === null) throw new Error("luu() phải THẬT SỰ ghi vào Map — timTheoId phải tìm thấy sau khi luu");
if (timDuoc.tongTien !== 100000) throw new Error("dữ liệu tìm được phải khớp đúng dữ liệu đã luu");

khoTest.xoa("DH-01" as MaDonHang);
if (khoTest.timTheoId("DH-01" as MaDonHang) !== null) throw new Error("xoa() phải THẬT SỰ xoá khỏi Map — timTheoId phải trả null sau khi xoa");

// Hai kho độc lập
const khoA = taoKhoDonHangBoNho();
const khoB = taoKhoDonHangBoNho();
khoA.luu({ ma: "DH-05" as MaDonHang, maKhachHang: "KH-05", tongTien: 5000 });
if (khoB.timTheoId("DH-05" as MaDonHang) !== null) throw new Error("hai lần gọi taoKhoDonHangBoNho() phải tạo state ĐỘC LẬP, không chia sẻ");
```

:::hints
- kind: attention
  body: "timTheoId: đọc từ duLieu bằng .get(ma), dùng ?? null để chuyển undefined (Map không có key) thành null. luu: GHI vào duLieu bằng .set(dh.ma, dh). xoa: XOÁ khỏi duLieu bằng .delete(ma)."
- kind: strategy
  body: "duLieu.get(ma) ?? null : duLieu.set(dh.ma, dh) : duLieu.delete(ma) — ba thao tác Map chuẩn, ứng với ba method."
- kind: one-line
  body: "___ (timTheoId) = duLieu.get(ma) ?? null\n___ (luu) = duLieu.set(dh.ma, dh)\n___ (xoa) = duLieu.delete(ma)"
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
In-memory repo: implementation ĐẦU TIÊN của khế ước, dùng LUÔN cho
test — nhanh, không I/O, state độc lập mỗi lần tạo. Bước tiếp theo:
"kho" THẬT thường có hình dạng KHÁC domain — cần dịch.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

"Kho" THẬT (database) thường lưu dữ liệu THEO HÌNH DẠNG KHÁC domain
(snake_case, kiểu thô) — nối bài 31's mapping, cần công cụ gì để CÔ
LẬP domain khỏi cấu trúc lưu trữ THẬT?
::::

::::checkpoint{mastery=0.8}
::::
