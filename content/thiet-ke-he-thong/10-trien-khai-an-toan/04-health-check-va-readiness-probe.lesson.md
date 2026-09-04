---
id: thiet-ke-he-thong.trien-khai-an-toan.health-check-va-readiness-probe
title: "Health check & readiness probe: chưa sẵn sàng thì chưa nhận traffic"
summary: "kiemTraLiveness(instance) chi hoi 'con song khong' (conSong); kiemTraReadiness(instance, dh) hoi 'da SAN SANG nhan traffic chua' -- can CON SONG VA da qua du thoiGianWarmUpMs tinh tu thoiDiemBatDau, dung DongHoMoPhong (thoiGianHienTai, tienThoiGian) da dung xuyen R7. Instance khoi dong t=0, warm up 3000ms: t=2000 con song NHUNG chua san sang; t=3000 (dung bien, dung >=) moi san sang. locInstanceSanSang loc chi instance readiness=true -- load balancer CHI route toi day."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.health-check-va-readiness-probe]
requires: [sd.feature-flag-targeting-theo-segment]
concepts: [sd.health-check-va-readiness-probe]
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
Rolling deploy (bài 1) khởi động instance MỚI trong lúc instance CŨ
vẫn đang phục vụ. Nhưng "khởi động xong tiến trình" KHÔNG có nghĩa là
"sẵn sàng nhận request" — cache còn RỖNG, kết nối database còn chưa
mở. Nếu load balancer gửi traffic tới NGAY, người dùng nhận lỗi từ
một instance kỹ thuật LÀ đang chạy, nhưng chưa hề sẵn sàng.
::::

::::explain{#liveness-khac-readiness}
`kiemTraLiveness` chỉ hỏi MỘT câu: tiến trình còn sống không (dùng để
biết CÓ cần restart hay không). `kiemTraReadiness` hỏi câu KHÁC hẳn:
instance ĐÃ sẵn sàng nhận traffic chưa — cần còn sống VÀ đã trôi qua
đủ `thoiGianWarmUpMs` kể từ lúc khởi động. Thời gian đo bằng
`DongHoMoPhong`, không đụng `Date.now()`:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }

function kiemTraLiveness(instance: Instance): boolean {
  return instance.conSong;
}

function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  if (!instance.conSong) return false;
  return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;
}

const dh = taoDongHoMoPhong();
const instA: Instance = { id: "i-a", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 3000 };

console.log("t=0, i-a con song:", kiemTraLiveness(instA));
console.log("t=0, i-a san sang nhan traffic:", kiemTraReadiness(instA, dh));

tienThoiGian(dh, 2000);
console.log("t=2000, i-a con song:", kiemTraLiveness(instA));
console.log("t=2000, i-a san sang (con dang warm up):", kiemTraReadiness(instA, dh));

tienThoiGian(dh, 1000);
console.log("t=3000, i-a san sang (het warm up):", kiemTraReadiness(instA, dh));
```

```text title=readonly
t=0, i-a con song: true
t=0, i-a san sang nhan traffic: false
t=2000, i-a con song: true
t=2000, i-a san sang (con dang warm up): false
t=3000, i-a san sang (het warm up): true
```

Tại `t=2000`, `i-a` VẪN sống (`kiemTraLiveness` trả `true`) NHƯNG chưa
sẵn sàng — đây CHÍNH LÀ trường hợp mà probe liveness một mình không
bắt được: nó sẽ báo "khoẻ", trong khi thực tế chưa nên nhận traffic.
Chỉ tại `t=3000`, khi đủ `3000ms` warm-up, `kiemTraReadiness` mới
chuyển sang `true`.
::::

::::example{#loc_instance_san_sang}
`locInstanceSanSang` lọc RA danh sách instance mà load balancer nên
route tới — CHỈ những instance có `readiness=true`. Một instance đã
CHẾT (`conSong=false`) không bao giờ được coi LÀ sẵn sàng, bất kể đã
"khởi động" từ bao lâu:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }

function kiemTraLiveness(instance: Instance): boolean {
  return instance.conSong;
}

function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  if (!instance.conSong) return false;
  return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;
}

function locInstanceSanSang(cacInstance: Instance[], dh: DongHoMoPhong): Instance[] {
  return cacInstance.filter((i) => kiemTraReadiness(i, dh));
}

// tai lap ba instance: i-cu (da chay lau, san sang), i-moi (vua khoi dong, con warm up),
// i-chet (crash, conSong=false)
const dh = taoDongHoMoPhong();
const cacInstance: Instance[] = [
  { id: "i-cu", conSong: true, thoiDiemBatDau: -5000, thoiGianWarmUpMs: 3000 },
  { id: "i-moi", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 3000 },
  { id: "i-chet", conSong: false, thoiDiemBatDau: -5000, thoiGianWarmUpMs: 3000 },
];

console.log("t=0, cac instance SAN SANG (chi routed toi day):", JSON.stringify(locInstanceSanSang(cacInstance, dh).map((i) => i.id)));
console.log("t=0, i-chet con song khong:", kiemTraLiveness(cacInstance[2]!));
console.log("t=0, i-chet san sang khong (mac du khong ai kiem lai liveness):", kiemTraReadiness(cacInstance[2]!, dh));

tienThoiGian(dh, 3000);
console.log("t=3000, cac instance SAN SANG (i-moi da warm up xong):", JSON.stringify(locInstanceSanSang(cacInstance, dh).map((i) => i.id)));
```

