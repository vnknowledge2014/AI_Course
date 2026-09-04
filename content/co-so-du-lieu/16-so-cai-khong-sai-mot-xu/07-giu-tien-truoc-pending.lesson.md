---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.giu-tien-truoc-pending
title: "Giữ tiền trước — pending"
summary: "guiButToan xử lý bút toán pending KHÁC bút toán thường: thay vì cộng debitsPosted/creditsPosted, nó cộng debitsPending/creditsPending — một khoản 'giữ chỗ' CHƯA thật sự di chuyển. soDuSoSach (posted balance) không đổi khi có pending; soDuKhaDung (available balance) trừ thêm debitsPending. Tài khoản có 1000 posted, gửi một pending 300: soDuSoSach vẫn 1000, soDuKhaDung giảm còn 700 — tiền chưa đi đâu cả, nhưng không còn 'khả dụng' để dùng tiếp."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.giu-tien-truoc-pending]
requires: [db.so-du-la-tong-dua-tru]
concepts: [db.giu-tien-truoc-pending]
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
Mọi bút toán TỚI giờ đều "chuyển NGAY". Nhưng đặt hàng THẺ (giữ chỗ
khách sạn, mua hàng ONLINE) thường CẦN hai bước: giữ TRƯỚC, xác nhận
SAU. TigerBeetle gọi bước ĐẦU LÀ "pending".
::::

::::explain{#but-toan-pending}
`guiButToan` xử LÝ bút toán `pending` KHÁC bút toán thường: thay VÌ
cộng `debitsPosted`/`creditsPosted`, nó cộng `debitsPending`/
`creditsPending` — một khoản "GIỮ chỗ" CHƯA thật sự di chuyển:

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; pending: boolean; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 };
}
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function soDuKhaDung(tk: TaiKhoan): number { return soDuSoSach(tk) - tk.debitsPending; }

function guiButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (bt.pending) {
    debit.debitsPending += bt.amount;
    credit.creditsPending += bt.amount;
  } else {
    debit.debitsPosted += bt.amount;
    credit.creditsPosted += bt.amount;
  }
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
guiButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000, pending: false });

console.log("truoc pending -- soSach:", soDuSoSach(cacTaiKhoan.get(1)!), "khaDung:", soDuKhaDung(cacTaiKhoan.get(1)!));

guiButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300, pending: true });

console.log("sau pending -- soSach:", soDuSoSach(cacTaiKhoan.get(1)!), "khaDung:", soDuKhaDung(cacTaiKhoan.get(1)!));
```

```text title=readonly
truoc pending -- soSach: 1000 khaDung: 1000
sau pending -- soSach: 1000 khaDung: 700
```

TRƯỚC bút toán `pending`, tài khoản `1` CÓ `soDuSoSach=1000` VÀ
`soDuKhaDung=1000` — bằng nhau, chưa CÓ gì bị giữ. SAU khi gửi
`pending` `300`, `soDuSoSach` VẪN `1000` (chưa CÓ gì THẬT sự di
chuyển) — nhưng `soDuKhaDung` giảm CÒN `700`: `300` đó đã bị "khoá
lại", KHÔNG còn khả DỤNG để chi tiếp, DÙ vẫn nằm TRONG sổ sách.
::::

::::example{#hai-loai-so-du-khac-nhau}
Đây LÀ điểm khác biệt CĂN bản SO với mọi bút toán trước ĐÓ (bài 1-6):
LẦN đầu tiên MỘT tài khoản có HAI con số "số dư" khác nhau CÙNG lúc.
`soDuSoSach` trả LỜI "sổ sách nói tài khoản NÀY có bao nhiêu" —
`soDuKhaDung` trả lời "CÒN bao nhiêu để CHI tiếp NGAY bây giờ". Chênh
LỆCH giữa hai con số ĐÓ CHÍNH LÀ tổng các khoản pending ĐANG chờ.
::::

::::predict{#doan-tai-khoan-nhan commitOnce}
Tài khoản `2` (bên ghi CÓ của bút toán `pending` `300`) — `soDuKhaDung`
của tài khoản `2` NGAY sau bút toán `pending` đó (TRƯỚC khi CÓ bất kỳ
`post`/`void` nào, sẽ học Ở bài SAU) LÀ bao nhiêu?
:::opt{correct}
`0` — `creditsPending` của tài khoản `2` tăng LÊN `300`, nhưng
`soDuKhaDung` CHỈ trừ `debitsPending` CỦA chính tài khoản đó (không
hề CỘNG `creditsPending`) — tài khoản `2` chưa TỪNG nhận `posted` gì,
NÊN cả `soDuSoSach` LẪN `soDuKhaDung` đều VẪN LÀ `0`
:::
:::opt
`300` — tài khoản `2` "sắp nhận được" `300`, NÊN số dư khả dụng phải
phản ánh khoản SẮP tới đó
::why
Trực giác NÀY hợp lý Ở góc độ "kỳ vọng nhận tiền" — nhưng
`soDuKhaDung` (đúng NHƯ code Ở trên) KHÔNG hề cộng `creditsPending`
vào CÔNG thức.

Chỗ lệch: `soDuKhaDung(tk) = soDuSoSach(tk) - tk.debitsPending` —
CHỈ trừ ĐI phần "SẮP rời khỏi" (`debitsPending` CỦA chính tài khoản),
KHÔNG cộng THÊM phần "sắp NHẬN được" (`creditsPending`). Đây LÀ lựa
chọn CÓ chủ đích: tiền CHỈ tính LÀ "khả dụng" SAU khi thật sự
`posted` (bài SAU), không phải khi CHỈ mới "đang chờ tới".
::
:::
::::

::::code{#viet_gui_but_toan}
Hoàn thiện `guiButToan` — VỚI bút toán `pending`, cộng
`debitsPending`/`creditsPending` thay VÌ `debitsPosted`/`creditsPosted`.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; pending: boolean; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 };
}
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function soDuKhaDung(tk: TaiKhoan): number { return soDuSoSach(tk) - tk.debitsPending; }

function guiButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (bt.pending) {
    ___
  } else {
    debit.debitsPosted += bt.amount;
    credit.creditsPosted += bt.amount;
  }
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
guiButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000, pending: false });
guiButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300, pending: true });
console.log(soDuKhaDung(cacTaiKhoan.get(1)!));
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; pending: boolean; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 };
}
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function soDuKhaDung(tk: TaiKhoan): number { return soDuSoSach(tk) - tk.debitsPending; }

function guiButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (bt.pending) {
    debit.debitsPending += bt.amount;
    credit.creditsPending += bt.amount;
  } else {
    debit.debitsPosted += bt.amount;
    credit.creditsPosted += bt.amount;
  }
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
guiButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000, pending: false });
guiButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300, pending: true });
console.log(soDuKhaDung(cacTaiKhoan.get(1)!));
```

