---
id: ky-nghe-phan-mem.mau-tai-cau-truc.ngan-xep-undo-bat-bien
title: "Ngăn xếp undo BẤT BIẾN — mảng snapshot, không mutate ngược"
summary: "Undo THẬT không cần hàm undo() nghịch đảo cho mỗi lệnh — chỉ cần GIỮ một mảng các state TRƯỚC MỖI lệnh (lichSu: string[]), \"undo\" = lấy state Ở VỊ TRÍ TRƯỚC ĐÓ. Giải quyết TRIỆT ĐỂ giới hạn bài 9: xoaKyTuCuoi giờ undo được CHÍNH XÁC (không cần đoán ký tự đã mất) vì state cũ VẪN CÒN NGUYÊN trong lịch sử."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.immutable-undo-stack]
requires: [mau.command-as-du]
concepts: [mau.immutable-undo-stack]
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
Bài 9's `xoaKyTuCuoi` KHÔNG THỂ undo đúng — ký tự đã mất. Nếu KHÔNG
đoán nghịch đảo, mà thay vào đó GIỮ LẠI trạng thái CŨ, vấn đề còn
không?
::::

::::explain{#giu-lai-thay-vi-doan-nguoc}
Undo THẬT KHÔNG cần hàm `undo()` NGƯỢC cho MỖI lệnh — chỉ cần GIỮ một
**MẢNG** các state TRƯỚC MỖI lệnh (`lichSu: string[]`). "Undo" =
LẤY state Ở VỊ TRÍ TRƯỚC ĐÓ trong mảng, KHÔNG "đoán" gì cả:

```typescript title=readonly
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}

function apDungLenhCoLichSu(lichSu: readonly string[], lenh: Lenh): readonly string[] {
  const hienTai = lichSu[lichSu.length - 1] ?? "";
  const moi = apDungLenh(hienTai, lenh);
  return [...lichSu, moi];
}

function undo(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}

let lichSu: readonly string[] = [""];
lichSu = apDungLenhCoLichSu(lichSu, { tag: "gopChu", text: "AB" });
lichSu = apDungLenhCoLichSu(lichSu, { tag: "xoaKyTuCuoi" });
console.log(lichSu[lichSu.length - 1]);
lichSu = undo(lichSu);
console.log(lichSu[lichSu.length - 1]);
```

```text title=readonly
A
AB
```

`apDungLenhCoLichSu` KHÔNG "sửa" state cũ — nó **ĐẨY THÊM** một state
MỚI vào CUỐI mảng (`[...lichSu, moi]`, bất biến HOÀN TOÀN). `undo`
CHỈ **CẮT** phần tử cuối (`slice`) — state TRƯỚC ĐÓ **VẪN CÒN NGUYÊN**
trong mảng, KHÔNG cần tính TOÁN LẠI hay đoán NGƯỢC gì cả.
::::

::::example{#giai-quyet-gioi-han-bai-9}
Đây LÀ điểm MẤU CHỐT: `xoaKyTuCuoi` (bài 9 KHÔNG undo đúng được) giờ
undo **CHÍNH XÁC** — vì ký tự "đã mất" VẪN CÒN trong state TRƯỚC ĐÓ:

```typescript title=readonly
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}
function apDungLenhCoLichSu(lichSu: readonly string[], lenh: Lenh): readonly string[] {
  const hienTai = lichSu[lichSu.length - 1] ?? "";
  const moi = apDungLenh(hienTai, lenh);
  return [...lichSu, moi];
}
function undo(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}

let lichSu: readonly string[] = ["Chao X"];
lichSu = apDungLenhCoLichSu(lichSu, { tag: "xoaKyTuCuoi" });
console.log(lichSu[lichSu.length - 1]);
lichSu = undo(lichSu);
console.log(lichSu[lichSu.length - 1]);
```

```text title=readonly
Chao 
Chao X
```

SO với bài 9 (`undo` trả `"Chao ?"` — SAI, đoán NHẦM ký tự): Ở ĐÂY
`undo` trả về ĐÚNG `"Chao X"` — vì `"Chao X"` chưa BAO GIỜ bị xoá,
nó VẪN NẰM trong `lichSu` Ở vị trí TRƯỚC. Đây LÀ lý do cách tiếp cận
"lưu snapshot" MẠNH HƠN "mỗi lệnh tự đoán nghịch đảo": nó hoạt động
CHO **MỌI** loại lệnh, kể cả những lệnh KHÔNG THỂ nghịch đảo được
(`xoaKyTuCuoi`, `lamMoi`).
::::

::::predict{#doan-undo-tai-diem-dau commitOnce}
```typescript
function undo(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}

let lichSu: readonly string[] = ["Xin"];
lichSu = undo(lichSu);
console.log(lichSu[lichSu.length - 1]);
console.log(lichSu.length);
```

Hai dòng cuối in ra gì? (`lichSu` BAN ĐẦU chỉ có ĐÚNG một phần tử.)

:::opt{correct}
`Xin` rồi `1`
:::

:::opt
`undefined` rồi `0` — vì điều kiện `lichSu.length <= 1` CHỈ chặn mảng
**RỖNG** (`length === 0`), KHÔNG chặn mảng có ĐÚNG một phần tử
(`length === 1`) — với `length === 1`, hàm VẪN chạy tới `slice(0, 0)`
(cắt HẾT, trả mảng RỖNG)
::why
Gần đúng ở việc bạn nhớ ĐÚNG có một RANH GIỚI liên quan tới "mảng
rỗng" trong bài toán NÀY — quan sát ĐÓ, VỀ HƯỚNG, không sai.

Chỗ lệch: `<=` (nhỏ hơn HOẶC BẰNG) **BAO GỒM CẢ** `1`, KHÔNG CHỈ `0`
— `lichSu.length <= 1` ĐÚNG với CẢ `length === 0` LẪN `length === 1`.
Với `lichSu = ["Xin"]` (`length === 1`), điều kiện `1 <= 1` LÀ `true`
— hàm `return lichSu` **NGAY**, KHÔNG chạy tới `slice`. `lichSu` GIỮ
NGUYÊN `["Xin"]`, KHÔNG đổi gì cả. Đây CHÍNH LÀ Ý NGHĨA của ranh giới
`<= 1`: "không undo QUA ĐƯỢC trạng thái ĐẦU TIÊN" (trạng thái BAN ĐẦU
LUÔN LÀ phần tử duy nhất khi `length === 1`).
::
:::

:::opt
Máy báo lỗi biên dịch — `lichSu` khai kiểu `readonly string[]`, hàm
`undo` gọi `lichSu.slice(...)` (một method MUTATE mảng), TypeScript
CẤM gọi method mutate trên kiểu `readonly`
::why
Gần đúng ở việc bạn để ý `lichSu` khai `readonly string[]` — TypeScript
THẬT SỰ chặn các method **MUTATE TẠI CHỖ** (như `.push`, `.sort`) trên
kiểu `readonly` — quan sát ĐÓ đúng NGUYÊN TẮC.

Chỗ lệch: `.slice()` **KHÔNG PHẢI** method mutate — nó LUÔN trả về
một **MẢNG MỚI**, KHÔNG đụng tới mảng gốc (giống `.map`/`.filter`,
KHÁC `.push`/`.splice`). `readonly string[]` cho phép gọi
`.slice()`/`.map()`/`.filter()` THOẢI MÁI — CHỈ cấm `.push()`,
`.pop()`, gán chỉ số (`arr[0] = ...`), VÀ tương tự. Biên dịch SẠCH.
::
:::
::::

::::code{#viet_lich_su_undo}
Hoàn thiện `apDungLenhCoLichSu` (đẩy state mới vào lịch sử) VÀ `undo`
(cắt bỏ mục cuối, GIỮ NGUYÊN nếu chỉ còn một mục).

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

function apDungLenhCoLichSu(lichSu: readonly string[], lenh: Lenh): readonly string[] {
  const hienTai = lichSu[lichSu.length - 1] ?? "";
  const moi = apDungLenh(hienTai, lenh);
  return ___;
}

function undo(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return ___;
}

let lichSu: readonly string[] = [""];
lichSu = apDungLenhCoLichSu(lichSu, { tag: "gopChu", text: "Hi" });
assertEqual(lichSu.length, 2, "lich su co hai muc sau mot lenh");
assertEqual(lichSu[lichSu.length - 1], "Hi", "trang thai hien tai la Hi");
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

function apDungLenhCoLichSu(lichSu: readonly string[], lenh: Lenh): readonly string[] {
  const hienTai = lichSu[lichSu.length - 1] ?? "";
  const moi = apDungLenh(hienTai, lenh);
  return [...lichSu, moi];
}

function undo(lichSu: readonly string[]): readonly string[] {
  if (lichSu.length <= 1) return lichSu;
  return lichSu.slice(0, lichSu.length - 1);
}

let lichSu: readonly string[] = [""];
lichSu = apDungLenhCoLichSu(lichSu, { tag: "gopChu", text: "Hi" });
assertEqual(lichSu.length, 2, "lich su co hai muc sau mot lenh");
assertEqual(lichSu[lichSu.length - 1], "Hi", "trang thai hien tai la Hi");
```

```typescript title=test
lichSu = apDungLenhCoLichSu(lichSu, { tag: "gopChu", text: "!!" });
assertEqual(lichSu[lichSu.length - 1], "Hi!!", "sau lenh thu hai");
lichSu = apDungLenhCoLichSu(lichSu, { tag: "xoaKyTuCuoi" });
assertEqual(lichSu[lichSu.length - 1], "Hi!", "sau xoa ky tu cuoi");
lichSu = undo(lichSu);
assertEqual(lichSu[lichSu.length - 1], "Hi!!", "undo khoi phuc DUNG ky tu da xoa");
lichSu = undo(lichSu);
lichSu = undo(lichSu);
assertEqual(lichSu[lichSu.length - 1], "", "undo ve toi trang thai dau tien");
assertEqual(lichSu.length, 1, "khong undo qua duoc trang thai dau");
lichSu = undo(lichSu);
assertEqual(lichSu.length, 1, "undo lan nua tren trang thai dau van an toan");
```

:::hints
- kind: attention
  body: "apDungLenhCoLichSu: ĐẨY moi vào CUỐI lichSu (mảng mới, giữ nguyên các mục cũ). undo: CẮT mục cuối, giữ lại phần còn lại."
- kind: strategy
  body: "[...lichSu, moi] : lichSu.slice(0, lichSu.length - 1) — mảng mới nối thêm phần tử, và mảng mới bỏ phần tử cuối."
- kind: one-line
  body: '___ (apDungLenhCoLichSu) = [...lichSu, moi]\n___ (undo) = lichSu.slice(0, lichSu.length - 1)'
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
Undo = quay lại snapshot, KHÔNG cần đoán nghịch đảo. Cụm gần xong:
ghép TOÀN BỘ (Command=DU, apDụng, lịch sử) vào một ứng dụng ghi chú.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`undo` di chuyển "con trỏ" VỀ MỘT bước — nếu di chuyển con trỏ ẤY
TIẾN LÊN một bước (SAU khi vừa undo, TRƯỚC khi có lệnh MỚI nào), đó
CHÍNH LÀ "redo". Bạn hình dung được cách LÀM ĐIỀU đó chưa?
::::

::::checkpoint{mastery=0.8}
::::
