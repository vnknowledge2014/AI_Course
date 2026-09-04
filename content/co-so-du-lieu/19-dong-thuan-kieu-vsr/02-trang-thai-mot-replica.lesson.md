---
id: co-so-du-lieu.dong-thuan-kieu-vsr.trang-thai-mot-replica
title: "Trạng thái một replica"
summary: "taoReplica(chiSo,tongSo) khởi tạo MỘT Replica -- viewNumber=0, status='normal', log=[] (mảng LogEntry{opNumber,bt}), opNumber=0, commitNumber=0. Hai replica độc lập (taoReplica(0,3) và taoReplica(1,3)) giữ HAI object HOÀN TOÀN tách biệt -- đổi log của replica0 (push một entry, opNumber=1) KHÔNG hề ảnh hưởng replica1 (log.length vẫn 0). Đây LÀ struct duy nhất mọi bài còn lại của q19 tái sử dụng nguyên xi."
locale: vi
track: co-so-du-lieu
module: dong-thuan-kieu-vsr
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 8
teaches: [db.trang-thai-mot-replica]
requires: [db.vi-sao-mot-ban-sao-khong-du]
concepts: [db.trang-thai-mot-replica]
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
Quorum VÀ primary/backup (bài trước) LÀ những PHÉP toán trên GIẤY.
Nhưng MỖI replica LÀ một MÁY thật, giữ một TRẠNG thái thật trong bộ
nhớ. Trạng thái ĐÓ gồm những GÌ, chính XÁC?
::::

