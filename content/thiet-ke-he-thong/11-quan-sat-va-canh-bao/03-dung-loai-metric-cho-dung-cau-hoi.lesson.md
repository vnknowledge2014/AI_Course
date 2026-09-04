---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.dung-loai-metric-cho-dung-cau-hoi
title: "Đúng loại metric cho đúng câu hỏi: counter, gauge, histogram"
summary: "Counter (tangCounter) chi TANG, dung cho tong-request; Gauge (capNhatGauge) len xuong tu do (delta am hoac duong), dung cho so ket-noi-dang-mo; Histogram (ghiHistogram) giu MOI gia tri rieng le, tinhPercentile(h, phanTram) sap xep roi lay dung vi tri (idx = Math.ceil(phanTram/100 * n) - 1) -- vi du [10,20,20,30,100] cho p50=20, p95=100. Dung Counter de cong don do tre (SAI loai) van ra duoc trung binh (180/5=36) nhung MAT vinh vien kha nang tinh p95, vi tung gia tri rieng khong con duoc giu."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.dung-loai-metric-cho-dung-cau-hoi]
requires: [sd.log-co-cau-truc-truy-van-duoc]
concepts: [sd.dung-loai-metric-cho-dung-cau-hoi]
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
Bài trước lọc log theo field cố định — nhưng log LÀ SỰ kiện rời rạc, không
phải LÀ một con số theo dõi LIÊN tục. "Tổng số request", "số kết nối đang
mở", "độ trễ" đều LÀ con số — nhưng chúng CẦN được lưu theo BA cách hoàn
toàn khác nhau, tuỳ vào câu hỏi muốn trả lời sau NÀY.
::::

