---
id: ky-nghe-phan-mem.ddd.optional-field-la-ke-ho
title: "Optional field = kẽ hở sinh trạng thái phi lý — tách thành DU riêng"
summary: "Optional field kèm boolean tạo kẽ hở — NguoiDungTe: daXacThuc: boolean; ngayXacThuc?: Date cho phép daXacThuc=false mà VẪN có ngayXacThuc. Quy tắc: biến field điều-kiện thành MỘT DU riêng — TrangThaiXacMinh."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.optional-field-to-du]
requires: [ddd.transition-functions]
concepts: [ddd.optional-field-to-du]
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
Bài 15 giải quyết boolean flags Ở CẤP TOÀN BỘ trạng thái. Nhưng vấn đề
CŨ có thể LÉN quay lại — ở một field ĐƠN LẺ.
::::

::::explain{#optional-field-ke-ho}
```typescript
type NguoiDungTe = {
  ten: string;
  daXacThuc: boolean;
  ngayXacThuc?: Date;
};

// VÔ LÝ: daXacThuc = false nhưng VẪN có ngayXacThuc
const nguoiDungPhiLy: NguoiDungTe = {
  ten: "An",
  daXacThuc: false,
  ngayXacThuc: new Date("2024-01-01"),
};
console.log(JSON.stringify(nguoiDungPhiLy));
```

```text
{"ten":"An","daXacThuc":false,"ngayXacThuc":"2024-01-01T00:00:00.000Z"}
```

`ngayXacThuc?: Date` (optional) KÈM `daXacThuc: boolean` là CHÍNH XÁC
vấn đề bài 14 — chỉ Ở QUY MÔ NHỎ HƠN (một field, không phải cả object)
— "chưa xác thực NHƯNG có ngày xác thực" là VÔ LÝ, nhưng TypeScript
CHO PHÉP viết ra.

**Quy tắc "khi nghi ngờ, biến field điều-kiện thành MỘT DU riêng"**:

```typescript
type TrangThaiXacMinh =
  | { tag: "chua_xac_minh" }
  | { tag: "da_xac_minh"; ngayXacThuc: Date };

type NguoiDungTot = { ten: string; xacMinh: TrangThaiXacMinh };

const nguoiDungTot: NguoiDungTot = {
  ten: "An",
  xacMinh: { tag: "chua_xac_minh" },
};
console.log(JSON.stringify(nguoiDungTot));
```

```text
{"ten":"An","xacMinh":{"tag":"chua_xac_minh"}}
```

`ngayXacThuc` giờ CHỈ tồn tại TRONG variant `"da_xac_minh"` — KHÔNG
THỂ viết `{ tag: "chua_xac_minh", ngayXacThuc: ... }` (thử sẽ bị
TypeScript TỪ CHỐI ngay, lỗi `TS2353`: "ngayXacThuc không tồn tại
trong type đó"). Kẽ hở BỊ ĐÓNG hoàn toàn — không phải bằng kỷ luật lập
trình viên, mà bằng CHÍNH kiểu dữ liệu.
::::

::::example{#doc-trang-thai-xac-minh}
Đọc `TrangThaiXacMinh` bằng `switch` (đã học T4.3) — mỗi nhánh có ĐÚNG
field nó CẦN, không phải kiểm `if (nd.daXacThuc && nd.ngayXacThuc)`
rườm rà:

```typescript title=readonly
type TrangThaiXacMinh =
  | { tag: "chua_xac_minh" }
  | { tag: "da_xac_minh"; ngayXacThuc: Date };
type NguoiDungTot = { ten: string; xacMinh: TrangThaiXacMinh };

function moTaXacMinh(nd: NguoiDungTot): string {
  switch (nd.xacMinh.tag) {
    case "chua_xac_minh": return `${nd.ten}: chưa xác minh`;
    case "da_xac_minh": return `${nd.ten}: đã xác minh lúc ${nd.xacMinh.ngayXacThuc.toISOString()}`;
  }
}

console.log(moTaXacMinh({ ten: "An", xacMinh: { tag: "chua_xac_minh" } }));
console.log(moTaXacMinh({ ten: "Binh", xacMinh: { tag: "da_xac_minh", ngayXacThuc: new Date("2024-05-01") } }));
```

```text title=readonly
An: chưa xác minh
Binh: đã xác minh lúc 2024-05-01T00:00:00.000Z
```

Ở nhánh `"da_xac_minh"`, TypeScript TỰ BIẾT `nd.xacMinh.ngayXacThuc`
CHẮC CHẮN tồn tại (không phải `Date | undefined`) — narrowing (T4.3) áp
dụng NGAY, không cần kiểm `!== undefined` thủ công như optional field
đòi hỏi.
::::

::::predict{#doan-tao-tu-optional-cu commitOnce}
```typescript
type NguoiDungTe = { ten: string; daXacThuc: boolean; ngayXacThuc?: Date };

function laDaXacThucThat(nd: NguoiDungTe): boolean {
  return nd.daXacThuc && nd.ngayXacThuc !== undefined;
}

const nguoiDungLoi: NguoiDungTe = { ten: "Chi", daXacThuc: true, ngayXacThuc: undefined };
console.log(laDaXacThucThat(nguoiDungLoi));
```

`nguoiDungLoi` có `daXacThuc: true` NHƯNG `ngayXacThuc: undefined` —
một trạng thái VÔ LÝ KHÁC mà `NguoiDungTe` (thiết kế CŨ) vẫn cho phép.
Dòng cuối in ra gì?

:::opt{correct}
`false`
:::

:::opt
`true` — vì `daXacThuc: true` là điều kiện QUAN TRỌNG NHẤT, `&&` với
bất kỳ gì phía sau cũng ưu tiên trả về giá trị của vế ĐẦU
::why
Gần đúng ở việc bạn nhớ ĐÚNG `daXacThuc` LÀ `true` — quan sát về giá
trị field đó đúng.

Chỗ lệch: `&&` (AND) trả về `true` CHỈ KHI CẢ HAI vế đều `true` — không
có "ưu tiên vế đầu". `nd.ngayXacThuc !== undefined` với
`ngayXacThuc: undefined` là `false` — `true && false` cho ra `false`.
Đây CHÍNH LÀ ví dụ SỐNG cho lý do bài học này tồn tại: `NguoiDungTe`
(optional field + boolean) tạo ra MỘT trạng thái vô lý KHÁC nữa
(`daXacThuc: true` mà KHÔNG có ngày) mà `TrangThaiXacMinh` (DU) sẽ
loại bỏ hoàn toàn.
::
:::

:::opt
Máy báo lỗi biên dịch — `ngayXacThuc: undefined` không hợp lệ cho một
field kiểu `Date | undefined` (optional)
::why
Gần đúng ở việc bạn để ý `ngayXacThuc?: Date` là một field OPTIONAL —
quan sát về TÍNH CHẤT optional đó đúng.

Chỗ lệch: field optional (`?:`) trong TypeScript có kiểu THỰC TẾ là
`Kiểu | undefined` — gán TƯỜNG MINH giá trị `undefined` cho nó là HOÀN
TOÀN HỢP LỆ (khác với việc BỎ HẲN field đó ra khỏi object literal,
cũng hợp lệ). Biên dịch và chạy bình thường.
::
:::
::::

::::code{#viet_mo_ta_xac_minh}
Tự viết `moTaXacMinh(nd: NguoiDungTot): string`.

```typescript title=starter
type TrangThaiXacMinh =
  | { tag: "chua_xac_minh" }
  | { tag: "da_xac_minh"; ngayXacThuc: Date };
type NguoiDungTot = { ten: string; xacMinh: TrangThaiXacMinh };

function moTaXacMinh(nd: NguoiDungTot): string {
  switch (nd.xacMinh.tag) {
    case "chua_xac_minh": return ___;
    case "da_xac_minh": return ___;
  }
}

console.log(moTaXacMinh({ ten: "An", xacMinh: { tag: "chua_xac_minh" } }));
```

```typescript title=solution
type TrangThaiXacMinh =
  | { tag: "chua_xac_minh" }
  | { tag: "da_xac_minh"; ngayXacThuc: Date };
type NguoiDungTot = { ten: string; xacMinh: TrangThaiXacMinh };

function moTaXacMinh(nd: NguoiDungTot): string {
  switch (nd.xacMinh.tag) {
    case "chua_xac_minh": return `${nd.ten}: chưa xác minh`;
    case "da_xac_minh": return `${nd.ten}: đã xác minh lúc ${nd.xacMinh.ngayXacThuc.toISOString()}`;
  }
}

console.log(moTaXacMinh({ ten: "An", xacMinh: { tag: "chua_xac_minh" } }));
```

```typescript title=test
const chuaXacMinh = moTaXacMinh({ ten: "An", xacMinh: { tag: "chua_xac_minh" } });
if (chuaXacMinh !== "An: chưa xác minh") throw new Error("nhánh chưa xác minh phải trả về đúng chuỗi mô tả, ghi rõ tên");

const daXacMinh = moTaXacMinh({ ten: "Binh", xacMinh: { tag: "da_xac_minh", ngayXacThuc: new Date("2024-05-01T00:00:00.000Z") } });
if (daXacMinh !== "Binh: đã xác minh lúc 2024-05-01T00:00:00.000Z") throw new Error("nhánh đã xác minh phải ghi rõ tên VÀ đúng thời điểm xác minh");
```

:::hints
- kind: attention
  body: "Nhánh chưa_xac_minh: chỉ có nd.ten (KHÔNG có ngayXacThuc — field đó không tồn tại ở nhánh này). Nhánh da_xac_minh: có CẢ nd.ten LẪN nd.xacMinh.ngayXacThuc (đã narrow, không cần kiểm undefined)."
- kind: strategy
  body: '`${nd.ten}: chưa xác minh` — cho nhánh đầu. `${nd.ten}: đã xác minh lúc ${nd.xacMinh.ngayXacThuc.toISOString()}` — cho nhánh sau.'
- kind: one-line
  body: "___ (chua_xac_minh) = `${nd.ten}: chưa xác minh`\n___ (da_xac_minh) = `${nd.ten}: đã xác minh lúc ${nd.xacMinh.ngayXacThuc.toISOString()}`"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "chưa xác minh"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Optional field kèm điều kiện khác = kẽ hở. Tách thành DU riêng — kẽ hở
đóng hoàn toàn, TypeScript tự lo phần narrowing.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Domain THẬT thường phức tạp hơn MỘT DU đơn — nhiều khía cạnh, mỗi
khía cạnh có trạng thái RIÊNG. Ghép NHIỀU DU lồng nhau trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
