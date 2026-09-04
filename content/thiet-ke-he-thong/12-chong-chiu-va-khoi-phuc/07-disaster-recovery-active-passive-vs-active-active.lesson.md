---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.disaster-recovery-active-passive-vs-active-active
title: "Disaster recovery: active-passive vs active-active, multi-region failover"
summary: "CauHinhChienLuoc{loai,thoiGianPhatHienMs,thoiGianChuyenDoiMs,chiPhiMotRegionThangUsd} mo ta hai chien luoc da-region. tinhRtoMs: active_active tra ve 0 NGAY (ca hai region deu dang xu ly traffic, mat mot region chi giam CAPACITY chu khong mat DICH VU, khong can failover); active_passive CONG thoiGianPhatHienMs + thoiGianChuyenDoiMs (60000+300000=360000ms=6 phut, phai PHAT HIEN roi CHUYEN DOI thu cong/tu dong sang region phu). tinhChiPhiThangUsd: active_active TON GAP DOI (ca hai region chay full cong suat, x2), active_passive CHI 1.5x (region phu chay o cong suat GIAM, du sung de nhan traffic bat cu luc nao)."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.disaster-recovery-active-passive-vs-active-active]
requires: [sd.backup-restore-rpo-rto-va-xac-thuc]
concepts: [sd.disaster-recovery-active-passive-vs-active-active]
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
Backup/restore (bài trước) khôi phục dữ liệu SAU khi hạ tầng đã đứng
vững TRỞ lại. Nhưng nếu CẢ một khu vực hạ tầng (region) — máy chủ,
mạng, trung tâm dữ liệu — sập HOÀN toàn CÙNG một lúc, không CÓ gì Ở đó
để khôi phục LÊN cả. Câu hỏi trở thành: hệ thống có một khu vực KHÁC sẵn
sàng tiếp QUẢN không, VÀ tiếp quản NHANH tới đâu?
::::

