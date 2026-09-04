---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.mot-giao-dich-la-hai-but-toan
title: "Một giao dịch là hai bút toán"
summary: "apDungButToan ghi MỘT ButToan vào HAI tài khoản cùng lúc — tăng debitsPosted của tài khoản ghi nợ, VÀ tăng creditsPosted của tài khoản ghi có, CÙNG amount. Ngay cả 'nạp tiền lần đầu' cũng LÀ một bút toán — từ một tài khoản đặc biệt (nguồn bên ngoài, id=0) — KHÔNG có 'tiền xuất hiện từ hư không'. Nạp 1000 cho tài khoản 1, 500 cho tài khoản 2, rồi chuyển 200 từ 1 sang 2: soDu cuối cùng là 800 và 700."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.mot-giao-dich-la-hai-but-toan]
requires: [db.danh-doi-bao-nhieu-bucket-la-du]
concepts: [db.mot-giao-dich-la-hai-but-toan]
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
R6-2 xây xong hệ phân tán — ghi, đọc, dọn rác. R6-3 hỏi một câu KHÁC
hẳn: dữ liệu ĐÓ có đúng không, THEO đúng nghĩa "tiền không tự sinh ra
hay biến mất"? Bắt đầu Ở nơi TigerBeetle (CSDL sổ cái thật) bắt đầu.
::::

