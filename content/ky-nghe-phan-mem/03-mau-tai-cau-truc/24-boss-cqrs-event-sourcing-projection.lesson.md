---
id: ky-nghe-phan-mem.mau-tai-cau-truc.boss-cqrs-event-sourcing-projection
title: "BOSS — Ghép CQRS + Event Sourcing + Projection thành một hệ nhỏ"
summary: "Bài BOSS track: một kho hàng nhỏ HOÀN CHỈNH theo CQRS+ES. WRITE side: xuLyLenh validate KHÔNG cho xuất QUÁ tồn kho HIỆN TẠI (tồn kho TÍNH LẠI từ sự kiện cũ mỗi lần, không lưu số riêng), trả Result<SuKienKho,LoiKho>. READ side: hai projection ĐỘC LẬP (tồn kho theo sản phẩm, lịch sử giao dịch) từ CÙNG một mảng sự kiện. Ghép trọn track: Strategy/Observer/Command/Visitor/Decorator/Middleware (hàm thuần + composition) và CQRS/ES/Projection (cụm này) vào MỘT bài thực hành cuối cùng."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 24
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.gate-boss-cqrs-es]
requires: [mau.projections-multiple-views]
concepts: [mau.gate-boss-cqrs-es]
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
Bài BOSS của track. Một kho hàng nhỏ: nhập/xuất hàng, KHÔNG được
xuất QUÁ tồn kho HIỆN CÓ. Ghép CQRS + Event Sourcing + Projection
thành MỘT hệ THỰC.
::::

