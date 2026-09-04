---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.bat-bien-tong-no-bang-tong-co
title: "Bất biến: tổng nợ bằng tổng có"
summary: "heThongCanBang cộng dồn debitsPosted VÀ creditsPosted trên TẤT CẢ tài khoản (kể cả NGUON_NGOAI) — hai tổng LUÔN bằng nhau, sau BAO NHIÊU bút toán cũng vậy: 5 bút toán (2 nạp + 3 chuyển) cho tổng nợ = tổng có = 1900. Đây KHÔNG phải trùng hợp — mỗi apDungButToan cộng CÙNG một amount vào đúng một debitsPosted VÀ một creditsPosted, nên tổng hai cột LUÔN đồng bộ theo TỪNG bước, trừ khi có lỗi ghi KHÔNG đối xứng."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.bat-bien-tong-no-bang-tong-co]
requires: [db.mot-giao-dich-la-hai-but-toan]
concepts: [db.bat-bien-tong-no-bang-tong-co]
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
`NGUON_NGOAI` (bài trước) khiến MỌI đồng tiền truy VỀ được một bút
toán — nhưng đó LÀ suy luận, chưa phải BẰNG chứng. Chứng minh bằng SỐ.
::::

::::explain{#tong-no-bang-tong-co}
`heThongCanBang` cộng dồn `debitsPosted` VÀ `creditsPosted` TRÊN TẤT
CẢ tài khoản (kể CẢ `NGUON_NGOAI`) — HAI tổng đó PHẢI LUÔN bằng nhau,
sau BAO nhiêu bút toán cũng VẬY:

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function apDungButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
}

function tinhTongNo(cacTaiKhoan: Map<number, TaiKhoan>): number {
  let tong = 0;
  for (const [, tk] of cacTaiKhoan) tong += tk.debitsPosted;
  return tong;
}
function tinhTongCo(cacTaiKhoan: Map<number, TaiKhoan>): number {
  let tong = 0;
  for (const [, tk] of cacTaiKhoan) tong += tk.creditsPosted;
  return tong;
}
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  return tinhTongNo(cacTaiKhoan) === tinhTongCo(cacTaiKhoan);
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2, 3]) cacTaiKhoan.set(id, taoTaiKhoan(id));

apDungButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
apDungButToan(cacTaiKhoan, { id: 101, debitAccountId: NGUON_NGOAI, creditAccountId: 2, amount: 500 });
apDungButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });
apDungButToan(cacTaiKhoan, { id: 2, debitAccountId: 2, creditAccountId: 3, amount: 150 });
apDungButToan(cacTaiKhoan, { id: 3, debitAccountId: 3, creditAccountId: 1, amount: 50 });

console.log("tong no:", tinhTongNo(cacTaiKhoan));
console.log("tong co:", tinhTongCo(cacTaiKhoan));
console.log("can bang:", heThongCanBang(cacTaiKhoan));
```

```text title=readonly
tong no: 1900
tong co: 1900
can bang: true
```

`5` bút toán (`2` bút nạp + `3` bút chuyển), tổng SỐ tiền di CHUYỂN
qua các bút toán CỘNG dồn LÀ `1900` (`1000+500+200+150+50`). Tổng
`debitsPosted` TRÊN mọi tài khoản ĐÚNG bằng tổng `creditsPosted` TRÊN
mọi tài khoản — CẢ hai đều `1900`. KHÔNG phải trùng hợp: MỖI
`apDungButToan` cộng CÙNG một `amount` VÀO đúng MỘT `debitsPosted` VÀ
MỘT `creditsPosted` — nên hai tổng LUÔN đồng bộ SAU từng bước, không
chỉ Ở bước CUỐI.
::::

::::example{#bat-bien-phat-hien-loi}
Bất biến NÀY không chỉ LÀ một sự thật TOÁN học đẹp — nó LÀ một CÔNG
cụ phát hiện LỖI thật. Nếu một đoạn mã (VÍ dụ do LỖI lập trình) chỉ
cộng `creditsPosted` MÀ quên cộng `debitsPosted` tương ứng ở phía
BÊN kia, `heThongCanBang` sẽ trả VỀ `false` NGAY — phát hiện sự "mất
cân bằng" MÀ không cần biết CHI tiết lỗi nằm Ở đâu.
::::

::::predict{#doan-nhieu-but-toan-hon commitOnce}
Thêm bút toán THỨ sáu: `{debitAccountId: 1, creditAccountId: 3,
amount: 300}` VÀO CÙNG hệ thống Ở readonly TRÊN (sau cả `5` bút toán
đã có). `heThongCanBang` sau đó CÓ còn LÀ `true` không?
:::opt{correct}
CÓ — bút toán THỨ sáu cộng THÊM đúng `300` VÀO CẢ `tinhTongNo` (qua
tài khoản `1`, ghi nợ) LẪN `tinhTongCo` (qua tài khoản `3`, ghi có) —
hai tổng CÙNG tăng `300`, VẪN bằng nhau (`2200 = 2200`)
:::
:::opt
KHÔNG chắc — càng NHIỀU bút toán, càng dễ tích luỹ SAI số làm hai
tổng LỆCH nhau dần
::why
Trực giác "càng nhiều PHÉP tính càng dễ tích luỹ sai SỐ" đúng cho
SỐ thực dấu chấm động (floating-point) trong một VÀI ngữ cảnh khác —
nhưng KHÔNG áp dụng Ở đây: `amount` LÀ số nguyên, phép `+=` số nguyên
KHÔNG hề tích luỹ sai số.

Chỗ lệch: MỖI `apDungButToan` LUÔN cộng CHÍNH XÁC cùng một `amount`
vào đúng MỘT `debitsPosted` VÀ MỘT `creditsPosted` — bất kể ĐÃ có
bao nhiêu bút toán TRƯỚC đó, bút toán MỚI luôn giữ ĐÚNG tính đối
xứng. Số bút toán CÀNG nhiều KHÔNG hề làm bất biến "mong manh" hơn.
::
:::
::::

::::code{#viet_he_thong_can_bang}
Hoàn thiện `heThongCanBang` — so sánh `tinhTongNo` VÀ `tinhTongCo`.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function apDungButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
}

function tinhTongNo(cacTaiKhoan: Map<number, TaiKhoan>): number {
  let tong = 0;
  for (const [, tk] of cacTaiKhoan) tong += tk.debitsPosted;
  return tong;
}
function tinhTongCo(cacTaiKhoan: Map<number, TaiKhoan>): number {
  let tong = 0;
  for (const [, tk] of cacTaiKhoan) tong += tk.creditsPosted;
  return tong;
}
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  return ___;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
apDungButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });
console.log(heThongCanBang(cacTaiKhoan));
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
function taoTaiKhoan(id: number): TaiKhoan { return { id, debitsPosted: 0, creditsPosted: 0 }; }
function apDungButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
}

function tinhTongNo(cacTaiKhoan: Map<number, TaiKhoan>): number {
  let tong = 0;
  for (const [, tk] of cacTaiKhoan) tong += tk.debitsPosted;
  return tong;
}
function tinhTongCo(cacTaiKhoan: Map<number, TaiKhoan>): number {
  let tong = 0;
  for (const [, tk] of cacTaiKhoan) tong += tk.creditsPosted;
  return tong;
}
function heThongCanBang(cacTaiKhoan: Map<number, TaiKhoan>): boolean {
  return tinhTongNo(cacTaiKhoan) === tinhTongCo(cacTaiKhoan);
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
for (const id of [NGUON_NGOAI, 1, 2]) cacTaiKhoan.set(id, taoTaiKhoan(id));
apDungButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
apDungButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });
console.log(heThongCanBang(cacTaiKhoan));
```

