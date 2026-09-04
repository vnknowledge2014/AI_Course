---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.trang-thai-online
title: "Trạng thái online: heartbeat và ngưỡng"
summary: "laDangOnline coi một người LÀ online nếu heartbeat GẦN nhất cách hiện tại KHÔNG QUÁ NGUONG_ONLINE_MS (30000ms) -- ĐÚNG bằng ngưỡng (30000ms) vẫn tính LÀ online, vượt ngưỡng dù chỉ 1ms thì offline ngay. Người CHƯA từng gửi heartbeat luôn offline. Một heartbeat MỚI làm TƯƠI lại đồng hồ đếm -- gửi heartbeat rồi đợi 25s, gửi heartbeat lần NỮA rồi đợi thêm 25s (tổng 50s tính từ heartbeat ĐẦU) vẫn còn online, vì mốc tính lại LÀ heartbeat SAU."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.trang-thai-online]
requires: [sd.thu-tu-tin-nhan]
concepts: [sd.trang-thai-online]
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
Sắp xếp lại tin nhắn xong — bài trước phục hồi ĐÚNG thứ tự dù mạng làm
xáo trộn thứ tự đến. Câu hỏi kế tiếp trong chat: người kia còn Ở đó
KHÔNG? Không thể hỏi thẳng — chỉ có thể ĐOÁN dựa trên tín hiệu gần nhất.
::::

::::explain{#heartbeat-va-nguong}
Client gửi một tín hiệu "còn sống" (heartbeat) ĐỊNH kỳ — mỗi vài giây một
lần. Server không cần chờ CÂU trả lời "tôi online" tức thời; nó chỉ ghi
LẠI thời điểm heartbeat GẦN nhất, VÀ coi người đó LÀ online nếu khoảng
cách từ heartbeat đó tới HIỆN tại chưa vượt một ngưỡng cho phép:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }

function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}

const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;
}

const dh = taoDongHoMoPhong();
const tt = taoTrangThaiPresence();
ghiNhanHeartbeat(tt, dh, "an");
tienThoiGian(dh, 30000);
console.log("dung nguong 30000ms, con online khong:", laDangOnline(tt, dh, "an"));
tienThoiGian(dh, 1);
console.log("vuot nguong 1ms, con online khong:", laDangOnline(tt, dh, "an"));
console.log("nguoi chua tung heartbeat, online khong:", laDangOnline(tt, dh, "chi"));
```

```text title=readonly
dung nguong 30000ms, con online khong: true
vuot nguong 1ms, con online khong: false
nguoi chua tung heartbeat, online khong: false
```

`laDangOnline` dùng phép so sánh `<=` — NGAY tại `30000ms` (đúng
`NGUONG_ONLINE_MS`) vẫn tính LÀ online. Chỉ khi vượt QUA ngưỡng, dù chỉ
`1ms`, người đó mới chuyển sang offline. `"chi"` chưa từng gọi
`ghiNhanHeartbeat` NÊN `lanCuoiHoatDong.get("chi")` trả về `undefined` —
hàm trả `false` ngay, không hề tính hiệu số nào cả.
::::

::::example{#heartbeat-moi-lam-tuoi-lai}
Mỗi lần `ghiNhanHeartbeat` chạy, nó GHI ĐÈ mốc thời gian cũ — nghĩa LÀ
đồng hồ đếm ngược tới ngưỡng được TÍNH LẠI từ heartbeat MỚI nhất, không
cộng dồn từ heartbeat đầu tiên:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }

function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}

const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;
}

// tai lap dung trang thai: mot nguoi dung "binh" moi bat dau, dong ho o t=0
const dh = taoDongHoMoPhong();
const tt = taoTrangThaiPresence();
ghiNhanHeartbeat(tt, dh, "binh");
tienThoiGian(dh, 25000);
console.log("sau 25s ke tu heartbeat dau, truoc khi gui heartbeat moi:", laDangOnline(tt, dh, "binh"));

ghiNhanHeartbeat(tt, dh, "binh"); // heartbeat moi lam moi lanCuoiHoatDong
tienThoiGian(dh, 25000); // tong 50s tu heartbeat DAU, nhung chi 25s tu heartbeat MOI
console.log("tong 50s tu heartbeat dau (nhung heartbeat moi lam tuoi lai):", laDangOnline(tt, dh, "binh"));
```

```text title=readonly
sau 25s ke tu heartbeat dau, truoc khi gui heartbeat moi: true
tong 50s tu heartbeat dau (nhung heartbeat moi lam tuoi lai): true
```

Tổng thời gian từ heartbeat ĐẦU tiên đã LÀ `50000ms`, vượt xa
`NGUONG_ONLINE_MS`. Nhưng `laDangOnline` không hề nhớ heartbeat đầu tiên
— nó chỉ SO với `lanCuoiHoatDong`, mà giá trị đó đã được GHI ĐÈ bởi
heartbeat thứ hai. Tính từ mốc MỚI, chỉ `25000ms` đã trôi qua — vẫn trong
ngưỡng.
::::

