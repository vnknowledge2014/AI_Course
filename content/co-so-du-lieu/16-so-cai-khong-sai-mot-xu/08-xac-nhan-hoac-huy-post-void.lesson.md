---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.xac-nhan-hoac-huy-post-void
title: "Xác nhận hoặc huỷ — post/void"
summary: "xacNhanPending tra cứu đúng pending transfer qua pendingId, chuyển amount TỪ debitsPending/creditsPending SANG debitsPosted/creditsPosted (tiền THẬT SỰ di chuyển lúc này, không phải lúc gửi pending), rồi xoá khỏi sổ chờ. Gọi lại với CÙNG pendingId lần hai trả về false (đã bị xoá, không tìm thấy) — không thể xác nhận hai lần. huyPending chỉ giải phóng hold, không đổi gì ở posted — tiền 'nhả' lại nguyên vẹn."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.xac-nhan-hoac-huy-post-void]
requires: [db.giu-tien-truoc-pending]
concepts: [db.xac-nhan-hoac-huy-post-void]
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
Tiền đã "giữ chỗ" (bài trước). Hai kết cục KHẢ dĩ: khách hàng thanh
toán THẬT (xác nhận), hoặc HUỶ đơn (giữ chỗ tan biến, tiền "nhả" lại).
::::

::::explain{#post-va-void}
`xacNhanPending` tra CỨU đúng pending transfer QUA `pendingId`,
CHUYỂN `amount` TỪ `debitsPending`/`creditsPending` SANG
`debitsPosted`/`creditsPosted` (tiền THẬT sự di chuyển LÚC này,
KHÔNG phải lúc gửi pending), RỒI xoá khỏi sổ CHỜ (`cacPendingDangCho`)
— gọi LẠI với CÙNG `pendingId` lần HAI phải THẤT bại:

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 };
}
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function soDuKhaDung(tk: TaiKhoan): number { return soDuSoSach(tk) - tk.debitsPending; }

function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiPending(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPending += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPending += bt.amount;
  cacPendingDangCho.set(bt.id, bt);
}

function xacNhanPending(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, pendingId: number): boolean {
  const bt = cacPendingDangCho.get(pendingId);
  if (bt === undefined) return false;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPending -= bt.amount;
  credit.creditsPending -= bt.amount;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  cacPendingDangCho.delete(pendingId);
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
const cacPendingDangCho = new Map<number, ButToan>();

apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
guiPending(cacTaiKhoan, cacPendingDangCho, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300 });