::::explain{#counter-va-gauge}
`Counter` chỉ được phép TĂNG — `tangCounter` nếu nhận `soLuong` âm sẽ NÉM
lỗi NGAY, vì "tổng số request đã xử lý" không bao giờ giảm. `Gauge` thì
NGƯỢC lại, lên xuống tự do — `capNhatGauge` chấp nhận cả delta dương LẪN âm,
phù hợp với "số kết nối đang mở" (mở thêm thì tăng, đóng lại thì giảm):

```typescript title=readonly
interface Counter { ten: string; giaTri: number; }
function taoCounter(ten: string): Counter { return { ten, giaTri: 0 }; }
function tangCounter(c: Counter, soLuong: number): void {
  if (soLuong < 0) throw new Error("Counter chi duoc tang, khong duoc giam");
  c.giaTri += soLuong;
}

interface Gauge { ten: string; giaTri: number; }
function taoGauge(ten: string): Gauge { return { ten, giaTri: 0 }; }
function capNhatGauge(g: Gauge, delta: number): void { g.giaTri += delta; }

const counterRequest = taoCounter("tong-request");
tangCounter(counterRequest, 1);
tangCounter(counterRequest, 1);
tangCounter(counterRequest, 1);
console.log("counter tong request sau 3 lan tang:", counterRequest.giaTri);

const gaugeKetNoi = taoGauge("ket-noi-dang-mo");
capNhatGauge(gaugeKetNoi, 5);
capNhatGauge(gaugeKetNoi, -2);
console.log("gauge ket noi dang mo (len roi xuong):", gaugeKetNoi.giaTri);
capNhatGauge(gaugeKetNoi, 10);
console.log("gauge sau khi mo them 10 ket noi:", gaugeKetNoi.giaTri);
```

```text title=readonly
counter tong request sau 3 lan tang: 3
gauge ket noi dang mo (len roi xuong): 3
gauge sau khi mo them 10 ket noi: 13
```

`gaugeKetNoi` đi TỪ `5` xuống `3` rồi lên `13` — hoàn toàn bình thường cho
một Gauge. Một `Counter` không bao giờ có thể tái hiện đường ĐI đó, VÌ bản
chất của nó LÀ chỉ tích luỹ, không bao giờ lùi lại.
::::

::::example{#histogram-va-dung-sai-loai}
`Histogram` giữ LẠI từng giá trị riêng lẻ đã ghi VÀO — nhờ vậy
`tinhPercentile` có thể sắp xếp VÀ tính ra bất kỳ mốc phần trăm nào. Dùng
NHẦM `Counter` để "đo" độ trễ bằng cách cộng dồn từng giá trị VẪN cho ra một
con số — nhưng con số ĐÓ chỉ LÀ tổng, VÀ percentile đã MẤT VĨNH VIỄN, không
có cách nào lấy lại từ chính `Counter` đó:

```typescript title=readonly
interface Histogram { ten: string; cacGiaTri: number[]; }
function taoHistogram(ten: string): Histogram { return { ten, cacGiaTri: [] }; }
function ghiHistogram(h: Histogram, giaTri: number): void { h.cacGiaTri.push(giaTri); }
function tinhPercentile(h: Histogram, phanTram: number): number | undefined {
  if (h.cacGiaTri.length === 0) return undefined;
  const sapXep = [...h.cacGiaTri].sort((a, b) => a - b);
  const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1;
  const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx));
  return sapXep[idxAnToan];
}

interface Counter { ten: string; giaTri: number; }
function taoCounter(ten: string): Counter { return { ten, giaTri: 0 }; }
function tangCounter(c: Counter, soLuong: number): void {
  if (soLuong < 0) throw new Error("Counter chi duoc tang, khong duoc giam");
  c.giaTri += soLuong;
}

const histDoTre = taoHistogram("do-tre-request");
for (const v of [10, 20, 20, 30, 100]) ghiHistogram(histDoTre, v);
console.log("cac gia tri histogram (thu tu ghi vao):", JSON.stringify(histDoTre.cacGiaTri));
console.log("p50:", tinhPercentile(histDoTre, 50));
console.log("p95:", tinhPercentile(histDoTre, 95));

// DUNG SAI: dung Counter de "do" do tre bang cach cong don tung gia tri
const counterSai = taoCounter("tong-do-tre-SAI");
for (const v of [10, 20, 20, 30, 100]) tangCounter(counterSai, v);
console.log("dung SAI counter de cong don do tre, chi con MOT con so:", counterSai.giaTri);
console.log("trung binh tinh duoc tu counter:", counterSai.giaTri / 5);
```

```text title=readonly
cac gia tri histogram (thu tu ghi vao): [10,20,20,30,100]
p50: 20
p95: 100
dung SAI counter de cong don do tre, chi con MOT con so: 180
trung binh tinh duoc tu counter: 36
```

`histDoTre` giữ ĐỦ cả `5` giá trị gốc, nên `tinhPercentile` tính ra `p50=20`
VÀ `p95=100` — hai câu trả lời KHÁC nhau cho hai câu hỏi khác nhau.
`counterSai` chỉ còn ĐÚNG một con số `180` — VẪN tính được trung bình
(`36`), nhưng KHÔNG CÓ CÁCH NÀO suy ngược ra `p95` từ con số ĐÓ, vì `10,
20, 20, 30, 100` đã bị GỘP thành một tổng duy nhất, không thể tách lại.
::::

::::predict{#doan-percentile-bien commitOnce}
Một histogram có ĐÚNG bốn giá trị đã sắp xếp: `[5, 15, 25, 35]`. Gọi
`tinhPercentile` với `phanTram=25` — kết quả LÀ gì?

:::opt{correct}
`5` — `idx = Math.ceil((25/100) * 4) - 1 = Math.ceil(1) - 1 = 1 - 1 = 0`,
tức LÀ phần tử ĐẦU tiên sau khi sắp xếp, không phải phần tử thứ hai
:::
:::opt
`15` — `p25` trên bốn phần tử NÊN rơi đúng VÀO phần tử thứ hai (vị trí
`1`), vì `25%` của `4` phần tử LÀ đúng một phần tử tính từ đầu
::why
Nhầm "25% của 4 phần tử LÀ một phần tử, nên bắt đầu từ chỉ số 1" VỚI công
thức THẬT sự dùng trong `tinhPercentile` — công thức trừ `1` SAU khi lấy
trần (`ceil`), không phải bỏ qua phần tử đầu.

Chỗ lệch: `(25 / 100) * 4 = 1` CHÍNH XÁC, VÀ `Math.ceil(1) = 1`. Dòng tiếp
theo LÀ `idx = ... - 1`, cho `idx = 0` — chỉ số ĐẦU tiên trong mảng đã sắp
xếp, tức giá trị `5`, không phải `15` (vị trí `1`).
::
:::
::::

::::code{#viet_tinh_percentile}
Hoàn thiện `tinhPercentile` — sắp xếp `cacGiaTri` tăng dần, tính chỉ số
bằng `Math.ceil((phanTram / 100) * sapXep.length) - 1`, chặn chỉ số trong
khoảng hợp lệ bằng `Math.max`/`Math.min`, RỒI trả về giá trị Ở đúng vị trí
đó. Trường hợp histogram rỗng đã được xử lý SẴN (trả về `undefined`).

```typescript title=starter
interface Histogram { ten: string; cacGiaTri: number[]; }
function taoHistogram(ten: string): Histogram { return { ten, cacGiaTri: [] }; }
function ghiHistogram(h: Histogram, giaTri: number): void { h.cacGiaTri.push(giaTri); }
function tinhPercentile(h: Histogram, phanTram: number): number | undefined {
  if (h.cacGiaTri.length === 0) return undefined;
  ___
}

const hX = taoHistogram("x");
for (const v of [40, 10, 30, 20]) ghiHistogram(hX, v);
console.log(tinhPercentile(hX, 50), tinhPercentile(hX, 100));
```

```typescript title=solution
interface Histogram { ten: string; cacGiaTri: number[]; }
function taoHistogram(ten: string): Histogram { return { ten, cacGiaTri: [] }; }
function ghiHistogram(h: Histogram, giaTri: number): void { h.cacGiaTri.push(giaTri); }
function tinhPercentile(h: Histogram, phanTram: number): number | undefined {
  if (h.cacGiaTri.length === 0) return undefined;
  const sapXep = [...h.cacGiaTri].sort((a, b) => a - b);
  const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1;
  const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx));
  return sapXep[idxAnToan];
}

const hX = taoHistogram("x");
for (const v of [40, 10, 30, 20]) ghiHistogram(hX, v);
console.log(tinhPercentile(hX, 50), tinhPercentile(hX, 100));
```

```typescript title=test
const rongT = taoHistogram("rong");
if (tinhPercentile(rongT, 50) !== undefined) throw new Error("histogram rong phai tra ve undefined, khong phai 0");

const hMotT = taoHistogram("mot-gia-tri");
ghiHistogram(hMotT, 42);
if (tinhPercentile(hMotT, 1) !== 42) throw new Error("chi mot gia tri thi bat ky percentile nao cung phai ra dung gia tri do");
if (tinhPercentile(hMotT, 99) !== 42) throw new Error("chi mot gia tri thi bat ky percentile nao cung phai ra dung gia tri do");

const hT = taoHistogram("nam-gia-tri");
for (const v of [10, 20, 20, 30, 100]) ghiHistogram(hT, v);
const p50T = tinhPercentile(hT, 50);
if (p50T !== 20) throw new Error("p50 cua [10,20,20,30,100] phai la 20");
const p95T = tinhPercentile(hT, 95);
if (p95T !== 100) throw new Error("p95 cua [10,20,20,30,100] phai la 100");
const soLuongGocT = hT.cacGiaTri.length;
if (soLuongGocT !== 5) throw new Error("tinhPercentile khong duoc lam thay doi so luong gia tri goc trong histogram");

const hQuartileT = taoHistogram("quartile");
for (const v of [5, 15, 25, 35]) ghiHistogram(hQuartileT, v);
const p25T = tinhPercentile(hQuartileT, 25);
if (p25T !== 5) throw new Error("p25 cua [5,15,25,35] phai la 5 (idx = ceil(0.25*4)-1 = 0), khong phai 15");
const p100T = tinhPercentile(hQuartileT, 100);
if (p100T !== 35) throw new Error("p100 phai la gia tri LON NHAT trong histogram");
```

:::hints
- kind: attention
  body: "Ba buoc: sap xep cacGiaTri tang dan vao mot mang moi; tinh idx = Math.ceil((phanTram/100) * sapXep.length) - 1; chan idx trong [0, sapXep.length-1] bang Math.max/Math.min; tra ve sapXep tai vi tri do."
- kind: strategy
  body: "const sapXep = [...h.cacGiaTri].sort((a, b) => a - b); const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1; const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx)); return sapXep[idxAnToan];"
- kind: one-line
  body: "const sapXep = [...h.cacGiaTri].sort((a, b) => a - b); const idx = Math.ceil((phanTram / 100) * sapXep.length) - 1; const idxAnToan = Math.max(0, Math.min(sapXep.length - 1, idx)); return sapXep[idxAnToan];"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "20 40"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba loại metric, ba cách trả lời — chọn ĐÚNG loại giữ được đúng thông tin
cần cho SAU này. Nhưng dù chọn đúng loại, một con số Ở MỘT service không
nói lên toàn bộ câu chuyện, khi request phải đi XUYÊN nhiều service khác
nhau để hoàn thành.
::::

::::reflect{#nghi-lai}
`tangCounter` NÉM lỗi khi gặp số âm, `capNhatGauge` chấp nhận cả hai chiều,
`ghiHistogram` không hề TÓM tắt gì cả — nó GIỮ nguyên. Ba hành vi khác biệt
NÀY không phải LÀ chi tiết cài đặt vụn vặt; chúng LÀ chính sự khác biệt VỀ
câu hỏi mà mỗi loại metric được sinh ra để trả lời. Chọn sai loại không
LÀM code lỗi ngay lập tức — như `counterSai` cho thấy, nó VẪN chạy, VẪN ra
một con số hợp lý — nhưng câu hỏi thật sự cần ("p95 LÀ bao nhiêu") đã trở
thành KHÔNG THỂ trả lời được nữa, mãi mãi.
::::

::::checkpoint{mastery=0.75}
::::
