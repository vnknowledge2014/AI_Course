---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.linked-tat-ca-cung-thang
title: "Linked — tất cả cùng thắng"
summary: "apDungChuoiLienKet KIỂM TRA TRƯỚC toàn bộ chuỗi bút toán (mọi debitAccountId/creditAccountId phải tồn tại) TRƯỚC KHI áp dụng bất kỳ cái nào — một bút toán tham chiếu tài khoản không tồn tại (id=99) khiến CẢ chuỗi bị từ chối, kể cả bút toán ĐẦU chuỗi vốn hoàn toàn hợp lệ riêng lẻ. Tài khoản 1 giữ nguyên 1000 sau chuỗi lỗi — không một bút toán nào trong chuỗi đó áp dụng, dù chỉ một cái sai."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [db.linked-tat-ca-cung-thang]
requires: [db.xac-nhan-hoac-huy-post-void]
concepts: [db.linked-tat-ca-cung-thang]
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
Two-phase (bài 7-8) xử lý ĐÚNG một bút toán, hai bước. NHƯNG đôi khi
CẦN nhiều bút toán "sống chết CÙNG nhau" — TigerBeetle gọi LÀ linked.
::::

::::explain{#chuoi-lien-ket}
`apDungChuoiLienKet` KIỂM tra TRƯỚC toàn bộ chuỗi (MỌI
`debitAccountId`/`creditAccountId` PHẢI tồn tại) TRƯỚC khi áp dụng
BẤT kỳ bút toán NÀO — một bút toán tham CHIẾU tài khoản không tồn
tại khiến CẢ chuỗi bị từ CHỐI, kể cả những bút toán ĐỨNG trước nó
trong chuỗi VỐN hợp lệ riêng lẻ:

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function apDungChuoiLienKet(cacTaiKhoan: Map<number, TaiKhoan>, dsButToan: ButToan[]): boolean {
  for (const bt of dsButToan) {
    if (!cacTaiKhoan.has(bt.debitAccountId) || !cacTaiKhoan.has(bt.creditAccountId)) {
      return false;
    }
  }
  for (const bt of dsButToan) {
    apDungButToanThuong(cacTaiKhoan, bt);
  }
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2, 3]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });

const chuoiCoLoi: ButToan[] = [
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 },
  { id: 2, debitAccountId: 2, creditAccountId: 99, amount: 50 },
  { id: 3, debitAccountId: 3, creditAccountId: 1, amount: 20 },
];

