---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.boss-so-cai-khong-sai-mot-xu
title: "BOSS — Sổ cái không sai một xu"
summary: "heThongSoCai ráp TRỌN quest: nạp 1000 cho tài khoản 1, gửi 5000 (vượt số dư) bị từ chối, gửi pending 300 rồi xác nhận thành công (soDu về 700), gửi LẠI đúng id đó (giả lập client retry, idempotent, không đổi gì thêm) RỒI gửi THÊM một giao dịch MỚI thật (id=3, 100 — soDu về 600) — VÀ heThongCanBang (bài 2) vẫn true sau MỌI bước, kể cả bước bị từ chối. Năm cơ chế (double-entry, 128-byte, pending/post/void, an toàn, idempotent) hoạt động CÙNG nhau, không cái nào phá vỡ bất biến của cái khác."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 12
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.id-trung-lap-la-idempotent]
concepts: [db.boss-q16]
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
Double-entry (bài 1-2), 128-byte DataView (bài 3-6), pending/post/void
(bài 7-8), linked (bài 9), từ chối số dư âm (bài 10), idempotent
(bài 11). Ráp TẤT cả thành một hệ sổ cái trông ra SAO?
::::

::::explain{#boss-that}
`heThongSoCai` nạp `1000` cho tài khoản `1`, THỬ gửi `5000` (vượt số
dư — bị từ CHỐI), gửi `pending` `300` rồi xác NHẬN (thành công),
RỒI gửi LẠI đúng `id` của bút toán ĐÓ (giả lập client retry SAU lỗi
mạng):

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function soDuKhaDung(tk: TaiKhoan): number { return soDuSoSach(tk) - tk.debitsPending; }

function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiButToanAnToan(cacTaiKhoan: Map<number, TaiKhoan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDuKhaDung(debit) - bt.amount < 0) return false;
  dsIdDaXuLy.add(bt.id);
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  return true;
}

function guiPendingAnToan(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDuKhaDung(debit) - bt.amount < 0) return false;
  dsIdDaXuLy.add(bt.id);
  debit.debitsPending += bt.amount;
  credit.creditsPending += bt.amount;
  cacPendingDangCho.set(bt.id, bt);
  return true;
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

function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}

function heThongSoCai() {
  const NGUON_NGOAI = 0;
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
  const cacPendingDangCho = new Map<number, ButToan>();
  const dsIdDaXuLy = new Set<number>();

  apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });

  const vuotQuaSoDu = guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 5000 });
  const guiPendingOk = guiPendingAnToan(cacTaiKhoan, cacPendingDangCho, dsIdDaXuLy, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 300 });
  const xacNhanOk = xacNhanPending(cacTaiKhoan, cacPendingDangCho, 2);
  const guiTrungLap = guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 300 })
    && guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 3, debitAccountId: 1, creditAccountId: 2, amount: 100 });

  return {
    vuotQuaSoDu, guiPendingOk, xacNhanOk, guiTrungLap,
    canBangCuoiCung: heThongCanBang(cacTaiKhoan),
    soDuCuoiTaiKhoan1: soDuSoSach(cacTaiKhoan.get(1)!),
  };
}