::::predict{#doan-ranh-gioi-nguong-khac-moc commitOnce}
`"dung"` gửi heartbeat LÚC đồng hồ mô phỏng đang Ở `t=1000`. Đồng hồ sau
đó tiến tới ĐÚNG `t=31000` (tức LÀ đã trôi qua đúng `30000ms` kể từ
heartbeat của `"dung"`, đúng bằng `NGUONG_ONLINE_MS`). `laDangOnline` LÚC
này trả về gì?

:::opt{correct}
`true` — hiệu `31000 - 1000 = 30000`, ĐÚNG bằng `NGUONG_ONLINE_MS`, mà
phép so sánh dùng LÀ `<=` (không phải `<`), nên đúng ngưỡng vẫn tính LÀ
online
:::
:::opt
`false` — vì đã "hết hạn" đúng 30 giây kể từ heartbeat, một mốc thời gian
tròn trịa như vậy thường được coi LÀ đã quá hạn
::why
Nhầm "đủ 30 giây" VỚI "quá 30 giây" — nhưng `laDangOnline` không hề coi
mốc TRÒN của ngưỡng LÀ đã hết hạn.

Chỗ lệch: điều kiện trong hàm LÀ `dh.thoiGianHienTai - lanCuoi <=
NGUONG_ONLINE_MS`, dùng dấu `<=` bao gồm CẢ trường hợp bằng nhau. Chỉ khi
hiệu số LỚN HƠN (không phải bằng) `30000` — tức LÀ từ `t=31001` trở đi —
người dùng mới chuyển sang offline.
::
:::
::::

::::code{#viet_la_dang_online}
Hoàn thiện `laDangOnline` — SAU khi đã loại trường hợp chưa từng
heartbeat, so sánh khoảng thời gian đã trôi qua VỚI `NGUONG_ONLINE_MS`
(dùng `<=`, không phải `<`).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }

function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}

const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  ___
}

const dhX = taoDongHoMoPhong();
const ttX = taoTrangThaiPresence();
ghiNhanHeartbeat(ttX, dhX, "dung");
tienThoiGian(dhX, 10000);
console.log(laDangOnline(ttX, dhX, "dung"));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface TrangThaiPresence { lanCuoiHoatDong: Map<string, number>; }
function taoTrangThaiPresence(): TrangThaiPresence { return { lanCuoiHoatDong: new Map() }; }

function ghiNhanHeartbeat(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): void {
  tt.lanCuoiHoatDong.set(nguoiDung, dh.thoiGianHienTai);
}

const NGUONG_ONLINE_MS = 30000;
function laDangOnline(tt: TrangThaiPresence, dh: DongHoMoPhong, nguoiDung: string): boolean {
  const lanCuoi = tt.lanCuoiHoatDong.get(nguoiDung);
  if (lanCuoi === undefined) return false;
  return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;
}

const dhX = taoDongHoMoPhong();
const ttX = taoTrangThaiPresence();
ghiNhanHeartbeat(ttX, dhX, "dung");
tienThoiGian(dhX, 10000);
console.log(laDangOnline(ttX, dhX, "dung"));
```

```typescript title=test
if (laDangOnline(taoTrangThaiPresence(), taoDongHoMoPhong(), "chua-tung-heartbeat") !== false) throw new Error("nguoi chua tung heartbeat phai la offline");

const dh = taoDongHoMoPhong();
const tt = taoTrangThaiPresence();
ghiNhanHeartbeat(tt, dh, "an");
tienThoiGian(dh, 10000);
if (laDangOnline(tt, dh, "an") !== true) throw new Error("10s < 30000ms nguong, phai con online");

tienThoiGian(dh, 20000); // tong 30000ms dung nguong
if (laDangOnline(tt, dh, "an") !== true) throw new Error("dung bang NGUONG_ONLINE_MS (30000ms) van phai tinh la online (bien <=)");

tienThoiGian(dh, 1); // 30001ms
if (laDangOnline(tt, dh, "an") !== false) throw new Error("vuot nguong dung 1ms phai la offline");

ghiNhanHeartbeat(tt, dh, "an"); // heartbeat moi lam tuoi lai
if (laDangOnline(tt, dh, "an") !== true) throw new Error("heartbeat moi phai lam nguoi dung online tro lai ngay lap tuc");
```

:::hints
- kind: attention
  body: "Dong con lai phai so sanh HIEU (dh.thoiGianHienTai - lanCuoi) voi NGUONG_ONLINE_MS -- dung <= (khong phai <) de dung moc cung tinh la online."
- kind: strategy
  body: "return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS; -- hieu cang nho nghia la heartbeat cang gan, cang chac chan con online."
- kind: one-line
  body: "return dh.thoiGianHienTai - lanCuoi <= NGUONG_ONLINE_MS;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Online hay offline giờ đo được — chỉ bằng một hiệu số VÀ một ngưỡng.
Nhưng "đã đọc" trong nhóm thì khác hẳn: cần MỌI người, không phải một.
::::

::::reflect{#nghi-lai}
`laDangOnline` không hề "hỏi" client có đang mở app hay không — nó chỉ
SO một con số với một ngưỡng. Cái khó không nằm Ở phép so sánh, mà Ở việc
chọn ĐÚNG dấu `<=` tại đường biên: sai một dấu, người dùng bị đá offline
sớm hơn ĐÚNG một nhịp heartbeat.
::::

::::checkpoint{mastery=0.68}
::::
