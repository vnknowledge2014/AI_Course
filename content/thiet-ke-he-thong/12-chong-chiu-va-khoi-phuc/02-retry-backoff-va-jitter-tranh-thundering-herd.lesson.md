---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.retry-backoff-va-jitter-tranh-thundering-herd
title: "Retry với backoff và jitter: tránh thundering herd khi retry đồng loạt"
summary: "taoSoNguSeed(seed) la bo sinh so gia-ngau-nhien DON GIAN co seed ((seed*9301+49297)%233280/233280), KHONG dung Math.random() de ket qua LAP LAI duoc. tinhThoiGianChoRetry(lanThu,doTreCoBanMs,seed) tinh doTreCoSo bang doTreCoBanMs nhan Math.pow(2, lanThu) (exponential backoff: lanThu=0,1,2 voi doTreCoBanMs=50 cho 50,100,200), cong them jitter = taoSoNguSeed(seed) * doTreCoBanMs. 5 client CUNG lanThu=1, doTreCoBanMs=100, KHONG jitter deu cho DUNG 200ms (dong loat -- thundering herd); CO jitter (seed=client) cho 225,229,233,237,241 -- trai deu ra, khong con dong thoi diem."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.retry-backoff-va-jitter-tranh-thundering-herd]
requires: [sd.circuit-breaker-mo-mach-tranh-cascading-failure]
concepts: [sd.retry-backoff-va-jitter-tranh-thundering-herd]
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
Circuit breaker (bài trước) mở mạch khi lỗi LIÊN tiếp vượt ngưỡng — nhưng
TRƯỚC khi tới mức đó, một lỗi ĐƠN lẻ (mạng chớp nhoáng, một request bị rớt)
KHÔNG nên bị coi LÀ thất bại vĩnh viễn. Thử lại (retry) LÀ hợp lý — nhưng
thử lại SAI cách còn nguy hiểm hơn không thử lại.
::::