console.log(heThongSoCai());
```

```text title=readonly
{
  vuotQuaSoDu: false,
  guiPendingOk: true,
  xacNhanOk: true,
  guiTrungLap: true,
  canBangCuoiCung: true,
  soDuCuoiTaiKhoan1: 600
}
```

`vuotQuaSoDu: false` — gửi `5000` khi CHỈ có `1000` bị TỪ chối (bài
10), KHÔNG hề đổi bất kỳ số dư NÀO. `guiPendingOk` VÀ `xacNhanOk` đều
`true` — pending `300` được GIỮ rồi xác NHẬN THẬT (bài 7-8), tài
khoản `1` giảm CÒN `700`. `guiTrungLap` nối HAI lệnh gọi bằng `&&`:
gửi LẠI đúng `id=2` (giả lập retry, KHÔNG đổi gì thêm — idempotent,
bài 11) RỒI gửi THÊM một giao dịch MỚI thật (`id=3`, `100`, CHƯA
từng gửi TRƯỚC đó) — CẢ hai đều thành công NÊN `guiTrungLap: true`,
NHƯNG chỉ giao dịch MỚI (`id=3`) làm `soDuCuoiTaiKhoan1` giảm TIẾP từ
`700` xuống `600` — retry KHÔNG hề đóng góp gì VÀO con số đó. VÀ
`canBangCuoiCung: true` — SAU tất cả các bước (kể CẢ bước bị từ
chối), tổng nợ VẪN bằng tổng có (bài 2).
::::

::::example{#buoc-tu-choi-khong-pha-bat-bien}
Điểm ĐÁNG chú ý nhất: bước `vuotQuaSoDu` (bị từ CHỐI) không hề LÀM
`heThongCanBang` sai LỆCH — VÌ `guiButToanAnToan` kiểm tra ĐIỀU kiện
`soDuKhaDung(debit) - amount < 0` TRƯỚC khi ghi bất cứ GÌ (bài 10),
một bút toán bị TỪ chối để LẠI ĐÚNG `0` byte thay ĐỔI trong hệ thống.
An toàn (không CHO âm) VÀ đúng đắn (nợ=có) LÀ hai bất biến ĐỘC lập,
nhưng CẢ hai cùng dựa VÀO một nguyên tắc: kiểm tra TRƯỚC khi ghi,
KHÔNG BAO giờ ghi rồi sửa SAU.
::::

::::predict{#doan-neu-bo-qua-an-toan commitOnce}
NẾU `guiButToanAnToan` KHÔNG hề kiểm tra `soDuKhaDung` (bỏ dòng
`if (soDuKhaDung(debit) - bt.amount < 0) return false;`), gửi `5000`
Ở TRÊN sẽ THÀNH công. `heThongCanBang` cuối CÙNG (`tổng nợ = tổng
có`) CÓ còn đúng KHÔNG?
:::opt{correct}
VẪN đúng — `heThongCanBang` chỉ kiểm TRA tính ĐỐI xứng của MỖI bút
toán (mỗi lần ghi LUÔN cộng CÙNG `amount` vào MỘT `debitsPosted` VÀ
MỘT `creditsPosted`), KHÔNG liên quan GÌ tới việc số dư CÓ âm hay
không — hai bất biến HOÀN toàn độc lập
:::
:::opt
SAI lệch — số dư ÂM chắc chắn phá VỠ tính cân bằng nợ/có
::why
Trực giác NÀY nhầm "một tài khoản CÓ số dư ÂM" VỚI "hệ thống mất cân
bằng nợ/có" — HAI khái niệm hoàn TOÀN khác nhau, dù NGHE có vẻ liên
quan.

Chỗ lệch: `heThongCanBang` (bài 2) CHỈ kiểm tra `tổng debitsPosted ===
tổng creditsPosted` TRÊN mọi tài khoản — bất kể MỖI tài khoản riêng
lẻ CÓ số dư âm hay dương. Dù gửi `5000` khi CHỈ có `1000` (khiến tài
khoản `1` có `soDuSoSach = -4000`), MỖI bút toán VẪN cộng ĐÚNG cùng
`amount` vào MỘT bên nợ VÀ một bên có — tổng nợ VẪN bằng tổng có, DÙ
một tài khoản CÁ nhân giờ "nợ nần". Đây chính LÀ LÝ do bài 10 (an
toàn) LÀ một bất biến RIÊNG, cần thêm VÀO chứ không TỰ động suy ra
từ double-entry.
::
:::
::::

::::code{#viet_he_thong_so_cai}
Hoàn thiện `heThongSoCai` — bước CUỐI gửi LẠI đúng bút toán `id=2`
(giả lập client retry) VÀ gửi THÊM một giao dịch MỚI thật (`id=3`,
`100`), nối HAI lệnh gọi `guiButToanAnToan` đó bằng `&&`.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function soDuKhaDung(tk: TaiKhoan): number { return soDuSoSach(tk) - tk.debitsPending; }

function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiButToanAnToan(cacTaiKhoan: Map<number, TaiKhoan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDuKhaDung(debit) - bt.amount < 0) return false;
  dsIdDaXuLy.add(bt.id);
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  return true;
}

function guiPendingAnToan(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDuKhaDung(debit) - bt.amount < 0) return false;
  dsIdDaXuLy.add(bt.id);
  debit.debitsPending += bt.amount;
  credit.creditsPending += bt.amount;
  cacPendingDangCho.set(bt.id, bt);
  return true;
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

function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}

function heThongSoCai() {
  const NGUON_NGOAI = 0;
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
  const cacPendingDangCho = new Map<number, ButToan>();
  const dsIdDaXuLy = new Set<number>();

  apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });

  const vuotQuaSoDu = guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 5000 });
  const guiPendingOk = guiPendingAnToan(cacTaiKhoan, cacPendingDangCho, dsIdDaXuLy, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 300 });
  const xacNhanOk = xacNhanPending(cacTaiKhoan, cacPendingDangCho, 2);
  const guiTrungLap = ___;

  return {
    vuotQuaSoDu, guiPendingOk, xacNhanOk, guiTrungLap,
    canBangCuoiCung: heThongCanBang(cacTaiKhoan),
    soDuCuoiTaiKhoan1: soDuSoSach(cacTaiKhoan.get(1)!),
  };
}

console.log(heThongSoCai().soDuCuoiTaiKhoan1);
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; debitsPending: number; creditsPending: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0, debitsPending: 0, creditsPending: 0 }; }
function soDuSoSach(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function soDuKhaDung(tk: TaiKhoan): number { return soDuSoSach(tk) - tk.debitsPending; }

function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiButToanAnToan(cacTaiKhoan: Map<number, TaiKhoan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDuKhaDung(debit) - bt.amount < 0) return false;
  dsIdDaXuLy.add(bt.id);
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  return true;
}

function guiPendingAnToan(cacTaiKhoan: Map<number, TaiKhoan>, cacPendingDangCho: Map<number, ButToan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDuKhaDung(debit) - bt.amount < 0) return false;
  dsIdDaXuLy.add(bt.id);
  debit.debitsPending += bt.amount;
  credit.creditsPending += bt.amount;
  cacPendingDangCho.set(bt.id, bt);
  return true;
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

function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  let no = 0, co = 0;
  for (const [, tk] of cacTaiKhoan) { no += tk.debitsPosted; co += tk.creditsPosted; }
  return no === co;
}

function heThongSoCai() {
  const NGUON_NGOAI = 0;
  const cacTaiKhoan = new Map<number, TaiKhoan>();
  for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
  const cacPendingDangCho = new Map<number, ButToan>();
  const dsIdDaXuLy = new Set<number>();

  apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });

  const vuotQuaSoDu = guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 5000 });
  const guiPendingOk = guiPendingAnToan(cacTaiKhoan, cacPendingDangCho, dsIdDaXuLy, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 300 });
  const xacNhanOk = xacNhanPending(cacTaiKhoan, cacPendingDangCho, 2);
  const guiTrungLap = guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 300 })
    && guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 3, debitAccountId: 1, creditAccountId: 2, amount: 100 });

  return {
    vuotQuaSoDu, guiPendingOk, xacNhanOk, guiTrungLap,
    canBangCuoiCung: heThongCanBang(cacTaiKhoan),
    soDuCuoiTaiKhoan1: soDuSoSach(cacTaiKhoan.get(1)!),
  };
}

console.log(heThongSoCai().soDuCuoiTaiKhoan1);
```

