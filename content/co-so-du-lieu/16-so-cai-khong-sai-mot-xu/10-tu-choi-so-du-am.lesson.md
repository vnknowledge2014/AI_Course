---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.tu-choi-so-du-am
title: "Từ chối số dư âm"
summary: "guiButToanAnToan kiểm tra TRƯỚC khi ghi: nếu soDu tài khoản ghi nợ trừ amount cho kết quả ÂM, từ chối NGAY (không ghi gì) và trả về false. Tài khoản có 500, gửi 600 bị từ chối -- soDu giữ nguyên 500, không rơi xuống -100. Gửi ĐÚNG 500 (bằng chính số dư) vẫn thành công -- soDu về 0, ranh giới là 'âm' chứ không phải 'không còn gì'."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.tu-choi-so-du-am]
requires: [db.linked-tat-ca-cung-thang]
concepts: [db.tu-choi-so-du-am]
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
Mọi hàm TỚI giờ (`apDungButToan`, `guiButToan`...) ghi VÔ điều kiện —
kể cả khi kết quả LÀ số dư ÂM. Bài 1 đã thấy điều NÀY (dự đoán sai
lầm). Sửa NGAY bây giờ.
::::

::::explain{#kiem-tra-truoc-khi-ghi}
`guiButToanAnToan` kiểm TRA TRƯỚC khi ghi: nếu `soDu` tài khoản ghi
NỢ trừ `amount` cho kết quả ÂM, từ CHỐI ngay (KHÔNG ghi gì) VÀ trả VỀ
`false`:

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }
function apDungButToanThuong(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  cacTaiKhoan.get(bt.debitAccountId)!.debitsPosted += bt.amount;
  cacTaiKhoan.get(bt.creditAccountId)!.creditsPosted += bt.amount;
}

function guiButToanAnToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): boolean {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDu(debit) - bt.amount < 0) return false;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToanThuong(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 500 });

console.log("truoc -- soDu tai khoan 1:", soDu(cacTaiKhoan.get(1)!));
console.log("gui 600 (vuot qua 500):", guiButToanAnToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 600 }));
console.log("sau khi tu choi -- soDu tai khoan 1:", soDu(cacTaiKhoan.get(1)!));
```

```text title=readonly
truoc -- soDu tai khoan 1: 500
gui 600 (vuot qua 500): false
sau khi tu choi -- soDu tai khoan 1: 500
```

Tài khoản `1` CÓ `500`. Gửi `600` (vượt SỐ dư) bị TỪ chối NGAY —
`guiButToanAnToan` trả VỀ `false`, VÀ `soDu` GIỮ NGUYÊN `500`, KHÔNG
hề rơi XUỐNG `-100`. So SÁNH với bài 1: `apDungButToan` (bài 1, VẪN
dùng lại xuyên SUỐT bài 2, 6, 9) ghi VÔ điều kiện — `guiButToanAnToan`
LÀ phiên bản "an toàn HƠN" của cùng ý tưởng, thêm đúng MỘT điều kiện.
::::

::::example{#ranh-gioi-la-am-khong-phai-het}
Ranh GIỚI từ chối LÀ "ÂM", KHÔNG phải "KHÔNG còn gì". Gửi ĐÚNG bằng
CHÍNH số dư hiện CÓ (VÍ dụ `500` khi đang CÓ `500`) VẪN thành công —
`soDu(debit) - amount = 500 - 500 = 0`, KHÔNG nhỏ hơn `0`, điều KIỆN
`< 0` KHÔNG kích hoạt. Một tài khoản `soDu=0` SAU giao dịch LÀ hợp
lệ hoàn toàn — y hệt điều Ở bài 1 đã dự đoán ĐÚNG (chuyển đúng toàn
bộ số dư KHÔNG bị coi LÀ lỗi).
::::

::::predict{#doan-gui-am-tu-dau commitOnce}
Tài khoản `2` (chưa TỪNG nhận GÌ, `soDu=0`). Gọi `guiButToanAnToan`
VỚI `{debitAccountId: 2, creditAccountId: 1, amount: 1}` (gửi ĐÚNG
`1` đơn vị, từ MỘT tài khoản ĐANG có `0`). Kết quả trả VỀ LÀ gì?
:::opt{correct}
`false` — `soDu(tài khoản 2) - 1 = 0 - 1 = -1`, NHỎ hơn `0`, bị từ
CHỐI — DÙ chỉ gửi `1` đơn vị DUY nhất, một tài khoản `0` KHÔNG có gì
để gửi
:::
:::opt
`true` — số tiền `1` quá NHỎ, hệ thống có THỂ "cho nợ" một chút VỚI
những khoản gần bằng KHÔNG
::why
Trực giác NÀY tưởng tượng MỘT ngưỡng "cho phép nợ chút ÍT" — nhưng
`guiButToanAnToan` (đúng NHƯ code) không hề CÓ khái niệm ngưỡng
KHOAN dung, CHỈ có đúng MỘT điều kiện `< 0`.

Chỗ lệch: `soDu(debit) - bt.amount` VỚI `soDu=0` VÀ `amount=1` LÀ
`-1` — MỘT số ÂM, dù RẤT nhỏ. Điều kiện `< 0` không PHÂN biệt "âm
nhiều" VỚI "âm MỘT chút" — CẢ hai đều bị từ CHỐI như nhau.
::
:::
::::

::::code{#viet_gui_but_toan_an_toan}
Hoàn thiện `guiButToanAnToan` — nếu `soDu` sau khi trừ `amount` ÂM,
từ chối (`return false`) TRƯỚC khi ghi bất cứ gì.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

function guiButToanAnToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): boolean {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  ___
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
cacTaiKhoan.get(1)!.creditsPosted = 500;
console.log(guiButToanAnToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 600 }));
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function soDu(tk: TaiKhoan): number { return tk.creditsPosted - tk.debitsPosted; }

function guiButToanAnToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): boolean {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  if (soDu(debit) - bt.amount < 0) return false;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
  return true;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
cacTaiKhoan.get(1)!.creditsPosted = 500;
console.log(guiButToanAnToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 600 }));
```

