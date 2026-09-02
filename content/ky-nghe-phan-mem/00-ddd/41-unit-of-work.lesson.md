---
id: ky-nghe-phan-mem.ddd.unit-of-work
title: "Unit of Work — transaction boundary cho nhiều repo cùng lúc"
summary: "Khi cần đổi NHIỀU \"kệ sách\" cùng lúc (đơn hàng VÀ tồn kho), gom các repository liên quan vào MỘT object {donHang, sanPham, commit, rollback} — TẤT CẢ thay đổi cùng THÀNH CÔNG hoặc cùng THẤT BẠI. Cơ chế pending Map vs committed Map: save() ghi vào pending trước, chỉ CHUYỂN sang committed khi commit() được gọi."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 41
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.unit-of-work]
requires: [ddd.generic-repository]
concepts: [ddd.unit-of-work]
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
Đặt một đơn hàng cần đổi CẢ đơn hàng LẪN tồn kho CÙNG LÚC. Nếu bước
THỨ HAI thất bại (hết hàng), bước ĐẦU đã lưu rồi — dữ liệu "nửa vời"?
::::

::::explain{#pending-vs-committed}
**Unit of Work**: gom các repository LIÊN QUAN vào MỘT object
`{donHang, sanPham, commit, rollback}` — đảm bảo TẤT CẢ thay đổi cùng
**THÀNH CÔNG** hoặc cùng **THẤT BẠI** (all-or-nothing). Cơ chế: MỌI
`luu()` ghi vào Map **pending** (`dangCho`) TRƯỚC — CHỈ chuyển sang
Map **committed** (`daCommit`) khi `commit()` được GỌI:

```typescript
function taoKhoUOW<Id extends string, ThucThe extends { id: Id }>() {
  const daCommit = new Map<Id, ThucThe>();
  let dangCho = new Map<Id, ThucThe>();
  return {
    timTheoId: (id: Id): ThucThe | null => dangCho.get(id) ?? daCommit.get(id) ?? null,
    luu: (thucThe: ThucThe): void => { dangCho.set(thucThe.id, thucThe); },
    commit: (): void => {
      dangCho.forEach((tt, id) => daCommit.set(id, tt));
      dangCho = new Map();
    },
    rollback: (): void => { dangCho = new Map(); },
  };
}

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPhamTonKho = { id: MaSanPham; conLai: number };

const kho = taoKhoUOW<MaSanPham, SanPhamTonKho>();
kho.luu({ id: "SP-01" as MaSanPham, conLai: 5 });
console.log(kho.timTheoId("SP-01" as MaSanPham)); // pending -- thấy được TRƯỚC commit

kho.commit();
kho.luu({ id: "SP-01" as MaSanPham, conLai: 2 });
kho.rollback();
console.log(kho.timTheoId("SP-01" as MaSanPham)); // rollback -- quay lại giá trị ĐÃ commit (5)
```

```text
{ id: 'SP-01', conLai: 5 }
{ id: 'SP-01', conLai: 5 }
```

`timTheoId` LUÔN kiểm `dangCho` TRƯỚC (đọc được thay đổi CHƯA commit
của CHÍNH luồng công việc HIỆN TẠI), rồi FALLBACK về `daCommit`. Sau
`kho.luu({conLai:2})` rồi `kho.rollback()`: `dangCho` bị RESET (Map
MỚI, RỖNG), `daCommit` (VẪN giữ `conLai:5` từ LẦN COMMIT trước đó)
KHÔNG BỊ ẢNH HƯỞNG — thay đổi CHƯA commit BIẾN MẤT HOÀN TOÀN.
::::

::::example{#all-or-nothing-that}
Gộp HAI kho (`donHang`, `sanPham`) vào MỘT Unit of Work — đơn hàng
THỨ HAI **KHÔNG ĐỦ tồn kho** phải **rollback SẠCH**, KHÔNG làm hỏng
state đơn hàng ĐẦU đã commit TRƯỚC ĐÓ:

```typescript title=readonly
type MaDonHang = string & { readonly __brand: "MaDonHang" };
type DonHang = { id: MaDonHang; maSanPham: string; soLuong: number };
type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPhamTonKho = { id: MaSanPham; conLai: number };
function taoKhoUOW<Id extends string, ThucThe extends { id: Id }>() {
  const daCommit = new Map<Id, ThucThe>();
  let dangCho = new Map<Id, ThucThe>();
  return {
    timTheoId: (id: Id): ThucThe | null => dangCho.get(id) ?? daCommit.get(id) ?? null,
    luu: (thucThe: ThucThe): void => { dangCho.set(thucThe.id, thucThe); },
    commit: (): void => { dangCho.forEach((tt, id) => daCommit.set(id, tt)); dangCho = new Map(); },
    rollback: (): void => { dangCho = new Map(); },
  };
}

function taoUnitOfWork() {
  const donHang = taoKhoUOW<MaDonHang, DonHang>();
  const sanPham = taoKhoUOW<MaSanPham, SanPhamTonKho>();
  return {
    donHang,
    sanPham,
    commit: () => { donHang.commit(); sanPham.commit(); },
    rollback: () => { donHang.rollback(); sanPham.rollback(); },
  };
}

function datHang(uow: ReturnType<typeof taoUnitOfWork>, maDon: MaDonHang, maSP: MaSanPham, soLuong: number): boolean {
  const tonKho = uow.sanPham.timTheoId(maSP);
  if (tonKho === null || tonKho.conLai < soLuong) {
    uow.rollback();
    return false;
  }
  uow.donHang.luu({ id: maDon, maSanPham: maSP, soLuong });
  uow.sanPham.luu({ id: maSP, conLai: tonKho.conLai - soLuong });
  uow.commit();
  return true;
}

const uow = taoUnitOfWork();
uow.sanPham.luu({ id: "SP-01" as MaSanPham, conLai: 10 });
uow.commit();

const ketQua1 = datHang(uow, "DH-01" as MaDonHang, "SP-01" as MaSanPham, 3);
console.log(ketQua1, uow.sanPham.timTheoId("SP-01" as MaSanPham)?.conLai);

const ketQua2 = datHang(uow, "DH-02" as MaDonHang, "SP-01" as MaSanPham, 100);
console.log(ketQua2, uow.sanPham.timTheoId("SP-01" as MaSanPham)?.conLai);
console.log(uow.donHang.timTheoId("DH-02" as MaDonHang));
console.log(uow.donHang.timTheoId("DH-01" as MaDonHang));
```

```text title=readonly
true 7
false 7
null
{"id":"DH-01","maSanPham":"SP-01","soLuong":3}
```

Đơn 1 (đặt `3`, còn `10`): thành công, tồn kho còn `7`. Đơn 2 (đặt
`100`, còn `7`, KHÔNG đủ): `datHang` gọi `rollback()` NGAY, trả
`false` — tồn kho VẪN `7` (KHÔNG bị trừ THÊM), `DH-02` **KHÔNG TỒN
TẠI** trong `donHang` — nhưng `DH-01` (đã commit TỪ TRƯỚC) **VẪN
NGUYÊN VẸN**. Rollback CHỈ xoá thay đổi CHƯA commit CỦA LẦN GỌI HIỆN
TẠI, không "lùi" quá khứ ĐÃ commit.
::::

::::predict{#doan-uow-rollback-khong-anh-huong commitOnce}
```typescript
type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPhamTonKho = { id: MaSanPham; conLai: number };
function taoKhoUOW<Id extends string, ThucThe extends { id: Id }>() {
  const daCommit = new Map<Id, ThucThe>();
  let dangCho = new Map<Id, ThucThe>();
  return {
    timTheoId: (id: Id): ThucThe | null => dangCho.get(id) ?? daCommit.get(id) ?? null,
    luu: (thucThe: ThucThe): void => { dangCho.set(thucThe.id, thucThe); },
    commit: (): void => { dangCho.forEach((tt, id) => daCommit.set(id, tt)); dangCho = new Map(); },
    rollback: (): void => { dangCho = new Map(); },
  };
}

const kho = taoKhoUOW<MaSanPham, SanPhamTonKho>();
kho.luu({ id: "SP-09" as MaSanPham, conLai: 20 });
kho.commit();

// GHI thêm (chưa commit), rồi rollback NGAY -- không commit lần này
kho.luu({ id: "SP-09" as MaSanPham, conLai: 999 });
kho.rollback();
console.log(kho.timTheoId("SP-09" as MaSanPham)?.conLai);
```

Dòng cuối in ra gì?

:::opt{correct}
`20`
:::

:::opt
`999` — vì `luu()` GHI TRỰC TIẾP vào state của repository, và
`rollback()` chỉ RESET biến `dangCho` CỤC BỘ bên trong closure, không
ảnh hưởng gì tới GIÁ TRỊ đã GÁN vào `SP-09` từ trước đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG `luu()` LÀ hành động GHI DỮ LIỆU — quan
sát về việc `luu` "làm gì đó" với dữ liệu đó đúng.

Chỗ lệch: `luu({conLai: 999})` KHÔNG "ghi trực tiếp" vào state CUỐI
CÙNG — nó CHỈ `dangCho.set(...)`, đưa `999` vào Map **pending**.
`timTheoId` (bài học ở trên) đọc `dangCho` TRƯỚC — nhưng `rollback()`
được gọi **NGAY SAU** `luu`, `dangCho` bị GÁN LẠI thành `new Map()`
(RỖNG HOÀN TOÀN) — `999` KHÔNG BAO GIỜ được `commit()`, KHÔNG BAO GIỜ
"chảy" sang `daCommit`. `daCommit` VẪN giữ `conLai: 20` từ lần
`commit()` ĐẦU TIÊN — `timTheoId` SAU `rollback` fallback về
`daCommit`, đọc `20`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `kho.luu(...)` HAI LẦN với CÙNG `id`
(`"SP-09"`) không hợp lệ, `Map.set` không cho phép GHI ĐÈ một key ĐÃ
tồn tại
::why
Gần đúng ở việc bạn để ý `"SP-09"` xuất hiện HAI LẦN trong đoạn code
— một quan sát ĐÚNG về việc CÓ sự lặp lại đó.

Chỗ lệch: `Map.set(key, value)` trong JavaScript **HOÀN TOÀN HỢP LỆ**
khi gọi NHIỀU LẦN với CÙNG `key` — nó đơn giản GHI ĐÈ giá trị CŨ bằng
giá trị MỚI (đúng ý nghĩa "cập nhật" mà mọi kho dữ liệu key-value cần
có). Không có ràng buộc "chỉ set một lần cho mỗi key" nào trong `Map`
— biên dịch VÀ chạy hoàn toàn bình thường.
::
:::
::::

::::code{#viet_taokhouow}
Tự viết `timTheoId` và phần lõi của `commit` trong `taoKhoUOW`.

```typescript title=starter
function taoKhoUOW<Id extends string, ThucThe extends { id: Id }>() {
  const daCommit = new Map<Id, ThucThe>();
  let dangCho = new Map<Id, ThucThe>();
  return {
    timTheoId: (id: Id): ThucThe | null => ___,
    luu: (thucThe: ThucThe): void => { dangCho.set(thucThe.id, thucThe); },
    commit: (): void => {
      ___;
      dangCho = new Map();
    },
    rollback: (): void => { dangCho = new Map(); },
  };
}

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPhamTonKho = { id: MaSanPham; conLai: number };
const kho = taoKhoUOW<MaSanPham, SanPhamTonKho>();
kho.luu({ id: "SP-01" as MaSanPham, conLai: 5 });
console.log(kho.timTheoId("SP-01" as MaSanPham));
kho.commit();
console.log(kho.timTheoId("SP-01" as MaSanPham));
```

```typescript title=solution
function taoKhoUOW<Id extends string, ThucThe extends { id: Id }>() {
  const daCommit = new Map<Id, ThucThe>();
  let dangCho = new Map<Id, ThucThe>();
  return {
    timTheoId: (id: Id): ThucThe | null => dangCho.get(id) ?? daCommit.get(id) ?? null,
    luu: (thucThe: ThucThe): void => { dangCho.set(thucThe.id, thucThe); },
    commit: (): void => {
      dangCho.forEach((tt, id) => daCommit.set(id, tt));
      dangCho = new Map();
    },
    rollback: (): void => { dangCho = new Map(); },
  };
}

type MaSanPham = string & { readonly __brand: "MaSanPham" };
type SanPhamTonKho = { id: MaSanPham; conLai: number };
const kho = taoKhoUOW<MaSanPham, SanPhamTonKho>();
kho.luu({ id: "SP-01" as MaSanPham, conLai: 5 });
console.log(kho.timTheoId("SP-01" as MaSanPham));
kho.commit();
console.log(kho.timTheoId("SP-01" as MaSanPham));
```

```typescript title=test
type MaX = string & { readonly __brand: "MaX" };
type X = { id: MaX; gia: number };
const khoTest = taoKhoUOW<MaX, X>();

if (khoTest.timTheoId("X-01" as MaX) !== null) throw new Error("kho rỗng phải trả về null");

khoTest.luu({ id: "X-01" as MaX, gia: 100 });
if (khoTest.timTheoId("X-01" as MaX)?.gia !== 100) throw new Error("pending phải đọc được TRƯỚC khi commit");

khoTest.commit();
if (khoTest.timTheoId("X-01" as MaX)?.gia !== 100) throw new Error("dữ liệu phải VẪN đọc được SAU commit");

khoTest.luu({ id: "X-01" as MaX, gia: 999 });
khoTest.rollback();
if (khoTest.timTheoId("X-01" as MaX)?.gia !== 100) throw new Error("rollback phải quay về giá trị ĐÃ commit trước đó (100), không giữ giá trị chưa commit (999)");
```

:::hints
- kind: attention
  body: "timTheoId: kiểm dangCho (pending) TRƯỚC, fallback sang daCommit, cuối cùng null. commit: chuyển TOÀN BỘ dangCho sang daCommit bằng forEach, rồi RESET dangCho (dòng reset đã có sẵn, không cần viết lại)."
- kind: strategy
  body: "dangCho.get(id) ?? daCommit.get(id) ?? null : dangCho.forEach((tt, id) => daCommit.set(id, tt)) — hai thao tác cốt lõi của cơ chế pending/committed."
- kind: one-line
  body: "___ (timTheoId) = dangCho.get(id) ?? daCommit.get(id) ?? null\n___ (commit) = dangCho.forEach((tt, id) => daCommit.set(id, tt))"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "conLai"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Unit of Work: nhiều repo cùng thành công hoặc cùng thất bại, pending
trước committed. Bài cuối track: gom TẤT CẢ dependency vào một use
case hoàn chỉnh.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài BOSS cuối track: gom TẤT CẢ (repository, service khác, helper
thuần) vào MỘT object dependency, viết use case ĐẦY ĐỦ — validate, IO
tìm/lưu, gửi thông báo. Trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