::::explain{#struct-replica}
`taoReplica(chiSo, tongSo)` khởi TẠO một `Replica`: `viewNumber`
(view HIỆN tại, bắt đầu `0`), `status` (`'normal' | 'view-change' |
'recovering'`, bắt đầu `'normal'`), `log` (mảng `LogEntry{opNumber,
bt}`, bắt đầu RỖNG), `opNumber` (SỐ hiệu bút toán MỚI nhất đã GHI cục
bộ, bắt đầu `0`), `commitNumber` (số hiệu bút toán MỚI nhất đã COMMIT,
bắt đầu `0`):

```typescript title=readonly
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number;
  tongSo: number;
  viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[];
  opNumber: number;
  commitNumber: number;
}

function taoReplica(chiSo: number, tongSo: number): Replica {
  return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };
}

const r0 = taoReplica(0, 3);
const r1 = taoReplica(1, 3);
console.log("r0:", JSON.stringify(r0));
r0.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } });
r0.opNumber = 1;
console.log("sau khi doi r0 -- r0.log.length:", r0.log.length, "r1.log.length:", r1.log.length);
```

```text title=readonly
r0: {"chiSo":0,"tongSo":3,"viewNumber":0,"status":"normal","log":[],"opNumber":0,"commitNumber":0}
sau khi doi r0 -- r0.log.length: 1 r1.log.length: 0
```

`r0` VÀ `r1` LÀ hai `object` HOÀN toàn tách biệt trong bộ nhớ — MỖI
lời gọi `taoReplica` tạo một `log: []` MỚI của RIÊNG nó. Đổi `r0` (đẩy
một entry, cập nhật `opNumber`) KHÔNG hề chạm tới `r1`.
::::

::::example{#tai-sao-tach-biet}
Đây LÀ khác biệt CĂN bản với "một máy chủ, nhiều KHÁCH hàng" (mô
hình client-server thông THƯỜNG): trong VSR, MỖI replica LÀ một BẢN
SAO đầy đủ, TỰ giữ log VÀ trạng thái của CHÍNH nó — không CÓ "bộ nhớ
dùng chung". `opNumber !== commitNumber` LÀ chuyện BÌNH thường (ghi
log cục bộ chưa CHẮC đã commit — bài 3 VÀ bài 6 sẽ làm RÕ khác biệt
này); nhưng NGAY từ bài NÀY, cả hai LUÔN bắt đầu bằng `0`.
::::

::::predict{#doan-status-ban-dau commitOnce}
`taoReplica(2, 5)` (replica CHỈ số `2` trong hệ THỐNG `N=5`) — trường
`status` của NÓ ngay sau khi TẠO LÀ giá trị NÀO?
:::opt{correct}
`'normal'` — MỌI replica MỚI tạo (dù `tongSo` LÀ `3` hay `5`) đều bắt
đầu Ở trạng thái BÌNH thường, sẵn SÀNG tham gia đồng thuận; `'view-
change'` chỉ xuất hiện SAU này (bài 9-12) khi replica NGHI ngờ primary
chết
:::
:::opt
Phụ thuộc `chiSo` — replica `chiSo=0` (có thể LÀ primary Ở view 0)
bắt đầu `'normal'`, các replica KHÁC bắt đầu `'recovering'`
::why
Trực giác NÀY đoán CÓ một sự phân BIỆT "primary khởi động khác backup"
— hợp LÝ về mặt Ý nghĩa VSR THẬT (rất phức tạp), nhưng SAI cho ĐÚNG
`taoReplica` Ở bài NÀY.

Chỗ lệch: `taoReplica` LUÔN trả về `status: 'normal'` — dòng CODE
KHÔNG hề có nhánh RẼ theo `chiSo`. Việc "AI LÀ primary" hoàn toàn LÀ
một phép TÍNH riêng (`laPrimary`, bài 1), không PHẢI một phần trạng
thái LƯU trong `Replica`.
::
:::
::::

::::code{#viet_tao_replica}
Hoàn thiện `taoReplica` — trả về ĐÚNG `object` với bảy trường ĐÃ mô
tả Ở trên.

```typescript title=starter
interface ButToan { id: number; debitAccountId: number; creditAccountId: number; amount: number; }
interface LogEntry { opNumber: number; bt: ButToan; }
interface Replica {
  chiSo: number; tongSo: number; viewNumber: number;
  status: 'normal' | 'view-change' | 'recovering';
  log: LogEntry[]; opNumber: number; commitNumber: number;
}

function taoReplica(chiSo: number, tongSo: number): Replica {
  ___
}

console.log(JSON.stringify(taoReplica(0, 3)));
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

console.log(JSON.stringify(taoReplica(0, 3)));
```

```typescript title=test
const r0 = taoReplica(0, 3);
if (r0.chiSo !== 0 || r0.tongSo !== 3) throw new Error("chiSo/tongSo phai dung nhu tham so truyen vao");
if (r0.viewNumber !== 0) throw new Error("viewNumber ban dau phai la 0");
if (r0.status !== 'normal') throw new Error("status ban dau phai la 'normal'");
if (r0.opNumber !== 0) throw new Error("opNumber ban dau phai la 0");
if (r0.commitNumber !== 0) throw new Error("commitNumber ban dau phai la 0");
if (r0.log.length !== 0) throw new Error("log ban dau phai rong");
const r1 = taoReplica(1, 3);
r0.log.push({ opNumber: 1, bt: { id: 1, debitAccountId: 0, creditAccountId: 1, amount: 100 } });
r0.opNumber = 1;
if (r1.log.length !== 0 || r1.opNumber !== 0) throw new Error("moi replica phai co trang thai RIENG (doi r0 khong duoc anh huong r1)");
```

:::hints
- kind: attention
  body: "Tra ve mot object voi dung bay truong: chiSo, tongSo, viewNumber:0, status:'normal', log:[], opNumber:0, commitNumber:0 -- mot dong."
- kind: strategy
  body: "return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };"
- kind: one-line
  body: "return { chiSo, tongSo, viewNumber: 0, status: 'normal', log: [], opNumber: 0, commitNumber: 0 };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "normal"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mỗi replica giờ CÓ một chỗ để GIỮ bút toán. Bước tiếp THEO: một
client THẬT gửi yêu cầu — primary nhận nó Ở ĐÂU trong struct NÀY?
::::

::::reflect{#nghi-lai}
`Replica` LÀ struct DUY nhất mọi bài CÒN lại của q19 tái sử dụng
NGUYÊN xi — `opNumber` (bài 3-5), `commitNumber` (bài 6-7), `status`
VÀ `viewNumber` (bài 9-12), `log` (hầu HẾT mọi bài). Nắm chắc bảy
trường NÀY LÀ nắm chắc "bộ nhớ" của toàn BỘ giao thức VSR sắp học.
::::

::::checkpoint{mastery=0.8}
::::