```typescript title=test
const kq = heThongSoCai();
if (kq.vuotQuaSoDu !== false) throw new Error("gui 5000 khi chi co 1000 phai bi tu choi (false)");
if (kq.guiPendingOk !== true) throw new Error("gui pending 300 (con du kha dung) phai thanh cong");
if (kq.xacNhanOk !== true) throw new Error("xac nhan pending id=2 phai thanh cong");
if (kq.guiTrungLap !== true) throw new Error("gui lai id=2 (retry) VA gui moi id=3 phai deu thanh cong (true && true)");
if (kq.canBangCuoiCung !== true) throw new Error("he thong phai VAN can bang (tong no = tong co) sau MOI buoc, ke ca buoc bi tu choi");
if (kq.soDuCuoiTaiKhoan1 !== 600) throw new Error("soDu cuoi cung tai khoan 1 phai la 600 (1000 nap - 300 da xac nhan - 100 giao dich MOI id=3; gui lai id=2 KHONG tru them)");
```

:::hints
- kind: attention
  body: "Noi hai lenh goi guiButToanAnToan (retry id=2, VA giao dich moi id=3) bang && -- mot bieu thuc."
- kind: strategy
  body: "const guiTrungLap = guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 300 }) && guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 3, debitAccountId: 1, creditAccountId: 2, amount: 100 });"
- kind: one-line
  body: "const guiTrungLap = guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 300 }) && guiButToanAnToan(cacTaiKhoan, dsIdDaXuLy, { id: 3, debitAccountId: 1, creditAccountId: 2, amount: 100 });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "600"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Double-entry, DataView 128-byte, pending/post/void, linked, an toàn,
idempotent — sáu cơ chế, một sổ cái không sai một xu. q16 khép lại —
R6-3 (63 bài, q16-q21, "TigerBeetle nhỏ") tiếp tục với nền TẤT ĐỊNH
cần thiết để CHẠY lại chính xác cùng một kịch bản, nhiều lần.
::::

::::reflect{#nghi-lai}
`heThongSoCai` không giới thiệu MỘT khái niệm mới nào — nó XẾP đúng
thứ TỰ mọi thứ đã học: nạp tiền (bài 1) LÀ một bút toán CÓ hai vế,
mỗi thao TÁC kiểm tra AN toàn TRƯỚC khi ghi (bài 10), pending/post
(bài 7-8) tách RIÊNG "giữ chỗ" khỏi "thật sự di chuyển", idempotent
(bài 11) LÀM client dám retry, VÀ `heThongCanBang` (bài 2) xác nhận
SAU CÙNG rằng không CÓ thao tác nào (kể cả thao TÁC bị từ chối) từng
làm tiền "RÒ rỉ". Bài học lớn NHẤT của quest này: một sổ cái ĐÁNG tin
không phải VÌ nó "không BAO giờ sai" — mà VÌ MỌI sai sót (số dư âm,
gửi trùng, chuỗi hỏng) đều bị CHẶN Ở đúng biên giới, TRƯỚC khi kịp
ghi vào bất cứ đâu.
::::

::::checkpoint{mastery=0.9}
::::