```typescript title=test
const NN2 = 0;
const kho2 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2]) kho2.set(id, taoTaiKhoan(id));
guiButToan(kho2, { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000, pending: false });
if (soDuSoSach(kho2.get(1)!) !== 1000) throw new Error("truoc pending, soDuSoSach phai la 1000");
if (soDuKhaDung(kho2.get(1)!) !== 1000) throw new Error("truoc pending, soDuKhaDung phai bang soDuSoSach = 1000");

guiButToan(kho2, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 300, pending: true });
if (soDuSoSach(kho2.get(1)!) !== 1000) throw new Error("pending KHONG duoc doi soDuSoSach -- van phai la 1000");
if (soDuKhaDung(kho2.get(1)!) !== 700) throw new Error("sau pending 300, soDuKhaDung phai giam con 700");
if (kho2.get(1)!.debitsPosted !== 0) throw new Error("but toan pending KHONG duoc cong vao debitsPosted");
if (kho2.get(1)!.debitsPending !== 300) throw new Error("but toan pending phai cong dung 300 vao debitsPending");

if (soDuSoSach(kho2.get(2)!) !== 0) throw new Error("tai khoan 2 (ben nhan pending) chua he co gi posted -- soDuSoSach phai la 0");
if (soDuKhaDung(kho2.get(2)!) !== 0) throw new Error("tai khoan 2 chua duoc cong creditsPending vao khaDung -- van phai la 0");
if (kho2.get(2)!.creditsPending !== 300) throw new Error("tai khoan 2 phai ghi nhan dung 300 o creditsPending");

guiButToan(kho2, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 700, pending: false });
if (soDuKhaDung(kho2.get(1)!) !== 0) throw new Error("but toan THUONG (khong pending) 700 tiep theo phai lam khaDung giam tiep tu 700 xuong 0");
```

:::hints
- kind: attention
  body: "Bui toan pending: cong debitsPending cua debit VA creditsPending cua credit, cung amount -- hai dong."
- kind: strategy
  body: "debit.debitsPending += bt.amount; credit.creditsPending += bt.amount;"
- kind: one-line
  body: "debit.debitsPending += bt.amount; credit.creditsPending += bt.amount;"
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
Tiền đã bị "giữ" — nhưng giữ MÃI thì vô nghĩa. Cần một bước THỨ hai:
xác nhận THẬT sự di chuyển, hoặc huỷ, trả lại "khả dụng".
::::

::::reflect{#nghi-lai}
`guiButToan` giờ CÓ hai NHÁNH ứng xử hoàn toàn khác NHAU cho CÙNG một
kiểu dữ liệu `ButToan` — chỉ khác Ở CỜ `pending`. Đây LÀ "two-phase"
đúng nghĩa: giai đoạn MỘT (pending) chỉ ĐẶT cọc, không di chuyển GÌ
thật; `soDuSoSach` (điều MỌI báo cáo tài chính quan TÂM) hoàn toàn
KHÔNG bị ảnh hưởng. Giai đoạn HAI — xác nhận (post) hay huỷ (void) —
LÀ nơi quyết định CUỐI cùng thật sự XẢY ra.
::::

::::checkpoint{mastery=0.85}
::::