::::explain{#kho-hang-cqrs-es}
WRITE side: `xuLyLenh` (bài 21's CQRS) validate — tồn kho HIỆN TẠI
được **TÍNH LẠI** TỪ sự kiện CŨ (bài 22's Event Sourcing, `tinhTonKho`
qua `reduce`), KHÔNG lưu số RIÊNG — trả `Result<SuKienKho, LoiKho>`
(đã học T4.5):

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

type LenhKho = { tag: "nhapKho"; maSp: string; soLuong: number } | { tag: "xuatKho"; maSp: string; soLuong: number };
type SuKienKho = { tag: "daNhapKho"; maSp: string; soLuong: number } | { tag: "daXuatKho"; maSp: string; soLuong: number };
type LoiKho = "khong_du_ton";

function tinhTonKho(cacSuKien: SuKienKho[], maSp: string): number {
  return cacSuKien.reduce((ton, sk) => {
    if (sk.maSp !== maSp) return ton;
    return sk.tag === "daNhapKho" ? ton + sk.soLuong : ton - sk.soLuong;
  }, 0);
}

function xuLyLenh(lenh: LenhKho, cacSuKienCu: SuKienKho[]): Result<SuKienKho, LoiKho> {
  if (lenh.tag === "nhapKho") {
    return { kind: "ok", giaTri: { tag: "daNhapKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
  }
  const tonHienTai = tinhTonKho(cacSuKienCu, lenh.maSp);
  if (lenh.soLuong > tonHienTai) {
    return { kind: "loi", loi: "khong_du_ton" };
  }
  return { kind: "ok", giaTri: { tag: "daXuatKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
}

// READ side (bài 23's Projection): hai view ĐỘC LẬP từ CÙNG một mảng sự kiện
function chieuTonKhoTheoSanPham(cacSuKien: SuKienKho[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    const hienTai = ketQua.get(sk.maSp) ?? 0;
    ketQua.set(sk.maSp, sk.tag === "daNhapKho" ? hienTai + sk.soLuong : hienTai - sk.soLuong);
  }
  return ketQua;
}
function chieuLichSuGiaoDich(cacSuKien: SuKienKho[]): string[] {
  return cacSuKien.map((sk) => `${sk.tag === "daNhapKho" ? "nhap" : "xuat"} ${sk.soLuong} ${sk.maSp}`);
}

let cacSuKien: SuKienKho[] = [];
const kq1 = xuLyLenh({ tag: "nhapKho", maSp: "A", soLuong: 100 }, cacSuKien);
if (kq1.kind === "ok") cacSuKien = [...cacSuKien, kq1.giaTri];

const kq2 = xuLyLenh({ tag: "xuatKho", maSp: "A", soLuong: 30 }, cacSuKien);
if (kq2.kind === "ok") cacSuKien = [...cacSuKien, kq2.giaTri];

const kq3 = xuLyLenh({ tag: "xuatKho", maSp: "A", soLuong: 1000 }, cacSuKien);
console.log(kq3.kind);

console.log(Object.fromEntries(chieuTonKhoTheoSanPham(cacSuKien)));
console.log(chieuLichSuGiaoDich(cacSuKien));
```

```text title=readonly
loi
{"A":70}
["nhap 100 A","xuat 30 A"]
```

Lệnh xuất `1000` (VƯỢT tồn kho `70`) bị TỪ CHỐI (`kq3.kind === "loi"`)
— KHÔNG sinh sự kiện MỚI, `cacSuKien` GIỮ NGUYÊN HAI mục. CẢ HAI
projection (`chieuTonKhoTheoSanPham`, `chieuLichSuGiaoDich`) đọc TỪ
`cacSuKien` ĐÓ — KHÔNG "biết" (VÀ KHÔNG CẦN biết) gì về lệnh BỊ TỪ
CHỐI.
::::

::::example{#xuat-dung-bang-ton-la-hop-le}
Ranh giới CHÍNH XÁC: `lenh.soLuong > tonHienTai` (KHÔNG PHẢI `>=`) —
xuất **ĐÚNG BẰNG** tồn kho hiện CÓ LÀ hợp lệ (làm sạch KHO hoàn
toàn):

```typescript title=readonly
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
type LenhKho = { tag: "nhapKho"; maSp: string; soLuong: number } | { tag: "xuatKho"; maSp: string; soLuong: number };
type SuKienKho = { tag: "daNhapKho"; maSp: string; soLuong: number } | { tag: "daXuatKho"; maSp: string; soLuong: number };
type LoiKho = "khong_du_ton";
function tinhTonKho(cacSuKien: SuKienKho[], maSp: string): number {
  return cacSuKien.reduce((ton, sk) => {
    if (sk.maSp !== maSp) return ton;
    return sk.tag === "daNhapKho" ? ton + sk.soLuong : ton - sk.soLuong;
  }, 0);
}
function xuLyLenh(lenh: LenhKho, cacSuKienCu: SuKienKho[]): Result<SuKienKho, LoiKho> {
  if (lenh.tag === "nhapKho") {
    return { kind: "ok", giaTri: { tag: "daNhapKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
  }
  const tonHienTai = tinhTonKho(cacSuKienCu, lenh.maSp);
  if (lenh.soLuong > tonHienTai) {
    return { kind: "loi", loi: "khong_du_ton" };
  }
  return { kind: "ok", giaTri: { tag: "daXuatKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
}

let cacSuKien: SuKienKho[] = [];
const kq1 = xuLyLenh({ tag: "nhapKho", maSp: "B", soLuong: 50 }, cacSuKien);
if (kq1.kind === "ok") cacSuKien = [...cacSuKien, kq1.giaTri];
const kq2 = xuLyLenh({ tag: "xuatKho", maSp: "B", soLuong: 50 }, cacSuKien);
console.log(kq2.kind);
```

```text title=readonly
ok
```

`tonHienTai = 50`, `soLuong = 50` — `50 > 50` LÀ `false`, LỆNH được
CHẤP NHẬN.
::::

::::predict{#doan-xuat-dung-bang-ton commitOnce}
```typescript
type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };
type LenhKho = { tag: "nhapKho"; maSp: string; soLuong: number } | { tag: "xuatKho"; maSp: string; soLuong: number };
type SuKienKho = { tag: "daNhapKho"; maSp: string; soLuong: number } | { tag: "daXuatKho"; maSp: string; soLuong: number };
type LoiKho = "khong_du_ton";
function tinhTonKho(cacSuKien: SuKienKho[], maSp: string): number {
  return cacSuKien.reduce((ton, sk) => {
    if (sk.maSp !== maSp) return ton;
    return sk.tag === "daNhapKho" ? ton + sk.soLuong : ton - sk.soLuong;
  }, 0);
}
function xuLyLenh(lenh: LenhKho, cacSuKienCu: SuKienKho[]): Result<SuKienKho, LoiKho> {
  if (lenh.tag === "nhapKho") {
    return { kind: "ok", giaTri: { tag: "daNhapKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
  }
  const tonHienTai = tinhTonKho(cacSuKienCu, lenh.maSp);
  if (lenh.soLuong > tonHienTai) {
    return { kind: "loi", loi: "khong_du_ton" };
  }
  return { kind: "ok", giaTri: { tag: "daXuatKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
}

let cacSuKien: SuKienKho[] = [];
const kq1 = xuLyLenh({ tag: "nhapKho", maSp: "C", soLuong: 20 }, cacSuKien);
if (kq1.kind === "ok") cacSuKien = [...cacSuKien, kq1.giaTri];
const kq2 = xuLyLenh({ tag: "xuatKho", maSp: "C", soLuong: 21 }, cacSuKien);
console.log(kq2.kind);
```

`soLuong = 21` (VƯỢT tồn kho `20` ĐÚNG MỘT ĐƠN VỊ). Dòng cuối in
ra gì?

:::opt{correct}
`loi`
:::

:::opt
`ok` — vì `21` VÀ `20` chỉ CHÊNH LỆCH đúng MỘT, ĐỦ GẦN để coi LÀ "về
cơ bản BẰNG NHAU" trong THỰC TẾ kinh doanh (sai số nhỏ CHẤP NHẬN
được)
::why
Gần đúng ở việc bạn ĐÚNG khi nhận ra `21` VÀ `20` GẦN NHAU — về mặt
TRỰC GIÁC con người, chênh lệch NHỎ.

Chỗ lệch: `xuLyLenh` KHÔNG có khái niệm "gần bằng" hay "sai số CHẤP
NHẬN được" — nó SO SÁNH SỐ HỌC CHÍNH XÁC: `lenh.soLuong > tonHienTai`
tức LÀ `21 > 20`, ĐÚNG LÀ `true` — CHỈ CẦN vượt quá **ĐÚNG MỘT ĐƠN
VỊ**, điều kiện ĐÃ ĐÚNG, LỆNH BỊ TỪ CHỐI. Phần mềm kho hàng THẬT hoạt
động CHÍNH XÁC tương tự: KHÔNG có "gần đủ hàng" — CHỈ CÓ "đủ" HOẶC
"không đủ".
::
:::

:::opt
Máy báo lỗi biên dịch — `xuLyLenh` khai kiểu trả về `Result<SuKienKho,
LoiKho>`, nhưng lời gọi `xuLyLenh(...)` KHÔNG ép kiểu (cast) kết quả
VỀ `Result<SuKienKho, LoiKho>` TRƯỚC khi đọc `.kind`, TypeScript đòi
ép kiểu TƯỜNG MINH cho generic type
::why
Gần đúng ở việc bạn để ý `Result<SuKienKho, LoiKho>` LÀ một kiểu
GENERIC (có tham số kiểu) — một quan sát ĐÚNG về ĐỊNH NGHĨA kiểu.

Chỗ lệch: TypeScript KHÔNG BAO GIỜ đòi ép kiểu TƯỜNG MINH khi hàm ĐÃ
khai kiểu trả về RÕ RÀNG (`Result<SuKienKho, LoiKho>`) — kết quả TRẢ
VỀ TỰ ĐỘNG có kiểu ĐÓ, đọc `.kind` NGAY được (SAU khi TypeScript hẹp
kiểu — narrowing — qua `if (kq.kind === "ok")`). Biên dịch SẠCH.
::
:::
::::

::::code{#viet_boss_cqrs_es}
Hoàn thiện `xuLyLenh` (điều kiện từ chối xuất kho) VÀ
`chieuTonKhoTheoSanPham` (cộng/trừ tồn kho theo sự kiện).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

type LenhKho = { tag: "nhapKho"; maSp: string; soLuong: number } | { tag: "xuatKho"; maSp: string; soLuong: number };
type SuKienKho = { tag: "daNhapKho"; maSp: string; soLuong: number } | { tag: "daXuatKho"; maSp: string; soLuong: number };
type LoiKho = "khong_du_ton";

function tinhTonKho(cacSuKien: SuKienKho[], maSp: string): number {
  return cacSuKien.reduce((ton, sk) => {
    if (sk.maSp !== maSp) return ton;
    return sk.tag === "daNhapKho" ? ton + sk.soLuong : ton - sk.soLuong;
  }, 0);
}

function xuLyLenh(lenh: LenhKho, cacSuKienCu: SuKienKho[]): Result<SuKienKho, LoiKho> {
  if (lenh.tag === "nhapKho") {
    return { kind: "ok", giaTri: { tag: "daNhapKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
  }
  const tonHienTai = tinhTonKho(cacSuKienCu, lenh.maSp);
  if (___) {
    return { kind: "loi", loi: "khong_du_ton" };
  }
  return { kind: "ok", giaTri: { tag: "daXuatKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
}

function chieuTonKhoTheoSanPham(cacSuKien: SuKienKho[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    const hienTai = ketQua.get(sk.maSp) ?? 0;
    ketQua.set(sk.maSp, ___);
  }
  return ketQua;
}

function chieuLichSuGiaoDich(cacSuKien: SuKienKho[]): string[] {
  return cacSuKien.map((sk) => `${sk.tag === "daNhapKho" ? "nhap" : "xuat"} ${sk.soLuong} ${sk.maSp}`);
}

let cacSuKien: SuKienKho[] = [];
const kq1 = xuLyLenh({ tag: "nhapKho", maSp: "A", soLuong: 100 }, cacSuKien);
if (kq1.kind === "ok") cacSuKien = [...cacSuKien, kq1.giaTri];
assertEqual(kq1.kind, "ok", "nhap kho luon thanh cong");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Result<T, E> = { kind: "ok"; giaTri: T } | { kind: "loi"; loi: E };

type LenhKho = { tag: "nhapKho"; maSp: string; soLuong: number } | { tag: "xuatKho"; maSp: string; soLuong: number };
type SuKienKho = { tag: "daNhapKho"; maSp: string; soLuong: number } | { tag: "daXuatKho"; maSp: string; soLuong: number };
type LoiKho = "khong_du_ton";

function tinhTonKho(cacSuKien: SuKienKho[], maSp: string): number {
  return cacSuKien.reduce((ton, sk) => {
    if (sk.maSp !== maSp) return ton;
    return sk.tag === "daNhapKho" ? ton + sk.soLuong : ton - sk.soLuong;
  }, 0);
}

function xuLyLenh(lenh: LenhKho, cacSuKienCu: SuKienKho[]): Result<SuKienKho, LoiKho> {
  if (lenh.tag === "nhapKho") {
    return { kind: "ok", giaTri: { tag: "daNhapKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
  }
  const tonHienTai = tinhTonKho(cacSuKienCu, lenh.maSp);
  if (lenh.soLuong > tonHienTai) {
    return { kind: "loi", loi: "khong_du_ton" };
  }
  return { kind: "ok", giaTri: { tag: "daXuatKho", maSp: lenh.maSp, soLuong: lenh.soLuong } };
}

function chieuTonKhoTheoSanPham(cacSuKien: SuKienKho[]): Map<string, number> {
  const ketQua = new Map<string, number>();
  for (const sk of cacSuKien) {
    const hienTai = ketQua.get(sk.maSp) ?? 0;
    ketQua.set(sk.maSp, sk.tag === "daNhapKho" ? hienTai + sk.soLuong : hienTai - sk.soLuong);
  }
  return ketQua;
}

function chieuLichSuGiaoDich(cacSuKien: SuKienKho[]): string[] {
  return cacSuKien.map((sk) => `${sk.tag === "daNhapKho" ? "nhap" : "xuat"} ${sk.soLuong} ${sk.maSp}`);
}

let cacSuKien: SuKienKho[] = [];
const kq1 = xuLyLenh({ tag: "nhapKho", maSp: "A", soLuong: 100 }, cacSuKien);
if (kq1.kind === "ok") cacSuKien = [...cacSuKien, kq1.giaTri];
assertEqual(kq1.kind, "ok", "nhap kho luon thanh cong");
```

```typescript title=test
const kq2 = xuLyLenh({ tag: "xuatKho", maSp: "A", soLuong: 30 }, cacSuKien);
if (kq2.kind === "ok") cacSuKien = [...cacSuKien, kq2.giaTri];
assertEqual(kq2.kind, "ok", "xuat trong pham vi ton kho thanh cong");

const kq3 = xuLyLenh({ tag: "xuatKho", maSp: "A", soLuong: 1000 }, cacSuKien);
assertEqual(kq3.kind, "loi", "xuat vuot ton kho bi tu choi");
if (kq3.kind === "loi") assertEqual(kq3.loi, "khong_du_ton", "ma loi dung");
assertEqual(cacSuKien.length, 2, "lenh bi tu choi KHONG sinh su kien moi");

const tonKho = chieuTonKhoTheoSanPham(cacSuKien);
assertEqual(tonKho.get("A"), 70, "ton kho A dung sau nhap 100 xuat 30");

const lichSu = chieuLichSuGiaoDich(cacSuKien);
assertEqual(lichSu.length, 2, "lich su co dung hai giao dich");
assertEqual(lichSu[0]!, "nhap 100 A", "giao dich dau dung");
assertEqual(lichSu[1]!, "xuat 30 A", "giao dich hai dung");

const kq4 = xuLyLenh({ tag: "xuatKho", maSp: "A", soLuong: 70 }, cacSuKien);
assertEqual(kq4.kind, "ok", "xuat dung bang ton kho con lai -- bien, van thanh cong");
```

:::hints
- kind: attention
  body: "Điều kiện từ chối: soLuong yêu cầu VƯỢT QUÁ tồn hiện tại (không phải >=, đúng bằng tồn vẫn hợp lệ). chieuTonKhoTheoSanPham: cộng khi nhập, trừ khi xuất — y hệt tinhTonKho nhưng gộp theo Map."
- kind: strategy
  body: "lenh.soLuong > tonHienTai : sk.tag === \"daNhapKho\" ? hienTai + sk.soLuong : hienTai - sk.soLuong"
- kind: one-line
  body: '___ (dieu kien tu choi) = lenh.soLuong > tonHienTai\n___ (chieuTonKho) = sk.tag === "daNhapKho" ? hienTai + sk.soLuong : hienTai - sk.soLuong'
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
Track HOÀN TẤT: 6 mẫu GoF "biến mất" thành hàm bậc cao/dữ liệu/
closure/composition, VÀ CQRS+ES+Projection ghép thành một hệ kho
hàng nhỏ, có validate, có lịch sử, có nhiều view — TẤT CẢ chỉ LÀ
hàm thuần và dữ liệu bất biến.
::::

::::reflect{#tong-ket-track}
Track "Mẫu thiết kế & Tái cấu trúc" khép lại Ở đây. Nhìn LẠI:
Strategy (hàm truyền vào), Observer (mảng hàm + đăng ký/huỷ),
Command (dữ liệu + hàm diễn giải + snapshot thay VÌ nghịch đảo),
Visitor (exhaustive switch trên ADT), Decorator (hàm bọc hàm),
Middleware (pipeline hàm) — SÁU mẫu GoF, KHÔNG mẫu NÀO cần class.
Extract Function VÀ Replace Conditional with Pattern Matching — HAI
kỹ thuật tái cấu trúc, ÁP DỤNG được cho MỌI hàm dài/rối. CQRS + Event
Sourcing + Projection — TÁCH ghi khỏi đọc, lưu sự kiện thay vì trạng
thái, chiếu MỘT nguồn thành NHIỀU view. Track TIẾP THEO của Realm 5
sẽ đưa những kỹ thuật NÀY vào bối cảnh LỚN hơn: CI/CD, kiểm thử tự
động Ở QUY MÔ hệ thống, VÀ observability.
::::

::::checkpoint{mastery=0.85}
::::
