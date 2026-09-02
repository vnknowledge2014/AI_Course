---
id: ky-nghe-phan-mem.kiem-thu.tu-viet-vong-lap-kiem-tinh-chat
title: "Tự viết vòng lặp \"sinh rồi kiểm\" — cốt lõi của MỌI công cụ PBT"
summary: "MỌI công cụ property-based testing (kể cả fast-check THẬT) quy VỀ ĐÚNG MỘT vòng lặp: SINH input → KIỂM tính chất → BÁO lỗi ở LẦN ĐẦU thất bại. chayThuTinhChat(sinhInput, tinhChat, soLan) — TỰ VIẾT vòng lặp ĐÓ trước khi đặt tên hoa mỹ, để thấy nó đơn giản tới mức nào."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 16
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.property-check-loop]
requires: [kt.example-vs-property]
concepts: [kt.property-check-loop]
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
Bài trước: mười mẫu "chọn tay" bắt được một bug. Nếu SINH input NGẪU
NHIÊN, LẶP LẠI quy trình NHIỀU lần — vòng lặp đó trông thế nào?
::::

::::explain{#chaythutinhchat-cot-loi}
MỌI công cụ property-based testing (KỂ CẢ `fast-check` THẬT — thư
viện phổ biến trong TypeScript, KHÔNG dùng được trong sandbox NÀY)
quy VỀ **ĐÚNG MỘT** vòng lặp: **SINH** input → **KIỂM** tính chất →
**BÁO lỗi** Ở LẦN ĐẦU thất bại:

```typescript
function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function soNguyenNgauNhien(min: number, max: number): number {
  return Math.floor(Math.random() * (max - min + 1)) + min;
}

chayThuTinhChat(() => soNguyenNgauNhien(-100, 100), (n) => Math.abs(n) >= 0, 50);
```

```text
[PASS] tinh chat dung tren ca 50 lan
```

`chayThuTinhChat<A>` LÀ GENERIC — HOÀN TOÀN KHÔNG biết `A` (kiểu dữ
liệu đang test) LÀ GÌ, CHỈ cần MỘT hàm `sinhInput` (TẠO input MỚI mỗi
lần gọi) VÀ MỘT hàm `tinhChat` (KIỂM tra input ĐÓ). Chạy `50` LẦN
TRÊN số nguyên NGẪU NHIÊN, tính chất `Math.abs(n) >= 0` (LUÔN đúng
với MỌI số) ĐƯƠNG NHIÊN `[PASS]` — CHỈ MỘT DÒNG code thay THẾ cho
việc viết `50` assertion tay.
::::

::::example{#bao-loi-o-lan-dau-that-bai}
KHI tính chất **THẤT BẠI**, `chayThuTinhChat` báo NGAY Ở **LẦN ĐẦU
TIÊN** phát hiện — KHÔNG chạy tiếp cho hết `soLan`:

```typescript title=readonly
function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

// bo sinh DAN GIAN, TAI LAP DUOC (mot bo dem tang dan) -- de vi du luon RA CUNG mot ket qua
let dem = 0;
const demTiepTheo = () => dem++;

try {
  chayThuTinhChat(demTiepTheo, (n) => n < 5, 10);
} catch (e) {
  console.log((e as Error).message);
}
```

```text title=readonly
that bai o lan thu 5: input = 5
```

Bộ sinh `demTiepTheo` TRẢ `0, 1, 2, 3, ...` TĂNG DẦN — tính chất
`n < 5` ĐÚNG với `0,1,2,3,4` (năm LẦN ĐẦU), NHƯNG SAI Ở `5` — vòng
lặp DỪNG **NGAY** Ở `lan = 5`, KHÔNG chạy tiếp `6,7,8,9`. Thông điệp
lỗi cho biết CHÍNH XÁC LẦN nào VÀ input NÀO gây thất bại — DỄ debug
hơn NHIỀU so với "test đã fail" KHÔNG có chi tiết.
::::

::::predict{#doan-doi-nguong-that-bai commitOnce}
```typescript
function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

let dem = 0;
const demTiepTheo = () => dem++;

try {
  chayThuTinhChat(demTiepTheo, (n) => n < 3, 10);
} catch (e) {
  console.log((e as Error).message);
}
```

Dòng cuối in ra gì?

:::opt{correct}
`that bai o lan thu 3: input = 3`
:::

:::opt
`that bai o lan thu 5: input = 5` — vì THÔNG ĐIỆP lỗi giống HỆT ví
dụ Ở phần "example" TRÊN (CÙNG cấu trúc code, CHỈ đổi ngưỡng), NÊN
kết quả CŨNG giống HỆT — con số `5` trong thông điệp KHÔNG phụ thuộc
NGƯỠNG `n < 3` hay `n < 5` truyền vào
::why
Gần đúng ở việc bạn để ý ĐOẠN code NÀY RẤT GIỐNG ví dụ Ở phần
"example" TRƯỚC ĐÓ — một quan sát ĐÚNG về SỰ TƯƠNG ĐỒNG cấu trúc.

Chỗ lệch: THAM SỐ THỨ HAI truyền cho `chayThuTinhChat` LÀ MỘT
**TÍNH CHẤT KHÁC** — `(n) => n < 3` (KHÔNG PHẢI `(n) => n < 5` như ví
dụ TRƯỚC). Bộ sinh `demTiepTheo` VẪN trả `0, 1, 2, 3, ...` TĂNG DẦN —
tính chất `n < 3` ĐÚNG với `0, 1, 2` (BA lần đầu), NHƯNG SAI NGAY Ở
`3` (`3 < 3` LÀ `false`). Vòng lặp DỪNG Ở `lan = 3`, thông điệp báo
`"that bai o lan thu 3: input = 3"` — NGƯỠNG khác nhau CHO kết quả
khác nhau.
::
:::

:::opt
Máy báo lỗi biên dịch — biến `dem` bị KHAI BÁO LẠI (đã dùng Ở đoạn
"example" TRƯỚC ĐÓ trong CÙNG MỘT file), TypeScript CẤM khai `let
dem` HAI LẦN trong CÙNG một chương trình
::why
Gần đúng ở việc bạn để ý TÊN biến `dem` XUẤT HIỆN Ở CẢ HAI đoạn code
(example VÀ predict NÀY) — một quan sát ĐÚNG về SỰ trùng TÊN.

Chỗ lệch: MỖI khối code (`example`, `predict`, `code`...) trong bài
học NÀY LÀ MỘT chương trình **ĐỘC LẬP**, chạy RIÊNG, KHÔNG chia sẻ
phạm vi (scope) VỚI NHAU — `let dem` Ở đoạn NÀY KHÔNG "va chạm" VỚI
`let dem` Ở đoạn KHÁC, vì chúng KHÔNG BAO GIỜ cùng tồn tại trong MỘT
lần chạy. Biên dịch sạch.
::
:::
::::

::::code{#viet_chaythutinhchat}
Tự viết PHẦN LÕI của `chayThuTinhChat<A>`.

```typescript title=starter
function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < ___; lan++) {
    const input = ___;
    if (!___(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

chayThuTinhChat(() => 5, (n) => n === 5, 3);
```

```typescript title=solution
function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

chayThuTinhChat(() => 5, (n) => n === 5, 3);
```

```typescript title=test
let daNemLoi = false;
try {
  chayThuTinhChat(() => 999, (n) => n < 5, 5); // 999 KHONG < 5 -- tinh chat SAI, PHAI throw
} catch {
  daNemLoi = true;
}
if (!daNemLoi) throw new Error("chayThuTinhChat PHAI nem loi khi tinh chat SAI, khong duoc im lang");

let soLanGoi = 0;
chayThuTinhChat(() => { soLanGoi++; return soLanGoi; }, () => true, 7);
if (soLanGoi !== 7) throw new Error(`chayThuTinhChat phai goi sinhInput DUNG soLan lan, mong 7 nhan ${soLanGoi}`);
console.log("[PASS] goi dung so lan va nem loi khi tinh chat sai");

let dem2 = 0;
const demTiepTheo2 = () => dem2++;
try {
  chayThuTinhChat(demTiepTheo2, (n) => n < 4, 10);
  throw new Error("KHONG throw -- sai");
} catch (e) {
  if (!(e as Error).message.includes("4")) throw new Error("thong diep loi phai nhac dung input gay that bai (4)");
  console.log("[PASS] bao dung lan va input that bai");
}
```

:::hints
- kind: attention
  body: "Vòng lặp chạy đúng soLan lần. Mỗi lần: sinh MỘT input mới, kiểm tính chất TRÊN input đó, ném lỗi NGAY nếu sai."
- kind: strategy
  body: 'soLan : sinhInput() : tinhChat — điều kiện dừng vòng lặp, lời gọi sinh input mới, hàm kiểm tính chất.'
- kind: one-line
  body: '___ (điều kiện lặp) = soLan\n___ (sinh input) = sinhInput()\n___ (kiem tinh chat) = tinhChat'
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
Vòng lặp sinh-rồi-kiểm: cốt lõi của MỌI công cụ PBT, chỉ vài dòng.
Bước tiếp theo: một kiểu BoSinh<T> tự chế, tái tạo được cho từng kiểu.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`chayThuTinhChat` nhận `sinhInput: () => A` — MỘT hàm KHÔNG tham số,
trả về `A`. Đóng gói "cách sinh một giá trị ngẫu nhiên" thành MỘT
kiểu dữ liệu RIÊNG (thay vì hàm rời rạc) sẽ trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