console.log("truoc chuoi -- soDu tai khoan 1:", soDu(cacTaiKhoan.get(1)!));
console.log("chuoi ap dung thanh cong:", apDungChuoiLienKet(cacTaiKhoan, chuoiCoLoi));
console.log("sau chuoi that bai -- soDu tai khoan 1:", soDu(cacTaiKhoan.get(1)!));
```

```text title=readonly
truoc chuoi -- soDu tai khoan 1: 1000
chuoi ap dung thanh cong: false
sau chuoi that bai -- soDu tai khoan 1: 1000
```

Bút toán THỨ hai (`id:2`) tham chiếu tài khoản `99` (KHÔNG tồn tại
trong `cacTaiKhoan`) — `apDungChuoiLienKet` phát hiện Ở vòng kiểm TRA
đầu tiên, trả VỀ `false` NGAY, KHÔNG hề chạm tới vòng ÁP dụng thứ
hai. Bút toán THỨ nhất (`debit=1, credit=2, amount=100`) HOÀN toàn
hợp LỆ riêng lẻ — nhưng VÌ nằm CÙNG chuỗi VỚI bút toán lỗi, nó CŨNG
không được áp DỤNG: `soDu` tài khoản `1` giữ NGUYÊN `1000`.
::::

::::example{#hai-vong-rieng-biet}
Bí quyết LÀ HAI vòng lặp TÁCH biệt: vòng ĐẦU chỉ KIỂM tra (không ghi
gì), vòng SAU mới THẬT sự áp dụng — VÀ vòng SAU chỉ chạy khi vòng
ĐẦU xác nhận TOÀN bộ chuỗi hợp lệ. Nếu gộp làm MỘT vòng (kiểm tra VÀ
áp dụng CÙNG lúc), bút toán `id:1` ĐàÁP dụng XONG rồi mới phát hiện
`id:2` lỗi — LÚC đó không CÒN cách "hoàn tác" bút toán `id:1` một
cách đơn giản. Kiểm tra TRƯỚC, áp dụng SAU LÀ điều kiện để atomicity
hoạt động ĐÚNG.
::::

::::predict{#doan-chuoi-tat-ca-hop-le commitOnce}
Chuỗi MỚI: `[{debit:1,credit:2,amount:100}, {debit:2,credit:3,
amount:50}]` — CẢ hai bút toán ĐỀU tham chiếu tài khoản CÓ THẬT
(`1`, `2`, `3`). Gọi `apDungChuoiLienKet` VỚI chuỗi NÀY (tài khoản `1`
đang CÓ `1000`). Kết quả trả VỀ VÀ `soDu` tài khoản `1` SAU đó LÀ gì?
:::opt{correct}
Trả VỀ `true`, `soDu` tài khoản `1` THÀNH `900` — CẢ hai bút toán
ĐỀU hợp lệ (mọi tài khoản tồn TẠI), NÊN vòng kiểm tra qua ĐƯỢC, vòng
áp dụng chạy CẢ hai, tài khoản `1` mất `100` (là bên ghi NỢ Ở bút
toán đầu)
:::
:::opt
Trả VỀ `true`, nhưng `soDu` tài khoản `1` VẪN LÀ `1000` — "hợp lệ"
CHỈ có nghĩa LÀ không LỖI, không có nghĩa LÀ ĐÃ thật sự ghi
::why
Trực giác NÀY tách RỜI "trả về `true`" VỚI "đã ghi THẬT" — nhưng
`apDungChuoiLienKet` (đúng NHƯ code) CHỈ trả `true` SAU khi vòng THỨ
hai (áp dụng THẬT) đã chạy XONG.

Chỗ lệch: KHI vòng kiểm tra ĐẦU tiên không phát hiện lỗi NÀO
(`return false` không hề được GỌI), hàm rơi XUỐNG đúng vòng thứ hai
— gọi `apDungButToanThuong` cho TỪNG bút toán TRONG chuỗi, GHI thật
vào `cacTaiKhoan`. `soDu` tài khoản `1` THẬT sự giảm `100` (bút toán
đầu, `debit=1`), THÀNH `900`.
::
:::
::::

::::code{#viet_ap_dung_chuoi_lien_ket}
Hoàn thiện `apDungChuoiLienKet` — vòng kiểm TRA: nếu tài khoản ghi
nợ HOẶC ghi có của bút toán KHÔNG tồn tại, từ chối cả chuỗi NGAY.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function apDungChuoiLienKet(cacTaiKhoan: Map<number, TaiKhoan>, dsButToan: ButToan[]): boolean {
  for (const bt of dsButToan) {
    ___
  }
  for (const bt of dsButToan) {
    apDungButToanThuong(cacTaiKhoan, bt);
  }
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2, 3]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
const chuoi: ButToan[] = [
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 },
  { id: 2, debitAccountId: 2, creditAccountId: 99, amount: 50 },
];
console.log(apDungChuoiLienKet(cacTaiKhoan, chuoi));
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function apDungChuoiLienKet(cacTaiKhoan: Map<number, TaiKhoan>, dsButToan: ButToan[]): boolean {
  for (const bt of dsButToan) {
    if (!cacTaiKhoan.has(bt.debitAccountId) || !cacTaiKhoan.has(bt.creditAccountId)) {
      return false;
    }
  }
  for (const bt of dsButToan) {
    apDungButToanThuong(cacTaiKhoan, bt);
  }
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2, 3]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
const chuoi: ButToan[] = [
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 },
  { id: 2, debitAccountId: 2, creditAccountId: 99, amount: 50 },
];
console.log(apDungChuoiLienKet(cacTaiKhoan, chuoi));
```