```text title=readonly
t=0, cac instance SAN SANG (chi routed toi day): ["i-cu"]
t=0, i-chet con song khong: false
t=0, i-chet san sang khong (mac du khong ai kiem lai liveness): false
t=3000, cac instance SAN SANG (i-moi da warm up xong): ["i-cu","i-moi"]
```

`i-cu` đã bắt đầu TỪ trước (`thoiDiemBatDau=-5000`), nên tại `t=0` nó
đã VƯỢT quá `3000ms` warm-up từ lâu — sẵn sàng NGAY. `i-chet` không
bao giờ xuất hiện trong danh sách sẵn sàng, dù `thoiDiemBatDau` của nó
GIỐNG HỆT `i-cu` — `kiemTraReadiness` kiểm tra `conSong` TRƯỚC tiên,
VÀ thoát sớm nếu instance đã chết. Tại `t=3000`, `i-moi` gia nhập danh
sách — vừa đủ `3000ms` kể từ lúc khởi động.
::::

::::predict{#doan-dung-bien-warmup commitOnce}
Một instance có `thoiDiemBatDau=1000`, `thoiGianWarmUpMs=2000`. Đồng
hồ mô phỏng đang Ở `thoiGianHienTai=3000` — TỨC đã trôi qua ĐÚNG
`2000ms` kể từ lúc khởi động, không hơn không kém. `kiemTraReadiness`
tại thời điểm NÀY trả về gì?

:::opt{correct}
`true` — điều kiện dùng `>=` (không phải `>`); `3000 - 1000 = 2000`,
VÀ `2000 >= 2000` LÀ `true`, nên instance được coi LÀ sẵn sàng NGAY
tại đúng thời điểm đủ warm-up, không cần chờ thêm
:::
:::opt
`false` — mới VỪA ĐỦ `2000ms`, cần trôi qua HƠN `2000ms` (ví dụ
`2001ms`) mới thật sự chắc chắn instance đã warm up xong
::why
Nhầm "vừa đủ" VỚI "chưa đủ" — nhưng điều kiện trong code LÀ
`dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs`,
dùng `>=`.

Chỗ lệch: `3000 - 1000 = 2000`, so với `thoiGianWarmUpMs = 2000`, biểu
thức LÀ `2000 >= 2000`, cho `true` NGAY. Không CÓ khoảng chờ thêm nào
sau khi đạt đúng ngưỡng — `thoiGianWarmUpMs` LÀ thời gian TỐI THIỂU
cần, đạt đúng bằng LÀ đủ điều kiện.
::
:::
::::

::::code{#viet_kiem_tra_readiness}
Hoàn thiện `kiemTraReadiness` — nếu instance ĐÃ chết, trả về `false`
NGAY. Ngược lại, so sánh thời gian đã trôi qua (từ `thoiDiemBatDau`
tới `dh.thoiGianHienTai`) với `thoiGianWarmUpMs`.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }

function kiemTraLiveness(instance: Instance): boolean {
  return instance.conSong;
}

function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  ___
}

