---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.bulkhead-co-lap-tai-nguyen-theo-nhom
title: "Bulkhead: cô lập tài nguyên giữa các phần hệ thống"
summary: "BoDemTaiNguyen{theoNhom: Map<string,{dangSuDung,toiDa}>} chia han muc RIENG cho tung nhom -- vi du {vip:10, thuong:20}. xinTaiNguyen tang dangSuDung neu con cho TRONG han muc CUA CHINH nhom do; nhom 'thuong' gui 21 yeu cau lien tiep chi 20 duoc cap (dung han muc), yeu cau 21 bi tu choi, nhung 'vip' HOAN TOAN khong bi anh huong -- van xin duoc binh thuong. Doi lap voi MOT pool DUNG CHUNG (vi du toi da 30): mot nhom THAM LAM dung het CA 30 khien nhom con lai KHONG con gi, du no chua he goi mot yeu cau nao."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.bulkhead-co-lap-tai-nguyen-theo-nhom]
requires: [sd.retry-backoff-va-jitter-tranh-thundering-herd]
concepts: [sd.bulkhead-co-lap-tai-nguyen-theo-nhom]
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
Retry (bài trước) giúp MỘT request vượt qua một lỗi tạm thời — nhưng nếu
TẤT cả request, thuộc MỌI khách hàng, đều tranh giành CHUNG một hàng chờ
kết nối tới downstream, thì một khách hàng gửi QUÁ nhiều request lỗi có
thể chiếm HẾT chỗ, khiến khách hàng khác — dù hoàn toàn KHÔNG liên quan —
cũng bị đói tài nguyên theo.
::::

