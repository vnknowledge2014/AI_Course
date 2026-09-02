---
id: ky-nghe-phan-mem.mau-tai-cau-truc.command-la-du-lieu
title: "Command = DỮ LIỆU — Discriminated Union thay vì class"
summary: "Một lệnh CHỈ LÀ dữ liệu mô tả ý định (ADT). apDungLenh(vanBan, lenh): string LÀ một hàm THUẦN, exhaustive switch trên lenh.tag — tách LOGIC (apDungLenh) khỏi Ý ĐỊNH (giá trị Lenh). Vì command LÀ dữ liệu, một MẢNG lệnh (kế hoạch) lưu được, truyền được, và replay được qua reduce."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.command-as-du]
requires: [mau.command-oop-shape]
concepts: [mau.command-as-du]
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
Bài 9's `{execute, undo}` MỖI lệnh TỰ chứa CÁCH thực thi. Nếu MỘT lệnh
CHỈ LÀ dữ liệu — KHÔNG chứa hàm gì cả — thì AI thực thi nó?
::::

::::explain{#lenh-la-du-lieu-mo-ta-y-dinh}
Một "lệnh" CHỈ LÀ dữ liệu **MÔ TẢ Ý ĐỊNH** — một ADT (Discriminated
Union, đã học T4.3): `Lenh = {tag:"gopChu", text} | {tag:"xoaKyTuCuoi"}
| {tag:"lamMoi"}`. MỘT hàm THUẦN RIÊNG (`apDungLenh`) diễn giải dữ
liệu đó — exhaustive `switch` trên `lenh.tag`:

```typescript title=readonly
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };

function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu":
      return vanBan + lenh.text;
    case "xoaKyTuCuoi":
      return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi":
      return "";
  }
}

let vb = "";
vb = apDungLenh(vb, { tag: "gopChu", text: "Xin chao" });
console.log(vb);
vb = apDungLenh(vb, { tag: "xoaKyTuCuoi" });
console.log(vb);
vb = apDungLenh(vb, { tag: "lamMoi" });
console.log(vb);
```

```text title=readonly
Xin chao
Xin cha

```

LOGIC (`apDungLenh`) VÀ Ý ĐỊNH (giá trị `Lenh`) giờ **TÁCH RIÊNG** —
`{ tag: "gopChu", text: "Xin chao" }` KHÔNG "biết" cách thực thi
CHÍNH NÓ (khác HẲN bài 9's `{execute, undo}`, nơi MỖI lệnh MANG THEO
hàm CỦA riêng nó). Thêm MỘT loại lệnh MỚI = thêm MỘT nhánh `switch`,
KHÔNG cần class MỚI.
::::

::::example{#du-lieu-luu-duoc-truyen-duoc}
Vì `Lenh` CHỈ LÀ dữ liệu, một **MẢNG** `Lenh[]` (một "kế hoạch") tồn
tại được TRƯỚC khi CHẠY — `reduce` phát lại TOÀN BỘ, THEO ĐÚNG thứ tự
mảng:

```typescript title=readonly
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}

const keHoach: Lenh[] = [
  { tag: "gopChu", text: "AB" },
  { tag: "gopChu", text: "CD" },
  { tag: "xoaKyTuCuoi" },
];

const ketQua = keHoach.reduce(apDungLenh, "");
console.log(ketQua);
```

```text title=readonly
ABC
```

`keHoach` LÀ một GIÁ TRỊ (mảng object thường) — lưu được vào biến,
`JSON.stringify` được (gửi qua mạng), LẶP LẠI được BAO NHIÊU LẦN tuỳ
ý. `reduce(apDungLenh, "")` áp dụng TỪNG lệnh THEO ĐÚNG THỨ TỰ mảng:
`""` → `"AB"` → `"ABCD"` → `"ABC"` (xoá ký tự cuối).
::::

::::predict{#doan-thu-tu-reduce commitOnce}
```typescript
type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };
function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu": return vanBan + lenh.text;
    case "xoaKyTuCuoi": return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi": return "";
  }
}

const keHoach: Lenh[] = [
  { tag: "gopChu", text: "AB" },
  { tag: "gopChu", text: "CD" },
  { tag: "xoaKyTuCuoi" },
];

const ketQuaNguoc = keHoach.reduceRight(apDungLenh, "");
console.log(ketQuaNguoc);
```

`reduceRight` (KHÁC `reduce`) — dòng cuối in ra gì?

:::opt{correct}
`CDAB`
:::

:::opt
`ABC` — vì `keHoach` LÀ dữ liệu KHÔNG đổi (bất biến), thứ tự các
phần tử BÊN TRONG mảng LUÔN cố định, VÀ `apDungLenh` LUÔN diễn giải
`gopChu` LÀ "nối VÀO CUỐI" bất kể phương thức duyệt nào được dùng
::why
Gần đúng ở việc bạn nhớ ĐÚNG `keHoach` LÀ dữ liệu BẤT BIẾN — MẢNG
GỐC KHÔNG hề bị thay đổi bởi `reduceRight` (quan sát ĐÓ đúng, mảng
VẪN nguyên thứ tự `[AB, CD, xoaKyTuCuoi]` sau khi gọi).

Chỗ lệch: `reduceRight` KHÔNG duyệt mảng THEO thứ tự ĐÃ khai — nó
duyệt **TỪ PHẦN TỬ CUỐI VỀ ĐẦU** (`xoaKyTuCuoi` trước, RỒI `CD`, RỒI
`AB`). Bắt đầu `""`: `xoaKyTuCuoi` trên `""` vẫn LÀ `""` (không gì để
xoá); `gopChu "CD"` → `"CD"`; `gopChu "AB"` → `"CDAB"`. Việc "diễn
giải Ý ĐỊNH nào" (nối vào cuối) KHÔNG đổi — NHƯNG **THỨ TỰ** áp dụng
CÁC lệnh đổi HOÀN TOÀN kết quả CUỐI CÙNG.
::
:::

:::opt
Máy báo lỗi biên dịch — `Array.prototype.reduceRight` yêu cầu hàm
callback có CHỮ KÝ tham số NGƯỢC (`(lenh, vanBan) => ...` thay vì
`(vanBan, lenh) => ...`), TypeScript phát hiện `apDungLenh` khai SAI
thứ tự tham số cho `reduceRight`
::why
Gần đúng ở việc bạn để ý `reduceRight` LÀ một PHƯƠNG THỨC "ngược" so
với `reduce` — một quan sát ĐÚNG Ở TÊN GỌI.

Chỗ lệch: `reduceRight` CHỈ đảo NGƯỢC **THỨ TỰ DUYỆT MẢNG** (từ cuối
về đầu) — KHÔNG đảo thứ tự THAM SỐ của hàm callback. Chữ ký callback
CỦA `reduceRight` VẪN LÀ `(accumulator, currentValue) => accumulator`
— GIỐNG HỆT `reduce`. `apDungLenh(vanBan, lenh)` khớp CHÍNH XÁC cả
hai — biên dịch SẠCH.
::
:::
::::

::::code{#viet_ap_dung_lenh}
Hoàn thiện `apDungLenh` — diễn giải TỪNG loại `Lenh` bằng `switch`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };

function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu":
      return ___;
    case "xoaKyTuCuoi":
      return ___;
    case "lamMoi":
      return "";
  }
}

let vb = "";
vb = apDungLenh(vb, { tag: "gopChu", text: "Hi" });
assertEqual(vb, "Hi", "gop chu Hi");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Lenh = { tag: "gopChu"; text: string } | { tag: "xoaKyTuCuoi" } | { tag: "lamMoi" };

function apDungLenh(vanBan: string, lenh: Lenh): string {
  switch (lenh.tag) {
    case "gopChu":
      return vanBan + lenh.text;
    case "xoaKyTuCuoi":
      return vanBan.slice(0, vanBan.length - 1);
    case "lamMoi":
      return "";
  }
}

let vb = "";
vb = apDungLenh(vb, { tag: "gopChu", text: "Hi" });
assertEqual(vb, "Hi", "gop chu Hi");
```

```typescript title=test
vb = apDungLenh(vb, { tag: "gopChu", text: "!!!" });
assertEqual(vb, "Hi!!!", "gop them !!!");
vb = apDungLenh(vb, { tag: "xoaKyTuCuoi" });
assertEqual(vb, "Hi!!", "xoa mot ky tu cuoi");
vb = apDungLenh(vb, { tag: "lamMoi" });
assertEqual(vb, "", "lam moi tra ve rong");
const vbDai = apDungLenh("Xin chao ban", { tag: "xoaKyTuCuoi" });
assertEqual(vbDai, "Xin chao ba", "xoa ky tu cuoi tren chuoi dai");
```

:::hints
- kind: attention
  body: "gopChu: nối lenh.text (trường của lệnh, KHÔNG PHẢI biến ngoài) vào vanBan. xoaKyTuCuoi: cắt vanBan, bỏ đúng 1 ký tự cuối."
- kind: strategy
  body: "vanBan + lenh.text : vanBan.slice(0, vanBan.length - 1) — nối trường text của lệnh, và cắt bớt 1 ký tự cuối."
- kind: one-line
  body: '___ (gopChu) = vanBan + lenh.text\n___ (xoaKyTuCuoi) = vanBan.slice(0, vanBan.length - 1)'
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
Command = dữ liệu + hàm diễn giải thuần. Vẫn còn thiếu: undo. Bài tiếp
theo giải quyết KHÔNG cần mỗi lệnh tự biết nghịch đảo của chính nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`apDungLenh` áp dụng TỪNG lệnh, TẠO RA vanBan MỚI mỗi lần (bất biến —
KHÔNG sửa vanBan cũ). Nếu ta GIỮ LẠI TỪNG vanBan trung gian (trước
MỖI lệnh) vào một MẢNG — "undo" trở thành việc gì?
::::

::::checkpoint{mastery=0.8}
::::
