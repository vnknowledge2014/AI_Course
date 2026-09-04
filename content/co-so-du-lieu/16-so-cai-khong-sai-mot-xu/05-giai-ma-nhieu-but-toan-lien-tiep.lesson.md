---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.giai-ma-nhieu-but-toan-lien-tiep
title: "Giải mã nhiều bút toán liên tiếp"
summary: "docButToanThuN đọc bút toán thứ N trong một buffer LỚN chứa nhiều bút toán 128-byte xếp CẠNH nhau — dùng DataView(bufLon, n*128, 128) để 'nhìn' đúng đúng 128 byte bắt đầu từ offset n*128, không cần đọc qua N-1 bút toán trước. Với 5 bút toán (640 byte), bút toán thứ 3 (offset 384) có amount=400 — truy cập TRỰC TIẾP, đúng lợi ích của kích cỡ bản ghi cố định đã nêu ở bài 3."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.giai-ma-nhieu-but-toan-lien-tiep]
requires: [db.dong-goi-bang-dataview]
concepts: [db.giai-ma-nhieu-but-toan-lien-tiep]
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
Một bút toán đóng gói đúng (bài trước). Một sổ CÁI thật có HÀNG triệu
bút toán, xếp CẠNH nhau trong MỘT file. Tìm bút toán thứ `N` thế NÀO?
::::