::::explain{#han-muc-rieng-theo-nhom}
`BoDemTaiNguyen` chia tài nguyên (ví dụ số connection tới downstream)
THÀNH các hạn mức RIÊNG theo từng nhóm — như khoang tàu được ngăn bằng
vách kín (bulkhead), một khoang ngập nước không kéo theo khoang khác chìm
CÙNG. `xinTaiNguyen` chỉ kiểm tra hạn mức của ĐÚNG nhóm được yêu cầu:

```typescript title=readonly
interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}
function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  if (trangThai.dangSuDung >= trangThai.toiDa) return false;
  trangThai.dangSuDung += 1;
  return true;
}

const bo = taoBoDemTaiNguyen({ vip: 10, thuong: 20 });
const ketQuaThuong: boolean[] = [];
for (let i = 0; i < 21; i++) ketQuaThuong.push(xinTaiNguyen(bo, "thuong"));
console.log("thuong (han muc 20), 21 yeu cau lien tiep -- so duoc cap:", ketQuaThuong.filter((k) => k).length);
console.log("yeu cau thu 21 (vuot han muc):", ketQuaThuong[20]);
console.log("thuong dang dung:", bo.theoNhom.get("thuong")?.dangSuDung, "/ 20");
console.log("vip HOAN TOAN chua bi dung toi, van con nguyen:", bo.theoNhom.get("vip")?.dangSuDung, "/ 10");
console.log("vip xin duoc binh thuong:", xinTaiNguyen(bo, "vip"));
```

```text title=readonly
thuong (han muc 20), 21 yeu cau lien tiep -- so duoc cap: 20
yeu cau thu 21 (vuot han muc): false
thuong dang dung: 20 / 20
vip HOAN TOAN chua bi dung toi, van con nguyen: 0 / 10
vip xin duoc binh thuong: true
```

`thuong` dùng HẾT sạch hạn mức `20` của chính nó — yêu cầu thứ `21` bị
TỪ chối. Nhưng `vip` không hề hay biết chuyện ĐÓ xảy ra: hạn mức `10` của
nó VẪN nguyên vẹn `0/10` (chưa ai dùng), VÀ xin tài nguyên vẫn thành
công NGAY. Hai nhóm sống trong hai "khoang" hoàn toàn tách biệt.
::::

::::example{#pool-chung-vs-bulkhead}
Điểm khác biệt CHỈ hiện rõ khi so sánh VỚI một pool DÙNG CHUNG: cùng tổng
dung lượng, nhưng KHÔNG chia theo nhóm — một nhóm "tham lam" (hoặc đang
gặp SỰ cố, retry liên tục) có thể chiếm HẾT phần của nhóm khác, dù nhóm
kia CHƯA từng gửi một yêu cầu nào:

```typescript title=readonly
interface BoDemChung { dangSuDung: number; toiDa: number; }
function taoBoDemChung(toiDa: number): BoDemChung { return { dangSuDung: 0, toiDa }; }
function xinTaiNguyenChung(bo: BoDemChung): boolean {
  if (bo.dangSuDung >= bo.toiDa) return false;
  bo.dangSuDung += 1;
  return true;
}

interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}
function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  if (trangThai.dangSuDung >= trangThai.toiDa) return false;
  trangThai.dangSuDung += 1;
  return true;
}
function traTaiNguyen(bo: BoDemTaiNguyen, nhom: string): void {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return;
  trangThai.dangSuDung = Math.max(0, trangThai.dangSuDung - 1);
}

// mot nhom THAM LAM (thuong) chiem het POOL CHUNG (30) -- vip khong con gi ca
const boChung = taoBoDemChung(30);
for (let i = 0; i < 30; i++) xinTaiNguyenChung(boChung);
console.log("pool DUNG CHUNG (30): thuong da dung het, vip xin duoc khong:", xinTaiNguyenChung(boChung));

// CUNG tinh huong, nhung bulkhead: moi nhom co han muc RIENG (vip:10, thuong:20 -- CUNG tong 30)
const bo = taoBoDemTaiNguyen({ vip: 10, thuong: 20 });
for (let i = 0; i < 30; i++) xinTaiNguyen(bo, "thuong"); // thuong co GANG dung toi 30, nhung han muc CHI 20
console.log("bulkhead: nhom vip HOAN TOAN khong bi dung toi, van xin duoc:", xinTaiNguyen(bo, "vip"));

traTaiNguyen(bo, "thuong");
console.log("thuong TRA lai 1 don vi (vi du mot request xu ly xong), xin lai duoc khong:", xinTaiNguyen(bo, "thuong"));
```

```text title=readonly
pool DUNG CHUNG (30): thuong da dung het, vip xin duoc khong: false
bulkhead: nhom vip HOAN TOAN khong bi dung toi, van xin duoc: true
thuong TRA lai 1 don vi (vi du mot request xu ly xong), xin lai duoc khong: true
```

CÙNG tổng dung lượng `30` (`10 + 20`) — nhưng pool DÙNG chung khiến `vip`
bị TỪ chối dù nó chưa hề gửi request nào, đơn giản VÌ `thuong` đã dùng hết
`30` PHẦN chung. Bulkhead ngăn đúng chuyện ĐÓ: `vip` xin được NGAY, vì hạn
mức của nó hoàn toàn tách biệt. `traTaiNguyen` (trả lại tài nguyên sau khi
dùng xong) cũng chỉ tác động ĐÚNG nhóm được chỉ định — không hề "trả nhầm"
sang nhóm khác.
::::

::::predict{#doan-nhom-chua-cau-hinh commitOnce}
Gọi `xinTaiNguyen(bo, "nhom-la")` với `"nhom-la"` LÀ một tên nhóm CHƯA
từng xuất hiện trong `cauHinh` lúc `taoBoDemTaiNguyen` được gọi. Kết quả
LÀ gì?

:::opt{correct}
`false` — `bo.theoNhom.get("nhom-la")` trả về `undefined` (nhóm chưa
được đăng ký), VÀ nhánh `if (trangThai === undefined) return false;` xử
LÝ trường hợp đó NGAY, không hề gây lỗi runtime
:::
:::opt
Chương trình NÉM lỗi (throw) — truy cập một nhóm CHƯA từng được cấu hình
LÀ một thao tác không hợp lệ, nên hàm PHẢI báo lỗi thay vì âm thầm trả
về `false`
::why
Nhầm "dữ liệu không tồn tại" VỚI "chương trình PHẢI dừng lại" — nhưng
`xinTaiNguyen` xử LÝ trường hợp `undefined` bằng một `return` bình
thường, không hề có `throw` nào trong toàn bộ hàm.

Chỗ lệch: dòng ĐẦU tiên trong `xinTaiNguyen` LÀ `const trangThai =
bo.theoNhom.get(nhom); if (trangThai === undefined) return false;` —
`Map.get` trên một khoá KHÔNG tồn tại trả về `undefined` (hành vi CHUẨN
của `Map`, không phải lỗi), VÀ hàm kiểm tra ĐIỀU đó tường minh, trả `false`
một cách CÓ chủ đích, coi một nhóm chưa cấu hình LÀ "không có hạn mức nào
để cấp", chứ không phải một trường hợp bất thường cần dừng chương trình.
::
:::
::::

::::code{#viet_xin_tai_nguyen}
Hoàn thiện `xinTaiNguyen` (phần kiểm tra `undefined` đã có sẵn) — nếu
nhóm ĐÃ dùng hết hạn mức (`dangSuDung >= toiDa`) thì từ chối; ngược lại,
tăng `dangSuDung` VÀ cho phép.

```typescript title=starter
interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}

function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  ___
}

const boX = taoBoDemTaiNguyen({ a: 1 });
console.log(xinTaiNguyen(boX, "a"), xinTaiNguyen(boX, "a"));
```

```typescript title=solution
interface TrangThaiNhom { dangSuDung: number; toiDa: number; }
interface BoDemTaiNguyen { theoNhom: Map<string, TrangThaiNhom>; }
function taoBoDemTaiNguyen(cauHinh: Record<string, number>): BoDemTaiNguyen {
  const theoNhom = new Map<string, TrangThaiNhom>();
  for (const [nhom, toiDa] of Object.entries(cauHinh)) theoNhom.set(nhom, { dangSuDung: 0, toiDa });
  return { theoNhom };
}

function xinTaiNguyen(bo: BoDemTaiNguyen, nhom: string): boolean {
  const trangThai = bo.theoNhom.get(nhom);
  if (trangThai === undefined) return false;
  if (trangThai.dangSuDung >= trangThai.toiDa) return false;
  trangThai.dangSuDung += 1;
  return true;
}

const boX = taoBoDemTaiNguyen({ a: 1 });
console.log(xinTaiNguyen(boX, "a"), xinTaiNguyen(boX, "a"));
```

```typescript title=test
function laySoDangDung(bo: BoDemTaiNguyen, nhom: string): number | undefined {
  return bo.theoNhom.get(nhom)?.dangSuDung;
}

const boT = taoBoDemTaiNguyen({ vip: 2, thuong: 3 });

if (xinTaiNguyen(boT, "thuong") !== true) throw new Error("thuong con cho trong (0/3) phai duoc cap");
if (xinTaiNguyen(boT, "thuong") !== true) throw new Error("thuong con cho (1/3) phai duoc cap");
if (xinTaiNguyen(boT, "thuong") !== true) throw new Error("thuong con cho (2/3) phai duoc cap");
if (xinTaiNguyen(boT, "thuong") !== false) throw new Error("thuong da day (3/3) phai bi TU CHOI");
const soDangDungThuongT = laySoDangDung(boT, "thuong");
if (soDangDungThuongT !== 3) throw new Error("tu choi khong duoc lam dangSuDung vuot qua toiDa");

if (xinTaiNguyen(boT, "vip") !== true) throw new Error("vip HOAN TOAN doc lap, khong bi anh huong boi thuong da day");
if (xinTaiNguyen(boT, "vip") !== true) throw new Error("vip (1/2) van con cho");
if (xinTaiNguyen(boT, "vip") !== false) throw new Error("vip da day (2/2) rieng no moi bi tu choi");

if (xinTaiNguyen(boT, "khong-ton-tai") !== false) throw new Error("nhom chua duoc cau hinh phai tu choi, khong throw");
```

:::hints
- kind: attention
  body: "Neu trangThai.dangSuDung >= trangThai.toiDa thi tu choi (return false). Nguoc lai, tang trangThai.dangSuDung len 1 roi return true."
- kind: strategy
  body: "if (trangThai.dangSuDung >= trangThai.toiDa) return false; trangThai.dangSuDung += 1; return true;"
- kind: one-line
  body: "if (trangThai.dangSuDung >= trangThai.toiDa) return false; trangThai.dangSuDung += 1; return true;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một nhóm dùng hết phần CỦA nó không còn kéo nhóm khác đói theo. Nhưng cô
lập tài nguyên chỉ ngăn ĐƯỢC lỗi LAN — nó không hề LÀM downstream trả về
kết quả. Khi downstream THẬT sự không phản hồi được, hệ thống cần một
phương án khác NGOÀI việc chờ hay từ chối.
::::

::::reflect{#nghi-lai}
`xinTaiNguyen` không hề biết gì VỀ "VIP" hay "thường" LÀ gì về mặt nghiệp
vụ — nó chỉ thấy hai KHOÁ khác nhau trong một `Map`, mỗi khoá mang trạng
thái RIÊNG của chính nó. Sức mạnh của bulkhead không nằm Ở một thuật toán
phức tạp — nó nằm Ở việc TỪ CHỐI chia sẻ trạng thái: pool DÙNG chung (bài
so sánh) VÀ bulkhead dùng ĐÚNG một dòng logic kiểm tra hạn mức, khác nhau
DUY nhất Ở chỗ hạn mức ĐÓ được đếm theo TỪNG nhóm hay đếm CHUNG một mối.
::::

::::checkpoint{mastery=0.77}
::::