```typescript title=test
const NN2 = 0;
const kho2 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2, 3]) kho2.set(id, taoTaiKhoan(id));
apDungButToanThuong(kho2, { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000 });

const chuoiLoi: ButToan[] = [
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 },
  { id: 2, debitAccountId: 2, creditAccountId: 99, amount: 50 },
  { id: 3, debitAccountId: 3, creditAccountId: 1, amount: 20 },
];
if (apDungChuoiLienKet(kho2, chuoiLoi) !== false) throw new Error("chuoi co but toan tham chieu tai khoan khong ton tai phai bi tu choi (false)");
if (soDu(kho2.get(1)!) !== 1000) throw new Error("tai khoan 1 phai GIU NGUYEN 1000 -- but toan dau chuoi (hop le rieng le) khong duoc ap dung");
if (soDu(kho2.get(2)!) !== 0) throw new Error("tai khoan 2 phai GIU NGUYEN 0 -- khong nhan gi tu chuoi bi tu choi");
if (soDu(kho2.get(3)!) !== 0) throw new Error("tai khoan 3 cung phai giu nguyen 0");

const chuoiHopLe: ButToan[] = [
  { id: 4, debitAccountId: 1, creditAccountId: 2, amount: 100 },
  { id: 5, debitAccountId: 2, creditAccountId: 3, amount: 50 },
];
if (apDungChuoiLienKet(kho2, chuoiHopLe) !== true) throw new Error("chuoi voi moi tai khoan hop le phai thanh cong (true)");
if (soDu(kho2.get(1)!) !== 900) throw new Error("chuoi hop le PHAI thuc su ap dung -- tai khoan 1 phai giam con 900");
if (soDu(kho2.get(2)!) !== 50) throw new Error("tai khoan 2 phai la 50 (nhan 100, gui di 50)");
if (soDu(kho2.get(3)!) !== 50) throw new Error("tai khoan 3 phai nhan dung 50");

if (apDungChuoiLienKet(kho2, []) !== true) throw new Error("chuoi rong phai duoc coi la thanh cong (khong co gi de kiem tra sai)");
```

:::hints
- kind: attention
  body: "Neu debitAccountId hoac creditAccountId KHONG co trong cacTaiKhoan thi tu choi ca chuoi -- mot dong."
- kind: strategy
  body: "if (!cacTaiKhoan.has(bt.debitAccountId) || !cacTaiKhoan.has(bt.creditAccountId)) { return false; }"
- kind: one-line
  body: "if (!cacTaiKhoan.has(bt.debitAccountId) || !cacTaiKhoan.has(bt.creditAccountId)) { return false; }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chuỗi ĐÃ atomicity — nhưng "tài khoản KHÔNG tồn tại" chỉ LÀ MỘT loại
lỗi. Loại quan TRỌNG hơn: số dư KHÔNG đủ để chi.
::::

::::reflect{#nghi-lai}
`apDungChuoiLienKet` chứng minh MỘT nguyên lý chung CỦA atomicity,
không CHỈ riêng ledger: kiểm tra TOÀN bộ TRƯỚC, thay đổi TRẠNG thái
SAU — không BAO giờ đảo ngược thứ TỰ đó. Nếu áp dụng NGAY khi kiểm
tra XONG từng bút toán MỘT, một lỗi Ở giữa chuỗi sẽ để LẠI hệ thống Ở
một trạng THÁI "nửa vời" — đúng chuyện double-entry (bài 1-2) được
sinh RA để ngăn chặn.
::::

::::checkpoint{mastery=0.85}
::::