::::explain{#doc-truc-tiep-theo-offset}
`docButToanThuN` đọc bút toán thứ `N` trong một buffer LỚN chứa
NHIỀU bút toán `128`-byte xếp CẠNH nhau — dùng
`new DataView(bufLon, n*128, 128)` để "nhìn" đúng `128` byte bắt đầu
TỪ offset `n*128`, KHÔNG cần đọc qua `N-1` bút toán trước:

```typescript title=readonly
interface ButToanDayDu {
  id: number; debitAccountId: number; creditAccountId: number;
  amount: number; timestamp: number; pendingId: number; flags: number;
}
const KICH_THUOC_BAN_GHI = 128;
const OFFSET_ID = 0, OFFSET_DEBIT = 4, OFFSET_CREDIT = 8, OFFSET_AMOUNT = 12, OFFSET_TIMESTAMP = 20, OFFSET_PENDING_ID = 28, OFFSET_FLAGS = 32;

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

function ghiNhieuButToan(cacButToan: ButToanDayDu[]): ArrayBuffer {
  const bufLon = new ArrayBuffer(cacButToan.length * KICH_THUOC_BAN_GHI);
  const mangByte = new Uint8Array(bufLon);
  cacButToan.forEach((bt, i) => {
    const goi = new Uint8Array(dongGoiButToan(bt));
    mangByte.set(goi, i * KICH_THUOC_BAN_GHI);
  });
  return bufLon;
}

function docButToanThuN(bufLon: ArrayBuffer, n: number): ButToanDayDu {
  const view = new DataView(bufLon, n * KICH_THUOC_BAN_GHI, KICH_THUOC_BAN_GHI);
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

const dsButToan: ButToanDayDu[] = [];
for (let i = 0; i < 5; i++) dsButToan.push({ id: i, debitAccountId: 1, creditAccountId: 2, amount: (i + 1) * 100, timestamp: 1000 + i, pendingId: 0, flags: 0 });

const bufLon = ghiNhieuButToan(dsButToan);
console.log("kich thuoc buffer lon:", bufLon.byteLength);
console.log("but toan thu 3 -- amount:", docButToanThuN(bufLon, 3).amount);
```

```text title=readonly
kich thuoc buffer lon: 640
but toan thu 3 -- amount: 400
```

`5` bút toán chiếm ĐÚNG `640` byte (`5 × 128`). Bút toán thứ `3`
(đếm TỪ `0`) nằm Ở offset `384` (`3 × 128`) — `docButToanThuN` NHẢY
thẳng TỚI đó bằng tham số thứ hai của `DataView`, KHÔNG cần đọc qua
bút toán `0`, `1`, `2` TRƯỚC nó — đúng lợi ích của kích cỡ bản ghi
CỐ định ĐÃ nêu Ở bài 3.
::::

::::example{#tham-so-thu-hai-cua-dataview}
`new DataView(buffer, byteOffset, byteLength)` — tham SỐ thứ hai
(`byteOffset`) LÀ chìa khoá của bài NÀY: nó tạo MỘT "cửa sổ nhìn"
BẮT đầu Ở đúng vị trí ĐÓ, khiến MỌI offset `getXXX`/`setXXX` GỌI sau
đó (VÍ dụ `OFFSET_AMOUNT=12`) được tính TƯƠNG đối SO với `byteOffset`
NÀY, không phải từ ĐẦU buffer gốc. `docButToanThuN` KHÔNG hề cần biết
`n-1` bút toán trước nó dài BAO nhiêu — VÌ chúng LUÔN dài đúng `128`.
::::

::::predict{#doan-but-toan-cuoi commitOnce}
CÙNG `5` bút toán Ở readonly TRÊN (`amount` LÀ `100, 200, 300, 400,
500` cho bút toán `0` tới `4`). Gọi `docButToanThuN(bufLon, 4)` (bút
toán CUỐI cùng, chỉ SỐ `4`). `amount` đọc RA LÀ bao nhiêu?
:::opt{correct}
`500` — bút toán thứ `4` (chỉ số ĐẾM từ `0`) ứng VỚI `i=4` lúc TẠO
(`(4+1)*100=500`), nằm Ở offset `512` (`4×128`)
:::
:::opt
LỖI — chỉ SỐ `4` vượt QUÁ phạm vi hợp lệ (chỉ có `5` bút toán, đánh
SỐ `0` tới `4`, nên `4` LÀ "phần tử thứ sáu")
::why
Trực giác NÀY nhầm "SỐ lượng phần tử" (`5`) VỚI "chỉ số LỚN nhất hợp
lệ" — HAI con SỐ khác nhau đúng `1` ĐƠN vị.

Chỗ lệch: `5` bút toán được đánh SỐ `0, 1, 2, 3, 4` (giống chỉ SỐ
mảng JavaScript) — `4` LÀ chỉ số của phần TỬ CUỐI cùng, hoàn toàn hợp
lệ, KHÔNG phải phần tử THỨ sáu (sẽ LÀ chỉ số `5`, thật SỰ ngoài phạm
vi). `docButToanThuN(bufLon, 4)` đọc offset `512`, ĐÚNG vị trí bút
toán CUỐI, `amount=500`.
::
:::
::::

::::code{#viet_doc_but_toan_thu_n}
Hoàn thiện `docButToanThuN` — tạo `DataView` bắt đầu Ở offset
`n * KICH_THUOC_BAN_GHI`, dài đúng `KICH_THUOC_BAN_GHI` byte.

```typescript title=starter
interface ButToanDayDu {
  id: number; debitAccountId: number; creditAccountId: number;
  amount: number; timestamp: number; pendingId: number; flags: number;
}
const KICH_THUOC_BAN_GHI = 128;
const OFFSET_ID = 0, OFFSET_DEBIT = 4, OFFSET_CREDIT = 8, OFFSET_AMOUNT = 12, OFFSET_TIMESTAMP = 20, OFFSET_PENDING_ID = 28, OFFSET_FLAGS = 32;

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

function ghiNhieuButToan(cacButToan: ButToanDayDu[]): ArrayBuffer {
  const bufLon = new ArrayBuffer(cacButToan.length * KICH_THUOC_BAN_GHI);
  const mangByte = new Uint8Array(bufLon);
  cacButToan.forEach((bt, i) => {
    const goi = new Uint8Array(dongGoiButToan(bt));
    mangByte.set(goi, i * KICH_THUOC_BAN_GHI);
  });
  return bufLon;
}

function docButToanThuN(bufLon: ArrayBuffer, n: number): ButToanDayDu {
  const view = ___;
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

const dsButToan: ButToanDayDu[] = [];
for (let i = 0; i < 5; i++) dsButToan.push({ id: i, debitAccountId: 1, creditAccountId: 2, amount: (i + 1) * 100, timestamp: 1000 + i, pendingId: 0, flags: 0 });
console.log(docButToanThuN(ghiNhieuButToan(dsButToan), 3).amount);
```

```typescript title=solution
interface ButToanDayDu {
  id: number; debitAccountId: number; creditAccountId: number;
  amount: number; timestamp: number; pendingId: number; flags: number;
}
const KICH_THUOC_BAN_GHI = 128;
const OFFSET_ID = 0, OFFSET_DEBIT = 4, OFFSET_CREDIT = 8, OFFSET_AMOUNT = 12, OFFSET_TIMESTAMP = 20, OFFSET_PENDING_ID = 28, OFFSET_FLAGS = 32;

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

function ghiNhieuButToan(cacButToan: ButToanDayDu[]): ArrayBuffer {
  const bufLon = new ArrayBuffer(cacButToan.length * KICH_THUOC_BAN_GHI);
  const mangByte = new Uint8Array(bufLon);
  cacButToan.forEach((bt, i) => {
    const goi = new Uint8Array(dongGoiButToan(bt));
    mangByte.set(goi, i * KICH_THUOC_BAN_GHI);
  });
  return bufLon;
}

function docButToanThuN(bufLon: ArrayBuffer, n: number): ButToanDayDu {
  const view = new DataView(bufLon, n * KICH_THUOC_BAN_GHI, KICH_THUOC_BAN_GHI);
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

const dsButToan: ButToanDayDu[] = [];
for (let i = 0; i < 5; i++) dsButToan.push({ id: i, debitAccountId: 1, creditAccountId: 2, amount: (i + 1) * 100, timestamp: 1000 + i, pendingId: 0, flags: 0 });
console.log(docButToanThuN(ghiNhieuButToan(dsButToan), 3).amount);
```

```typescript title=test
const ds2: ButToanDayDu[] = [];
for (let i = 0; i < 5; i++) ds2.push({ id: i, debitAccountId: 1, creditAccountId: 2, amount: (i + 1) * 100, timestamp: 1000 + i, pendingId: 0, flags: 0 });
const buf2 = ghiNhieuButToan(ds2);
if (buf2.byteLength !== 640) throw new Error("5 but toan phai chiem dung 640 byte (5*128)");

if (docButToanThuN(buf2, 0).amount !== 100) throw new Error("but toan thu 0 phai co amount=100");
if (docButToanThuN(buf2, 3).amount !== 400) throw new Error("but toan thu 3 phai co amount=400");
if (docButToanThuN(buf2, 4).amount !== 500) throw new Error("but toan thu 4 (CUOI cung, khong phai ngoai pham vi) phai co amount=500");
if (docButToanThuN(buf2, 4).id !== 4) throw new Error("but toan thu 4 phai co id=4");

for (let i = 0; i < 5; i++) {
  if (docButToanThuN(buf2, i).timestamp !== 1000 + i) throw new Error(`but toan thu ${i} phai co timestamp=${1000 + i}`);
}

const mot: ButToanDayDu[] = [{ id: 9, debitAccountId: 1, creditAccountId: 2, amount: 777, timestamp: 5, pendingId: 0, flags: 0 }];
const bufMot = ghiNhieuButToan(mot);
if (bufMot.byteLength !== 128) throw new Error("1 but toan phai chiem dung 128 byte");
if (docButToanThuN(bufMot, 0).amount !== 777) throw new Error("but toan duy nhat phai doc dung amount=777");
```

:::hints
- kind: attention
  body: "Tao DataView bat dau o offset n*KICH_THUOC_BAN_GHI, dai dung KICH_THUOC_BAN_GHI -- mot dong."
- kind: strategy
  body: "const view = new DataView(bufLon, n * KICH_THUOC_BAN_GHI, KICH_THUOC_BAN_GHI);"
- kind: one-line
  body: "const view = new DataView(bufLon, n * KICH_THUOC_BAN_GHI, KICH_THUOC_BAN_GHI);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "400"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đọc/ghi nhị phân đã vững — quay LẠI câu hỏi nghiệp vụ: số dư MỘT tài
khoản được TÍNH từ TOÀN bộ lịch sử bút toán THẾ nào?
::::

::::reflect{#nghi-lai}
`docButToanThuN` LÀ minh chứng THẬT cho lợi ích của kích cỡ CỐ định
(bài 3): truy cập NGẪU nhiên (random access) vào bút toán THỨ `N`
BẤT kỳ, chi phí HẰNG số, KHÔNG phụ thuộc `N` lớn cỡ NÀO — y hệt
`offset = N × kichThuocSector` ĐÃ dùng cho SimDisk (q00-q02, Python).
Bây giờ CÓ thể đọc bút toán bất KỲ — bước tiếp theo LÀ dùng chúng để
tính LẠI số dư một tài khoản, không dựa VÀO `debitsPosted`/
`creditsPosted` đã lưu SẴN mà tính từ CHÍNH lịch sử bút toán.
::::

::::checkpoint{mastery=0.85}
::::