const dhX = taoDongHoMoPhong();
const instX: Instance = { id: "i-x", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 1000 };
tienThoiGian(dhX, 1000);
console.log(kiemTraReadiness(instX, dhX), kiemTraLiveness(instX));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface Instance { id: string; conSong: boolean; thoiDiemBatDau: number; thoiGianWarmUpMs: number; }

function kiemTraLiveness(instance: Instance): boolean {
  return instance.conSong;
}

function kiemTraReadiness(instance: Instance, dh: DongHoMoPhong): boolean {
  if (!instance.conSong) return false;
  return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;
}

const dhX = taoDongHoMoPhong();
const instX: Instance = { id: "i-x", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 1000 };
tienThoiGian(dhX, 1000);
console.log(kiemTraReadiness(instX, dhX), kiemTraLiveness(instX));
```

```typescript title=test
function locInstanceSanSang(cacInstance: Instance[], dh: DongHoMoPhong): Instance[] {
  return cacInstance.filter((i) => kiemTraReadiness(i, dh));
}

const dhT = taoDongHoMoPhong();
const chuaWarmT: Instance = { id: "chua-warm", conSong: true, thoiDiemBatDau: 0, thoiGianWarmUpMs: 5000 };
if (kiemTraReadiness(chuaWarmT, dhT) !== false) throw new Error("t=0, warm up 5000ms, chua sang, phai la false");

tienThoiGian(dhT, 4999);
if (kiemTraReadiness(chuaWarmT, dhT) !== false) throw new Error("con thieu 1ms nua moi du warm up, phai VAN la false");

tienThoiGian(dhT, 1);
if (kiemTraReadiness(chuaWarmT, dhT) !== true) throw new Error("dung luc du warm up (>=) phai la true");

const chetT: Instance = { id: "chet", conSong: false, thoiDiemBatDau: -10000, thoiGianWarmUpMs: 1000 };
if (kiemTraLiveness(chetT) !== false) throw new Error("conSong=false thi liveness phai la false");
if (kiemTraReadiness(chetT, dhT) !== false) throw new Error("instance da chet KHONG duoc coi la san sang, du da qua thoi gian warm up rat lau");

const dsT: Instance[] = [chuaWarmT, chetT];
const sanSangT = locInstanceSanSang(dsT, dhT);
if (sanSangT.length !== 1) throw new Error("chi DUNG 1 trong 2 instance dang san sang (chuaWarmT, vi da qua warm up)");
if (sanSangT[0]?.id !== "chua-warm") throw new Error("instance san sang phai la 'chua-warm', khong phai 'chet'");
```

:::hints
- kind: attention
  body: "Neu !instance.conSong thi return false NGAY. Nguoc lai return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs -- mot dong."
- kind: strategy
  body: "if (!instance.conSong) return false; return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;"
- kind: one-line
  body: "if (!instance.conSong) return false; return dh.thoiGianHienTai - instance.thoiDiemBatDau >= instance.thoiGianWarmUpMs;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chỉ route traffic tới instance đã THẬT sự sẵn sàng — không phải chỉ
"còn sống". Nhưng deploy không chỉ đụng vào CODE — nó thường kéo theo
thay đổi ở tầng LƯU trữ. Đổi schema database mà không cẩn thận thì
code CŨ vẫn đang chạy song song (bài 1) sẽ vỡ NGAY lập tức.
::::

::::reflect{#nghi-lai}
`kiemTraLiveness` VÀ `kiemTraReadiness` trả lời hai câu hỏi hoàn toàn
khác nhau, dù cùng nhìn vào MỘT instance — nhầm lẫn giữa "còn sống" VÀ
"sẵn sàng" LÀ nguồn gốc của một lớp lỗi kinh điển: traffic bị route
tới một instance kỹ thuật LÀ khoẻ mạnh, nhưng chưa hề chuẩn bị xong để
phục vụ. `locInstanceSanSang` biến sự phân biệt ĐÓ thành một quyết
định routing cụ thể, không chỉ LÀ một khái niệm trên giấy.
::::

::::checkpoint{mastery=0.77}
::::
