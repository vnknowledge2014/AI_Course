---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.id-trung-lap-la-idempotent
title: "id trùng lặp là idempotent"
summary: "guiIdempotent kiểm tra id của bút toán đã từng xử lý (Set dsIdDaXuLy) TRƯỚC khi ghi -- nếu ĐÃ có, trả về true (coi như thành công) mà KHÔNG ghi thêm lần nào nữa. Gửi bút toán id=1 amount=100 hai lần liên tiếp (mô phỏng client gửi lại sau lỗi mạng): soDu chỉ giảm đúng 100 một lần duy nhất, không phải 200 -- lần gửi lại 'thành công' nhưng không có tác dụng phụ nào thêm."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 11
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.id-trung-lap-la-idempotent]
requires: [db.tu-choi-so-du-am]
concepts: [db.id-trung-lap-la-idempotent]
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
Mạng CÓ thể mất gói tin NGAY sau khi server xử lý xong, TRƯỚC khi
client kịp nhận PHẢN hồi. Client, không biết chuyện GÌ đã xảy ra,
GỬI lại. Server nhận ĐÚNG bút toán đó — LẦN thứ hai.
::::

::::explain{#gui-idempotent}
`guiIdempotent` kiểm TRA `id` của bút toán ĐÃ từng xử lý
(`dsIdDaXuLy`) TRƯỚC khi ghi — nếu ĐÃ có, trả VỀ `true` (coi NHƯ
thành công) MÀ KHÔNG ghi thêm LẦN nào nữa:

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiIdempotent(cacTaiKhoan: Map<number, TaiKhoan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  dsIdDaXuLy.add(bt.id);
  apDungButToanThuong(cacTaiKhoan, bt);
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
const dsIdDaXuLy = new Set<number>();

console.log("gui lan 1:", guiIdempotent(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 }));
console.log("sau lan 1 -- soDu tai khoan 1:", soDu(cacTaiKhoan.get(1)!));
console.log("gui LAI (cung id=1, gia lap client retry):", guiIdempotent(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 }));
console.log("sau lan 2 -- soDu tai khoan 1:", soDu(cacTaiKhoan.get(1)!));
```

```text title=readonly
gui lan 1: true
sau lan 1 -- soDu tai khoan 1: 900
gui LAI (cung id=1, gia lap client retry): true
sau lan 2 -- soDu tai khoan 1: 900
```

LẦN gửi ĐẦU (`id=1`, `amount=100`) trừ tài khoản `1` TỪ `1000` xuống
`900`. GỬI LẠI đúng bút toán ĐÓ (CÙNG `id=1`) — `guiIdempotent` phát
hiện `id` ĐÃ có trong `dsIdDaXuLy`, trả VỀ `true` NGAY (giống LẦN
đầu — client KHÔNG cách nào phân biệt "lần ĐẦU thành công" VỚI "lần
LẶP lại") — nhưng `soDu` KHÔNG hề đổi thêm, VẪN LÀ `900`.
::::

::::example{#idempotent-khong-phai-loi-trung-lap}
Đây LÀ điểm khác biệt CĂN bản VỚI post/void HAI lần (bài 8): post
lại MỘT `pendingId` ĐÃ xử lý trả VỀ `false` (THẤT bại RÕ ràng, hệ
thống "biết" đó LÀ lỗi). `guiIdempotent` trả VỀ `true` CHO lần gửi
LẶP — VÌ từ góc nhìn CỦA client gửi lại (retry SAU lỗi mạng), đây
KHÔNG phải một lỗi: đó LÀ hành vi ĐÚNG đắn khi KHÔNG biết chắc yêu
CẦU trước đã tới nơi hay CHƯA. "An toàn khi gọi LẶP lại" (idempotent)
LÀ điều kiện SỐNG còn để client CÓ thể retry MÀ không sợ nhân đôi
tiền.
::::

::::predict{#doan-id-khac-nhau commitOnce}
Gửi HAI bút toán KHÁC `id` (`id=2` VÀ `id=3`), CÙNG `amount=50`,
CÙNG cặp tài khoản. `soDu` tài khoản `1` sau CẢ hai LẦN gửi giảm
BAO nhiêu SO với trước khi gửi?
:::opt{correct}
`100` (`50 + 50`) — hai `id` KHÁC nhau nghĩa LÀ hai bút toán THẬT sự
riêng biệt, `dsIdDaXuLy` KHÔNG hề coi chúng LÀ trùng lặp, CẢ hai đều
được ÁP dụng
:::
:::opt
`50` — hệ thống "gộp" các bút toán CÓ cùng `debitAccountId`/
`creditAccountId`/`amount` LẠI, coi LÀ MỘT giao dịch LẶP
::why
Trực giác NÀY nhầm "idempotent theo `id`" VỚI "gộp theo NỘI dung
giống nhau" — hai khái niệm KHÁC hẳn.

Chỗ lệch: `dsIdDaXuLy.has(bt.id)` CHỈ so sánh `id` — KHÔNG hề nhìn
tới `debitAccountId`/`creditAccountId`/`amount`. Hai bút toán `id=2`
VÀ `id=3`, DÙ giống hệt nhau Ở MỌI trường KHÁC, VẪN LÀ hai `id` phân
BIỆT — cả hai đều CHƯA có trong `dsIdDaXuLy`, cả hai đều được áp
dụng THẬT, tổng trừ ĐÚNG `100`.
::
:::
::::

::::code{#viet_gui_idempotent}
Hoàn thiện `guiIdempotent` — nếu `id` ĐÃ xử lý, trả về `true` NGAY
không ghi gì; NẾU chưa, đánh dấu ĐÃ xử lý rồi ghi THẬT.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiIdempotent(cacTaiKhoan: Map<number, TaiKhoan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  ___
  dsIdDaXuLy.add(bt.id);
  apDungButToanThuong(cacTaiKhoan, bt);
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
const dsIdDaXuLy = new Set<number>();
guiIdempotent(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 });
guiIdempotent(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 });
console.log(soDu(cacTaiKhoan.get(1)!));
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

function guiIdempotent(cacTaiKhoan: Map<number, TaiKhoan>, dsIdDaXuLy: Set<number>, bt: ButToan): boolean {
  if (dsIdDaXuLy.has(bt.id)) return true;
  dsIdDaXuLy.add(bt.id);
  apDungButToanThuong(cacTaiKhoan, bt);
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
const dsIdDaXuLy = new Set<number>();
guiIdempotent(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 });
guiIdempotent(cacTaiKhoan, dsIdDaXuLy, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 });
console.log(soDu(cacTaiKhoan.get(1)!));
```

