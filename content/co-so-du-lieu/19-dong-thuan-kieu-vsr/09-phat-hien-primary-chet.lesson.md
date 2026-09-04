---
id: co-so-du-lieu.dong-thuan-kieu-vsr.phat-hien-primary-chet
title: "Phát hiện primary chết"
summary: "phatHienVaNghiNgoPrimary(replica,dh,thoiDiemNgheCuoi,nguongTimeoutMs) so đo dh.hienTai-thoiDiemNgheCuoi với ngưỡng bằng '<=' (đúng ngưỡng LÀ CHƯA nghi ngờ, khác kiemTraCanGuiLai ở bài 8 vốn dùng '>=') -- vượt QUA mới nghi ngờ: chuyển status='view-change', viewNumber+=1, trả true. t=200 (đúng ngưỡng 200): false, status vẫn 'normal'. t=201: true, status='view-change', viewNumber 0->1."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [db.phat-hien-primary-chet]
requires: [db.tiem-loi-mang-va-gui-lai]
concepts: [db.phat-hien-primary-chet]
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
Mất MỘT gói (bài trước) hệ thống VẪN tiến. Nhưng NẾU primary chết
HẲN — không CÒN Prepare hay Commit NÀO nữa — backup phải TỰ nhận ra
điều ĐÓ, không CÓ ai báo.
::::

