---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.sliding-window-log
title: "Sliding window log: đếm chính xác"
summary: "CuaSoTruotLog lưu TỪNG timestamp request vào mảng; donDepCuaSoTruotLog xoá các mốc đã quá cũ (<= ngưỡng = hiện tại − kichThuocCuaSoMs) trước khi đếm, choPhepCuaSoTruotLog từ chối khi số mốc còn lại >= gioiHan. Cửa sổ 500ms giới hạn 2: mốc t=0 vừa đủ 500ms tuổi tại t=500 bị dọn, mở đúng một chỗ cho request mới; 500 request cách nhau 1ms trong CÙNG một cửa sổ 1000ms tốn ĐÚNG 500 phần tử bộ nhớ thật, không phải một bộ đếm gọn như fixed window."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.sliding-window-log]
requires: [sd.fixed-window-loi-o-ranh-gioi]
concepts: [sd.sliding-window-log]
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
`demTrongKhoangThoiGian` (bài trước) đã ĐO đúng "cửa sổ trượt" —
nhưng nó CHỈ đếm trên một mảng CÓ SẴN. Rate limiter THẬT cần TỰ giữ
mảng đó, VÀ tự dọn nó theo thời gian.
::::

::::explain{#log-tung-timestamp}
Sliding window log lưu NGUYÊN mỗi timestamp của MỖI request đã cho
qua. Mỗi lần CÓ request mới, trước tiên DỌN các mốc đã quá cũ (ngoài
cửa sổ `kichThuocCuaSoMs` gần NHẤT), rồi đếm SỐ mốc còn lại — nếu
chưa đủ hạn mức thì cho qua VÀ ghi thêm timestamp MỚI:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface CuaSoTruotLog { cacThoiDiem: number[]; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoTruotLog(gioiHan: number, kichThuocCuaSoMs: number): CuaSoTruotLog {
  return { cacThoiDiem: [], gioiHan, kichThuocCuaSoMs };
}
function donDepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): void {
  const nguong = dh.thoiGianHienTai - cs.kichThuocCuaSoMs;
  while (cs.cacThoiDiem.length > 0 && cs.cacThoiDiem[0]! <= nguong) cs.cacThoiDiem.shift();
}
function choPhepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): boolean {
  donDepCuaSoTruotLog(cs, dh);
  if (cs.cacThoiDiem.length >= cs.gioiHan) return false;
  cs.cacThoiDiem.push(dh.thoiGianHienTai);
  return true;
}

const dh = taoDongHoMoPhong();
const cs = taoCuaSoTruotLog(3, 1000); // toi da 3 request / 1000ms TRUOT (bat ky khoang 1000ms nao)

for (const t of [0, 100, 200]) {
  dh.thoiGianHienTai = t;
  console.log(`t=${t}: duoc phep=${choPhepCuaSoTruotLog(cs, dh)}, log=[${cs.cacThoiDiem.join(",")}]`);
}
dh.thoiGianHienTai = 300;
console.log(`t=300: duoc phep=${choPhepCuaSoTruotLog(cs, dh)}, log=[${cs.cacThoiDiem.join(",")}]`);

