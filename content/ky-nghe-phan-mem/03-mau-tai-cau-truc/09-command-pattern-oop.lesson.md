---
id: ky-nghe-phan-mem.mau-tai-cau-truc.command-pattern-oop
title: "Command pattern OOP — mỗi hành động một class có execute()/undo()"
summary: "OOP Command: MỖI hành động (thêm chữ, xoá chữ) LÀ một class implement execute()/undo(). FP naive: object {execute, undo} — CÙNG hình dạng, chỉ bỏ class. Giới hạn THÀNH THẬT: undo phải TỰ SUY ra nghịch đảo (dễ với gộp chữ, KHÔNG THỂ với xoá — ký tự đã mất không khôi phục lại đúng được), và undo PHẢI gọi ĐÚNG thứ tự ngược (LIFO), sai thứ tự làm hỏng state."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.command-oop-shape]
requires: [mau.gate-boss-observer]
concepts: [mau.command-oop-shape]
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
Cụm mới. Ứng dụng ghi chú cần Undo — mỗi thao tác (gõ chữ, xoá chữ)
PHẢI "quay ngược" được. OOP gọi đây LÀ Command pattern.
::::

::::explain{#oop-command-shape}
OOP Command: MỖI hành động (thêm chữ, xoá chữ, đổi màu) LÀ một class
RIÊNG, implement `interface Command { execute(); undo() }` — Undo/Redo
LÀ một STACK các object Command đã thực thi.

Bản FP naive (bỏ class, GIỮ NGUYÊN hình dạng — object nhóm HAI hành
vi, giống `taoKhoHang` bài 3):

```typescript title=readonly
type Lenh = {
  execute: (vanBan: string) => string;
  undo: (vanBan: string) => string;
};

function taoLenhGopChu(chu: string): Lenh {
  return {
    execute: (vanBan) => vanBan + chu,
    undo: (vanBan) => vanBan.slice(0, vanBan.length - chu.length),
  };
}

const lenh1 = taoLenhGopChu("Xin chao");
let vb = "";
vb = lenh1.execute(vb);
console.log(vb);
vb = lenh1.undo(vb);
console.log(vb);
```

```text title=readonly
Xin chao

```

`taoLenhGopChu` TRẢ VỀ một cặp `{execute, undo}` — `undo` LÀ nghịch
đảo TOÁN HỌC của `execute` (nối chữ VÀO thì undo CẮT ĐÚNG số ký tự
ĐÓ ra). Hoạt động HOÀN HẢO — CHO tới khi hành động KHÔNG CÓ nghịch
đảo rõ ràng.
::::

::::example{#gioi-han-that-cua-undo-tu-suy}
"Xoá ký tự cuối" KHÔNG CÓ nghịch đảo TỰ NHIÊN — ký tự BỊ xoá đã MẤT,
KHÔNG CÁCH NÀO `undo` "đoán" LẠI đúng ký tự đó:

```typescript title=readonly
type Lenh = {
  execute: (vanBan: string) => string;
  undo: (vanBan: string) => string;
};

function taoLenhXoaKyTuCuoi(): Lenh {
  return {
    execute: (vanBan) => vanBan.slice(0, vanBan.length - 1),
    undo: (vanBan) => vanBan + "?", // KHONG THE khoi phuc ky tu DA MAT
  };
}

const lenh2 = taoLenhXoaKyTuCuoi();
let vb = "Chao X";
vb = lenh2.execute(vb);
console.log(vb);
vb = lenh2.undo(vb);
console.log(vb);
```

```text title=readonly
Chao 
Chao ?
```

`execute` xoá `"X"` (mất VĨNH VIỄN), `undo` chỉ BIẾT thêm LẠI một ký
tự PLACEHOLDER (`"?"`) — KHÔNG PHẢI `"X"` gốc. Đây LÀ giới hạn THẬT
của cách "mỗi command TỰ chứa nghịch đảo của chính nó": nghịch đảo
CHỈ đúng khi hành động ĐỦ ĐƠN GIẢN để "đoán ngược" — bài 11 sẽ giải
quyết vấn đề NÀY bằng một cách TIẾP CẬN khác hẳn (lưu SNAPSHOT, không
cần TỰ suy nghịch đảo).
::::

::::predict{#doan-undo-sai-thu-tu commitOnce}
```typescript
type Lenh = {
  execute: (vanBan: string) => string;
  undo: (vanBan: string) => string;
};
function taoLenhGopChu(chu: string): Lenh {
  return {
    execute: (vanBan) => vanBan + chu,
    undo: (vanBan) => vanBan.slice(0, vanBan.length - chu.length),
  };
}

const lenhA = taoLenhGopChu("AB");
const lenhB = taoLenhGopChu("XYZ");

let vb = "";
vb = lenhA.execute(vb);
vb = lenhB.execute(vb);
console.log(vb);
vb = lenhA.undo(vb);
console.log(vb);
```

`lenhA.undo` được gọi TRƯỚC `lenhB.undo` (SAI thứ tự — phải undo
`B` trước vì nó thực thi SAU). Hai dòng in ra gì?

:::opt{correct}
`ABXYZ` rồi `ABX`
:::

:::opt
`ABXYZ` rồi `AB` — vì `lenhA.undo` "biết" nó cần XOÁ ĐÚNG hai ký tự
`"AB"` mà NÓ đã thêm vào, BẤT KỂ những gì XẢY RA SAU ĐÓ (kể cả
`lenhB` đã thêm THÊM ký tự)
::why
Gần đúng ở việc bạn nhớ ĐÚNG `lenhA` "liên quan" tới CHUỖI `"AB"` —
quan sát ĐÓ, VỀ MẶT Ý ĐỊNH của người viết, không sai.

Chỗ lệch: `undo` KHÔNG "biết" NỘI DUNG nó cần xoá LÀ GÌ — nó CHỈ biết
**ĐỘ DÀI** (`chu.length = 2`) VÀ luôn `slice` bỏ ĐÚNG số ký tự đó Ở
**CUỐI** chuỗi, BẤT KỂ CUỐI chuỗi LÚC ĐÓ LÀ GÌ. Sau hai `execute`,
chuỗi LÀ `"ABXYZ"` — CUỐI chuỗi LÚC NÀY LÀ `"YZ"` (thuộc VỀ `lenhB`,
KHÔNG PHẢI `lenhA`). `lenhA.undo("ABXYZ")` cắt bỏ HAI ký tự CUỐI
(`"YZ"`), để LẠI `"ABX"` — SAI hoàn toàn, VÌ gọi undo KHÔNG đúng
THỨ TỰ NGƯỢC (phải `lenhB.undo` TRƯỚC, `lenhA.undo` SAU).
::
:::

:::opt
Máy báo lỗi lúc chạy — `lenhA.undo(vb)` được gọi khi `vb` chứa CẢ nội
dung của `lenhB` (`"XYZ"`) TRỘN LẪN vào, TypeScript phát hiện `vb`
KHÔNG khớp với "trạng thái mà `lenhA` mong đợi", ném lỗi kiểu NGAY
LÚC CHẠY
::why
Gần đúng ở việc bạn để ý `vb` lúc gọi `lenhA.undo` chứa NHIỀU HƠN
những gì `lenhA` ĐÃ thêm vào — một quan sát ĐÚNG về TRẠNG THÁI dữ
liệu.

Chỗ lệch: `undo: (vanBan: string) => string` NHẬN **BẤT KỲ** `string`
nào — KHÔNG CÓ khái niệm "trạng thái mà lệnh NÀY mong đợi" Ở CẤP ĐỘ
kiểu (TypeScript KHÔNG theo dõi LỊCH SỬ giá trị). Gọi `lenhA.undo`
VỚI BẤT KỲ chuỗi nào ĐỀU biên dịch VÀ chạy SẠCH — nó CHỈ đơn giản
LÀM đúng phép `slice`, dù kết quả LÀ SAI VỀ MẶT NGHIỆP VỤ (như dòng
`predict` Ở TRÊN cho thấy).
::
:::
::::

::::code{#viet_lenh_gop_chu}
Hoàn thiện `taoLenhGopChu` — `execute` nối chữ vào CUỐI, `undo` cắt
ĐÚNG số ký tự ĐÃ nối.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Lenh = {
  execute: (vanBan: string) => string;
  undo: (vanBan: string) => string;
};

function taoLenhGopChu(chu: string): Lenh {
  return {
    execute: (vanBan) => ___,
    undo: (vanBan) => ___,
  };
}

const lenh = taoLenhGopChu("Hello");
let vb = "";
vb = lenh.execute(vb);
assertEqual(vb, "Hello", "sau execute la Hello");
vb = lenh.undo(vb);
assertEqual(vb, "", "sau undo tro ve rong");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type Lenh = {
  execute: (vanBan: string) => string;
  undo: (vanBan: string) => string;
};

function taoLenhGopChu(chu: string): Lenh {
  return {
    execute: (vanBan) => vanBan + chu,
    undo: (vanBan) => vanBan.slice(0, vanBan.length - chu.length),
  };
}

const lenh = taoLenhGopChu("Hello");
let vb = "";
vb = lenh.execute(vb);
assertEqual(vb, "Hello", "sau execute la Hello");
vb = lenh.undo(vb);
assertEqual(vb, "", "sau undo tro ve rong");
```

```typescript title=test
const lenh2 = taoLenhGopChu("AB");
let vb2 = "Xin chao ";
vb2 = lenh2.execute(vb2);
assertEqual(vb2, "Xin chao AB", "gop chu vao cuoi chuoi co san");
vb2 = lenh2.undo(vb2);
assertEqual(vb2, "Xin chao ", "undo tra ve dung chuoi ban dau");

const lenhDai = taoLenhGopChu("1234567890");
let vb3 = "";
vb3 = lenhDai.execute(vb3);
assertEqual(vb3, "1234567890", "gop chuoi dai");
vb3 = lenhDai.undo(vb3);
assertEqual(vb3, "", "undo chuoi dai tra ve rong");
```

:::hints
- kind: attention
  body: "execute: nối chu vào SAU vanBan. undo: cắt vanBan, giữ lại phần TRƯỚC đúng chu.length ký tự cuối."
- kind: strategy
  body: "vanBan + chu : vanBan.slice(0, vanBan.length - chu.length) — nối chuỗi, và cắt bỏ đúng độ dài đã nối."
- kind: one-line
  body: '___ (execute) = vanBan + chu\n___ (undo) = vanBan.slice(0, vanBan.length - chu.length)'
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
{execute, undo} hoạt động — CHO tới khi hành động không nghịch đảo
được, hoặc undo gọi sai thứ tự. Bài tiếp theo: một cách nhìn khác
hẳn — Command LÀ dữ liệu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoLenhXoaKyTuCuoi`'s `undo` KHÔNG THỂ khôi phục đúng — vì "ký tự
đã xoá LÀ GÌ" bị MẤT ngay khi `execute` chạy. Nếu, THAY VÌ tự suy
nghịch đảo, ta LƯU LẠI trạng thái TRƯỚC MỖI lệnh — vấn đề NÀY còn
tồn tại không?
::::

::::checkpoint{mastery=0.8}
::::