::::explain{#nghi-ngo-primary}
`phatHienVaNghiNgoPrimary(replica, dh, thoiDiemNgheCuoi, nguongTimeoutMs)`
so ĐO `dh.hienTai - thoiDiemNgheCuoi` VỚI ngưỡng — nhưng dùng `<=`
(ĐÚNG ngưỡng LÀ CHƯA nghi ngờ, khác `kiemTraCanGuiLai` Ở bài 8 dùng
`>=`): phải VƯỢT QUA hẳn mới nghi ngờ. NẾU vượt: chuyển
`status='view-change'`, tăng `viewNumber`, trả `true`:

```typescript title=readonly
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

function phatHienVaNghiNgoPrimary(
  replica: Replica, dh: DongHoAo, thoiDiemNgheCuoi: number, nguongTimeoutMs: number,
): boolean {
  if (dh.hienTai - thoiDiemNgheCuoi <= nguongTimeoutMs) return false;
  replica.status = 'view-change';
  replica.viewNumber += 1;
  return true;
}

const backup1 = taoReplica(1, 3);
const dh = taoDongHoAo();
tienToi(dh, 200);
console.log("t=200 (dung nguong):", phatHienVaNghiNgoPrimary(backup1, dh, 0, 200), backup1.status);
tienToi(dh, 1);
console.log("t=201:", phatHienVaNghiNgoPrimary(backup1, dh, 0, 200), backup1.status, backup1.viewNumber);
```

```text title=readonly
t=200 (dung nguong): false normal
t=201: true view-change 1
```

`t=200` CHƯA nghi ngờ (`<=`, đúng NGƯỠNG là AN toàn) — nhưng `t=201`
(vượt QUA) kích hoạt: `status` chuyển `'view-change'`, `viewNumber`
tăng từ `0` LÊN `1`. Đây LÀ tín hiệu ĐẦU tiên của "cuộc bầu cử mới".
::::

::::example{#vi-sao-khac-dau-bai-8}
`kiemTraCanGuiLai` (bài 8) dùng `>=` (ĐÚNG ngưỡng ĐÃ đủ để gửi LẠI —
càng SỚM càng tốt, retry KHÔNG tốn kém gì lớn). `phatHienVaNghiNgoPrimary`
dùng `<=` (ĐÚNG ngưỡng VẪN còn AN toàn — phải VƯỢT hẳn mới hành động)
vì hậu QUẢ của nghi ngờ NHẦM (khởi động MỘT view change không cần
thiết, LÀM gián đoạn hệ thống ĐANG hoạt động BÌNH thường) nặng NỀ
hơn nhiều so VỚI gửi lại một Prepare THỪA.
::::

::::predict{#doan-nghi-ngo-hai-lan commitOnce}
`backup1` ĐÃ nghi ngờ MỘT lần Ở TRÊN (`viewNumber=1`, `status='view-
change'`). Gọi LẠI `phatHienVaNghiNgoPrimary(backup1, dh, 201, 200)`
NGAY (KHÔNG tiến thêm `dh`, ĐỘ lệch `= dh.hienTai(201) - 201 = 0`).
Kết quả LÀ gì?
:::opt{correct}
`false` — độ LỆCH `0 <= 200`, CHƯA vượt ngưỡng; `viewNumber` GIỮ
nguyên `1`, KHÔNG tăng thêm — hàm chỉ nghi ngờ THÊM khi thời gian
THẬT sự trôi qua ĐỦ lâu tính TỪ mốc `thoiDiemNgheCuoi` mới
:::
:::opt
`true` — đã nghi ngờ MỘT lần rồi thì lần GỌI tiếp theo càng CHẮC
chắn primary đã chết, viewNumber tăng LÊN `2`
::why
Trực giác NÀY nhầm "đã nghi ngờ TRƯỚC đó" với "điều KIỆN thời gian
hiện tại". Hàm KHÔNG hề nhớ "đã từng trả `true`" — nó chỉ TÍNH lại
`dh.hienTai - thoiDiemNgheCuoi` MỖI lần gọi, HOÀN toàn dựa VÀO hai
tham số truyền VÀO lúc đó.

Chỗ lệch: tham số `thoiDiemNgheCuoi=201` (mốc MỚI, giả định "vừa
nghe" NGAY sau khi nghi ngờ LẦN đầu) khiến độ lệch RESET về `0`. Muốn
nghi ngờ THÊM lần nữa (view TIẾP theo), phải để `dh` tiến thêm ÍT
nhất `nguongTimeoutMs` KỂ TỪ mốc `thoiDiemNgheCuoi` MỚI này.
::
:::
::::

::::code{#viet_phat_hien_primary}
Hoàn thiện `phatHienVaNghiNgoPrimary` — nếu vượt ngưỡng, chuyển
`status`, tăng `viewNumber`, trả `true`.

```typescript title=starter
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

function phatHienVaNghiNgoPrimary(
  replica: Replica, dh: DongHoAo, thoiDiemNgheCuoi: number, nguongTimeoutMs: number,
): boolean {
  if (dh.hienTai - thoiDiemNgheCuoi <= nguongTimeoutMs) return false;
  ___
}

const backup1 = taoReplica(1, 3);
const dh = taoDongHoAo();
tienToi(dh, 250);
console.log(phatHienVaNghiNgoPrimary(backup1, dh, 0, 200));
```

```typescript title=solution
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}
function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}
interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

function phatHienVaNghiNgoPrimary(
  replica: Replica, dh: DongHoAo, thoiDiemNgheCuoi: number, nguongTimeoutMs: number,
): boolean {
  if (dh.hienTai - thoiDiemNgheCuoi <= nguongTimeoutMs) return false;
  replica.status = 'view-change';
  replica.viewNumber += 1;
  return true;
}

const backup1 = taoReplica(1, 3);
const dh = taoDongHoAo();
tienToi(dh, 250);
console.log(phatHienVaNghiNgoPrimary(backup1, dh, 0, 200));
```

```typescript title=test
function layStatus(r: Replica): Replica['status'] { return r.status; }
function layViewNumber(r: Replica): number { return r.viewNumber; }

const b2 = taoReplica(2, 3);
const dh2 = taoDongHoAo();
tienToi(dh2, 200);
const kq1 = phatHienVaNghiNgoPrimary(b2, dh2, 0, 200);
if (kq1 !== false) throw new Error("t=200 == nguong=200 -- CHUA vuot qua, phai la false (dieu kien la <=, khong phai <)");
if (layStatus(b2) !== 'normal') throw new Error("chua nghi ngo thi status phai VAN la normal");
if (layViewNumber(b2) !== 0) throw new Error("chua nghi ngo thi viewNumber KHONG duoc tang");
tienToi(dh2, 1); // t=201
const kq2 = phatHienVaNghiNgoPrimary(b2, dh2, 0, 200);
if (kq2 !== true) throw new Error("t=201 > nguong=200 -- da vuot qua, phai la true");
if (layStatus(b2) !== 'view-change') throw new Error("nghi ngo roi thi status phai chuyen thanh 'view-change'");
if (layViewNumber(b2) !== 1) throw new Error("nghi ngo roi thi viewNumber phai TANG THEM 1 (tu 0 len 1)");
const dh3 = taoDongHoAo();
tienToi(dh3, 300);
const kq3 = phatHienVaNghiNgoPrimary(b2, dh3, 201, 200);
if (kq3 !== false) throw new Error("t=300, ngheCuoi=201, do lech=99 <= 200 -- CHUA vuot nguong, phai la false");
if (layViewNumber(b2) !== 1) throw new Error("chua nghi ngo THEM thi viewNumber KHONG duoc tang tiep");
```

:::hints
- kind: attention
  body: "Chuyen status='view-change', tang viewNumber them 1, tra ve true -- ba dong."
- kind: strategy
  body: "replica.status = 'view-change'; replica.viewNumber += 1; return true;"
- kind: one-line
  body: "replica.status = 'view-change'; replica.viewNumber += 1; return true;"
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
Một backup vừa nghi ngờ. Nhưng MỘT mình nó nghi ngờ chưa đủ để đổi
primary — nó phải RỦ những replica khác cùng đồng Ý.
::::

::::reflect{#nghi-lai}
`phatHienVaNghiNgoPrimary` LÀ cánh cửa BƯỚC vào "view change"
(MASTERPLAN §9.2) — dựa hoàn TOÀN trên `DongHoAo` (KHÔNG `Date.now()`),
tái sử dụng đúng Ý tưởng "đo độ LỆCH thời gian ẢO" của bài 8 nhưng
đảo NGƯỢC hướng của phép so SÁNH, vì hậu quả của HAI quyết định khác
nhau hoàn TOÀN. Từ đây, `status` VÀ `viewNumber` (bài 2) chính THỨC
bắt đầu chuyển ĐỘNG.
::::

::::checkpoint{mastery=0.85}
::::