dh.thoiGianHienTai = 1000;
console.log(`t=1000 (moc t=0 vua du 1000ms tuoi, bi don): duoc phep=${choPhepCuaSoTruotLog(cs, dh)}, log=[${cs.cacThoiDiem.join(",")}]`);
```

```text title=readonly
t=0: duoc phep=true, log=[0]
t=100: duoc phep=true, log=[0,100]
t=200: duoc phep=true, log=[0,100,200]
t=300: duoc phep=false, log=[0,100,200]
t=1000 (moc t=0 vua du 1000ms tuoi, bi don): duoc phep=true, log=[100,200,1000]
```

Ba request đầu (`t=0,100,200`) lấp đầy hạn mức `3`; request THỨ tư
(`t=300`) bị chặn — log VẪN giữ nguyên `[0,100,200]`, KHÔNG hề thêm
mốc bị TỪ chối. Tại `t=1000`, mốc `t=0` đã "già" ĐÚNG `1000ms` —
`donDepCuaSoTruotLog` dùng `<=` (giống quy ước hết HẠN Ở TTL, R6) nên
coi NÓ là quá cũ, dọn NÓ đi, mở đúng MỘT chỗ cho request MỚI.
::::

::::example{#chi-phi-bo-nho}
Cái GIÁ của độ chính xác NÀY: log giữ đúng MỘT phần tử cho MỖI
request còn TRONG cửa sổ — nếu hạn mức cao VÀ nhiều request dồn dập,
bộ nhớ tốn tỉ lệ THUẬN với số request, không phải một con số CỐ định
như fixed window (bài 4):

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface CuaSoTruotLog { cacThoiDiem: number[]; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoTruotLog(gioiHan: number, kichThuocCuaSoMs: number): CuaSoTruotLog {
  return { cacThoiDiem: [], gioiHan, kichThuocCuaSoMs };
}
function donDepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): void {
  const nguong = dh.thoiGianHienTai - cs.kichThuocCuaSoMs;
  while (cs.cacThoiDiem.length > 0 && cs.cacThoiDiem[0]! <= nguong) cs.cacThoiDiem.shift();
}
function choPhepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): boolean {
  donDepCuaSoTruotLog(cs, dh);
  if (cs.cacThoiDiem.length >= cs.gioiHan) return false;
  cs.cacThoiDiem.push(dh.thoiGianHienTai);
  return true;
}

// gioi han CAO (1000), cua so 1000ms -- gui 500 request, moi request cach nhau 1ms
const dh = taoDongHoMoPhong();
const cs = taoCuaSoTruotLog(1000, 1000);
for (let i = 0; i < 500; i++) {
  dh.thoiGianHienTai = i;
  choPhepCuaSoTruotLog(cs, dh);
}
console.log("sau 500 request (moi cach 1ms, cung trong 1 cua so 1000ms):");
console.log("  so phan tu THAT SU luu trong log:", cs.cacThoiDiem.length, "(= dung so request, khong phai mot bo dem gon)");
console.log("  bo nho ti le thuan voi SO REQUEST trong cua so, khong phai hang so nhu fixed window counter");
```

```text title=readonly
sau 500 request (moi cach 1ms, cung trong 1 cua so 1000ms):
  so phan tu THAT SU luu trong log: 500 (= dung so request, khong phai mot bo dem gon)
  bo nho ti le thuan voi SO REQUEST trong cua so, khong phai hang so nhu fixed window counter
```

`fixed window` (bài 4) CHỈ cần đúng HAI con số dù CÓ bao nhiêu request
— `sliding window log` cần MỘT phần tử mảng cho MỖI request còn hiệu
LỰC. Đổi lấy sự chính xác TUYỆT đối (không hề CÓ lỗ hổng ranh giới
như fixed window), cái GIÁ LÀ bộ nhớ.
::::