```typescript title=test
const NN2 = 0;
const kho2 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2]) kho2.set(id, taoTaiKhoan(id));
kho2.get(1)!.creditsPosted = 500;

if (guiButToanAnToan(kho2, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 600 }) !== false) throw new Error("gui 600 khi chi co 500 phai bi tu choi (false)");
if (soDu(kho2.get(1)!) !== 500) throw new Error("sau khi tu choi, soDu phai GIU NGUYEN 500, khong duoc am");
if (soDu(kho2.get(2)!) !== 0) throw new Error("tai khoan 2 khong duoc nhan gi tu but toan bi tu choi");

if (guiButToanAnToan(kho2, { id: 2, debitAccountId: 1, creditAccountId: 2, amount: 500 }) !== true) throw new Error("gui DUNG BANG so du (500) phai thanh cong (true)");
if (soDu(kho2.get(1)!) !== 0) throw new Error("sau khi gui dung so du, soDu phai la 0 (khong am, hop le)");
if (soDu(kho2.get(2)!) !== 500) throw new Error("tai khoan 2 phai nhan dung 500");

const khoRong = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2]) khoRong.set(id, taoTaiKhoan(id));
if (guiButToanAnToan(khoRong, { id: 3, debitAccountId: 1, creditAccountId: 2, amount: 1 }) !== false) throw new Error("tai khoan CHUA CO GI (soDu=0, kho moi hoan toan) gui du 1 don vi cung phai bi tu choi");

const kho3 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2]) kho3.set(id, taoTaiKhoan(id));
if (guiButToanAnToan(kho3, { id: 5, debitAccountId: 1, creditAccountId: 2, amount: 0 }) !== true) throw new Error("gui dung 0 don vi phai LUON thanh cong, khong bao gio bi tu choi vi so du am");
```

:::hints
- kind: attention
  body: "Neu soDu(debit) - bt.amount nho hon 0 thi tu choi ngay -- mot dong."
- kind: strategy
  body: "if (soDu(debit) - bt.amount < 0) return false;"
- kind: one-line
  body: "if (soDu(debit) - bt.amount < 0) return false;"
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
Số dư âm bị chặn — nhưng NẾU cùng một bút toán (cùng `id`) được gửi
HAI lần (VÍ dụ do lỗi mạng khiến client GỬI lại), số tiền có bị trừ
HAI lần không?
::::

::::reflect{#nghi-lai}
`guiButToanAnToan` chỉ THÊM đúng MỘT điều kiện VÀO trước `apDungButToan`
(bài 1) — nhưng điều kiện ĐÓ LÀ ranh giới GIỮA một sổ cái ĐÁNG tin VÀ
một sổ cái CHO phép "ảo THUẬT tiền": không tài khoản NÀO được PHÉP nợ
nhiều hơn nó CÓ, TRỪ khi hệ thống CỐ Ý cho phép (VÍ dụ tài khoản tín
dụng — ngoài phạm VI quest này). Kiểm tra TRƯỚC khi ghi, KHÔNG BAO
giờ ghi RỒI kiểm tra SAU — cùng nguyên tắc atomicity ĐÃ thấy Ở bài 9.
::::

::::checkpoint{mastery=0.85}
::::
