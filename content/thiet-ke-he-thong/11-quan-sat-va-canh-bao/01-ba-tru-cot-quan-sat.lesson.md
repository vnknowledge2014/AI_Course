---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.ba-tru-cot-quan-sat
title: "Ba trụ cột observability: logs, metrics, traces — khi nào dùng cái nào"
summary: "phatHienBatThuongMetric(cacDiem, nguongMs) loc diem co doTreMs > nguong -- metric TRA LOI 'co van de khong', mot con so tong hop, khong noi request nao. timLogTheoRequestId(cacLog, requestId) loc CHINH XAC log cua MOT request -- log TRA LOI 'chuyen gi xay ra o su kien nay', chi tiet cu the. tinhThoiGianTungBuoc(cacDoan) tru thoiGianKetThucMs - thoiGianBatDauMs cho tung DoanTrace -- trace TRA LOI 'thoi gian troi qua o dau', tung buoc mot trong CUNG mot request xuyen nhieu dich vu. Ba cau hoi khac nhau, ba cong cu khac nhau, khong thay the nhau duoc."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.ba-tru-cot-quan-sat]
requires: [sd.boss-trien-khai-an-toan]
concepts: [sd.ba-tru-cot-quan-sat]
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
Chín bài trước đã dạy cách ĐƯA một thay đổi vào production an toàn — canary,
rollback, health check. Nhưng sau khi đã launch, hệ thống CHẠY hàng ngày,
hàng giờ. Một request đột nhiên chậm bất thường. Câu hỏi đầu tiên không phải
"triển khai thế nào" nữa — mà LÀ: nhìn vào ĐÂU để biết chuyện gì đang xảy ra?
::::