console.log("truoc xac nhan -- khaDung:", soDuKhaDung(cacTaiKhoan.get(1)!));
console.log("xac nhan thanh cong:", xacNhanPending(cacTaiKhoan, cacPendingDangCho, 1));
console.log("sau xac nhan -- soSach:", soDuSoSach(cacTaiKhoan.get(1)!));
console.log("xac nhan LAI cung id:", xacNhanPending(cacTaiKhoan, cacPendingDangCho, 1));
```

```text title=readonly
truoc xac nhan -- khaDung: 700
xac nhan thanh cong: true
sau xac nhan -- soSach: 700
xac nhan LAI cung id: false
```

LẦN xác nhận ĐẦU: `debitsPending` (`300`) CHUYỂN sang `debitsPosted`,
`soDuSoSach` giảm TỪ `1000` xuống `700` — tiền THẬT sự đã đi. Xác
nhận `pendingId=1` LẦN THỨ hai: `cacPendingDangCho.get(1)` không CÒN
tìm thấy gì (đã bị `delete` Ở lần TRƯỚC) — trả VỀ `false`, KHÔNG cộng
thêm `300` LẦN nữa. Post HAI lần cùng MỘT pending LÀ không thể.
::::

::::example{#void-nha-tien-lai}
`huyPending` (KHÔNG có Ở readonly TRÊN, nhưng cùng CẤU trúc) chỉ đảo
NGƯỢC bước `guiPending`: TRỪ đi `debitsPending`/`creditsPending` ĐÃ
cộng, KHÔNG hề đụng TỚI `debitsPosted`/`creditsPosted` — `soDuSoSach`
không hề THAY đổi (VÌ chưa BAO giờ đổi), `soDuKhaDung` TRỞ về đúng
giá trị TRƯỚC khi gửi pending. Post VÀ void LÀ hai kết CỤC loại trừ
NHAU của CÙNG một pending — GỌI cái NÀY thì cái KIA không CÒN áp dụng
được NỮA (`pendingId` đã bị xoá khỏi sổ CHỜ sau bất kỳ cái NÀO).
::::

::::predict{#doan-huy-roi-xac-nhan commitOnce}
Gửi pending `id=5` (`amount=100`). Gọi `huyPending(..., 5)` TRƯỚC
(huỷ NÓ) — trả VỀ `true`. NGAY sau đó, gọi TIẾP `xacNhanPending(...,
5)` (cố xác nhận đúng pending ĐÃ huỷ). Kết quả LÀ gì?
:::opt{correct}
`false` — `huyPending` ĐÃ `delete` `pendingId=5` khỏi
`cacPendingDangCho`, NÊN `xacNhanPending` tra cứu KHÔNG tìm thấy gì,
trả VỀ `false` giống hệt trường hợp gọi HAI lần liên tiếp `xacNhanPending`
:::
:::opt
`true`, nhưng KHÔNG cộng thêm tiền — hệ THỐNG "nhớ" pending ĐÓ đã
từng tồn tại nên vẫn xác nhận được, chỉ KHÔNG có tác dụng
::why
Trực giác NÀY tưởng tượng MỘT trạng thái thứ BA ("đã xử lý nhưng ghi
nhớ LỊCH sử") — nhưng `cacPendingDangCho` (đúng NHƯ code Ở TRÊN) chỉ
có ĐÚNG hai trạng thái: "CÓ trong Map" (đang chờ) HOẶC "KHÔNG có
trong Map" (đã xử lý XONG, bằng CÁCH nào không quan trọng).

Chỗ lệch: `huyPending` GỌI `cacPendingDangCho.delete(pendingId)`
GIỐNG hệt `xacNhanPending` — SAU khi huỷ, `pendingId=5` KHÔNG còn
tồn tại TRONG Map NỮA. `xacNhanPending` sau đó tra CỨU
`cacPendingDangCho.get(5)`, nhận VỀ `undefined`, trả về `false` —
HỆT như đã post rồi post LẠI. Post VÀ void đều "TIÊU thụ" pending,
không CÓ cách nào phân biệt "đã post" VỚI "đã void" chỉ NHÌN vào sổ
chờ.
::
:::
::::

::::code{#viet_xac_nhan_pending}
Hoàn thiện `xacNhanPending` — CHUYỂN `amount` TỪ pending SANG posted,
RỒI xoá khỏi sổ chờ.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 };
}
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiPending(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPending += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPending += bt.amount;
  cacPendingDangCho.set(bt.id, bt);
}

function xacNhanPending(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, pendingId: number): boolean {
  const bt = cacPendingDangCho.get(pendingId);
  if (bt === undefined) return false;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  ___
  cacPendingDangCho.delete(pendingId);
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
const cacPendingDangCho = new Map<number, ButToan>();
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
guiPending(cacTaiKhoan, cacPendingDangCho, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300 });
xacNhanPending(cacTaiKhoan, cacPendingDangCho, 1);
console.log(soDuSoSach(cacTaiKhoan.get(1)!));
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 };
}
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiPending(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPending += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPending += bt.amount;
  cacPendingDangCho.set(bt.id, bt);
}

function xacNhanPending(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, pendingId: number): boolean {
  const bt = cacPendingDangCho.get(pendingId);
  if (bt === undefined) return false;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPending -= bt.amount;
  credit.creditsPending -= bt.amount;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  cacPendingDangCho.delete(pendingId);
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
const cacPendingDangCho = new Map<number, ButToan>();
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
guiPending(cacTaiKhoan, cacPendingDangCho, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300 });
xacNhanPending(cacTaiKhoan, cacPendingDangCho, 1);
console.log(soDuSoSach(cacTaiKhoan.get(1)!));
```