::::explain{#hai-chien-luoc}
`CauHinhChienLuoc` mô tả hai cách bố trí NHIỀU region. Active-passive: MỘT
region CHÍNH xử lý mọi traffic, region PHỤ đứng chờ — cần PHÁT hiện sự cố
RỒI chuyển đổi mới có dịch vụ TRỞ lại. Active-active: CẢ hai region đều
đang xử LÝ traffic — mất một region CHỈ LÀM giảm CAPACITY, dịch vụ KHÔNG
hề gián đoạn, nên `tinhRtoMs` trả về `0` ngay:

```typescript title=readonly
type LoaiChienLuoc = "active_passive" | "active_active";
interface CauHinhChienLuoc {
  loai: LoaiChienLuoc;
  thoiGianPhatHienMs: number;
  thoiGianChuyenDoiMs: number;
  chiPhiMotRegionThangUsd: number;
}
function tinhRtoMs(c: CauHinhChienLuoc): number {
  if (c.loai === "active_active") return 0;
  return c.thoiGianPhatHienMs + c.thoiGianChuyenDoiMs;
}

const activePassive: CauHinhChienLuoc = {
  loai: "active_passive", thoiGianPhatHienMs: 60000, thoiGianChuyenDoiMs: 300000, chiPhiMotRegionThangUsd: 4000,
};
const activeActive: CauHinhChienLuoc = {
  loai: "active_active", thoiGianPhatHienMs: 60000, thoiGianChuyenDoiMs: 300000, chiPhiMotRegionThangUsd: 4000,
};

console.log("active-passive RTO (ms):", tinhRtoMs(activePassive));
console.log("active-active RTO (ms):", tinhRtoMs(activeActive));
console.log("active-passive RTO (phut):", tinhRtoMs(activePassive) / 60000);
```

```text title=readonly
active-passive RTO (ms): 360000
active-active RTO (ms): 0
active-passive RTO (phut): 6
```

CÙNG bộ tham số THỜI gian (`60000`ms phát hiện, `300000`ms chuyển đổi) —
nhưng `active_passive` CỘNG cả hai LẠI thành `360000`ms (`6` phút KHÔNG có
dịch vụ), trong khi `active_active` bỏ QUA hoàn toàn hai tham số ĐÓ, trả
về `0`. Đây KHÔNG phải LÀ một phép TÍNH khác — nó LÀ một MÔ hình khác:
active-active giả định KHÔNG hề CÓ khoảnh khắc "chuyển đổi" nào cả, vì
region còn LẠI vốn đã đang phục vụ SẴN từ trước.
::::

::::example{#danh-doi-chi-phi}
RTO thấp hơn không MIỄN phí — `tinhChiPhiThangUsd` cho thấy đánh đổi:
active-active chạy CẢ hai region Ở CÔNG suất đầy đủ (tốn GẤP đôi một
region), active-passive chỉ chạy region phụ Ở công suất GIẢM (đủ để tiếp
nhận khi cần, nhưng rẻ hơn):

```typescript title=readonly
type LoaiChienLuoc = "active_passive" | "active_active";
interface CauHinhChienLuoc {
  loai: LoaiChienLuoc;
  thoiGianPhatHienMs: number;
  thoiGianChuyenDoiMs: number;
  chiPhiMotRegionThangUsd: number;
}
function tinhRtoMs(c: CauHinhChienLuoc): number {
  if (c.loai === "active_active") return 0;
  return c.thoiGianPhatHienMs + c.thoiGianChuyenDoiMs;
}
function tinhChiPhiThangUsd(c: CauHinhChienLuoc): number {
  if (c.loai === "active_active") return c.chiPhiMotRegionThangUsd * 2;
  return c.chiPhiMotRegionThangUsd * 1.5;
}

const activePassive: CauHinhChienLuoc = {
  loai: "active_passive", thoiGianPhatHienMs: 60000, thoiGianChuyenDoiMs: 300000, chiPhiMotRegionThangUsd: 4000,
};
const activeActive: CauHinhChienLuoc = {
  loai: "active_active", thoiGianPhatHienMs: 60000, thoiGianChuyenDoiMs: 300000, chiPhiMotRegionThangUsd: 4000,
};

console.log("active-passive -- RTO:", tinhRtoMs(activePassive), "ms, chi phi/thang:", tinhChiPhiThangUsd(activePassive), "USD");
console.log("active-active   -- RTO:", tinhRtoMs(activeActive), "ms, chi phi/thang:", tinhChiPhiThangUsd(activeActive), "USD");
```

```text title=readonly
active-passive -- RTO: 360000 ms, chi phi/thang: 6000 USD
active-active   -- RTO: 0 ms, chi phi/thang: 8000 USD
```

Chuyển TỪ active-passive sang active-active LÀM RTO giảm từ `6` phút
xuống `0` — nhưng chi phí hạ TẦNG tăng từ `6000` lên `8000` USD/tháng
(GẤP đôi giá MỘT region, thay vì `1,5` lần). KHÔNG có chiến lược nào
"đúng" tuyệt đối — CHỈ có câu hỏi: hệ thống NÀY có đáng để trả THÊM tiền
mỗi tháng, đổi lấy việc KHÔNG hề có một khoảnh khắc gián đoạn nào khi mất
một region?
::::

::::predict{#doan-active-active-bo-qua-tham-so commitOnce}
Một `CauHinhChienLuoc` có `loai="active_active"`, nhưng
`thoiGianPhatHienMs=999999` VÀ `thoiGianChuyenDoiMs=999999` (RẤT lớn).
`tinhRtoMs` trả về bao nhiêu?

:::opt{correct}
`0` — nhánh `if (c.loai === "active_active") return 0;` chạy VÀ trả về
NGAY LẬP TỨC, hai trường thời gian KHÔNG hề được đọc tới, DÙ giá trị của
chúng lớn cỡ nào
:::
:::opt
`1999998` (tổng hai trường) — dù LÀ active-active, hệ thống VẪN cần một
khoảng THỜI gian nào đó để phát hiện VÀ phản ứng, nên hai tham số ĐÓ vẫn
phải được cộng VÀO
::why
Nhầm "mọi chiến lược ĐỀU cần thời gian phản ứng" VỚI cách `tinhRtoMs`
THẬT sự phân nhánh — hàm coi active-active LÀ một MÔ hình không có khái
niệm "chuyển đổi" chút nào, không phải LÀ "chuyển đổi cực nhanh".

Chỗ lệch: dòng ĐẦU tiên trong `tinhRtoMs` LÀ `if (c.loai ===
"active_active") return 0;` — một `return` SỚM, xảy ra TRƯỚC khi bất kỳ
dòng nào khác được thực thi. Hai trường `thoiGianPhatHienMs` VÀ
`thoiGianChuyenDoiMs` hoàn toàn KHÔNG được đọc trong nhánh NÀY — chúng
chỉ CÓ Ý nghĩa cho `active_passive`, nơi CÓ một khoảnh khắc chuyển đổi
THẬT sự cần đo.
::
:::
::::

::::code{#viet_tinh_rto_ms}
Hoàn thiện `tinhRtoMs` — nếu chiến lược LÀ `"active_active"`, trả về `0`
ngay; ngược lại (`"active_passive"`), trả về TỔNG `thoiGianPhatHienMs` VÀ
`thoiGianChuyenDoiMs`.

```typescript title=starter
type LoaiChienLuoc = "active_passive" | "active_active";
interface CauHinhChienLuoc {
  loai: LoaiChienLuoc;
  thoiGianPhatHienMs: number;
  thoiGianChuyenDoiMs: number;
  chiPhiMotRegionThangUsd: number;
}

function tinhRtoMs(c: CauHinhChienLuoc): number {
  ___
}

const cauHinhX: CauHinhChienLuoc = { loai: "active_passive", thoiGianPhatHienMs: 10000, thoiGianChuyenDoiMs: 20000, chiPhiMotRegionThangUsd: 1000 };
console.log(tinhRtoMs(cauHinhX));
```

```typescript title=solution
type LoaiChienLuoc = "active_passive" | "active_active";
interface CauHinhChienLuoc {
  loai: LoaiChienLuoc;
  thoiGianPhatHienMs: number;
  thoiGianChuyenDoiMs: number;
  chiPhiMotRegionThangUsd: number;
}

function tinhRtoMs(c: CauHinhChienLuoc): number {
  if (c.loai === "active_active") return 0;
  return c.thoiGianPhatHienMs + c.thoiGianChuyenDoiMs;
}

const cauHinhX: CauHinhChienLuoc = { loai: "active_passive", thoiGianPhatHienMs: 10000, thoiGianChuyenDoiMs: 20000, chiPhiMotRegionThangUsd: 1000 };
console.log(tinhRtoMs(cauHinhX));
```

```typescript title=test
const passiveT: CauHinhChienLuoc = { loai: "active_passive", thoiGianPhatHienMs: 60000, thoiGianChuyenDoiMs: 300000, chiPhiMotRegionThangUsd: 4000 };
const rtoPassiveT = tinhRtoMs(passiveT);
if (rtoPassiveT !== 360000) throw new Error("active-passive RTO phai la TONG thoiGianPhatHienMs + thoiGianChuyenDoiMs = 360000");

const activeT: CauHinhChienLuoc = { loai: "active_active", thoiGianPhatHienMs: 60000, thoiGianChuyenDoiMs: 300000, chiPhiMotRegionThangUsd: 4000 };
const rtoActiveT = tinhRtoMs(activeT);
if (rtoActiveT !== 0) throw new Error("active-active RTO phai la 0, khong can failover -- mat mot region chi giam capacity");

const activeThoiGianLonT: CauHinhChienLuoc = { loai: "active_active", thoiGianPhatHienMs: 999999, thoiGianChuyenDoiMs: 999999, chiPhiMotRegionThangUsd: 4000 };
const rtoActiveLonT = tinhRtoMs(activeThoiGianLonT);
if (rtoActiveLonT !== 0) throw new Error("active-active RTO phai la 0 DU thoiGianPhatHienMs/thoiGianChuyenDoiMs lon bao nhieu -- hai truong nay khong duoc doc toi");

const passiveNhanhT: CauHinhChienLuoc = { loai: "active_passive", thoiGianPhatHienMs: 5000, thoiGianChuyenDoiMs: 10000, chiPhiMotRegionThangUsd: 2000 };
const rtoPassiveNhanhT = tinhRtoMs(passiveNhanhT);
if (rtoPassiveNhanhT !== 15000) throw new Error("active-passive RTO phai cong dung hai truong thoi gian, ra 15000");
```

:::hints
- kind: attention
  body: "Neu c.loai === 'active_active' thi return 0 NGAY. Nguoc lai (active_passive), return c.thoiGianPhatHienMs + c.thoiGianChuyenDoiMs."
- kind: strategy
  body: "if (c.loai === 'active_active') return 0; return c.thoiGianPhatHienMs + c.thoiGianChuyenDoiMs;"
- kind: one-line
  body: "if (c.loai === 'active_active') return 0; return c.thoiGianPhatHienMs + c.thoiGianChuyenDoiMs;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "30000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai chiến lược, hai con số RTO, hai mức chi phí — một đánh đổi RÕ ràng,
đo được, không còn LÀ cảm giác "nên dự phòng cẩn thận hơn". Bảy bài đã
xong: mạch ngắt, retry, bulkhead, dự phòng, tự kìm tốc độ, backup/restore,
VÀ đa khu vực. Giờ LÀ lúc ráp BỐN mảnh đầu tiên thành MỘT pipeline DUY
nhất — VÀ khép lại toàn bộ Realm 7.
::::

::::reflect{#nghi-lai}
`tinhRtoMs` VÀ `tinhChiPhiThangUsd` không hề nói chiến lược NÀO "tốt hơn"
— chúng chỉ dịch một LỰA chọn kiến trúc THÀNH hai con số CÓ thể so sánh
được: bao nhiêu THỜI gian gián đoạn, VÀ bao nhiêu tiền mỗi tháng. Active-
active không hề "thông minh" hơn VỀ mặt thuật toán — nó chỉ đổi tiền LẤY
một mô hình ĐƠN giản hơn (không CÓ khái niệm "chuyển đổi" chút nào). Đây
LÀ khuôn mẫu lặp lại xuyên suốt CẢ track T7.4: mỗi cơ chế chống chịu
(canary, rollback, circuit breaker, đa khu vực) đều LÀ một điểm cân bằng
giữa RỦI ro chấp nhận được VÀ chi phí sẵn SÀNG trả, không phải MỘT đáp án
đúng duy nhất.
::::

::::checkpoint{mastery=0.85}
::::
