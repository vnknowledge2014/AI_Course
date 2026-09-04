---
id: thiet-ke-he-thong.quan-sat-va-canh-bao.distributed-tracing-theo-trace-id
title: "Distributed tracing: theo dõi một request qua nhiều service bằng trace ID"
summary: "taoSpanMoi(bo, traceId, parentSpanId, tenDichVu, batDau, ketThuc) tao Span voi spanId tu bo dem don gian (bo.soDem += 1, spanId = 'span-'+soDem) -- MOI span trong CUNG mot request mang CHUNG traceId nhung spanId RIENG, parentSpanId tro ve span goi no (undefined neu la span goc). tinhTongThoiGian lay max(ketThuc) - min(batDau) tren TAT CA span. timDichVuChiemNhieuNhat chi xet cac span CON (parentSpanId != undefined), tim thoiGianMs = ketThuc-batDau LON NHAT -- vi du api-gateway(0-500) goi dich-vu-a(20-460) goi dich-vu-b(60-420): tong=500ms, dich-vu-a chiem NHIEU nhat (440ms), vi no BAO GOM ca thoi gian goi dich-vu-b ben trong."
locale: vi
track: thiet-ke-he-thong
module: quan-sat-va-canh-bao
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.distributed-tracing-theo-trace-id]
requires: [sd.dung-loai-metric-cho-dung-cau-hoi]
concepts: [sd.distributed-tracing-theo-trace-id]
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
Bài 1 tính thời gian từng bước bằng cách trừ hai mốc thời gian — đủ khi chỉ
có MỘT danh sách phẳng. Nhưng request THẬT đi qua nhiều service theo một
CHUỖI gọi lồng nhau: gateway gọi service A, service A lại gọi service B.
Cần một cách nối các mảnh thời gian ĐÓ lại thành ĐÚNG một request duy nhất
— VÀ biết mảnh nào gọi mảnh nào.
::::

