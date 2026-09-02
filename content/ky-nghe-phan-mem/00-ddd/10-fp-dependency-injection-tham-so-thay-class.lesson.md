---
id: ky-nghe-phan-mem.ddd.fp-dependency-injection-tham-so-thay-class
title: "FP Dependency Injection — tham số hàm thay vì class + DI container"
summary: "Thay vì class + constructor + DI container, truyền dependencies (đồng hồ, sinh ID...) làm THAM SỐ hàm. taoHoaDon(maKhachHang, soTien, dongHo, sinhId). Test double: object literal thoả interface, KHÔNG cần thư viện mock."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.fp-dependency-injection]
requires: [ddd.module-boundaries]
concepts: [ddd.fp-dependency-injection]
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
Một hàm domain đôi khi CẦN thứ nó không tự tạo được — giờ hiện tại
(`Date.now()`), một ID mới. Làm sao vẫn giữ hàm ĐÓ dễ TEST?
::::

::::explain{#fp-di}
OOP truyền thống giải quyết bằng class + constructor + DI container
(Spring, NestJS...) — một hệ thống PHỨC TẠP chỉ để "tiêm" một vài giá
trị vào. **FP Dependency Injection** đơn giản hơn NHIỀU: truyền
dependencies (đồng hồ, bộ sinh ID, logger...) làm THAM SỐ HÀM bình
thường — thường là tham số CUỐI:

```typescript
type DongHo = () => Date;
type SinhId = () => string;

type HoaDon = { maHoaDon: string; maKhachHang: string; soTien: number; ngayTao: Date };

function taoHoaDon(maKhachHang: string, soTien: number, dongHo: DongHo, sinhId: SinhId): HoaDon {
  return { maHoaDon: sinhId(), maKhachHang, soTien, ngayTao: dongHo() };
}

// test double: MỘT object literal, KHÔNG cần thư viện mock nào
const dongHoGia: DongHo = () => new Date("2024-01-01T00:00:00Z");
const sinhIdGia: SinhId = () => "HD-TEST-01";

const hd = taoHoaDon("KH-01", 100000, dongHoGia, sinhIdGia);
console.log(hd.maHoaDon);
console.log(hd.ngayTao.toISOString());
```

```text
HD-TEST-01
2024-01-01T00:00:00.000Z
```

`taoHoaDon` KHÔNG tự gọi `new Date()` hay `Math.random()` bên trong —
nó NHẬN `dongHo`/`sinhId` làm THAM SỐ. Test dùng `dongHoGia`/`sinhIdGia`
— hai HÀM đơn giản LUÔN trả về giá trị CỐ ĐỊNH — kết quả test HOÀN
TOÀN dự đoán được (không phụ thuộc "giờ THẬT lúc chạy test").

**Test double bằng object literal**: nhờ TypeScript's **structural
typing**, một object literal (hay một hàm mũi tên) THOẢ đúng shape của
`DongHo`/`SinhId` là ĐỦ — KHÔNG cần thư viện mock (Jest mock, Sinon).
::::

::::example{#dependency-thuc-va-gia}
Đối chiếu dependency THẬT (dùng lúc chạy production) và dependency GIẢ
(dùng lúc test) — CÙNG interface, khác implementation:

```typescript title=readonly
type DongHo = () => Date;
type SinhId = () => string;
type HoaDon = { maHoaDon: string; maKhachHang: string; soTien: number; ngayTao: Date };
function taoHoaDon(maKhachHang: string, soTien: number, dongHo: DongHo, sinhId: SinhId): HoaDon {
  return { maHoaDon: sinhId(), maKhachHang, soTien, ngayTao: dongHo() };
}

// "Thật" (production) — không dùng ở đây vì output không dự đoán được,
// nhưng CÙNG SHAPE với bản giả bên dưới
const dongHoThat: DongHo = () => new Date();

// "Giả" (test) — CỐ ĐỊNH, luôn ra CÙNG kết quả
let dem = 0;
const sinhIdGiaTangDan: SinhId = () => {
  dem = dem + 1;
  return `HD-${dem}`;
};

const hd1 = taoHoaDon("KH-01", 50000, () => new Date("2024-01-01"), sinhIdGiaTangDan);
const hd2 = taoHoaDon("KH-02", 70000, () => new Date("2024-01-01"), sinhIdGiaTangDan);
console.log(hd1.maHoaDon);
console.log(hd2.maHoaDon);
```

```text title=readonly
HD-1
HD-2
```

`dongHoThat` (không dùng trong ví dụ trên, chỉ để đối chiếu) và
`dongHoGia`/`() => new Date("2024-01-01")` CÙNG kiểu `DongHo` — `taoHoaDon`
KHÔNG cần biết ĐANG dùng bản nào, chỉ cần shape ĐÚNG. `sinhIdGiaTangDan`
minh hoạ: test double có thể có STATE riêng (`dem`), miễn vẫn khớp
`SinhId = () => string`.
::::

::::predict{#doan-doi-dong-ho commitOnce}
```typescript
type DongHo = () => Date;
type SinhId = () => string;
type HoaDon = { maHoaDon: string; maKhachHang: string; soTien: number; ngayTao: Date };
function taoHoaDon(maKhachHang: string, soTien: number, dongHo: DongHo, sinhId: SinhId): HoaDon {
  return { maHoaDon: sinhId(), maKhachHang, soTien, ngayTao: dongHo() };
}

const dongHoCoDinh: DongHo = () => new Date("2025-06-15T00:00:00Z");
const sinhIdCoDinh: SinhId = () => "HD-FIXED";

const hd1 = taoHoaDon("KH-05", 30000, dongHoCoDinh, sinhIdCoDinh);
const hd2 = taoHoaDon("KH-06", 40000, dongHoCoDinh, sinhIdCoDinh);
console.log(hd1.ngayTao.getTime() === hd2.ngayTao.getTime());
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
`false` — vì mỗi lời gọi `taoHoaDon` tạo một `HoaDon` MỚI, nên
`ngayTao` của `hd1` và `hd2` chắc chắn KHÁC nhau về thời điểm
::why
Gần đúng ở việc bạn nhớ ĐÚNG `hd1` và `hd2` LÀ hai object HOÀN TOÀN
riêng biệt (đúng — mỗi lời gọi `taoHoaDon` tạo MỘT object mới) —
quan sát đó đúng.

Chỗ lệch: `dongHoCoDinh` LUÔN trả về CÙNG một `Date` (`"2025-06-15T00:
00:00Z"`) — nó KHÔNG gọi `new Date()` THẬT (giờ hệ thống, đổi mỗi mili
giây), nó là một hàm test double CỐ ĐỊNH. `hd1.ngayTao` và `hd2.ngayTao`
LÀ hai OBJECT `Date` khác nhau về THAM CHIẾU (đều được `new Date(...)`
tạo riêng), NHƯNG `.getTime()` so sánh GIÁ TRỊ thời gian bên trong —
CÙNG giá trị mili-giây, `true`.
::
:::

:::opt
Máy báo lỗi biên dịch — `dongHoCoDinh` và `sinhIdCoDinh` được TÁI SỬ
DỤNG cho HAI lời gọi `taoHoaDon` KHÁC nhau, TypeScript không cho phép
dùng lại CÙNG một tham số hai lần
::why
Gần đúng ở việc bạn để ý `dongHoCoDinh`/`sinhIdCoDinh` được DÙNG LẶP
LẠI ở CẢ HAI lời gọi — quan sát về việc CÓ tái sử dụng đó đúng.

Chỗ lệch: TÁI SỬ DỤNG một hàm (biến giữ MỘT function) cho NHIỀU lời
gọi KHÁC nhau là HOÀN TOÀN bình thường trong TypeScript/JavaScript —
không có ràng buộc nào cấm "dùng lại tham số". Đây CHÍNH LÀ lợi ích của
FP DI: MỘT test double, tái dùng cho BAO NHIÊU test case cũng được.
::
:::
::::

::::code{#viet_tao_hoa_don}
Tự viết `taoHoaDon(maKhachHang, soTien, dongHo, sinhId): HoaDon`.

```typescript title=starter
type DongHo = () => Date;
type SinhId = () => string;
type HoaDon = { maHoaDon: string; maKhachHang: string; soTien: number; ngayTao: Date };

function taoHoaDon(maKhachHang: string, soTien: number, dongHo: DongHo, sinhId: SinhId): HoaDon {
  return { maHoaDon: ___, maKhachHang, soTien, ngayTao: ___ };
}

const dongHoGia: DongHo = () => new Date("2024-01-01T00:00:00Z");
const sinhIdGia: SinhId = () => "HD-TEST-01";
console.log(taoHoaDon("KH-01", 100000, dongHoGia, sinhIdGia).maHoaDon);
```

```typescript title=solution
type DongHo = () => Date;
type SinhId = () => string;
type HoaDon = { maHoaDon: string; maKhachHang: string; soTien: number; ngayTao: Date };

function taoHoaDon(maKhachHang: string, soTien: number, dongHo: DongHo, sinhId: SinhId): HoaDon {
  return { maHoaDon: sinhId(), maKhachHang, soTien, ngayTao: dongHo() };
}

const dongHoGia: DongHo = () => new Date("2024-01-01T00:00:00Z");
const sinhIdGia: SinhId = () => "HD-TEST-01";
console.log(taoHoaDon("KH-01", 100000, dongHoGia, sinhIdGia).maHoaDon);
```

```typescript title=test
const dongHoGia1: DongHo = () => new Date("2024-01-01T00:00:00Z");
const sinhIdGia1: SinhId = () => "HD-TEST-01";
const hd = taoHoaDon("KH-01", 100000, dongHoGia1, sinhIdGia1);
if (hd.maHoaDon !== "HD-TEST-01") throw new Error("maHoaDon phải LẤY từ sinhId(), không phải hằng số cố định");
if (hd.maKhachHang !== "KH-01") throw new Error("maKhachHang phải giữ nguyên tham số truyền vào");
if (hd.soTien !== 100000) throw new Error("soTien phải giữ nguyên tham số truyền vào");
if (hd.ngayTao.getTime() !== new Date("2024-01-01T00:00:00Z").getTime()) throw new Error("ngayTao phải LẤY từ dongHo(), không phải giờ hệ thống thật");

let demGoi = 0;
const sinhIdDemGoi: SinhId = () => { demGoi = demGoi + 1; return `HD-${demGoi}`; };
const hdA = taoHoaDon("KH-02", 1, dongHoGia1, sinhIdDemGoi);
const hdB = taoHoaDon("KH-03", 2, dongHoGia1, sinhIdDemGoi);
if (hdA.maHoaDon === hdB.maHoaDon) throw new Error("sinhId phải được GỌI MỖI LẦN taoHoaDon chạy, không cache kết quả cũ");
```

:::hints
- kind: attention
  body: "maHoaDon KHÔNG phải một chuỗi cố định — nó LẤY từ việc GỌI sinhId(). ngayTao KHÔNG phải new Date() trực tiếp — nó LẤY từ việc GỌI dongHo()."
- kind: strategy
  body: "maHoaDon: sinhId(), ngayTao: dongHo() — GỌI cả hai dependency như hàm không tham số, lấy giá trị chúng trả về."
- kind: one-line
  body: "___ (maHoaDon) = sinhId()\n___ (ngayTao) = dongHo()"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "HD-TEST-01"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
FP DI: truyền dependency làm THAM SỐ hàm, không cần class/container.
Test double chỉ là một object literal khớp shape — TypeScript's
structural typing lo phần còn lại.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Truyền `dongHo`/`sinhId` làm tham số ở MỖI lời gọi hàm hơi LẶP LẠI —
có cách nào "khoá" dependencies MỘT LẦN, dùng lại cho NHIỀU hàm không?
::::

::::checkpoint{mastery=0.8}
::::