```typescript title=test
const NN2 = 0;
const kho2 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2]) kho2.set(id, taoTaiKhoan(id));
apDungButToanThuong(kho2, { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000 });
const ds2 = new Set<number>();

if (guiIdempotent(kho2, ds2, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 }) !== true) throw new Error("gui lan dau phai thanh cong (true)");
if (soDu(kho2.get(1)!) !== 900) throw new Error("sau lan gui dau, soDu phai la 900");

if (guiIdempotent(kho2, ds2, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 100 }) !== true) throw new Error("gui LAI cung id phai VAN tra ve true (idempotent, khong phai loi)");
if (soDu(kho2.get(1)!) !== 900) throw new Error("gui lai cung id KHONG duoc doi soDu them lan nao -- van phai la 900, khong phai 800");
if (kho2.get(1)!.debitsPosted !== 100) throw new Error("debitsPosted CHI duoc cong dung 1 lan (100), khong phai 200");

if (guiIdempotent(kho2, ds2, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 50 }) !== true) throw new Error("id KHAC (id=2) phai duoc coi la but toan MOI, ap dung that");
if (soDu(kho2.get(1)!) !== 850) throw new Error("id=2 (50 don vi, id khac id=1) phai TRU THEM, soDu phai giam tu 900 xuong 850");

if (guiIdempotent(kho2, ds2, { id: 3, debitAccountId: 1, creditAccountId: 2, amount: 50 }) !== true) throw new Error("id=3 cung phai duoc ap dung, du cung amount voi id=2");
if (soDu(kho2.get(1)!) !== 800) throw new Error("id=3 phai tru them 50 nua, soDu phai la 800");

if (ds2.size !== 3) throw new Error("dsIdDaXuLy phai co dung 3 id phan biet (1,2,3), khong tinh lan gui lai id=1");
```

:::hints
- kind: attention
  body: "Neu id da co trong dsIdDaXuLy thi tra ve true ngay, khong lam gi them -- mot dong."
- kind: strategy
  body: "if (dsIdDaXuLy.has(bt.id)) return true;"
- kind: one-line
  body: "if (dsIdDaXuLy.has(bt.id)) return true;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "900"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mọi cơ chế đã sẵn sàng: double-entry, pending/post/void, linked, từ
chối số dư âm, idempotent. Ráp TẤT cả thành một hệ sổ cái thật.
::::

::::reflect{#nghi-lai}
`guiIdempotent` LÀ mảnh ghép CUỐI cho một hệ THỐNG "an toàn khi retry"
— kết hợp VỚI `apDungChuoiLienKet` (bài 9, atomicity) VÀ
`guiButToanAnToan` (bài 10, không cho ÂM), một client CÓ thể gửi bút
toán MÀ KHÔNG lo sợ: gửi thiếu (mạng lỗi giữa chừng) thì retry AN
toàn (idempotent); gửi CHUỖI thì hoặc TẤT cả hoặc không GÌ (linked);
gửi vượt QUÁ số dư thì bị chặn NGAY (không âm). Ba đảm bảo NÀY, cộng
double-entry (bài 1-2) VÀ two-phase (bài 7-8), LÀ toàn bộ những gì
TigerBeetle THẬT hứa hẹn.
::::

::::checkpoint{mastery=0.85}
::::