::::predict{#doan-don-dep-thu-tu commitOnce}
Log hiện có `[100, 300, 700]` (BA timestamp), cửa sổ `1000ms`. Gọi
`donDepCuaSoTruotLog` tại `dh.thoiGianHienTai = 1100`. Sau khi dọn,
log CÒN LẠI đúng những mốc NÀO?

:::opt{correct}
`[300, 700]` — ngưỡng LÀ `1100 - 1000 = 100`; mốc `100` (`<= 100`)
bị dọn, CÒN `300` và `700` (đều `> 100`) VẪN giữ nguyên
:::
:::opt
`[]` (rỗng) — VÌ hàm dọn LUÔN xoá SẠCH log mỗi lần được gọi, chuẩn bị
cho MỘT lượt đếm hoàn toàn MỚI
::why
Nhầm "dọn CÁC mốc quá cũ" VỚI "xoá TOÀN bộ log" — nhưng `while` CHỈ
lặp CHỪNG nào phần tử ĐẦU mảng (mốc CŨ nhất) còn thoả điều kiện quá
cũ, KHÔNG hề xoá vô điều KIỆN.

Chỗ lệch: `donDepCuaSoTruotLog` tính `nguong = 1100 - 1000 = 100`,
rồi CHỈ `shift()` các mốc `<= 100` Ở ĐẦU mảng. `100 <= 100` đúng
(dọn), nhưng `300 <= 100` SAI — vòng lặp `while` DỪNG ngay tại đó,
`300` VÀ `700` (còn mới HƠN ngưỡng) được giữ NGUYÊN.
::
:::
::::

::::code{#viet_don_dep_cua_so_truot_log}
Hoàn thiện `donDepCuaSoTruotLog` — lặp dọn CÁC mốc Ở đầu mảng CHỪNG
nào chúng CÒN quá cũ (`<= nguong`).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface CuaSoTruotLog { cacThoiDiem: number[]; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoTruotLog(gioiHan: number, kichThuocCuaSoMs: number): CuaSoTruotLog {
  return { cacThoiDiem: [], gioiHan, kichThuocCuaSoMs };
}
function donDepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): void {
  const nguong = dh.thoiGianHienTai - cs.kichThuocCuaSoMs;
  ___
}
function choPhepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): boolean {
  donDepCuaSoTruotLog(cs, dh);
  if (cs.cacThoiDiem.length >= cs.gioiHan) return false;
  cs.cacThoiDiem.push(dh.thoiGianHienTai);
  return true;
}

const dh = taoDongHoMoPhong();
const cs = taoCuaSoTruotLog(1, 1000);
console.log(choPhepCuaSoTruotLog(cs, dh));
dh.thoiGianHienTai = 500;
console.log(choPhepCuaSoTruotLog(cs, dh));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface CuaSoTruotLog { cacThoiDiem: number[]; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoTruotLog(gioiHan: number, kichThuocCuaSoMs: number): CuaSoTruotLog {
  return { cacThoiDiem: [], gioiHan, kichThuocCuaSoMs };
}
function donDepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): void {
  const nguong = dh.thoiGianHienTai - cs.kichThuocCuaSoMs;
  while (cs.cacThoiDiem.length > 0 && cs.cacThoiDiem[0]! <= nguong) cs.cacThoiDiem.shift();
}
function choPhepCuaSoTruotLog(cs: CuaSoTruotLog, dh: DongHoMoPhong): boolean {
  donDepCuaSoTruotLog(cs, dh);
  if (cs.cacThoiDiem.length >= cs.gioiHan) return false;
  cs.cacThoiDiem.push(dh.thoiGianHienTai);
  return true;
}

const dh = taoDongHoMoPhong();
const cs = taoCuaSoTruotLog(1, 1000);
console.log(choPhepCuaSoTruotLog(cs, dh));
dh.thoiGianHienTai = 500;
console.log(choPhepCuaSoTruotLog(cs, dh));
```

```typescript title=test
function layDoDaiLog(cs: CuaSoTruotLog): number { return cs.cacThoiDiem.length; }

const dhT = taoDongHoMoPhong();
const csT = taoCuaSoTruotLog(2, 500);
dhT.thoiGianHienTai = 0;
if (choPhepCuaSoTruotLog(csT, dhT) !== true) throw new Error("request 1 (t=0) phai duoc phep");
dhT.thoiGianHienTai = 100;
if (choPhepCuaSoTruotLog(csT, dhT) !== true) throw new Error("request 2 (t=100) phai duoc phep");
dhT.thoiGianHienTai = 200;
if (choPhepCuaSoTruotLog(csT, dhT) !== false) throw new Error("request 3 (t=200), da du gioi han 2 trong cua so 500ms, phai bi tu choi");
if (layDoDaiLog(csT) !== 2) throw new Error("log phai giu dung 2 phan tu sau khi tu choi (khong them phan tu bi tu choi)");

dhT.thoiGianHienTai = 500;
if (choPhepCuaSoTruotLog(csT, dhT) !== true) throw new Error("t=500, moc t=0 vua du 500ms tuoi PHAI bi don, mo cho cho request moi");
if (layDoDaiLog(csT) !== 2) throw new Error("sau khi don t=0 va them t=500, log phai con dung 2 phan tu ([100,500])");

dhT.thoiGianHienTai = 700;
if (choPhepCuaSoTruotLog(csT, dhT) !== true) throw new Error("t=700, moc t=100 da qua cu (<=200) bi don, chi con 500 trong cua so -- duoc phep");
if (layDoDaiLog(csT) !== 2) throw new Error("sau khi don 100 va them 700, log phai la [500,700], dai 2");
```

:::hints
- kind: attention
  body: "Lap while: chung nao phan tu DAU mang con ton tai VA no <= nguong thi shift() no ra -- mot dong."
- kind: strategy
  body: "while (cs.cacThoiDiem.length > 0 && cs.cacThoiDiem[0]! <= nguong) cs.cacThoiDiem.shift();"
- kind: one-line
  body: "while (cs.cacThoiDiem.length > 0 && cs.cacThoiDiem[0]! <= nguong) cs.cacThoiDiem.shift();"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chính xác tuyệt đối, nhưng tốn bộ nhớ theo SỐ request. Có cách nào
GIỮ được gần đúng độ chính xác NÀY mà rẻ hơn HẲN — chỉ cần vài con số,
không cần một mảng?
::::

::::reflect{#nghi-lai}
`donDepCuaSoTruotLog` chỉ LÀ một vòng `while` dọn phần tử CŨ Ở đầu
mảng — nhưng chính NÓ LÀ điều làm sliding window log CHÍNH xác tuyệt
đối: nó biết CHÍNH XÁC từng request đã tới LÚC nào, không hề "làm
tròn" VỀ một cửa sổ cố định như bài 4. Cái GIÁ trả cho sự chính xác
đó LÀ bộ nhớ tỉ lệ THUẬN với lưu lượng.
::::

::::checkpoint{mastery=0.75}
::::