::::explain{#span-va-trace-id}
`taoSpanMoi` tạo một `Span` mới — `spanId` sinh từ một bộ đếm ĐƠN giản
(`bo.soDem` tăng dần, ghép thành chuỗi `"span-N"`), KHÔNG cần thuật toán
băm nào. Mọi span trong CÙNG một request mang CHUNG `traceId`, nhưng mỗi
span có `spanId` RIÊNG, VÀ `parentSpanId` trỏ VỀ span đã gọi nó — span gốc
(không ai gọi nó) có `parentSpanId` LÀ `undefined`:

```typescript title=readonly
interface Span {
  traceId: string;
  spanId: string;
  parentSpanId: string | undefined;
  tenDichVu: string;
  thoiGianBatDauMs: number;
  thoiGianKetThucMs: number;
}
interface BoDemSpan { soDem: number; }
function taoBoDemSpan(): BoDemSpan { return { soDem: 0 }; }
function taoSpanMoi(
  bo: BoDemSpan,
  traceId: string,
  parentSpanId: string | undefined,
  tenDichVu: string,
  batDau: number,
  ketThuc: number
): Span {
  bo.soDem += 1;
  return { traceId, spanId: `span-${bo.soDem}`, parentSpanId, tenDichVu, thoiGianBatDauMs: batDau, thoiGianKetThucMs: ketThuc };
}

const bo = taoBoDemSpan();
const spanGateway = taoSpanMoi(bo, "trace-9", undefined, "api-gateway", 0, 500);
console.log("span goc (khong co cha):", JSON.stringify(spanGateway));

const spanA = taoSpanMoi(bo, "trace-9", spanGateway.spanId, "dich-vu-a", 20, 460);
console.log("span con, goi tu api-gateway:", JSON.stringify(spanA));

const spanB = taoSpanMoi(bo, "trace-9", spanA.spanId, "dich-vu-b", 60, 420);
console.log("span chau, goi tu dich-vu-a:", JSON.stringify(spanB));

console.log("ca ba span CUNG mot traceId:", spanGateway.traceId === spanA.traceId && spanA.traceId === spanB.traceId);
```

```text title=readonly
span goc (khong co cha): {"traceId":"trace-9","spanId":"span-1","tenDichVu":"api-gateway","thoiGianBatDauMs":0,"thoiGianKetThucMs":500}
span con, goi tu api-gateway: {"traceId":"trace-9","spanId":"span-2","parentSpanId":"span-1","tenDichVu":"dich-vu-a","thoiGianBatDauMs":20,"thoiGianKetThucMs":460}
span chau, goi tu dich-vu-a: {"traceId":"trace-9","spanId":"span-3","parentSpanId":"span-2","tenDichVu":"dich-vu-b","thoiGianBatDauMs":60,"thoiGianKetThucMs":420}
ca ba span CUNG mot traceId: true
```

`spanGateway.spanId` LÀ `"span-1"`, VÀ vì `bo.soDem` LÀ trạng thái CHIA sẻ,
hai lần gọi TIẾP theo cho ra `"span-2"` VÀ `"span-3"` — không bao giờ trùng
nhau. Lưu Ý: `parentSpanId` của `spanGateway` mang giá trị `undefined`, nên
`JSON.stringify` không hề in field ĐÓ ra — CHỈ những span có cha THẬT sự
mới thấy `"parentSpanId"` xuất hiện trong chuỗi JSON. `spanA.parentSpanId`
ĐÚNG BẰNG `spanGateway.spanId`, VÀ `spanB.parentSpanId` ĐÚNG BẰNG
`spanA.spanId` — một chuỗi liên kết rõ ràng, từ gateway xuống tới lời gọi
sâu nhất.
::::

::::example{#tong-thoi-gian-va-diem-nghen}
`tinhTongThoiGian` lấy mốc SỚM nhất VÀ mốc TRỄ nhất trong TOÀN bộ danh sách
span — không cần biết span nào LÀ gốc. `timDichVuChiemNhieuNhat` chỉ xét
những span CÓ cha (bỏ span gốc, vì span gốc luôn BAO trùm mọi thứ Ở BÊN
trong nó), rồi tìm span có thời gian tồn tại DÀI nhất:

```typescript title=readonly
interface Span {
  traceId: string;
  spanId: string;
  parentSpanId: string | undefined;
  tenDichVu: string;
  thoiGianBatDauMs: number;
  thoiGianKetThucMs: number;
}
function tinhTongThoiGian(cacSpan: Span[]): number {
  const batDauSom = Math.min(...cacSpan.map((s) => s.thoiGianBatDauMs));
  const ketThucMuon = Math.max(...cacSpan.map((s) => s.thoiGianKetThucMs));
  return ketThucMuon - batDauSom;
}
function timDichVuChiemNhieuNhat(cacSpan: Span[]): { tenDichVu: string; thoiGianMs: number } | undefined {
  const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined);
  if (conSpan.length === 0) return undefined;
  let max = conSpan[0]!;
  for (const s of conSpan) {
    if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s;
  }
  return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };
}

// ba span cua CUNG mot request trace-9: api-gateway -> dich-vu-a -> dich-vu-b
const cacSpanTrace9: Span[] = [
  { traceId: "trace-9", spanId: "span-1", parentSpanId: undefined, tenDichVu: "api-gateway", thoiGianBatDauMs: 0, thoiGianKetThucMs: 500 },
  { traceId: "trace-9", spanId: "span-2", parentSpanId: "span-1", tenDichVu: "dich-vu-a", thoiGianBatDauMs: 20, thoiGianKetThucMs: 460 },
  { traceId: "trace-9", spanId: "span-3", parentSpanId: "span-2", tenDichVu: "dich-vu-b", thoiGianBatDauMs: 60, thoiGianKetThucMs: 420 },
];
console.log("tong thoi gian toan bo request:", tinhTongThoiGian(cacSpanTrace9));
console.log("dich vu chiem nhieu thoi gian nhat (trong cac span CON):", JSON.stringify(timDichVuChiemNhieuNhat(cacSpanTrace9)));
```

```text title=readonly
tong thoi gian toan bo request: 500
dich vu chiem nhieu thoi gian nhat (trong cac span CON): {"tenDichVu":"dich-vu-a","thoiGianMs":440}
```

Toàn bộ request mất `500ms`, đúng bằng thời lượng của `api-gateway` — nó
BAO quanh mọi lời gọi bên trong, nên không được tính LÀ "điểm nghẽn" (bị
loại khỏi `conSpan`). Trong các span CÒN lại, `dich-vu-a` chiếm `440ms` —
NHIỀU hơn `dich-vu-b` (`360ms`) — vì thời gian của `dich-vu-a` BAO GỒM cả
lúc nó đợi `dich-vu-b` trả lời. Đây LÀ giới hạn của phép đo ĐƠN giản này:
nó chỉ cho biết span nào TỒN TẠI lâu nhất, không tách được "tự nó chậm" với
"chờ span con của nó".
::::

::::predict{#doan-hoa-thoi-gian commitOnce}
Hai span CON của cùng một request có thời gian tồn tại BẰNG NHAU HỆT: cả
`"dich-vu-x"` VÀ `"dich-vu-y"` đều LÀ `200ms`, VÀ `"dich-vu-x"` đứng TRƯỚC
`"dich-vu-y"` trong mảng truyền vào. `timDichVuChiemNhieuNhat` trả về span
nào?

:::opt{correct}
`"dich-vu-x"` — vòng lặp dùng so sánh `>` (nghiêm ngặt); khi gặp
`"dich-vu-y"` có CÙNG thời gian với `max` hiện tại, `200 > 200` LÀ `false`,
nên `max` KHÔNG bị thay thế, VÀ span ĐẦU tiên gặp được giữ nguyên
:::
:::opt
`"dich-vu-y"` — khi hai span có thời gian bằng nhau, span xuất hiện SAU
trong danh sách nên được ưu tiên, vì dữ liệu mới hơn thường phản ánh đúng
tình trạng hiện tại hơn
::why
Nhầm "phần tử sau trong mảng LÀ dữ liệu mới hơn nên đáng tin hơn" VỚI cách
vòng lặp THẬT sự hoạt động — `timDichVuChiemNhieuNhat` không hề biết gì về
"mới hơn" hay "cũ hơn", nó chỉ so sánh SỐ.

Chỗ lệch: điều kiện cập nhật LÀ `s.thoiGianKetThucMs - s.thoiGianBatDauMs >
max...`, dùng `>`. Khi `"dich-vu-y"` có thời gian ĐÚNG BẰNG `max` (đang LÀ
`"dich-vu-x"`), biểu thức `200 > 200` cho `false` — `max` không được gán
lại. Kết quả cuối vẫn LÀ `"dich-vu-x"`, span ĐẦU tiên đạt mức cao nhất.
::
:::
::::

::::code{#viet_tim_dich_vu_chiem_nhieu_nhat}
Hoàn thiện `timDichVuChiemNhieuNhat` — lọc RA các span CÓ cha (bỏ span
gốc), nếu không còn span nào thì trả về `undefined`; ngược lại, duyệt qua
VÀ giữ lại span có thời gian tồn tại (`thoiGianKetThucMs -
thoiGianBatDauMs`) LỚN NHẤT.

```typescript title=starter
interface Span {
  traceId: string;
  spanId: string;
  parentSpanId: string | undefined;
  tenDichVu: string;
  thoiGianBatDauMs: number;
  thoiGianKetThucMs: number;
}
function timDichVuChiemNhieuNhat(cacSpan: Span[]): { tenDichVu: string; thoiGianMs: number } | undefined {
  ___
}

const spanXX: Span = { traceId: "t", spanId: "s1", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 100 };
const spanYX: Span = { traceId: "t", spanId: "s2", parentSpanId: "s1", tenDichVu: "con-cham", thoiGianBatDauMs: 0, thoiGianKetThucMs: 90 };
console.log(JSON.stringify(timDichVuChiemNhieuNhat([spanXX, spanYX])));
```

```typescript title=solution
interface Span {
  traceId: string;
  spanId: string;
  parentSpanId: string | undefined;
  tenDichVu: string;
  thoiGianBatDauMs: number;
  thoiGianKetThucMs: number;
}
function timDichVuChiemNhieuNhat(cacSpan: Span[]): { tenDichVu: string; thoiGianMs: number } | undefined {
  const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined);
  if (conSpan.length === 0) return undefined;
  let max = conSpan[0]!;
  for (const s of conSpan) {
    if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s;
  }
  return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };
}

const spanXX: Span = { traceId: "t", spanId: "s1", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 100 };
const spanYX: Span = { traceId: "t", spanId: "s2", parentSpanId: "s1", tenDichVu: "con-cham", thoiGianBatDauMs: 0, thoiGianKetThucMs: 90 };
console.log(JSON.stringify(timDichVuChiemNhieuNhat([spanXX, spanYX])));
```

```typescript title=test
const chiCoGocT: Span[] = [{ traceId: "t", spanId: "s1", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 100 }];
if (timDichVuChiemNhieuNhat(chiCoGocT) !== undefined) throw new Error("chi co span goc (khong co con) phai tra ve undefined");

const baSpanT: Span[] = [
  { traceId: "t2", spanId: "r", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 500 },
  { traceId: "t2", spanId: "a", parentSpanId: "r", tenDichVu: "dich-vu-a", thoiGianBatDauMs: 20, thoiGianKetThucMs: 460 },
  { traceId: "t2", spanId: "b", parentSpanId: "a", tenDichVu: "dich-vu-b", thoiGianBatDauMs: 60, thoiGianKetThucMs: 420 },
];
const ketQuaT = timDichVuChiemNhieuNhat(baSpanT);
if (ketQuaT === undefined || ketQuaT.tenDichVu !== "dich-vu-a") throw new Error("dich-vu-a phai la span con chiem nhieu thoi gian nhat (440ms)");
if (ketQuaT.thoiGianMs !== 440) throw new Error("thoiGianMs cua dich-vu-a phai la 440 (460-20)");

const hoaThoiGianT: Span[] = [
  { traceId: "t3", spanId: "r", parentSpanId: undefined, tenDichVu: "root", thoiGianBatDauMs: 0, thoiGianKetThucMs: 1000 },
  { traceId: "t3", spanId: "x", parentSpanId: "r", tenDichVu: "dich-vu-x", thoiGianBatDauMs: 0, thoiGianKetThucMs: 200 },
  { traceId: "t3", spanId: "y", parentSpanId: "r", tenDichVu: "dich-vu-y", thoiGianBatDauMs: 200, thoiGianKetThucMs: 400 },
];
const ketQuaHoaT = timDichVuChiemNhieuNhat(hoaThoiGianT);
if (ketQuaHoaT === undefined || ketQuaHoaT.tenDichVu !== "dich-vu-x") throw new Error("hoa thoi gian (deu 200ms) thi span DAU TIEN gap phai thang, dieu kien la > khong phai >=");
```

:::hints
- kind: attention
  body: "Loc cacSpan giu lai s co s.parentSpanId !== undefined vao conSpan. Neu conSpan.length === 0 thi return undefined. Nguoc lai dat max = conSpan[0]!, duyet for de cap nhat max khi mot span co thoiGianKetThucMs - thoiGianBatDauMs LON HON (>) max, roi tra ve { tenDichVu: max.tenDichVu, thoiGianMs: ... }."
- kind: strategy
  body: "const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined); if (conSpan.length === 0) return undefined; let max = conSpan[0]!; for (const s of conSpan) { if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s; } return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };"
- kind: one-line
  body: "const conSpan = cacSpan.filter((s) => s.parentSpanId !== undefined); if (conSpan.length === 0) return undefined; let max = conSpan[0]!; for (const s of conSpan) { if (s.thoiGianKetThucMs - s.thoiGianBatDauMs > max.thoiGianKetThucMs - max.thoiGianBatDauMs) max = s; } return { tenDichVu: max.tenDichVu, thoiGianMs: max.thoiGianKetThucMs - max.thoiGianBatDauMs };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "{\"tenDichVu\":\"con-cham\",\"thoiGianMs\":90}"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`traceId` nối các span thành MỘT request, `parentSpanId` nối chúng thành
một CHUỖI lồng nhau — giờ biết CHÍNH XÁC service nào chiếm nhiều thời gian
nhất. Nhưng biết "chậm" không giống biết "đủ chậm để BÁO ĐỘNG". Câu hỏi kế
tiếp: bao nhiêu LÀ đủ tốt, VÀ còn dư bao nhiêu để chấp nhận rủi ro?
::::

::::reflect{#nghi-lai}
`taoSpanMoi` không hề cần một thuật toán phức tạp để sinh ID DUY nhất — một
bộ đếm tăng dần LÀ đủ, vì tính DUY NHẤT chỉ cần đúng TRONG một tiến trình
mô phỏng. Cái thật sự quan trọng nằm Ở CẤU trúc: `traceId` CHUNG buộc mọi
span VỀ cùng một request, `parentSpanId` biến một danh sách PHẲNG thành một
CÂY lồng nhau. `timDichVuChiemNhieuNhat` chỉ có Ý nghĩa NHỜ cấu trúc đó —
loại bỏ span gốc trước khi so sánh không phải LÀ một mẹo tuỳ tiện, mà LÀ
hệ quả trực tiếp của việc span gốc LUÔN bao trùm mọi span con của nó.
::::

::::checkpoint{mastery=0.77}
::::