```typescript title=test
function huyPending(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, pendingId: number): boolean {
  const bt = cacPendingDangCho.get(pendingId);
  if (bt === undefined) return false;
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPending -= bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPending -= bt.amount;
  cacPendingDangCho.delete(pendingId);
  return true;
}

const NN2 = 0;
const kho2 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2]) kho2.set(id, taoTaiKhoan(id));
const cho2 = new Map<number, ButToan>();
apDungButToanThuong(kho2, { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000 });
guiPending(kho2, cho2, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300 });

if (xacNhanPending(kho2, cho2, 1) !== true) throw new Error("xac nhan lan dau phai thanh cong");
if (soDuSoSach(kho2.get(1)!) !== 700) throw new Error("sau xac nhan, soDuSoSach tai khoan 1 phai la 700");
if (soDuSoSach(kho2.get(2)!) !== 300) throw new Error("sau xac nhan, soDuSoSach tai khoan 2 phai la 300");
if (kho2.get(1)!.debitsPending !== 0) throw new Error("sau xac nhan, debitsPending phai ve 0 (da chuyen sang posted)");
if (xacNhanPending(kho2, cho2, 1) !== false) throw new Error("xac nhan LAI cung pendingId phai that bai (false)");
if (cho2.has(1)) throw new Error("sau xac nhan, pendingId=1 phai bi xoa khoi so cho");

const kho3 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2]) kho3.set(id, taoTaiKhoan(id));
const cho3 = new Map<number, ButToan>();
apDungButToanThuong(kho3, { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000 });
guiPending(kho3, cho3, { id: 5, debitAccountId: 1, creditAccountId: 2, amount: 100 });
if (huyPending(kho3, cho3, 5) !== true) throw new Error("huy lan dau phai thanh cong");
if (xacNhanPending(kho3, cho3, 5) !== false) throw new Error("xac nhan MOT pending DA HUY phai that bai (khong con trong so cho)");
if (soDuSoSach(kho3.get(1)!) !== 1000) throw new Error("pending da huy khong duoc doi soDuSoSach");

if (xacNhanPending(kho3, cho3, 999) !== false) throw new Error("xac nhan mot pendingId chua tung ton tai phai that bai");
```

:::hints
- kind: attention
  body: "Truoc khi cong vao posted, tru dung amount khoi debitsPending/creditsPending -- bon dong."
- kind: strategy
  body: "debit.debitsPending -= bt.amount; credit.creditsPending -= bt.amount; debit.debitsPosted += bt.amount; credit.creditsPosted += bt.amount;"
- kind: one-line
  body: "debit.debitsPending -= bt.amount; credit.creditsPending -= bt.amount; debit.debitsPosted += bt.amount; credit.creditsPosted += bt.amount;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "700"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bút toán, hai giai đoạn — xong. Nhưng ĐÔI khi NHIỀU bút toán cần
"sống chết CÙNG nhau": hoặc TẤT cả áp dụng, hoặc KHÔNG cái nào cả.
::::

::::reflect{#nghi-lai}
Post VÀ void LÀ hai đường dẫn KHÁC nhau đến CÙNG một đích: `pendingId`
biến MẤT khỏi sổ chờ. Chính SỰ đơn giản đó (Map CHỈ có "còn" HAY
"hết", không CÓ trạng thái thứ ba) LÀ điều khiến "post/void hai lần"
KHÔNG thể xảy ra — không PHẢI nhờ một cờ đánh dấu RIÊNG, mà nhờ CẤU
trúc dữ liệu tự nó LOẠI trừ khả năng đó.
::::

::::checkpoint{mastery=0.85}
::::
