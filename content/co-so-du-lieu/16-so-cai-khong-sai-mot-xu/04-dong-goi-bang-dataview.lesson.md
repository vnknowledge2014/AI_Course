---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.dong-goi-bang-dataview
title: "Đóng gói bằng DataView"
summary: "dongGoiButToan dùng DataView THẬT (API JS/TS, không mô phỏng) để ghi 7 trường vào đúng offset trong một ArrayBuffer 128 byte — setUint32 cho id/debitAccountId/creditAccountId/pendingId/flags (4 byte), setBigUint64 cho amount/timestamp (8 byte, cần BigInt vì number JS không an toàn quá 2^53). moGoiButToan đọc ngược lại — round-trip khớp tuyệt đối: đóng gói rồi mở gói cho lại ĐÚNG bộ giá trị ban đầu, không sai một byte."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.dong-goi-bang-dataview]
requires: [db.128-byte-cho-mot-but-toan]
concepts: [db.dong-goi-bang-dataview]
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
Layout Ở bài trước CHỈ có trên GIẤY — bảy trường, bảy offset. Ghi
THẬT vào một `ArrayBuffer` 128 byte nhị phân, dùng đúng API `DataView`
CỦA JavaScript/TypeScript — không phải mô phỏng.
::::

::::explain{#dataview-that}
`dongGoiButToan` dùng `DataView` (API THẬT, không mô phỏng) để ghi
BẢY trường VÀO đúng offset TRONG một `ArrayBuffer` `128` byte:
`setUint32` cho các trường `4` byte, `setBigUint64` cho `amount` VÀ
`timestamp` (`8` byte — CẦN `BigInt` VÌ kiểu `number` của JS KHÔNG an
toàn quá `2^53`). `moGoiButToan` đọc NGƯỢC lại:

```typescript title=readonly
interface ButToanDayDu {
  id: number; debitAccountId: number; creditAccountId: number;
  amount: number; timestamp: number; pendingId: number; flags: number;
}

const KICH_THUOC_BAN_GHI = 128;
const OFFSET_ID = 0;
const OFFSET_DEBIT = 4;
const OFFSET_CREDIT = 8;
const OFFSET_AMOUNT = 12;
const OFFSET_TIMESTAMP = 20;
const OFFSET_PENDING_ID = 28;
const OFFSET_FLAGS = 32;

function dongGoiButToan(bt: ButToanDayDu): ArrayBuffer {
  const buf = new ArrayBuffer(KICH_THUOC_BAN_GHI);
  const view = new DataView(buf);
  view.setUint32(OFFSET_ID, bt.id);
  view.setUint32(OFFSET_DEBIT, bt.debitAccountId);
  view.setUint32(OFFSET_CREDIT, bt.creditAccountId);
  view.setBigUint64(OFFSET_AMOUNT, BigInt(bt.amount));
  view.setBigUint64(OFFSET_TIMESTAMP, BigInt(bt.timestamp));
  view.setUint32(OFFSET_PENDING_ID, bt.pendingId);
  view.setUint32(OFFSET_FLAGS, bt.flags);
  return buf;
}

function moGoiButToan(buf: ArrayBuffer): ButToanDayDu {
  const view = new DataView(buf);
  return {
    id: view.getUint32(OFFSET_ID),
    debitAccountId: view.getUint32(OFFSET_DEBIT),
    creditAccountId: view.getUint32(OFFSET_CREDIT),
    amount: Number(view.getBigUint64(OFFSET_AMOUNT)),
    timestamp: Number(view.getBigUint64(OFFSET_TIMESTAMP)),
    pendingId: view.getUint32(OFFSET_PENDING_ID),
    flags: view.getUint32(OFFSET_FLAGS),
  };
}

const bt: ButToanDayDu = { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200, timestamp: 1717000000000, pendingId: 0, flags: 0 };
const buf = dongGoiButToan(bt);
const btDoc = moGoiButToan(buf);

console.log("kich thuoc buffer:", buf.byteLength);
console.log("amount doc lai:", btDoc.amount);
console.log("round-trip toan bo khop:", JSON.stringify(btDoc) === JSON.stringify(bt));
```

```text title=readonly
kich thuoc buffer: 128
amount doc lai: 200
round-trip toan bo khop: true
```

`ArrayBuffer` TẠO ra đúng `128` byte (`buf.byteLength`). `amount`
(`200`) được ghi bằng `setBigUint64` (CẦN `BigInt(200)`, không phải
`200` trần — `DataView` từ CHỐI nhận `number` thường cho các phương
thức `64-bit`) VÀ đọc LẠI bằng `getBigUint64`, RỒI chuyển VỀ `number`
qua `Number(...)` — round-trip khớp TUYỆT đối cho CẢ bảy trường.
::::

::::example{#tai-sao-can-bigint}
`view.setBigUint64(offset, 200)` (KHÔNG bọc `BigInt`) NÉM lỗi ngay
lập tức: `TypeError: Cannot convert 200 to a BigInt` — CÁC phương
thức `64-bit` của `DataView` (`setBigUint64`/`getBigUint64`,
`setBigInt64`/`getBigInt64`) LUÔN làm việc VỚI kiểu `bigint`, KHÔNG
BAO giờ tự động ép kiểu TỪ `number`. Đây LÀ lý do `amount`/`timestamp`
Ở `ButToanDayDu` khai kiểu `number` (dễ TÍNH toán VỚI Ở tầng nghiệp
vụ), nhưng phải CHUYỂN sang `BigInt(...)`/`Number(...)` NGAY tại ranh
giới đóng gói/mở gói.
::::

::::predict{#doan-offset-sai commitOnce}
Nếu `moGoiButToan` đọc `amount` TỪ SAI offset (VÍ dụ `OFFSET_TIMESTAMP`
thay VÌ `OFFSET_AMOUNT` — đọc NHẦM sang trường LÂN cận), kết quả
round-trip khớp (`JSON.stringify(btDoc) === JSON.stringify(bt)`) CÓ
còn `true` không?
:::opt{correct}
KHÔNG — đọc NHẦM offset LÀ đọc SAI byte, `amount` đọc RA sẽ LÀ giá
trị của `timestamp` (`1717000000000`), KHÁC hẳn `200` ĐÃ ghi — round-
trip khớp trở THÀNH `false`
:::
:::opt
VẪN `true`, MIỄN là `dongGoiButToan` VÀ `moGoiButToan` dùng CÙNG một
offset sai — sai NHẤT quán cả HAI chiều thì VẪN khớp nhau
::why
Trực giác NÀY đúng NẾU cả HAI hàm CÙNG đọc/ghi SAI offset GIỐNG hệt
nhau — nhưng câu hỏi chỉ nói `moGoiButToan` đọc SAI, `dongGoiButToan`
VẪN ghi ĐÚNG NHƯ code gốc.

Chỗ lệch: `dongGoiButToan` VẪN ghi `amount=200` VÀO `OFFSET_AMOUNT`
(`12`) ĐÚNG như code gốc — CHỈ `moGoiButToan` đọc TỪ
`OFFSET_TIMESTAMP` (`20`), LÀ vị trí của `timestamp`
(`1717000000000`), không PHẢI `amount`. Hai bên KHÔNG còn khớp offset
VỚI nhau — `btDoc.amount` LÀ `1717000000000`, khác `200` ĐÃ ghi, VÀ
round-trip LÀ `false`.
::
:::
::::

::::code{#viet_dong_goi_but_toan}
Hoàn thiện `dongGoiButToan` — ghi `amount` (kiểu `8` byte) VÀO đúng
`OFFSET_AMOUNT` bằng `setBigUint64`, nhớ bọc `BigInt(...)`.

```typescript title=starter
interface ButToanDayDu {
  id: number; debitAccountId: number; creditAccountId: number;
  amount: number; timestamp: number; pendingId: number; flags: number;
}

const KICH_THUOC_BAN_GHI = 128;
const OFFSET_ID = 0;
const OFFSET_DEBIT = 4;
const OFFSET_CREDIT = 8;
const OFFSET_AMOUNT = 12;
const OFFSET_TIMESTAMP = 20;
const OFFSET_PENDING_ID = 28;
const OFFSET_FLAGS = 32;

function dongGoiButToan(bt: ButToanDayDu): ArrayBuffer {
  const buf = new ArrayBuffer(KICH_THUOC_BAN_GHI);
  const view = new DataView(buf);
  view.setUint32(OFFSET_ID, bt.id);
  view.setUint32(OFFSET_DEBIT, bt.debitAccountId);
  view.setUint32(OFFSET_CREDIT, bt.creditAccountId);
  ___
  view.setBigUint64(OFFSET_TIMESTAMP, BigInt(bt.timestamp));
  view.setUint32(OFFSET_PENDING_ID, bt.pendingId);
  view.setUint32(OFFSET_FLAGS, bt.flags);
  return buf;
}

function moGoiButToan(buf: ArrayBuffer): ButToanDayDu {
  const view = new DataView(buf);
  return {
    id: view.getUint32(OFFSET_ID),
    debitAccountId: view.getUint32(OFFSET_DEBIT),
    creditAccountId: view.getUint32(OFFSET_CREDIT),
    amount: Number(view.getBigUint64(OFFSET_AMOUNT)),
    timestamp: Number(view.getBigUint64(OFFSET_TIMESTAMP)),
    pendingId: view.getUint32(OFFSET_PENDING_ID),
    flags: view.getUint32(OFFSET_FLAGS),
  };
}

const bt: ButToanDayDu = { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200, timestamp: 1717000000000, pendingId: 0, flags: 0 };
console.log(moGoiButToan(dongGoiButToan(bt)).amount);
```

```typescript title=solution
interface ButToanDayDu {
  id: number; debitAccountId: number; creditAccountId: number;
  amount: number; timestamp: number; pendingId: number; flags: number;
}

const KICH_THUOC_BAN_GHI = 128;
const OFFSET_ID = 0;
const OFFSET_DEBIT = 4;
const OFFSET_CREDIT = 8;
const OFFSET_AMOUNT = 12;
const OFFSET_TIMESTAMP = 20;
const OFFSET_PENDING_ID = 28;
const OFFSET_FLAGS = 32;

function dongGoiButToan(bt: ButToanDayDu): ArrayBuffer {
  const buf = new ArrayBuffer(KICH_THUOC_BAN_GHI);
  const view = new DataView(buf);
  view.setUint32(OFFSET_ID, bt.id);
  view.setUint32(OFFSET_DEBIT, bt.debitAccountId);
  view.setUint32(OFFSET_CREDIT, bt.creditAccountId);
  view.setBigUint64(OFFSET_AMOUNT, BigInt(bt.amount));
  view.setBigUint64(OFFSET_TIMESTAMP, BigInt(bt.timestamp));
  view.setUint32(OFFSET_PENDING_ID, bt.pendingId);
  view.setUint32(OFFSET_FLAGS, bt.flags);
  return buf;
}

function moGoiButToan(buf: ArrayBuffer): ButToanDayDu {
  const view = new DataView(buf);
  return {
    id: view.getUint32(OFFSET_ID),
    debitAccountId: view.getUint32(OFFSET_DEBIT),
    creditAccountId: view.getUint32(OFFSET_CREDIT),
    amount: Number(view.getBigUint64(OFFSET_AMOUNT)),
    timestamp: Number(view.getBigUint64(OFFSET_TIMESTAMP)),
    pendingId: view.getUint32(OFFSET_PENDING_ID),
    flags: view.getUint32(OFFSET_FLAGS),
  };
}

const bt: ButToanDayDu = { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200, timestamp: 1717000000000, pendingId: 0, flags: 0 };
console.log(moGoiButToan(dongGoiButToan(bt)).amount);
```

```typescript title=test
const bt2: ButToanDayDu = { id: 42, debitAccountId: 7, creditAccountId: 9, amount: 123456, timestamp: 1700000000000, pendingId: 3, flags: 1 };
const buf2 = dongGoiButToan(bt2);
if (buf2.byteLength !== 128) throw new Error("buffer phai co dung 128 byte");
const doc2 = moGoiButToan(buf2);
if (JSON.stringify(doc2) !== JSON.stringify(bt2)) throw new Error("round-trip phai khop TUYET DOI voi bt2 goc");
if (doc2.amount !== 123456) throw new Error("amount doc lai phai la 123456");
if (doc2.timestamp !== 1700000000000) throw new Error("timestamp doc lai phai la 1700000000000");

const btZero: ButToanDayDu = { id: 0, debitAccountId: 0, creditAccountId: 0, amount: 0, timestamp: 0, pendingId: 0, flags: 0 };
const docZero = moGoiButToan(dongGoiButToan(btZero));
if (JSON.stringify(docZero) !== JSON.stringify(btZero)) throw new Error("but toan toan gia tri 0 van phai round-trip dung");

const btLon: ButToanDayDu = { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 9007199254740991, timestamp: 1, pendingId: 0, flags: 0 };
const docLon = moGoiButToan(dongGoiButToan(btLon));
if (docLon.amount !== 9007199254740991) throw new Error("amount lon (Number.MAX_SAFE_INTEGER) phai round-trip dung, khong mat chinh xac");
```

:::hints
- kind: attention
  body: "Ghi amount (8 byte) vao OFFSET_AMOUNT bang setBigUint64, nho boc BigInt(...) -- mot dong."
- kind: strategy
  body: "view.setBigUint64(OFFSET_AMOUNT, BigInt(bt.amount));"
- kind: one-line
  body: "view.setBigUint64(OFFSET_AMOUNT, BigInt(bt.amount));"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "200"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bút toán đóng gói đúng — nhưng một sổ cái THẬT có HÀNG triệu bút
toán. Chúng nằm CẠNH nhau thế NÀO trong MỘT buffer LỚN hơn?
::::

::::reflect{#nghi-lai}
`dongGoiButToan`/`moGoiButToan` LÀ cặp hàm đối XỨNG — mỗi `setXXX`
CÓ đúng một `getXXX` tương ỨNG Ở cùng offset. Layout Ở bài trước GIỜ
không CÒN là lý thuyết TRÊN giấy — nó LÀ `128` byte THẬT, đọc/ghi
được BẰNG chính API JavaScript dùng cho binary data. Bước tiếp theo:
NHIỀU bút toán như VẬY xếp CẠNH nhau — tìm bút toán thứ `N` KHÔNG cần
đọc hết `N-1` bút toán trước nó.
::::

::::checkpoint{mastery=0.85}
::::
