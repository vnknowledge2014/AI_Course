---
id: co-so-du-lieu.so-cai-khong-sai-mot-xu.so-du-la-tong-dua-tru
title: "Số dư là tổng đủ trừ"
summary: "tinhSoDuTuLichSu tính số dư MỘT tài khoản KHÔNG dựa vào debitsPosted/creditsPosted lưu sẵn (bài 1-2), mà quét TOÀN BỘ lịch sử bút toán: mỗi lần tài khoản là bên ghi nợ thì TRỪ amount, bên ghi có thì CỘNG. Trên cùng 5 bút toán của bài 2, tài khoản 1 có số dư 850 — khớp CHÍNH XÁC với 1050-200 tính từ debitsPosted/creditsPosted đã lưu — hai cách tính ĐỘC LẬP cho cùng một kết quả."
locale: vi
track: co-so-du-lieu
module: so-cai-khong-sai-mot-xu
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.so-du-la-tong-dua-tru]
requires: [db.giai-ma-nhieu-but-toan-lien-tiep]
concepts: [db.so-du-la-tong-dua-tru]
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
Bài 1-2 cộng dồn `debitsPosted`/`creditsPosted` NGAY khi mỗi bút toán
xảy ra. Nhưng NẾU chỉ có lịch sử bút toán THÔ (như bài 5 đọc RA), số
dư TÍNH lại từ ĐẦU thế nào?
::::