```typescript title=test
const NN2 = 0;
const kho2 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1, 2, 3]) kho2.set(id, taoTaiKhoan(id));
apDungButToan(kho2, { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000 });
apDungButToan(kho2, { id: 101, debitAccountId: NN2, creditAccountId: 2, amount: 500 });
apDungButToan(kho2, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });
apDungButToan(kho2, { id: 2, debitAccountId: 2, creditAccountId: 3, amount: 150 });
apDungButToan(kho2, { id: 3, debitAccountId: 3, creditAccountId: 1, amount: 50 });

if (tinhTongNo(kho2) !== 1900) throw new Error("tong no phai la 1900");
if (tinhTongCo(kho2) !== 1900) throw new Error("tong co phai la 1900");
if (heThongCanBang(kho2) !== true) throw new Error("he thong dung phai bao can bang = true");

const kho3 = new Map<number, TaiKhoan>();
for (const id of [NN2, 1]) kho3.set(id, taoTaiKhoan(id));
kho3.get(1)!.creditsPosted += 999;
if (heThongCanBang(kho3) !== false) throw new Error("gia lap ghi KHONG doi xung (chi creditsPosted, khong debitsPosted o ben kia) phai bi phat hien: can bang = false");

const khoRong = new Map<number, TaiKhoan>();
if (heThongCanBang(khoRong) !== true) throw new Error("he thong rong (0 tai khoan) phai duoc coi la can bang (0 == 0)");
if (tinhTongNo(khoRong) !== 0) throw new Error("tong no cua he thong rong phai la 0");
```

:::hints
- kind: attention
  body: "So sanh tinhTongNo va tinhTongCo bang === -- mot dong."
- kind: strategy
  body: "return tinhTongNo(cacTaiKhoan) === tinhTongCo(cacTaiKhoan);"
- kind: one-line
  body: "return tinhTongNo(cacTaiKhoan) === tinhTongCo(cacTaiKhoan);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bất biến ĐÃ chứng minh được — nhưng TigerBeetle THẬT không lưu bút
toán dưới dạng object linh hoạt NHƯ `ButToan` Ở đây. Nó lưu GÌ, VÀ
tại sao?
::::

::::reflect{#nghi-lai}
`heThongCanBang` LÀ một hàm rất nhỏ, nhưng nó biến "double-entry giữ
tiền không mất" TỪ một khẩu hiệu THÀNH một phép kiểm CÓ thể chạy được
— VÀ chạy được TRÊN bất kỳ trạng thái NÀO của hệ thống, không CHỈ lúc
khởi tạo. Đây LÀ tinh thần "đếm THẬT thay vì tin" đã xuyên suốt CẢ
khoá học — bất biến KHÔNG phải điều ta TIN, mà LÀ điều ta ĐO được.
::::

::::checkpoint{mastery=0.85}
::::
