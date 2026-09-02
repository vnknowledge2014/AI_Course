---
id: ky-nghe-phan-mem.mau-tai-cau-truc.capstone-undo-redo-ghi-chu
title: "Capstone: Undo/Redo cho ứng dụng ghi chú"
summary: "Bài chốt cụm 3: SoThao = {lichSu, viTri} — undo/redo di chuyển CON TRỎ viTri trong lichSu (KHÔNG xoá gì), một lệnh MỚI thì CẮT BỎ phần \"tương lai\" (nhánh redo cũ) trước khi thêm state mới. Ghép Command=DU (bài 10) + lịch sử bất biến (bài 11) vào một mô hình soạn thảo có Undo/Redo đúng nghĩa."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.gate-boss-command]
requires: [mau.immutable-undo-stack]
concepts: [mau.gate-boss-command]
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
Bài chốt cụm 3. Bài 11's `undo` XOÁ VĨNH VIỄN phần "tương lai" — KHÔNG
redo được. Sửa bằng CÁCH NÀO mà KHÔNG mất khả năng "quay lại"?
::::

::::explain{#con-tro-thay-vi-xoa}
Thay vì XOÁ phần tử cuối (bài 11), GIỮ NGUYÊN `lichSu` VÀ THÊM một
**CON TRỎ** `viTri` (chỉ SỐ của state HIỆN TẠI trong mảng) —
`SoThao = {lichSu: string[], viTri: number}`. `undo`/`redo` CHỈ di
chuyển `viTri` (KHÔNG đụng `lichSu`); MỘT lệnh MỚI mới THẬT SỰ cắt bỏ
phần "tương lai" (nếu đang KHÔNG đứng Ở cuối lịch sử):

```typescript title=readonly
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}

type SoThao = { lichSu: readonly string[]; viTri: number };
function trangThaiHienTai(st: SoThao): string {
  return st.lichSu[st.viTri] ?? "";
}

function thucHienLenh(st: SoThao, lenh: Lenh): SoThao {
  const moi = apDungLenh(trangThaiHienTai(st), lenh);
  const lichSuMoi = [...st.lichSu.slice(0, st.viTri + 1), moi];
  return { lichSu: lichSuMoi, viTri: lichSuMoi.length - 1 };
}

function undo(st: SoThao): SoThao {
  if (st.viTri <= 0) return st;
  return { ...st, viTri: st.viTri - 1 };
}

function redo(st: SoThao): SoThao {
  if (st.viTri >= st.lichSu.length - 1) return st;
  return { ...st, viTri: st.viTri + 1 };
}

let st: SoThao = { lichSu: [""], viTri: 0 };
st = thucHienLenh(st, { tag: "gopChu", text: "AB" });
st = thucHienLenh(st, { tag: "gopChu", text: "CD" });
console.log(trangThaiHienTai(st));
st = undo(st);
console.log(trangThaiHienTai(st));
st = redo(st);
console.log(trangThaiHienTai(st));
```

```text title=readonly
ABCD
AB
ABCD
```

`undo` LÙI `viTri` (KHÔNG cắt `lichSu`) — `"ABCD"` VẪN CÒN Ở vị trí
cuối mảng. `redo` TIẾN `viTri` LÊN, LẤY LẠI `"ABCD"` ĐÚNG CHÍNH XÁC —
KHÔNG mất gì cả, vì KHÔNG có gì bị XOÁ, CHỈ có con trỏ DI CHUYỂN.
`thucHienLenh` MỚI LÀ nơi THẬT SỰ cắt (`slice(0, st.viTri + 1)`) —
loại BỎ phần "tương lai" (nếu CÓ) TRƯỚC khi thêm state MỚI.
::::

::::example{#lenh-moi-xoa-nhanh-redo-cu}
Nếu, SAU khi undo, một lệnh MỚI được thực hiện (KHÔNG PHẢI redo) —
nhánh "tương lai" CŨ (đã bị undo QUA) bị **THAY THẾ VĨNH VIỄN**, ĐÚNG
hành vi Undo/Redo chuẩn của MỌI trình soạn thảo THẬT:

```typescript title=readonly
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}
type SoThao = { lichSu: readonly string[]; viTri: number };
function trangThaiHienTai(st: SoThao): string { return st.lichSu[st.viTri] ?? ""; }
function thucHienLenh(st: SoThao, lenh: Lenh): SoThao {
  const moi = apDungLenh(trangThaiHienTai(st), lenh);
  const lichSuMoi = [...st.lichSu.slice(0, st.viTri + 1), moi];
  return { lichSu: lichSuMoi, viTri: lichSuMoi.length - 1 };
}
function undo(st: SoThao): SoThao {
  if (st.viTri <= 0) return st;
  return { ...st, viTri: st.viTri - 1 };
}
function redo(st: SoThao): SoThao {
  if (st.viTri >= st.lichSu.length - 1) return st;
  return { ...st, viTri: st.viTri + 1 };
}

let st: SoThao = { lichSu: [""], viTri: 0 };
st = thucHienLenh(st, { tag: "gopChu", text: "Z" });
st = undo(st);
st = thucHienLenh(st, { tag: "gopChu", text: "Q" });
st = redo(st);
console.log(trangThaiHienTai(st));
console.log(st.lichSu.length);
```

```text title=readonly
Q
2
```

`"Z"` bị GHI ĐÈ (`thucHienLenh` SAU `undo` cắt bỏ nhánh chứa `"Z"`
TRƯỚC khi thêm `"Q"`) — `redo` SAU ĐÓ **KHÔNG** khôi phục được `"Z"`
NỮA (KHÔNG còn gì Ở phía trước để tiến tới, `viTri` ĐÃ Ở CUỐI). Đây
LÀ hành vi ĐÚNG (giống Word/VS Code THẬT): thực hiện một lệnh MỚI
SAU khi undo XOÁ VĨNH VIỄN nhánh "tương lai" cũ.
::::

::::predict{#doan-lich-su-chi-con-hai-muc commitOnce}
```typescript
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}
type SoThao = { lichSu: readonly string[]; viTri: number };
function trangThaiHienTai(st: SoThao): string { return st.lichSu[st.viTri] ?? ""; }
function thucHienLenh(st: SoThao, lenh: Lenh): SoThao {
  const moi = apDungLenh(trangThaiHienTai(st), lenh);
  const lichSuMoi = [...st.lichSu.slice(0, st.viTri + 1), moi];
  return { lichSu: lichSuMoi, viTri: lichSuMoi.length - 1 };
}
function undo(st: SoThao): SoThao {
  if (st.viTri <= 0) return st;
  return { ...st, viTri: st.viTri - 1 };
}

let st: SoThao = { lichSu: [""], viTri: 0 };
st = thucHienLenh(st, { tag: "gopChu", text: "A" });
st = thucHienLenh(st, { tag: "gopChu", text: "B" });
st = thucHienLenh(st, { tag: "gopChu", text: "C" });
st = undo(st);
st = thucHienLenh(st, { tag: "gopChu", text: "X" });
console.log(st.lichSu);
```

Dòng cuối in ra gì?

:::opt{correct}
`["","A","AB","ABX"]`
:::

:::opt
`["","A","AB","ABC","ABX"]` — vì `thucHienLenh` LUÔN THÊM state MỚI
vào **CUỐI** `lichSu` (`[...cu, moi]`), KHÔNG BAO GIỜ xoá phần tử NÀO
CÓ SẴN trong mảng
::why
Gần đúng ở việc bạn nhớ ĐÚNG `thucHienLenh` LUÔN kết thúc bằng việc
THÊM `moi` VÀO một MẢNG — quan sát ĐÓ, VỀ PHẦN "THÊM", CHÍNH XÁC.

Chỗ lệch: mảng được thêm vào **KHÔNG PHẢI** `st.lichSu` NGUYÊN VẸN —
nó LÀ `st.lichSu.slice(0, st.viTri + 1)` (CẮT trước, CHỈ giữ phần TỪ
ĐẦU tới VỊ TRÍ HIỆN TẠI). Ba lệnh ĐẦU đưa `lichSu` tới
`["","A","AB","ABC"]` với `viTri = 3` (trỏ `"ABC"`). `undo()` đưa
`viTri` VỀ `2` (trỏ `"AB"`) — `lichSu` CHƯA đổi gì. Lệnh MỚI
(`gopChu "X"`) tính `moi = apDungLenh("AB", ...) = "ABX"`, RỒI cắt
`slice(0, viTri+1=3)` = `["","A","AB"]` (LOẠI BỎ `"ABC"`, đứng SAU vị
trí hiện tại), RỒI nối `"ABX"` vào — kết quả ĐÚNG LÀ
`["","A","AB","ABX"]`, KHÔNG CÒN `"ABC"` NỮA.
::
:::

:::opt
Máy báo lỗi biên dịch — `thucHienLenh` khai kiểu trả về `SoThao`
nhưng gọi `undo` TRƯỚC ĐÓ đã đổi `st.viTri`, TypeScript ĐÒI kiểu
`SoThao` PHẢI "khớp" state TRƯỚC ĐÓ (viTri KHÔNG được PHÉP giảm rồi
tăng lại)
::why
Gần đúng ở việc bạn để ý `viTri` THAY ĐỔI qua NHIỀU lần gọi hàm (giảm
Ở `undo`, tăng Ở `thucHienLenh`) — một quan sát ĐÚNG VỀ HÀNH VI runtime.

Chỗ lệch: `SoThao = {lichSu: readonly string[], viTri: number}` LÀ
một KIỂU DỮ LIỆU ĐƠN GIẢN — `viTri` CHỈ LÀ một `number`, TypeScript
KHÔNG theo dõi (VÀ KHÔNG THỂ theo dõi) "giá trị NÀY đã TỪNG giảm hay
tăng" Ở CẤP ĐỘ kiểu. MỌI `SoThao` với `viTri` HỢP LỆ (nằm trong
khoảng chỉ số `lichSu`) ĐỀU được chấp nhận — biên dịch SẠCH.
::
:::
::::

::::code{#viet_capstone_undo_redo}
Hoàn thiện `thucHienLenh` (cắt nhánh tương lai RỒI thêm state mới)
VÀ `redo` (tiến con trỏ LÊN một bước).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}

type SoThao = { lichSu: readonly string[]; viTri: number };
function trangThaiHienTai(st: SoThao): string {
  return st.lichSu[st.viTri] ?? "";
}

function thucHienLenh(st: SoThao, lenh: Lenh): SoThao {
  const moi = apDungLenh(trangThaiHienTai(st), lenh);
  const lichSuMoi = ___;
  return { lichSu: lichSuMoi, viTri: lichSuMoi.length - 1 };
}

function undo(st: SoThao): SoThao {
  if (st.viTri <= 0) return st;
  return { ...st, viTri: st.viTri - 1 };
}

function redo(st: SoThao): SoThao {
  if (st.viTri >= st.lichSu.length - 1) return st;
  return ___;
}

let st: SoThao = { lichSu: [""], viTri: 0 };
st = thucHienLenh(st, { tag: "gopChu", text: "AB" });
assertEqual(trangThaiHienTai(st), "AB", "sau mot lenh");
st = thucHienLenh(st, { tag: "gopChu", text: "CD" });
assertEqual(trangThaiHienTai(st), "ABCD", "sau lenh thu hai");
st = undo(st);
assertEqual(trangThaiHienTai(st), "AB", "undo ve truoc");
st = redo(st);
assertEqual(trangThaiHienTai(st), "ABCD", "redo tien lai");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}

type SoThao = { lichSu: readonly string[]; viTri: number };
function trangThaiHienTai(st: SoThao): string {
  return st.lichSu[st.viTri] ?? "";
}

function thucHienLenh(st: SoThao, lenh: Lenh): SoThao {
  const moi = apDungLenh(trangThaiHienTai(st), lenh);
  const lichSuMoi = [...st.lichSu.slice(0, st.viTri + 1), moi];
  return { lichSu: lichSuMoi, viTri: lichSuMoi.length - 1 };
}

function undo(st: SoThao): SoThao {
  if (st.viTri <= 0) return st;
  return { ...st, viTri: st.viTri - 1 };
}

function redo(st: SoThao): SoThao {
  if (st.viTri >= st.lichSu.length - 1) return st;
  return { ...st, viTri: st.viTri + 1 };
}

let st: SoThao = { lichSu: [""], viTri: 0 };
st = thucHienLenh(st, { tag: "gopChu", text: "AB" });
assertEqual(trangThaiHienTai(st), "AB", "sau mot lenh");
st = thucHienLenh(st, { tag: "gopChu", text: "CD" });
assertEqual(trangThaiHienTai(st), "ABCD", "sau lenh thu hai");
st = undo(st);
assertEqual(trangThaiHienTai(st), "AB", "undo ve truoc");
st = redo(st);
assertEqual(trangThaiHienTai(st), "ABCD", "redo tien lai");
```

```typescript title=test
st = undo(st);
st = undo(st);
assertEqual(trangThaiHienTai(st), "", "undo ve trang thai dau");
st = thucHienLenh(st, { tag: "gopChu", text: "XY" });
assertEqual(trangThaiHienTai(st), "XY", "lenh moi sau khi undo");
st = redo(st);
assertEqual(trangThaiHienTai(st), "XY", "redo khong lam gi vi nhanh cu da bi thay the");
assertEqual(st.lichSu.length, 2, "lich su chi con hai muc, nhanh cu bi xoa");
```

:::hints
- kind: attention
  body: "thucHienLenh: cắt lichSu tới ĐÚNG viTri+1 phần tử (bỏ nhánh tương lai), rồi nối thêm moi. redo: tiến viTri lên 1, giữ nguyên lichSu."
- kind: strategy
  body: "[...st.lichSu.slice(0, st.viTri + 1), moi] : { ...st, viTri: st.viTri + 1 } — cắt-rồi-nối, và object mới với viTri tăng."
- kind: one-line
  body: '___ (thucHienLenh) = [...st.lichSu.slice(0, st.viTri + 1), moi]\n___ (redo) = { ...st, viTri: st.viTri + 1 }'
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
Cụm 3 hoàn tất: Command OOP shape, Command = dữ liệu, lịch sử bất
biến, capstone Undo/Redo con trỏ. Cụm tiếp theo: Visitor & Decorator.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`BieuThuc` (một cây phép tính, ví dụ `(2 + 3) * 4`) KHÔNG PHẢI một
chuỗi PHẲNG như `SoThao` — nó LÀ một CẤU TRÚC lồng nhau. `apDungLenh`
(hàm THUẦN, exhaustive switch, bài 10) có áp dụng được cho một CÂY
thay vì một CHUỖI không?
::::

::::checkpoint{mastery=0.8}
::::
