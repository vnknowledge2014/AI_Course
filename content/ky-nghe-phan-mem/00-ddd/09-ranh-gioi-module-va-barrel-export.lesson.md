---
id: ky-nghe-phan-mem.ddd.ranh-gioi-module-va-barrel-export
title: "Module Boundaries & Barrel Exports — index.ts là cổng công khai"
summary: "index.ts = public API của một module — chỉ export những gì module MUỐN lộ ra, phần còn lại PRIVATE. Module = Bounded Context: mỗi module một cấu trúc thống nhất. Quy tắc import cụ thể hoá Dependency Rule ở cấp module."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [ddd.module-boundaries]
requires: [ddd.fc-is-at-scale]
concepts: [ddd.module-boundaries]
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
Bốn tầng — bài 7 đã dạy. Nhưng trong MỖI tầng, code chia thành nhiều
"module" — ai được PHÉP thấy những gì?
::::

::::explain{#index-ts-la-cong}
Trong một dự án THẬT, `index.ts` = **public API** của một module — chỉ
export những gì module ĐÓ MUỐN lộ ra ngoài, phần còn lại là PRIVATE
(chi tiết implementation, không ai bên ngoài cần biết). Module Đơn
Hàng, Module Thanh Toán, Module Vận Chuyển — mỗi module MỘT "cổng công
khai" DUY NHẤT.

Sandbox ở đây chỉ chạy MỘT file (không có nhiều file thật để `import`/
`export` giữa chúng) — nhưng Ý TƯỞNG mô phỏng được bằng MỘT object
gom lại "những gì public", tách biệt khỏi các hàm "nội bộ":

```typescript
type DonHang = { maDonHang: string; tongTien: number };

// --- nội bộ module: KHÔNG nên dùng từ bên ngoài (tương đương "private") ---
function tinhThueNoiBo(tongTien: number): number {
  return tongTien * 0.1;
}

// --- public API của module (tương đương "index.ts") ---
function taoDonHang(maDonHang: string, tongTien: number): DonHang {
  return { maDonHang, tongTien };
}
function tinhTongCoThue(dh: DonHang): number {
  return dh.tongTien + tinhThueNoiBo(dh.tongTien);
}

const ModuleDonHang = { taoDonHang, tinhTongCoThue };

// "module khác" chỉ dùng qua ModuleDonHang — KHÔNG gọi tinhThueNoiBo trực tiếp
const dh = ModuleDonHang.taoDonHang("DH-01", 100000);
console.log(ModuleDonHang.tinhTongCoThue(dh));
```

```text
110000
```

`tinhThueNoiBo` KHÔNG nằm trong `ModuleDonHang` — "module khác" chỉ
NHÌN THẤY `taoDonHang`/`tinhTongCoThue` qua object đó, KHÔNG BIẾT gì
về CÁCH thuế được tính bên trong. Đây LÀ đúng tinh thần barrel export:
`ModuleDonHang` đóng vai `index.ts`, những hàm KHÔNG nằm trong nó là
"chi tiết riêng tư" của module.
::::

::::example{#module-la-bounded-context}
Module = Bounded Context (nối bài 3): mỗi module một cấu trúc thống
nhất, TÊN gọi TRỰC TIẾP ánh xạ ranh giới nghiệp vụ:

```typescript title=readonly
// Cấu trúc thư mục THẬT của một dự án (chỉ MÔ TẢ, không chạy được ở đây):
//
// src/
//   don-hang/
//     types.ts     — DonHang, TrangThaiDonHang
//     domain.ts     — logic thuần: validateDonHang, tinhTongCoThue
//     index.ts      — CHỈ export những gì module khác cần
//   thanh-toan/
//     types.ts
//     domain.ts
//     index.ts
//   van-chuyen/
//     types.ts
//     domain.ts
//     index.ts

// Quy tắc import (cụ thể hoá Dependency Rule bài 7 ở CẤP MODULE):
// ✅ import { taoDonHang } from "../don-hang";        (từ barrel — index.ts)
// ❌ import { tinhThueNoiBo } from "../don-hang/domain"; (từ ĐƯỜNG DẪN NỘI BỘ)
console.log("cấu trúc module — xem comment phía trên");
```

```text title=readonly
cấu trúc module — xem comment phía trên
```

Quy tắc: import LUÔN đi qua BARREL (`index.ts`), KHÔNG BAO GIỜ đi
thẳng vào file nội bộ (`domain.ts`) của MỘT module KHÁC. Domain→bất kỳ
tầng ngoài nào = CẤM (bài 7); ở CẤP module, quy tắc tương tự: chỉ được
chạm những gì module ĐÓ chủ động PHƠI ra qua barrel.
::::

::::predict{#doan-goi-ham-noi-bo commitOnce}
```typescript
type DonHang = { maDonHang: string; tongTien: number };
function tinhThueNoiBo(tongTien: number): number {
  return tongTien * 0.1;
}
function taoDonHang(maDonHang: string, tongTien: number): DonHang {
  return { maDonHang, tongTien };
}
function tinhTongCoThue(dh: DonHang): number {
  return dh.tongTien + tinhThueNoiBo(dh.tongTien);
}
const ModuleDonHang = { taoDonHang, tinhTongCoThue };

console.log("tinhThueNoiBo" in ModuleDonHang);
```

Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `tinhThueNoiBo` VẪN được `tinhTongCoThue` gọi TỪ BÊN TRONG
`ModuleDonHang`, nên nó cũng được coi là một phần của object đó
::why
Gần đúng ở việc bạn nhớ ĐÚNG `tinhThueNoiBo` ĐƯỢC dùng — nó THẬT SỰ
được `tinhTongCoThue` gọi (một lời gọi HÀM, không phải một PHÉP GÁN
FIELD).

Chỗ lệch: `ModuleDonHang` chỉ CHỨA những field ĐƯỢC GÁN TƯỜNG MINH lúc
khai báo — `{ taoDonHang, tinhTongCoThue }` — CHỈ HAI field đó.
`tinhThueNoiBo` là một hàm ĐỘC LẬP, được `tinhTongCoThue` gọi NGẦM bên
trong THÂN hàm — điều đó KHÔNG khiến nó tự động trở thành MỘT FIELD
của `ModuleDonHang`. `"tinhThueNoiBo" in ModuleDonHang` kiểm object có
KEY đó không — không có, trả `false`.
::
:::

:::opt
Máy báo lỗi biên dịch — `tinhThueNoiBo` không nằm trong `ModuleDonHang`
nên TypeScript coi đây là một tham chiếu tới BIẾN CHƯA khai báo
::why
Gần đúng ở việc bạn để ý `tinhThueNoiBo` KHÔNG nằm trong
`ModuleDonHang` — quan sát đó đúng.

Chỗ lệch: `"tinhThueNoiBo"` ở đây là một CHUỖI KÝ TỰ (literal string),
KHÔNG phải một tham chiếu tới biến `tinhThueNoiBo` — toán tử `in` kiểm
xem MỘT CHUỖI TÊN có LÀ key của object không, hoàn toàn hợp lệ về cú
pháp và kiểu, không hề đụng tới biến `tinhThueNoiBo` nào cả. Biên dịch
và chạy bình thường.
::
:::
::::

::::code{#viet_module_don_hang}
Tự viết `ModuleDonHang` — barrel export CHỈ chứa hai hàm public, giữ
`tinhThueNoiBo` riêng tư.

```typescript title=starter
type DonHang = { maDonHang: string; tongTien: number };

function tinhThueNoiBo(tongTien: number): number {
  return tongTien * 0.1;
}
function taoDonHang(maDonHang: string, tongTien: number): DonHang {
  return { maDonHang, tongTien };
}
function tinhTongCoThue(dh: DonHang): number {
  return ___;
}

const ModuleDonHang = { taoDonHang, tinhTongCoThue: ___ };

const dh = ModuleDonHang.taoDonHang("DH-01", 100000);
console.log(ModuleDonHang.tinhTongCoThue(dh));
```

```typescript title=solution
type DonHang = { maDonHang: string; tongTien: number };

function tinhThueNoiBo(tongTien: number): number {
  return tongTien * 0.1;
}
function taoDonHang(maDonHang: string, tongTien: number): DonHang {
  return { maDonHang, tongTien };
}
function tinhTongCoThue(dh: DonHang): number {
  return dh.tongTien + tinhThueNoiBo(dh.tongTien);
}

const ModuleDonHang = { taoDonHang, tinhTongCoThue };

const dh = ModuleDonHang.taoDonHang("DH-01", 100000);
console.log(ModuleDonHang.tinhTongCoThue(dh));
```

```typescript title=test
const dh1 = ModuleDonHang.taoDonHang("DH-01", 100000);
if (dh1.maDonHang !== "DH-01") throw new Error("taoDonHang phải giữ đúng mã đơn hàng");
if (ModuleDonHang.tinhTongCoThue(dh1) !== 110000) throw new Error("tinhTongCoThue(100000) phải ra 110000 (100000 + thuế 10000)");

const dh2 = ModuleDonHang.taoDonHang("DH-02", 200000);
if (ModuleDonHang.tinhTongCoThue(dh2) !== 220000) throw new Error("tinhTongCoThue(200000) phải ra 220000 (200000 + thuế 20000)");

if ("tinhThueNoiBo" in ModuleDonHang) throw new Error("tinhThueNoiBo phải là hàm RIÊNG TƯ, không nằm trong ModuleDonHang (barrel export)");
```

:::hints
- kind: attention
  body: "tinhTongCoThue: cộng tongTien với thuế (gọi tinhThueNoiBo). ModuleDonHang: gán tinhTongCoThue là CHÍNH hàm tinhTongCoThue đã viết (không viết lại logic)."
- kind: strategy
  body: "dh.tongTien + tinhThueNoiBo(dh.tongTien) — cho tinhTongCoThue. tinhTongCoThue — cho field trong ModuleDonHang (shorthand cùng tên, hoặc viết đầy đủ tinhTongCoThue: tinhTongCoThue)."
- kind: one-line
  body: "___ (tinhTongCoThue) = dh.tongTien + tinhThueNoiBo(dh.tongTien)\n___ (field trong Module) = tinhTongCoThue"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "110000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`index.ts` — cổng công khai của một module, CHỈ export những gì module
MUỐN lộ ra. Import LUÔN qua barrel, KHÔNG BAO GIỜ đi thẳng vào chi tiết
nội bộ của module khác.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bốn tầng, ranh giới module rõ ràng — nhưng LÀM SAO domain function
"nhận" được một dependency (như đồng hồ hệ thống, hay bộ sinh ID) mà
KHÔNG cần import trực tiếp thứ tạo ra chúng?
::::

::::checkpoint{mastery=0.8}
::::