::::explain{#quet-lich-su}
`tinhSoDuTuLichSu` KHÔNG dựa VÀO trường lưu sẵn nào — nó quét TOÀN bộ
`dsButToan`: MỖI lần tài khoản LÀ bên ghi NỢ thì TRỪ `amount`, bên
ghi CÓ thì CỘNG:

```typescript title=readonly
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function tinhSoDuTuLichSu(dsButToan: ButToan[], taiKhoanId: number): number {
  let soDu = 0;
  for (const bt of dsButToan) {
    if (bt.debitAccountId === taiKhoanId) soDu -= bt.amount;
    if (bt.creditAccountId === taiKhoanId) soDu += bt.amount;
  }
  return soDu;
}

const NGUON_NGOAI = 0;
const dsButToan: ButToan[] = [
  { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 },
  { id: 101, debitAccountId: NGUON_NGOAI, creditAccountId: 2, amount: 500 },
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 },
  { id: 2, debitAccountId: 2, creditAccountId: 3, amount: 150 },
  { id: 3, debitAccountId: 3, creditAccountId: 1, amount: 50 },
];

console.log("so du tai khoan 1:", tinhSoDuTuLichSu(dsButToan, 1));
console.log("so du tai khoan 2:", tinhSoDuTuLichSu(dsButToan, 2));
console.log("so du tai khoan 3:", tinhSoDuTuLichSu(dsButToan, 3));
```

```text title=readonly
so du tai khoan 1: 850
so du tai khoan 2: 550
so du tai khoan 3: 100
```

ĐÚNG `5` bút toán Ở bài `2` (`heThongCanBang`) — Ở ĐÓ, tài khoản `1`
CÓ `creditsPosted=1050` (`1000` bút toán `100` + `50` bút toán `3`)
VÀ `debitsPosted=200` (bút toán `1`), `soDu = 1050 - 200 = 850`.
`tinhSoDuTuLichSu` tính LẠI TỪ ĐẦU bằng cách quét TOÀN bộ lịch sử —
VÀ cho ra ĐÚNG `850`, khớp TUYỆT đối. Hai cách tính HOÀN toàn độc
lập, cùng MỘT kết quả.
::::

::::example{#hai-cach-tinh-doc-lap}
`debitsPosted`/`creditsPosted` (bài 1-2) LÀ một CACHE — được cập
nhật DẦN theo TỪNG bút toán, tra cứu NGAY (`O(1)`). `tinhSoDuTuLichSu`
LÀ tính LẠI từ NGUỒN (lịch sử bút toán), chi phí `O(n)` VỚI `n` LÀ số
bút toán — chậm HƠN, nhưng KHÔNG hề phụ thuộc bất kỳ trường lưu sẵn
nào CÓ bị hỏng hay không. Đây LÀ một dạng khác của bất biến Ở bài 2:
nếu hai cách tính LỆCH nhau, có LỖI Ở đâu đó — VÀ "tính lại TỪ nguồn"
luôn LÀ trọng tài cuối CÙNG.
::::

::::predict{#doan-tai-khoan-chua-tung-thay commitOnce}
Gọi `tinhSoDuTuLichSu(dsButToan, 999)` (tài khoản `999` CHƯA từng
xuất hiện Ở bất kỳ bút toán NÀO trong `dsButToan`). Kết quả trả VỀ LÀ
gì?
:::opt{correct}
`0` — vòng LẶP không hề tìm thấy bút toán NÀO CÓ `debitAccountId`
HOẶC `creditAccountId` bằng `999`, `soDu` giữ NGUYÊN giá trị khởi tạo
`0`
:::
:::opt
Lỗi runtime — hàm giả ĐỊNH tài khoản truyền vào PHẢI tồn tại TRONG
lịch sử
::why
Không CÓ dòng CODE nào trong `tinhSoDuTuLichSu` giả định tài khoản
PHẢI xuất hiện — hàm chỉ SO sánh `===`, không tra cứu `Map`/mảng
theo chỉ số nào CÓ thể ném lỗi.

Chỗ lệch: vòng `for` chỉ đơn giản KHÔNG bao giờ khớp điều kiện
`debitAccountId === 999` hay `creditAccountId === 999` VỚI bất kỳ
`bt` NÀO — `soDu` không hề bị THAY đổi, kết thúc vòng lặp VẪN LÀ `0`.
Một tài khoản "chưa TỪNG giao dịch" hợp LỆ có số dư `0`, không phải
lỗi.
::
:::
::::

::::code{#viet_tinh_so_du_tu_lich_su}
Hoàn thiện `tinhSoDuTuLichSu` — trừ `amount` khi tài khoản LÀ bên ghi
nợ, cộng `amount` khi LÀ bên ghi có.

```typescript title=starter
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function tinhSoDuTuLichSu(dsButToan: ButToan[], taiKhoanId: number): number {
  let soDu = 0;
  for (const bt of dsButToan) {
    ___
  }
  return soDu;
}

const NGUON_NGOAI = 0;
const dsButToan: ButToan[] = [
  { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 },
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 },
];
console.log(tinhSoDuTuLichSu(dsButToan, 1));
```

```typescript title=solution
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }

function tinhSoDuTuLichSu(dsButToan: ButToan[], taiKhoanId: number): number {
  let soDu = 0;
  for (const bt of dsButToan) {
    if (bt.debitAccountId === taiKhoanId) soDu -= bt.amount;
    if (bt.creditAccountId === taiKhoanId) soDu += bt.amount;
  }
  return soDu;
}

const NGUON_NGOAI = 0;
const dsButToan: ButToan[] = [
  { id: 100, debitAccountId: NGUON_NGOAI, creditAccountId: 1, amount: 1000 },
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 },
];
console.log(tinhSoDuTuLichSu(dsButToan, 1));
```

```typescript title=test
const NN2 = 0;
const ds2: ButToan[] = [
  { id: 100, debitAccountId: NN2, creditAccountId: 1, amount: 1000 },
  { id: 101, debitAccountId: NN2, creditAccountId: 2, amount: 500 },
  { id: 1, debitAccountId: 1, creditAccountId: 2, amount: 200 },
  { id: 2, debitAccountId: 2, creditAccountId: 3, amount: 150 },
  { id: 3, debitAccountId: 3, creditAccountId: 1, amount: 50 },
];

if (tinhSoDuTuLichSu(ds2, 1) !== 850) throw new Error("tai khoan 1 phai co so du 850");
if (tinhSoDuTuLichSu(ds2, 2) !== 550) throw new Error("tai khoan 2 phai co so du 550");
if (tinhSoDuTuLichSu(ds2, 3) !== 100) throw new Error("tai khoan 3 phai co so du 100");
if (tinhSoDuTuLichSu(ds2, NN2) !== -1500) throw new Error("nguon ngoai phai co so du -1500");
if (tinhSoDuTuLichSu(ds2, 999) !== 0) throw new Error("tai khoan chua tung xuat hien phai co so du 0, khong loi");
if (tinhSoDuTuLichSu([], 1) !== 0) throw new Error("lich su rong phai cho so du 0");

const tongTatCa = tinhSoDuTuLichSu(ds2, 1) + tinhSoDuTuLichSu(ds2, 2) + tinhSoDuTuLichSu(ds2, 3) + tinhSoDuTuLichSu(ds2, NN2);
if (tongTatCa !== 0) throw new Error("tong so du CA HE THONG (moi tai khoan cong lai) phai bang dung 0 -- tien khong sinh khong mat");
```

:::hints
- kind: attention
  body: "Neu la ben ghi no thi TRU amount, neu la ben ghi co thi CONG amount -- hai dong if."
- kind: strategy
  body: "if (bt.debitAccountId === taiKhoanId) soDu -= bt.amount; if (bt.creditAccountId === taiKhoanId) soDu += bt.amount;"
- kind: one-line
  body: "if (bt.debitAccountId === taiKhoanId) soDu -= bt.amount; if (bt.creditAccountId === taiKhoanId) soDu += bt.amount;"
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
Số dư tính được, TỪ hai cách độc lập, LUÔN khớp — nhưng "chuyển tiền
NGAY LẬP TỨC" không phải LÚC nào cũng đúng nghiệp vụ. Đôi lúc CẦN
"giữ chỗ TRƯỚC, xác nhận SAU".
::::

::::reflect{#nghi-lai}
`tinhSoDuTuLichSu` VÀ `debitsPosted`/`creditsPosted` (bài 1-2) LÀ hai
con ĐƯỜNG tính ra CÙNG một sự thật — MỘT nhanh (cache CẬP nhật dần),
MỘT chậm nhưng KHÔNG phụ thuộc bất cứ gì ngoài LỊCH sử THÔ. TigerBeetle
THẬT dùng cache (giống bài 1-2) VÌ hiệu năng, nhưng NGUYÊN tắc "tính
lại được TỪ lịch sử" LÀ điều LÀM double-entry đáng TIN — sổ cái không
chỉ LÀ một con SỐ, mà LÀ một chuỗi bằng CHỨNG dẫn tới con số ĐÓ.
::::

::::checkpoint{mastery=0.85}
::::