::::explain{#mot-but-toan-hai-thay-doi}
`apDungButToan` ghi MỘT `ButToan` (transfer) VÀO hai tài khoản CÙNG
lúc — KHÔNG phải "trừ tài khoản NÀY, cộng tài khoản KIA" hai bước
riêng, mà LÀ MỘT thao tác NGUYÊN tử: tăng `debitsPosted` của tài
khoản ghi NỢ, VÀ tăng `creditsPosted` của tài khoản ghi CÓ, CÙNG một
`amount`. NGAY cả "nạp tiền LẦN đầu" cũng LÀ một bút toán — từ một
tài khoản ĐẶC biệt đại diện tiền TỪ bên ngoài hệ thống (`NGUON_NGOAI`,
`id=0`) — KHÔNG hề có "tiền xuất hiện từ HƯ không":

```typescript title=readonly
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0 };
}

function soDu(tk: TaiKhoan): number {
  return tk.creditsPosted - tk.debitsPosted;
}

function apDungButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
cacTaiKhoan.set(NGUON_NGOAI, taoTaiKhoan(NGUON_NGOAI));
cacTaiKhoan.set(1, taoTaiKhoan(1));
cacTaiKhoan.set(2, taoTaiKhoan(2));

apDungButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
apDungButToan(cacTaiKhoan, { id: 101, debitAccountId: NGUON_NGOAI, creditAccountId: 2, amount: 500 });
apDungButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });

console.log("tai khoan 1 -- soDu:", soDu(cacTaiKhoan.get(1)!));
console.log("tai khoan 2 -- soDu:", soDu(cacTaiKhoan.get(2)!));
```

```text title=readonly
tai khoan 1 -- soDu: 800
tai khoan 2 -- soDu: 700
```

Tài khoản `1` NHẬN `1000` từ `NGUON_NGOAI` (bút toán `100`), RỒI làm
ghi NỢ (`debitAccountId`) cho bút toán chuyển `200` sang tài khoản
`2` — `soDu` giảm CÒN `800` (`1000 - 200`). Tài khoản `2` nhận `500`
từ `NGUON_NGOAI` (bút toán `101`), RỒI làm ghi CÓ cho bút toán `200`
đó — `soDu` tăng THÀNH `700` (`500 + 200`). MỖI thay đổi số dư, KỂ CẢ
"nạp tiền LẦN đầu", đều LÀ một bút toán CÓ đủ hai vế.
::::

::::example{#khong-co-chuyen-mot-chieu}
Quy ước Ở ĐÂY: `debitsPosted` tăng LÀM `soDu` GIẢM (tiền RỜI khỏi tài
khoản), `creditsPosted` tăng LÀM `soDu` TĂNG (tiền VÀO tài khoản) —
ĐÚNG cảm giác trực quan của một "ví tiền" (không PHẢI quy ước kế toán
DUY nhất — tài khoản LOẠI khác CÓ thể đảo ngược — nhưng quest NÀY giữ
NHẤT quán một quy ước xuyên suốt). Điểm mấu CHỐT không nằm Ở dấu
CỘNG/trừ, mà LÀ: KHÔNG hề tồn tại một API "chuyển tiền một chiều" —
MỌI `ButToan`, KỂ CẢ bút toán "nạp tiền", LUÔN mang CẢ `debitAccountId`
LẪN `creditAccountId`.
::::

::::predict{#doan-tai-khoan-thu-ba commitOnce}
Thêm tài khoản `3` (`taoTaiKhoan(3)`, chưa nhận GÌ). Gọi
`apDungButToan` VỚI `{debitAccountId: 2, creditAccountId: 3, amount:
700}` (chuyển TOÀN bộ số dư tài khoản `2` sang tài khoản `3`). `soDu`
tài khoản `2` sau đó LÀ bao nhiêu?
:::opt{correct}
`0` — `debitsPosted` của tài khoản `2` tăng THÊM `700`
(`creditsPosted` VẪN `700` từ TRƯỚC), `soDu = 700 - 700 = 0`
:::
:::opt
Âm — chuyển ĐÚNG toàn bộ số dư PHẢI bị từ chối, VÌ không CÒN gì để
"dự phòng" SAU giao dịch
::why
Trực giác NÀY đúng Ở một số hệ THỐNG có yêu cầu số dư TỐI thiểu — Ở
`apDungButToan` bài NÀY thì CHƯA hề có kiểm tra như VẬY (bài học VỀ
từ chối số dư ÂM còn Ở phía TRƯỚC, bài 10).

Chỗ lệch: `apDungButToan` (đúng NHƯ code Ở trên) chỉ CỘNG dồn
`debitsPosted`/`creditsPosted` — KHÔNG hề kiểm tra kết quả `soDu` sau
CÙNG. Chuyển đúng `700` (bằng CHÍNH số dư hiện có) cho `soDu` VỀ đúng
`0`, KHÔNG âm, KHÔNG lỗi — một tài khoản `soDu=0` VẪN hợp lệ.
::
:::
::::

::::code{#viet_ap_dung_but_toan}
Hoàn thiện `apDungButToan` — tăng `debitsPosted` của tài khoản ghi
NỢ, VÀ tăng `creditsPosted` của tài khoản ghi CÓ, CÙNG `amount`.

```typescript title=starter
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0 };
}

function soDu(tk: TaiKhoan): number {
  return tk.creditsPosted - tk.debitsPosted;
}

function apDungButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  ___
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
cacTaiKhoan.set(NGUON_NGOAI, taoTaiKhoan(NGUON_NGOAI));
cacTaiKhoan.set(1, taoTaiKhoan(1));
cacTaiKhoan.set(2, taoTaiKhoan(2));
apDungButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
apDungButToan(cacTaiKhoan, { id: 101, debitAccountId: NGUON_NGOAI, creditAccountId: 2, amount: 500 });
apDungButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });
console.log(soDu(cacTaiKhoan.get(1)!));
```

```typescript title=solution
interface TaiKhoan { id: number; debitsPosted: number; creditsPosted: number; }
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function taoTaiKhoan(id: number): TaiKhoan {
  return { id, debitsPosted: 0, creditsPosted: 0 };
}

function soDu(tk: TaiKhoan): number {
  return tk.creditsPosted - tk.debitsPosted;
}

function apDungButToan(cacTaiKhoan: Map<number, TaiKhoan>, bt: ButToan): void {
  const debit = cacTaiKhoan.get(bt.debitAccountId)!;
  const credit = cacTaiKhoan.get(bt.creditAccountId)!;
  debit.debitsPosted += bt.amount;
  credit.creditsPosted += bt.amount;
}

const NGUON_NGOAI = 0;
const cacTaiKhoan = new Map<number, TaiKhoan>();
cacTaiKhoan.set(NGUON_NGOAI, taoTaiKhoan(NGUON_NGOAI));
cacTaiKhoan.set(1, taoTaiKhoan(1));
cacTaiKhoan.set(2, taoTaiKhoan(2));
apDungButToan(cacTaiKhoan, { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 });
apDungButToan(cacTaiKhoan, { id: 101, debitAccountId: NGUON_NGOAI, creditAccountId: 2, amount: 500 });
apDungButToan(cacTaiKhoan, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });
console.log(soDu(cacTaiKhoan.get(1)!));
```

```typescript title=test
const NN2 = 0;
const kho2 = new Map<number, TaiKhoan>();
kho2.set(NN2, taoTaiKhoan(NN2));
kho2.set(1, taoTaiKhoan(1));
kho2.set(2, taoTaiKhoan(2));
apDungButToan(kho2, { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000 });
apDungButToan(kho2, { id: 101, debitAccountId: NN2, creditAccountId: 2, amount: 500 });
apDungButToan(kho2, { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 });

if (soDu(kho2.get(1)!) !== 800) throw new Error("tai khoan 1 phai con 800 sau khi nap 1000 roi chuyen di 200");
if (soDu(kho2.get(2)!) !== 700) throw new Error("tai khoan 2 phai thanh 700 sau khi nap 500 roi nhan them 200");
if (kho2.get(1)!.debitsPosted !== 200) throw new Error("debitsPosted cua tai khoan 1 phai la 200 (chi but toan chuyen di, KHONG tinh but toan nap)");
if (kho2.get(1)!.creditsPosted !== 1000) throw new Error("creditsPosted cua tai khoan 1 phai giu nguyen 1000 (tu but toan nap), KHONG bi dong cham boi but toan chuyen di");
if (kho2.get(2)!.creditsPosted !== 700) throw new Error("creditsPosted cua tai khoan 2 phai la 700 (500 nap + 200 nhan them)");
if (kho2.get(2)!.debitsPosted !== 0) throw new Error("tai khoan 2 chua tung la ben ghi no -- debitsPosted phai la 0");
if (soDu(kho2.get(NN2)!) !== -1500) throw new Error("nguon ben ngoai da 'phat ra' 1500 (1000+500) -- soDu phai la -1500");

apDungButToan(kho2, { id: 2, debitAccountId: 2, creditAccountId: 1, amount: 700 });
if (soDu(kho2.get(2)!) !== 0) throw new Error("chuyen dung toan bo so du (700) phai lam tai khoan 2 con dung 0, khong am khong loi");
if (soDu(kho2.get(1)!) !== 1500) throw new Error("tai khoan 1 nhan lai 700 phai thanh 800+700=1500");
```

:::hints
- kind: attention
  body: "Tang debitsPosted cua debit VA creditsPosted cua credit, cung amount -- hai dong."
- kind: strategy
  body: "debit.debitsPosted += bt.amount; credit.creditsPosted += bt.amount;"
- kind: one-line
  body: "debit.debitsPosted += bt.amount; credit.creditsPosted += bt.amount;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "800"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một bút toán, hai thay đổi CÙNG lúc — nhưng LÀM sao biết CẢ hệ thống
(không CHỈ hai tài khoản NÀY, và cả `NGUON_NGOAI`) chưa hề "làm rơi"
hay "sinh ra" tiền?
::::

::::reflect{#nghi-lai}
`apDungButToan` KHÔNG kiểm tra GÌ cả — nó CHỈ ghi. Nhưng CÁCH quest
NÀY thiết kế "nạp tiền" (qua `NGUON_NGOAI`, KHÔNG qua một tham số
"số dư mở đầu" tuỳ tiện) đã âm THẦM tạo RA một tính chất mạnh: MỌI
đồng tiền trong hệ thống ĐỀU có thể truy VỀ đúng một bút toán — không
CÓ "tiền xuất hiện từ hư không". Bước tiếp theo: CHỨNG minh tính chất
NÀY bằng SỐ, trên toàn bộ hệ thống, không chỉ đoán.
::::

::::checkpoint{mastery=0.8}
::::
