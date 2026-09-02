---
id: ky-nghe-phan-mem.ddd.reader-pattern-dong-goi-dependencies
title: "Reader Pattern — factory nhận dependencies MỘT LẦN, dùng lại nhiều hàm"
summary: "Factory function nhận deps MỘT LẦN, trả về TẬP hàm đã \"khoá\" deps trong closure. taoDichVuGiaCa(deps) trả về {tinhTong, layPhiVanChuyen, tinhGiaCuoi} — cả ba dùng chung deps mà không cần truyền lại."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [ddd.reader-pattern]
requires: [ddd.fp-dependency-injection]
concepts: [ddd.reader-pattern]
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
Truyền dependency làm tham số ở MỖI lời gọi hàm — bài 10 đã dạy. Nếu
CÙNG dependency cần cho BA hàm khác nhau, phải truyền BA LẦN?
::::

::::explain{#reader-pattern}
**Reader Pattern**: một FACTORY FUNCTION nhận dependencies MỘT LẦN, trả
về TẬP hàm đã "khoá" deps trong CLOSURE — mỗi lần gọi các hàm trong tập
đó, KHÔNG cần truyền lại `deps`:

```typescript
type PhiVanChuyenDeps = { phiCoBan: number; phiTheoKg: number };

function taoDichVuGiaCa(deps: PhiVanChuyenDeps) {
  function tinhTong(giaHang: number, soLuong: number): number {
    return giaHang * soLuong;
  }
  function layPhiVanChuyen(trongLuongKg: number): number {
    return deps.phiCoBan + deps.phiTheoKg * trongLuongKg;
  }
  function tinhGiaCuoi(giaHang: number, soLuong: number, trongLuongKg: number): number {
    return tinhTong(giaHang, soLuong) + layPhiVanChuyen(trongLuongKg);
  }
  return { tinhTong, layPhiVanChuyen, tinhGiaCuoi };
}

const dichVu = taoDichVuGiaCa({ phiCoBan: 20000, phiTheoKg: 5000 });
console.log(dichVu.tinhTong(50000, 2));
console.log(dichVu.layPhiVanChuyen(3));
console.log(dichVu.tinhGiaCuoi(50000, 2, 3));
```

```text
100000
35000
135000
```

`taoDichVuGiaCa(deps)` gọi MỘT LẦN — `deps` bị "KHOÁ" vào closure của
BA hàm bên trong. Từ đó, `dichVu.tinhTong(...)`, `dichVu.layPhiVanChuyen(...)`,
`dichVu.tinhGiaCuoi(...)` gọi được KHÔNG CẦN nhắc lại `deps` — và
`tinhGiaCuoi` còn GỌI LẠI `tinhTong`/`layPhiVanChuyen` NỘI BỘ, tự động
dùng CHUNG `deps` đó.

**Interface định nghĩa Ở Domain, implement Ở Infra** (nối bài 9): domain
khai `type KhoDonHang = { timTheoId, luu }` (SHAPE nó CẦN) — infra viết
implementation THẬT sau — domain phụ thuộc ABSTRACTION (cái SHAPE),
KHÔNG phụ thuộc implementation cụ thể nào.
::::

::::example{#khong-can-truyen-lai}
So sánh SỐ LẦN truyền `deps` — FP DI thường (bài 10) vs Reader Pattern:

```typescript title=readonly
type PhiVanChuyenDeps = { phiCoBan: number; phiTheoKg: number };

// Cách bài 10: truyền deps Ở MỖI lời gọi
function layPhiVanChuyenThuong(deps: PhiVanChuyenDeps, trongLuongKg: number): number {
  return deps.phiCoBan + deps.phiTheoKg * trongLuongKg;
}

// Cách Reader Pattern: truyền deps MỘT LẦN duy nhất
function taoDichVuGiaCa(deps: PhiVanChuyenDeps) {
  function layPhiVanChuyen(trongLuongKg: number): number {
    return deps.phiCoBan + deps.phiTheoKg * trongLuongKg;
  }
  return { layPhiVanChuyen };
}

const depsChung = { phiCoBan: 20000, phiTheoKg: 5000 };

// Cách thường: nhắc lại depsChung ở MỖI lời gọi
console.log(layPhiVanChuyenThuong(depsChung, 1));
console.log(layPhiVanChuyenThuong(depsChung, 2));

// Cách Reader: nhắc MỘT LẦN, gọi nhiều lần sau đó
const dichVu = taoDichVuGiaCa(depsChung);
console.log(dichVu.layPhiVanChuyen(1));
console.log(dichVu.layPhiVanChuyen(2));
```

```text title=readonly
25000
30000
25000
30000
```

CÙNG kết quả — nhưng cách Reader Pattern chỉ nhắc `depsChung` ĐÚNG MỘT
LẦN (lúc gọi `taoDichVuGiaCa`), trong khi cách thường nhắc lại ở MỖI
lời gọi hàm. Khi có NHIỀU hàm cùng cần `deps`, chênh lệch càng rõ.
::::

::::predict{#doan-hai-dich-vu-khac-nhau commitOnce}
```typescript
type PhiVanChuyenDeps = { phiCoBan: number; phiTheoKg: number };
function taoDichVuGiaCa(deps: PhiVanChuyenDeps) {
  function layPhiVanChuyen(trongLuongKg: number): number {
    return deps.phiCoBan + deps.phiTheoKg * trongLuongKg;
  }
  return { layPhiVanChuyen };
}

const dichVuA = taoDichVuGiaCa({ phiCoBan: 10000, phiTheoKg: 2000 });
const dichVuB = taoDichVuGiaCa({ phiCoBan: 50000, phiTheoKg: 2000 });

console.log(dichVuA.layPhiVanChuyen(5));
console.log(dichVuB.layPhiVanChuyen(5));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`20000` rồi `60000`
:::

:::opt
`60000` rồi `60000` — vì `dichVuB` được tạo SAU, nó "ghi đè" deps cho
CẢ HAI biến, vì cả hai đều gọi CHUNG hàm `taoDichVuGiaCa`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `dichVuA` và `dichVuB` CÙNG được tạo TỪ
MỘT hàm `taoDichVuGiaCa` — quan sát về việc dùng CHUNG factory đó
đúng.

Chỗ lệch: MỖI lần GỌI `taoDichVuGiaCa(...)` tạo một CLOSURE MỚI, RIÊNG
BIỆT, với `deps` CỦA RIÊNG lần gọi đó — `dichVuA`'s closure "nhớ"
`{phiCoBan: 10000, ...}`, `dichVuB`'s closure "nhớ"
`{phiCoBan: 50000, ...}` — HAI closure ĐỘC LẬP, không hề "ghi đè" lẫn
nhau. `dichVuA.layPhiVanChuyen(5)` = `10000 + 2000*5` = `20000`;
`dichVuB.layPhiVanChuyen(5)` = `50000 + 2000*5` = `60000`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `taoDichVuGiaCa` HAI LẦN với hai object
`deps` KHÁC NHAU không hợp lệ, vì TypeScript coi `deps` là kiểu CỐ ĐỊNH
sau lần gọi đầu tiên
::why
Gần đúng ở việc bạn để ý CÓ HAI object `deps` KHÁC NHAU được truyền
vào — quan sát về việc có SỰ khác biệt đó đúng.

Chỗ lệch: `taoDichVuGiaCa` là một hàm BÌNH THƯỜNG — gọi được BAO NHIÊU
LẦN tuỳ ý, MỖI lần với đối số RIÊNG, miễn khớp kiểu `PhiVanChuyenDeps`.
TypeScript KHÔNG "khoá" kiểu tham số sau lần gọi đầu — biên dịch và
chạy hoàn toàn bình thường cho CẢ hai lần gọi.
::
:::
::::

::::code{#viet_tao_dich_vu_gia_ca}
Tự viết `taoDichVuGiaCa(deps): {tinhTong, layPhiVanChuyen, tinhGiaCuoi}`.

```typescript title=starter
type PhiVanChuyenDeps = { phiCoBan: number; phiTheoKg: number };

function taoDichVuGiaCa(deps: PhiVanChuyenDeps) {
  function tinhTong(giaHang: number, soLuong: number): number {
    return ___;
  }
  function layPhiVanChuyen(trongLuongKg: number): number {
    return ___;
  }
  function tinhGiaCuoi(giaHang: number, soLuong: number, trongLuongKg: number): number {
    return ___;
  }
  return { tinhTong, layPhiVanChuyen, tinhGiaCuoi };
}

const dichVu = taoDichVuGiaCa({ phiCoBan: 20000, phiTheoKg: 5000 });
console.log(dichVu.tinhGiaCuoi(50000, 2, 3));
```

```typescript title=solution
type PhiVanChuyenDeps = { phiCoBan: number; phiTheoKg: number };

function taoDichVuGiaCa(deps: PhiVanChuyenDeps) {
  function tinhTong(giaHang: number, soLuong: number): number {
    return giaHang * soLuong;
  }
  function layPhiVanChuyen(trongLuongKg: number): number {
    return deps.phiCoBan + deps.phiTheoKg * trongLuongKg;
  }
  function tinhGiaCuoi(giaHang: number, soLuong: number, trongLuongKg: number): number {
    return tinhTong(giaHang, soLuong) + layPhiVanChuyen(trongLuongKg);
  }
  return { tinhTong, layPhiVanChuyen, tinhGiaCuoi };
}

const dichVu = taoDichVuGiaCa({ phiCoBan: 20000, phiTheoKg: 5000 });
console.log(dichVu.tinhGiaCuoi(50000, 2, 3));
```

```typescript title=test
const dv = taoDichVuGiaCa({ phiCoBan: 20000, phiTheoKg: 5000 });
if (dv.tinhTong(50000, 2) !== 100000) throw new Error("tinhTong(50000, 2) phải ra 100000 (50000 * 2)");
if (dv.layPhiVanChuyen(3) !== 35000) throw new Error("layPhiVanChuyen(3) phải ra 35000 (20000 + 5000*3)");
if (dv.tinhGiaCuoi(50000, 2, 3) !== 135000) throw new Error("tinhGiaCuoi phải ra 135000 (100000 + 35000)");

const dv2 = taoDichVuGiaCa({ phiCoBan: 10000, phiTheoKg: 1000 });
if (dv2.layPhiVanChuyen(4) !== 14000) throw new Error("mỗi lần gọi taoDichVuGiaCa phải dùng ĐÚNG deps của lần gọi đó, không lẫn với lần khác");
```

:::hints
- kind: attention
  body: "tinhTong: nhân giaHang với soLuong. layPhiVanChuyen: cộng deps.phiCoBan với (deps.phiTheoKg nhân trongLuongKg). tinhGiaCuoi: gọi LẠI cả hai hàm vừa viết, cộng kết quả."
- kind: strategy
  body: "giaHang * soLuong — cho tinhTong. deps.phiCoBan + deps.phiTheoKg * trongLuongKg — cho layPhiVanChuyen. tinhTong(giaHang, soLuong) + layPhiVanChuyen(trongLuongKg) — cho tinhGiaCuoi."
- kind: one-line
  body: "___ (tinhTong) = giaHang * soLuong\n___ (layPhiVanChuyen) = deps.phiCoBan + deps.phiTheoKg * trongLuongKg\n___ (tinhGiaCuoi) = tinhTong(giaHang, soLuong) + layPhiVanChuyen(trongLuongKg)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "135000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Reader Pattern: factory nhận deps MỘT LẦN, trả về tập hàm dùng CHUNG
closure đó — không cần truyền lại deps ở mỗi lời gọi.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn có TẤT CẢ mảnh ghép — Domain thuần, Infra interfaces, FP DI, Reader
Pattern. Ghép LẠI thành MỘT use case hoàn chỉnh trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