::::explain{#exponential-backoff}
`tinhThoiGianChoRetry` tính thời gian chờ TRƯỚC lần thử tiếp theo, tăng
theo CẤP số nhân — `doTreCoBanMs * Math.pow(2, lanThu)`. `taoSoNguSeed` LÀ một bộ sinh
số giả-ngẫu-nhiên ĐƠN giản, nhận `seed` LÀM tham số — KHÔNG dùng
`Math.random()`, để kết quả LUÔN lặp lại được cho CÙNG một `seed`:

```typescript title=readonly
function taoSoNguSeed(seed: number): number {
  return ((seed * 9301 + 49297) % 233280) / 233280;
}
function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu);
  const heSoNgauNhien = taoSoNguSeed(seed);
  const doTreJitterMs = heSoNgauNhien * doTreCoBanMs;
  return Math.round(doTreCoSo + doTreJitterMs);
}

console.log("--- tang dan theo lanThu, doTreCoBanMs=50, seed=7 (co dinh) ---");
for (let lanThu = 0; lanThu <= 3; lanThu++) console.log(`lanThu=${lanThu}:`, tinhThoiGianChoRetry(lanThu, 50, 7));
```

```text title=readonly
--- tang dan theo lanThu, doTreCoBanMs=50, seed=7 (co dinh) ---
lanThu=0: 75
lanThu=1: 125
lanThu=2: 225
lanThu=3: 425
```

`doTreCoSo` tăng CẤP số nhân đúng nghĩa: `50, 100, 200, 400` (nhân đôi mỗi
lần) — RỒI cộng THÊM một khoản jitter cố định (dựa trên `seed=7`, LUÔN ra
CÙNG một hệ số mỗi lần gọi). Kết quả CUỐI `75, 125, 225, 425` không hề
ngẫu nhiên thật sự — nó xác định HOÀN toàn bởi `lanThu` VÀ `seed`, nên có
thể kiểm tra bằng test tự động, giống HỆT mọi hàm khác trong track NÀY.
::::

::::example{#thundering-herd}
Lý do CẦN jitter: nếu HÀNG trăm client cùng gặp lỗi Ở CÙNG một thời điểm
(downstream vừa sập), VÀ tất cả tính backoff theo CÙNG một công thức
KHÔNG có phần ngẫu nhiên, chúng sẽ retry ĐỒNG loạt tại CHÍNH XÁC cùng một
mili-giây — dồn tải LẠI đúng lúc downstream vừa kịp hồi phục:

```typescript title=readonly
function taoSoNguSeed(seed: number): number {
  return ((seed * 9301 + 49297) % 233280) / 233280;
}
function tinhThoiGianKhongJitter(lanThu: number, doTreCoBanMs: number): number {
  return doTreCoBanMs * Math.pow(2, lanThu);
}
function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu);
  const heSoNgauNhien = taoSoNguSeed(seed);
  const doTreJitterMs = heSoNgauNhien * doTreCoBanMs;
  return Math.round(doTreCoSo + doTreJitterMs);
}

console.log("--- KHONG jitter: 5 client CUNG lanThu=1, doTreCoBanMs=100 ---");
for (let client = 1; client <= 5; client++) console.log(`client ${client}:`, tinhThoiGianKhongJitter(1, 100));

console.log("--- CO jitter: 5 client CUNG lanThu=1, doTreCoBanMs=100, seed=client ---");
for (let client = 1; client <= 5; client++) console.log(`client ${client}:`, tinhThoiGianChoRetry(1, 100, client));
```

```text title=readonly
--- KHONG jitter: 5 client CUNG lanThu=1, doTreCoBanMs=100 ---
client 1: 200
client 2: 200
client 3: 200
client 4: 200
client 5: 200
--- CO jitter: 5 client CUNG lanThu=1, doTreCoBanMs=100, seed=client ---
client 1: 225
client 2: 229
client 3: 233
client 4: 237
client 5: 241
```

KHÔNG có jitter: CẢ năm client đều tính ra ĐÚNG `200`ms — nếu năm client
NÀY thật sự tồn tại, chúng sẽ gửi request LẠI Ở cùng một thời điểm, tạo ra
một đợt dồn tải MỚI ngay khi downstream vừa đứng dậy. CÓ jitter (mỗi client
dùng `seed` riêng, ví dụ ID của chính nó): năm kết quả TRẢI ra
`225, 229, 233, 237, 241` — không còn client NÀO trùng thời điểm retry với
client khác, tải được dàn ĐỀU theo thời gian thay vì dồn thành một đỉnh.
::::

::::predict{#doan-lan-thu-dau-tien commitOnce}
Gọi `tinhThoiGianChoRetry(0, 100, 1)` — LẦN thử LẠI đầu tiên (`lanThu=0`).
`doTreCoSo` (phần TRƯỚC khi cộng jitter) LÀ bao nhiêu?

:::opt{correct}
`100` — `Math.pow(2, 0)` bằng `1` (số MŨ `0` cho kết quả `1`, không phải
`0`), NÊN `doTreCoSo = 100 * 1 = 100`, ĐÚNG bằng `doTreCoBanMs`
:::
:::opt
`0` — LẦN thử đầu tiên (`lanThu=0`) nghĩa LÀ "chưa từng backoff", nên
`doTreCoSo` nên bắt đầu TỪ `0`, không cộng gì thêm vào jitter
::why
Nhầm "lanThu=0 nghĩa LÀ chưa nhân gì cả" VỚI "kết quả phép nhân LÀ 0" —
nhưng số MŨ `0` của BẤT kỳ số nào khác `0` LUÔN cho kết quả `1`, không
phải `0`.

Chỗ lệch: `Math.pow(2, 0)` trả về `1` (quy tắc TOÁN học chuẩn, không phải
lỗi code) — VÀ dòng `const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu)`
NHÂN `doTreCoBanMs` VỚI `1`, cho đúng LẠI `100`, không hề TRIỆT tiêu về
`0`. Backoff LUÔN bắt đầu TỪ `doTreCoBanMs`, rồi mới NHÂN đôi dần Ở các
lần thử SAU.
::
:::
::::

::::code{#viet_tinh_thoi_gian_cho_retry}
Hoàn thiện `tinhThoiGianChoRetry` — tính `doTreCoSo` bằng `doTreCoBanMs *
Math.pow(2, lanThu)`, tính hệ số ngẫu nhiên bằng `taoSoNguSeed(seed)` (đã
có sẵn), nhân hệ số ĐÓ với `doTreCoBanMs` để ra jitter, RỒI trả về TỔNG đã
làm tròn (`Math.round`).

```typescript title=starter
function taoSoNguSeed(seed: number): number {
  return ((seed * 9301 + 49297) % 233280) / 233280;
}

function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  ___
}

console.log(tinhThoiGianChoRetry(0, 100, 1), tinhThoiGianChoRetry(1, 100, 1));
```

```typescript title=solution
function taoSoNguSeed(seed: number): number {
  return ((seed * 9301 + 49297) % 233280) / 233280;
}

function tinhThoiGianChoRetry(lanThu: number, doTreCoBanMs: number, seed: number): number {
  const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu);
  const heSoNgauNhien = taoSoNguSeed(seed);
  const doTreJitterMs = heSoNgauNhien * doTreCoBanMs;
  return Math.round(doTreCoSo + doTreJitterMs);
}

console.log(tinhThoiGianChoRetry(0, 100, 1), tinhThoiGianChoRetry(1, 100, 1));
```

```typescript title=test
const laKhongJitterT = tinhThoiGianChoRetry(0, 100, 0);
if (Math.abs(laKhongJitterT - 121) > 0.5) throw new Error("lanThu=0, doTreCoBanMs=100, seed=0 phai xap xi 121 (100 + jitter tu seed=0)");

const laLanThu0T = tinhThoiGianChoRetry(0, 100, 1);
if (laLanThu0T !== 125) throw new Error("lanThu=0, doTreCoBanMs=100, seed=1 phai la 125 (doTreCoSo bang doTreCoBanMs, cong jitter)");

const laLanThu1T = tinhThoiGianChoRetry(1, 100, 1);
if (laLanThu1T !== 225) throw new Error("lanThu=1 phai gap doi doTreCoSo so voi lanThu=0 (tuc 200) cong jitter, ra 225");

const laLanThu2T = tinhThoiGianChoRetry(2, 100, 1);
if (laLanThu2T !== 425) throw new Error("lanThu=2 phai co doTreCoSo tang tiep len 400, cong jitter, ra 425");

const clientAT = tinhThoiGianChoRetry(1, 100, 2);
const clientBT = tinhThoiGianChoRetry(1, 100, 3);
if (clientAT === clientBT) throw new Error("hai seed khac nhau phai cho jitter khac nhau, tranh thundering herd");
if (clientAT !== 229) throw new Error("seed=2, lanThu=1, doTreCoBanMs=100 phai la 229");
if (clientBT !== 233) throw new Error("seed=3, lanThu=1, doTreCoBanMs=100 phai la 233");
```

:::hints
- kind: attention
  body: "Ba buoc: doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu); heSoNgauNhien = taoSoNguSeed(seed); doTreJitterMs = heSoNgauNhien * doTreCoBanMs. Tra ve Math.round(doTreCoSo + doTreJitterMs)."
- kind: strategy
  body: "const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu); const heSoNgauNhien = taoSoNguSeed(seed); const doTreJitterMs = heSoNgauNhien * doTreCoBanMs; return Math.round(doTreCoSo + doTreJitterMs);"
- kind: one-line
  body: "const doTreCoSo = doTreCoBanMs * Math.pow(2, lanThu); const heSoNgauNhien = taoSoNguSeed(seed); const doTreJitterMs = heSoNgauNhien * doTreCoBanMs; return Math.round(doTreCoSo + doTreJitterMs);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "125 225"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Backoff tăng dần, jitter trải đều — retry giờ KHÔNG còn dồn tải đồng loạt.
Nhưng retry vô hạn định vào MỘT dịch vụ đang chật vật cũng có thể khiến CHÍNH
retry đó chiếm hết tài nguyên của những phần KHÁC trong hệ thống.
::::

::::reflect{#nghi-lai}
`taoSoNguSeed` không hề LÀ một bộ sinh số ngẫu nhiên "thật" — nó LÀ một
công thức xác định, VÀ đó chính LÀ điểm mạnh của nó Ở đây: hai lần gọi
`tinhThoiGianChoRetry` với CÙNG `seed` LUÔN cho CÙNG một kết quả, nên có
thể viết test khẳng định con số CHÍNH XÁC, không phải chỉ khẳng định
"có vẻ ngẫu nhiên". Jitter không hề LÀM backoff kém tin cậy hơn — nó LÀM
NHIỀU client tin cậy CÙNG một cơ chế nhưng không còn hành động y HỆT nhau
tại cùng một mili-giây.
::::

::::checkpoint{mastery=0.75}
::::