::::explain{#ba-cong-cu-ba-cau-hoi}
`phatHienBatThuongMetric` lọc những điểm dữ liệu có độ trễ VƯỢT một ngưỡng —
đây LÀ câu hỏi mà METRIC trả lời: "có vấn đề không", bằng một con số tổng
hợp, không biết request NÀO gây ra. `timLogTheoRequestId` lọc CHÍNH XÁC các
dòng log của MỘT request cụ thể — đây LÀ câu hỏi mà LOG trả lời: "chuyện gì
đã xảy ra Ở sự kiện này", chi tiết CỤ THỂ:

```typescript title=readonly
interface DiemMetric { thoiDiem: number; doTreMs: number; }
function phatHienBatThuongMetric(cacDiem: DiemMetric[], nguongMs: number): DiemMetric[] {
  return cacDiem.filter((d) => d.doTreMs > nguongMs);
}

interface BanGhiLog { thoiDiem: number; mucDo: string; requestId: string; thongDiep: string; }
function timLogTheoRequestId(cacLog: BanGhiLog[], requestId: string): BanGhiLog[] {
  return cacLog.filter((l) => l.requestId === requestId);
}

const cacDiem: DiemMetric[] = [
  { thoiDiem: 0, doTreMs: 120 },
  { thoiDiem: 10, doTreMs: 130 },
  { thoiDiem: 20, doTreMs: 800 },
  { thoiDiem: 30, doTreMs: 140 },
];
console.log("metric bat thuong (nguong 500ms):", JSON.stringify(phatHienBatThuongMetric(cacDiem, 500)));

const cacLog: BanGhiLog[] = [
  { thoiDiem: 18, mucDo: "info", requestId: "req-40", thongDiep: "nhan request" },
  { thoiDiem: 19, mucDo: "loi", requestId: "req-42", thongDiep: "ket noi dich vu thanh toan timeout sau 700ms" },
  { thoiDiem: 20, mucDo: "info", requestId: "req-42", thongDiep: "nhan request" },
];
console.log("log cua req-42 (chi tiet mot request cu the):", JSON.stringify(timLogTheoRequestId(cacLog, "req-42")));
console.log("so log tim duoc:", timLogTheoRequestId(cacLog, "req-42").length);
```

```text title=readonly
metric bat thuong (nguong 500ms): [{"thoiDiem":20,"doTreMs":800}]
log cua req-42 (chi tiet mot request cu the): [{"thoiDiem":19,"mucDo":"loi","requestId":"req-42","thongDiep":"ket noi dich vu thanh toan timeout sau 700ms"},{"thoiDiem":20,"mucDo":"info","requestId":"req-42","thongDiep":"nhan request"}]
so log tim duoc: 2
```

`phatHienBatThuongMetric` chỉ trả về ĐÚNG một điểm — `doTreMs=800` — nhưng
điểm đó KHÔNG hề nói request nào gây ra nó, hay TẠI SAO nó chậm. Muốn biết
chi tiết, phải quay sang log: `timLogTheoRequestId` tìm ra CHÍNH XÁC dòng
log giải thích — `"ket noi dich vu thanh toan timeout sau 700ms"` — một câu
mô tả cụ thể mà metric không bao giờ mang theo được.
::::

::::example{#trace-theo-tung-buoc}
Log kể lại CHUYỆN gì đã xảy ra Ở một điểm — nhưng khi một request đi XUYÊN
nhiều dịch vụ, câu hỏi "THỜI GIAN trôi Ở ĐÂU nhiều nhất" cần một góc nhìn
khác hẳn: `tinhThoiGianTungBuoc` tính thời gian trôi qua Ở TỪNG bước của
CÙNG một request, dựa trên thời điểm bắt đầu VÀ kết thúc của mỗi dịch vụ nó
đi qua:

```typescript title=readonly
interface DoanTrace { tenDichVu: string; thoiGianBatDauMs: number; thoiGianKetThucMs: number; }
function tinhThoiGianTungBuoc(cacDoan: DoanTrace[]): { tenDichVu: string; thoiGianMs: number }[] {
  return cacDoan.map((d) => ({ tenDichVu: d.tenDichVu, thoiGianMs: d.thoiGianKetThucMs - d.thoiGianBatDauMs }));
}

// CUNG mot request req-42 tu vi du truoc, di qua ba dich vu
const cacDoan: DoanTrace[] = [
  { tenDichVu: "api-gateway", thoiGianBatDauMs: 0, thoiGianKetThucMs: 800 },
  { tenDichVu: "dich-vu-don-hang", thoiGianBatDauMs: 20, thoiGianKetThucMs: 760 },
  { tenDichVu: "dich-vu-thanh-toan", thoiGianBatDauMs: 50, thoiGianKetThucMs: 730 },
];
console.log("thoi gian tung buoc (tung phan cua CUNG mot request):", JSON.stringify(tinhThoiGianTungBuoc(cacDoan)));
```

```text title=readonly
thoi gian tung buoc (tung phan cua CUNG mot request): [{"tenDichVu":"api-gateway","thoiGianMs":800},{"tenDichVu":"dich-vu-don-hang","thoiGianMs":740},{"tenDichVu":"dich-vu-thanh-toan","thoiGianMs":680}]
```

`dich-vu-thanh-toan` MỘT mình đã chiếm `680ms` trong tổng `800ms` của toàn
bộ request — đây LÀ điều chỉ trace mới cho thấy: KHÔNG phải "có lỗi Ở đâu đó"
(metric), cũng KHÔNG phải "log nói gì Ở một điểm" (log), mà LÀ thời gian
PHÂN BỔ như thế nào giữa các dịch vụ tham gia CÙNG một request. Ba câu hỏi —
"có vấn đề không", "chuyện gì đã xảy ra", "thời gian Ở đâu" — không câu nào
thay thế được câu còn lại.
::::

::::predict{#doan-bien-nguong-metric commitOnce}
Gọi `phatHienBatThuongMetric` với một điểm CÓ `doTreMs` ĐÚNG BẰNG `nguongMs`
(ví dụ `doTreMs=500`, `nguongMs=500`). Điểm ĐÓ có xuất hiện trong kết quả trả
về không?

:::opt{correct}
Không — điều kiện lọc LÀ `d.doTreMs > nguongMs` (nghiêm ngặt); `500 > 500` LÀ
`false`, nên điểm CHẠM đúng ngưỡng KHÔNG được coi LÀ bất thường
:::
:::opt
Có — chạm đúng ngưỡng cảnh báo cũng nên được coi LÀ dấu hiệu bất thường, để
không bỏ sót trường hợp Ở RANH giới
::why
Nhầm "chạm đúng ngưỡng" VỚI "vượt ngưỡng" — nhưng `phatHienBatThuongMetric`
dùng phép so sánh `>` (nghiêm ngặt), KHÔNG phải `>=`.

Chỗ lệch: dòng `cacDiem.filter((d) => d.doTreMs > nguongMs)` chỉ giữ những
điểm THẬT sự lớn hơn ngưỡng. Với `doTreMs=500` VÀ `nguongMs=500`, biểu thức
`500 > 500` cho `false` — điểm bị loại KHỎI kết quả, không phải giữ lại.
::
:::
::::

::::code{#viet_phat_hien_bat_thuong_metric}
Hoàn thiện `phatHienBatThuongMetric` — lọc RA những điểm có `doTreMs` VƯỢT
(nghiêm ngặt, không phải bằng) `nguongMs`.

```typescript title=starter
interface DiemMetric { thoiDiem: number; doTreMs: number; }
function phatHienBatThuongMetric(cacDiem: DiemMetric[], nguongMs: number): DiemMetric[] {
  ___
}

const cacDiemX: DiemMetric[] = [
  { thoiDiem: 0, doTreMs: 100 },
  { thoiDiem: 1, doTreMs: 500 },
  { thoiDiem: 2, doTreMs: 900 },
];
const ketQuaX = phatHienBatThuongMetric(cacDiemX, 500);
console.log(ketQuaX.length, JSON.stringify(ketQuaX));
```

```typescript title=solution
interface DiemMetric { thoiDiem: number; doTreMs: number; }
function phatHienBatThuongMetric(cacDiem: DiemMetric[], nguongMs: number): DiemMetric[] {
  return cacDiem.filter((d) => d.doTreMs > nguongMs);
}

const cacDiemX: DiemMetric[] = [
  { thoiDiem: 0, doTreMs: 100 },
  { thoiDiem: 1, doTreMs: 500 },
  { thoiDiem: 2, doTreMs: 900 },
];
const ketQuaX = phatHienBatThuongMetric(cacDiemX, 500);
console.log(ketQuaX.length, JSON.stringify(ketQuaX));
```

```typescript title=test
const rongT = phatHienBatThuongMetric([], 100);
if (rongT.length !== 0) throw new Error("danh sach rong phai tra ve mang rong");

const khongAiVuotT = phatHienBatThuongMetric(
  [
    { thoiDiem: 0, doTreMs: 50 },
    { thoiDiem: 1, doTreMs: 80 },
  ],
  100
);
if (khongAiVuotT.length !== 0) throw new Error("khong diem nao vuot nguong thi ket qua phai rong");

const coVuotT = phatHienBatThuongMetric(
  [
    { thoiDiem: 0, doTreMs: 50 },
    { thoiDiem: 1, doTreMs: 800 },
    { thoiDiem: 2, doTreMs: 900 },
  ],
  100
);
if (coVuotT.length !== 2) throw new Error("phai co DUNG 2 diem vuot nguong 100");
const diemDauT = coVuotT[0];
if (diemDauT === undefined || diemDauT.doTreMs !== 800) throw new Error("phan tu dau tien phai giu THU TU ban dau, doTreMs=800");

const bienT = phatHienBatThuongMetric([{ thoiDiem: 0, doTreMs: 100 }], 100);
if (bienT.length !== 0) throw new Error("doTreMs DUNG BANG nguong (100 = 100) khong duoc tinh la bat thuong -- dieu kien phai la >, khong phai >=");
```

:::hints
- kind: attention
  body: "Dung filter tren cacDiem, giu lai nhung d co d.doTreMs > nguongMs (so sanh nghiem ngat, khong phai >=)."
- kind: strategy
  body: "return cacDiem.filter((d) => d.doTreMs > nguongMs);"
- kind: one-line
  body: "return cacDiem.filter((d) => d.doTreMs > nguongMs);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1 [{\"thoiDiem\":2,\"doTreMs\":900}]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba công cụ, ba câu hỏi riêng biệt — không còn LÀ ba từ mơ hồ nữa. Nhưng
"metric cho biết CÓ vấn đề" chỉ đúng nếu con số Ở ĐÓ được ghi ĐÚNG cách. Bước
tiếp theo: log — hiện đang LÀ chuỗi text tự do — có thể trở thành thứ TRUY
VẤN được, thay vì chỉ đọc bằng mắt.
::::

::::reflect{#nghi-lai}
`phatHienBatThuongMetric`, `timLogTheoRequestId`, VÀ `tinhThoiGianTungBuoc`
không hề LÀM cùng một việc theo ba cách khác nhau — chúng trả lời ba câu hỏi
KHÁC nhau hoàn toàn, trên ba loại dữ liệu khác nhau. Metric nén hàng nghìn
điểm THÀNH một tín hiệu bất thường; log giữ NGUYÊN chi tiết của từng sự kiện;
trace phân RÃ một request thành các đoạn thời gian xuyên nhiều dịch vụ. Một
hệ thống quan sát tốt không chọn MỘT trong ba — nó dùng đúng công cụ cho
đúng câu hỏi đang được đặt ra.
::::

::::checkpoint{mastery=0.72}
::::
