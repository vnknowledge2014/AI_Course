---
id: ky-nghe-phan-mem.mau-tai-cau-truc.them-thao-tac-khong-sua-adt
title: "Thêm thao tác MỚI mà KHÔNG sửa định nghĩa ADT gốc"
summary: "Điểm mạnh THẬT của \"Visitor=switch\": viết inBieuThuc(bt): string (đệ quy, in dạng chữ) — HOÀN TOÀN ĐỘC LẬP với tinhGiaTri (bài 13), CÙNG hoạt động trên MỘT BieuThuc KHÔNG đổi. Đối lập OOP: thêm thao tác MỚI cần sửa TỪNG class node để thêm accept; ở đây CHỈ cần viết MỘT hàm MỚI."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 14
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.add-operation-no-touch-adt]
requires: [mau.visitor-as-switch]
concepts: [mau.add-operation-no-touch-adt]
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
`tinhGiaTri` (bài 13) TÍNH giá trị `BieuThuc`. Cần THÊM một thao tác
KHÁC — IN biểu thức RA dạng chữ, ví dụ `"(2 + 3) * 4"` — sửa gì?
::::

::::explain{#ham-moi-doc-lap}
KHÔNG sửa GÌ CẢ Ở `BieuThuc`, KHÔNG sửa `tinhGiaTri` — CHỈ viết một
hàm **MỚI**, `inBieuThuc`, đệ quy TƯƠNG TỰ nhưng trả về `string` thay
vì `number`:

```typescript title=readonly
type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };

function tinhGiaTri(bt: BieuThuc): number {
  switch (bt.tag) {
    case "so": return bt.giaTri;
    case "cong": return tinhGiaTri(bt.trai) + tinhGiaTri(bt.phai);
    case "nhan": return tinhGiaTri(bt.trai) * tinhGiaTri(bt.phai);
  }
}

function inBieuThuc(bt: BieuThuc): string {
  switch (bt.tag) {
    case "so": return String(bt.giaTri);
    case "cong": return `(${inBieuThuc(bt.trai)} + ${inBieuThuc(bt.phai)})`;
    case "nhan": return `(${inBieuThuc(bt.trai)} * ${inBieuThuc(bt.phai)})`;
  }
}

const bt: BieuThuc = {
  tag: "nhan",
  trai: { tag: "cong", trai: { tag: "so", giaTri: 2 }, phai: { tag: "so", giaTri: 3 } },
  phai: { tag: "so", giaTri: 4 },
};

console.log(inBieuThuc(bt));
console.log(tinhGiaTri(bt));
```

```text title=readonly
((2 + 3) * 4)
20
```

`inBieuThuc` VÀ `tinhGiaTri` LÀ HAI HÀM **HOÀN TOÀN ĐỘC LẬP** — cùng
NHẬN `BieuThuc`, MỖI hàm exhaustive `switch` RIÊNG, KHÔNG hàm nào GỌI
hàm kia, KHÔNG sửa ĐỔI `BieuThuc`. Đối lập TRỰC TIẾP với OOP Visitor:
thêm thao tác MỚI trong OOP cần sửa TỪNG class node (thêm `accept`
MỚI cho `PrintVisitor`); Ở ĐÂY CHỈ cần viết MỘT hàm MỚI, KHÔNG ĐỘNG
tới BẤT KỲ code CŨ nào.
::::

::::example{#tung-tang-ngoat-ngoac}
`inBieuThuc` bọc dấu ngoặc `()` Ở **MỌI** tầng `cong`/`nhan` (KỂ CẢ
tầng NGOÀI CÙNG) — KHÔNG chỉ những chỗ "cần thiết":

```typescript title=readonly
type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };
function inBieuThuc(bt: BieuThuc): string {
  switch (bt.tag) {
    case "so": return String(bt.giaTri);
    case "cong": return `(${inBieuThuc(bt.trai)} + ${inBieuThuc(bt.phai)})`;
    case "nhan": return `(${inBieuThuc(bt.trai)} * ${inBieuThuc(bt.phai)})`;
  }
}
const bt: BieuThuc = {
  tag: "nhan",
  trai: { tag: "so", giaTri: 2 },
  phai: { tag: "cong", trai: { tag: "so", giaTri: 3 }, phai: { tag: "so", giaTri: 4 } },
};
console.log(inBieuThuc(bt));
```

```text title=readonly
(2 * (3 + 4))
```

MỘT cặp `()` cho `3 + 4` (nhánh CON, `phai` của `nhan`), VÀ MỘT cặp
`()` KHÁC bọc TOÀN BỘ biểu thức (nhánh GỐC, `nhan` NGOÀI CÙNG). Hàm
KHÔNG "biết" nó ĐANG Ở gốc hay Ở nhánh CON — MỖI lần GẶP `cong`/`nhan`
(BẤT KỂ Ở ĐÂU) đều bọc `()` GIỐNG NHAU.
::::

::::predict{#doan-ngoac-ngoai-cung commitOnce}
```typescript
type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };
function inBieuThuc(bt: BieuThuc): string {
  switch (bt.tag) {
    case "so": return String(bt.giaTri);
    case "cong": return `(${inBieuThuc(bt.trai)} + ${inBieuThuc(bt.phai)})`;
    case "nhan": return `(${inBieuThuc(bt.trai)} * ${inBieuThuc(bt.phai)})`;
  }
}
const bt: BieuThuc = {
  tag: "cong",
  trai: { tag: "so", giaTri: 1 },
  phai: { tag: "nhan", trai: { tag: "so", giaTri: 2 }, phai: { tag: "so", giaTri: 3 } },
};
console.log(inBieuThuc(bt));
```

Dòng cuối in ra gì?

:::opt{correct}
`(1 + (2 * 3))`
:::

:::opt
`1 + (2 * 3)` — vì dấu ngoặc CHỈ cần thiết Ở NHỮNG chỗ có THỂ gây
NHẦM LẪN thứ tự tính TOÁN (nhân LỒNG trong cộng), tầng NGOÀI CÙNG
KHÔNG cần ngoặc vì KHÔNG có gì Ở "BÊN NGOÀI" nó để so sánh thứ tự
::why
Gần đúng ở việc bạn nhận ra dấu ngoặc Ở đây LIÊN QUAN tới việc LÀM RÕ
thứ tự TÍNH TOÁN — quan sát ĐÓ, VỀ MỤC ĐÍCH của dấu ngoặc, hợp lý.

Chỗ lệch: `inBieuThuc` KHÔNG có LOGIC "chỉ bọc ngoặc KHI CẦN" — nhánh
`case "cong"` VÀ `case "nhan"` LUÔN LUÔN trả về CHUỖI bọc trong
`` `(...)` ``, KHÔNG PHÂN BIỆT "đây LÀ gốc hay LÀ nhánh con". `bt` CÓ
`tag: "cong"` (NGAY tại GỐC) — GỌI `inBieuThuc(bt)` CHẠY vào NHÁNH
`"cong"`, TRẢ VỀ `` `(${...} + ${...})` `` — dấu ngoặc XUẤT HIỆN Ở
CẢ tầng gốc, KHÔNG có ngoại lệ nào cho "tầng NGOÀI CÙNG".
::
:::

:::opt
Máy báo lỗi biên dịch — `inBieuThuc` trả về kiểu `string`, nhưng
`case "so"` dùng `String(bt.giaTri)` (một HÀM built-in) trong khi
`case "cong"`/`case "nhan"` dùng TEMPLATE LITERAL (`` `...` ``),
TypeScript đòi TẤT CẢ nhánh `switch` PHẢI tạo chuỗi bằng CÙNG MỘT
cách
::why
Gần đúng ở việc bạn để ý BA nhánh `switch` tạo chuỗi bằng HAI CÁCH
KHÁC NHAU (`String(...)` VÀ template literal) — một quan sát ĐÚNG về
CÚ PHÁP.

Chỗ lệch: TypeScript CHỈ quan tâm **KIỂU TRẢ VỀ** (`string`) — KHÔNG
quan tâm "cú pháp NÀO tạo ra chuỗi ĐÓ". `String(bt.giaTri)` VÀ
`` `(${...})` `` ĐỀU trả về `string` — khớp CHÍNH XÁC chữ ký hàm.
KHÔNG có quy tắc "MỌI nhánh phải dùng CÙNG cú pháp". Biên dịch SẠCH.
::
:::
::::

::::code{#viet_in_bieu_thuc}
Hoàn thiện `inBieuThuc` — đệ quy tạo chuỗi, bọc `()` cho `cong`/`nhan`.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };

function inBieuThuc(bt: BieuThuc): string {
  switch (bt.tag) {
    case "so":
      return String(bt.giaTri);
    case "cong":
      return ___;
    case "nhan":
      return ___;
  }
}

const bt1: BieuThuc = { tag: "so", giaTri: 9 };
assertEqual(inBieuThuc(bt1), "9", "so don gian");

const bt2: BieuThuc = { tag: "cong", trai: { tag: "so", giaTri: 3 }, phai: { tag: "so", giaTri: 4 } };
assertEqual(inBieuThuc(bt2), "(3 + 4)", "cong hai so");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type BieuThuc =
  | { tag: "so"; giaTri: number }
  | { tag: "cong"; trai: BieuThuc; phai: BieuThuc }
  | { tag: "nhan"; trai: BieuThuc; phai: BieuThuc };

function inBieuThuc(bt: BieuThuc): string {
  switch (bt.tag) {
    case "so":
      return String(bt.giaTri);
    case "cong":
      return `(${inBieuThuc(bt.trai)} + ${inBieuThuc(bt.phai)})`;
    case "nhan":
      return `(${inBieuThuc(bt.trai)} * ${inBieuThuc(bt.phai)})`;
  }
}

const bt1: BieuThuc = { tag: "so", giaTri: 9 };
assertEqual(inBieuThuc(bt1), "9", "so don gian");

const bt2: BieuThuc = { tag: "cong", trai: { tag: "so", giaTri: 3 }, phai: { tag: "so", giaTri: 4 } };
assertEqual(inBieuThuc(bt2), "(3 + 4)", "cong hai so");
```

```typescript title=test
const bt3: BieuThuc = { tag: "nhan", trai: { tag: "so", giaTri: 5 }, phai: { tag: "so", giaTri: 6 } };
assertEqual(inBieuThuc(bt3), "(5 * 6)", "nhan hai so");

const bt4: BieuThuc = {
  tag: "nhan",
  trai: { tag: "cong", trai: { tag: "so", giaTri: 2 }, phai: { tag: "so", giaTri: 3 } },
  phai: { tag: "so", giaTri: 4 },
};
assertEqual(inBieuThuc(bt4), "((2 + 3) * 4)", "long nhau (2+3)*4");
```

:::hints
- kind: attention
  body: "cong: bọc ngoặc, đệ quy trai VÀ phai, nối bằng \" + \". nhan: y hệt nhưng nối bằng \" * \"."
- kind: strategy
  body: '`(${inBieuThuc(bt.trai)} + ${inBieuThuc(bt.phai)})` : `(${inBieuThuc(bt.trai)} * ${inBieuThuc(bt.phai)})` — template literal đệ quy, khác nhau đúng MỘT ký hiệu phép toán.'
- kind: one-line
  body: '___ (cong) = `(${inBieuThuc(bt.trai)} + ${inBieuThuc(bt.phai)})`\n___ (nhan) = `(${inBieuThuc(bt.trai)} * ${inBieuThuc(bt.phai)})`'
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
Thêm thao tác MỚI = viết hàm MỚI, KHÔNG sửa ADT/hàm cũ. Bài tiếp
theo: một kỹ thuật KHÁC — bọc HÀNH VI quanh một hàm, không sửa nó.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinhGiaTri` VÀ `inBieuThuc` ĐỘC LẬP nhau — nhưng nếu muốn THÊM hành
vi (như đếm SỐ LẦN `tinhGiaTri` được GỌI) MÀ KHÔNG sửa THÂN hàm ĐÓ,
cách nào?
::::

::::checkpoint{mastery=0.8}
::::
