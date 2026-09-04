---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.fixed-window-loi-o-ranh-gioi
title: "Fixed window: lỗ hổng ở ranh giới"
summary: "demTrongKhoangThoiGian đếm số timestamp THẬT trong [tuMs,denMs). Với gioiHan=5/cua-so-1000ms: 5 request sát cuối cửa sổ 0 (t=980..999) + 5 request sát đầu cửa sổ 1 (t=1000..1004) -- fixed window (bài 4) cho qua CẢ 10 (mỗi cửa sổ tự đếm riêng, cả hai đều ≤5), nhưng một cửa sổ TRƯỢT thật 1000ms bao trùm ranh giới ([5,1005)) đếm ra ĐÚNG 10 -- gấp đôi hạn mức 5, chứng minh burst thật sự xảy ra chứ không phải suy diễn."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 5
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.fixed-window-loi-o-ranh-gioi]
requires: [sd.fixed-window-counter]
concepts: [sd.fixed-window-loi-o-ranh-gioi]
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
Bài dự đoán CUỐI bài trước đã hé lộ: `2` request tại `t=999` cộng `2`
request tại `t=1000` — CẢ bốn đều qua, dù chỉ cách nhau `1ms`. Giờ ĐO
CHÍNH XÁC lỗ hổng NÀY bằng số liệu thật, quy mô LỚN hơn.
::::

::::explain{#do-that-ranh-gioi}
Để đo "THỰC SỰ có bao nhiêu request lọt qua trong MỘT khoảng thời
gian bất kỳ", ta CẦN một hàm đếm timestamp TRỰC tiếp — không dựa VÀO
chỉ số cửa sổ, mà đếm XEM có bao nhiêu mốc thời gian nằm TRONG một
khoảng `[tuMs, denMs)`:

```typescript title=readonly
function demTrongKhoangThoiGian(cacThoiDiem: number[], tuMs: number, denMs: number): number {
  let dem = 0;
  for (const t of cacThoiDiem) {
    if (t >= tuMs && t < denMs) dem++;
  }
  return dem;
}

const cacThoiDiemGui = [980, 985, 990, 995, 999, 1000, 1001, 1002, 1003, 1004];

// dung 5 (dung gioi han moi cua so) neu dem THEO dung ranh gioi cua so co dinh [0,1000) va [1000,2000)
console.log("dem trong cua so co dinh [0,1000):", demTrongKhoangThoiGian(cacThoiDiemGui, 0, 1000));
console.log("dem trong cua so co dinh [1000,2000):", demTrongKhoangThoiGian(cacThoiDiemGui, 1000, 2000));

// nhung mot cua so TRUOT 1000ms, neo tai t=5 (bao trum ca hai phia ranh gioi), lai thay ca 10
console.log("dem trong cua so TRUOT [5,1005):", demTrongKhoangThoiGian(cacThoiDiemGui, 5, 1005));
```

```text title=readonly
dem trong cua so co dinh [0,1000): 5
dem trong cua so co dinh [1000,2000): 5
dem trong cua so TRUOT [5,1005): 10
```

Đếm THEO đúng ranh giới cửa sổ CỐ định (`[0,1000)` VÀ `[1000,2000)`),
mỗi bên đều đúng `5` — KHỚP với hạn mức, "hợp LỆ" theo cách fixed
window nhìn nhận. Nhưng MỘT khoảng `1000ms` bất kỳ (VÍ dụ `[5,1005)`,
"trượt" qua đúng RANH giới) lại chứa TRỌN cả `10` mốc — gấp ĐÔI hạn
mức thật sự cho phép trong BẤT kỳ cửa sổ `1000ms` nào.
::::

::::example{#fixed-window-thuc-su-cho-qua}
Ráp LẠI `choPhepCuaSoCoDinh` (bài 4) với đúng MƯỜI mốc thời gian Ở
trên — chứng minh fixed window THẬT SỰ cho qua cả `10` request NÀY,
không chỉ LÀ lý thuyết:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface CuaSoCoDinh { soRequestTrongCuaSo: number; chiSoCuaSoHienTai: number; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoCoDinh(gioiHan: number, kichThuocCuaSoMs: number): CuaSoCoDinh {
  return { soRequestTrongCuaSo: 0, chiSoCuaSoHienTai: 0, gioiHan, kichThuocCuaSoMs };
}
function choPhepCuaSoCoDinh(cs: CuaSoCoDinh, dh: DongHoMoPhong): boolean {
  const chiSoHienTai = Math.floor(dh.thoiGianHienTai / cs.kichThuocCuaSoMs);
  if (chiSoHienTai !== cs.chiSoCuaSoHienTai) {
    cs.chiSoCuaSoHienTai = chiSoHienTai;
    cs.soRequestTrongCuaSo = 0;
  }
  if (cs.soRequestTrongCuaSo >= cs.gioiHan) return false;
  cs.soRequestTrongCuaSo++;
  return true;
}

// gioi han 5 request / cua so 1000ms -- gui 5 request SAT cuoi cua so 0, roi 5 request SAT dau cua so 1
const dh = taoDongHoMoPhong();
const cs = taoCuaSoCoDinh(5, 1000);
const cacThoiDiemGui = [980, 985, 990, 995, 999, 1000, 1001, 1002, 1003, 1004];
const ketQua: { t: number; duocPhep: boolean }[] = [];
for (const t of cacThoiDiemGui) {
  dh.thoiGianHienTai = t;
  ketQua.push({ t, duocPhep: choPhepCuaSoCoDinh(cs, dh) });
}
for (const kq of ketQua) console.log(`t=${kq.t}ms: duoc phep=${kq.duocPhep}`);
const tongDuocPhep = ketQua.filter((k) => k.duocPhep).length;
console.log("tong so request duoc cho qua trong khoang 24ms (980..1004):", tongDuocPhep, "/ 10");
console.log("gioi han danh cho MOI cua so 1000ms:", cs.gioiHan);
```

```text title=readonly
t=980ms: duoc phep=true
t=985ms: duoc phep=true
t=990ms: duoc phep=true
t=995ms: duoc phep=true
t=999ms: duoc phep=true
t=1000ms: duoc phep=true
t=1001ms: duoc phep=true
t=1002ms: duoc phep=true
t=1003ms: duoc phep=true
t=1004ms: duoc phep=true
tong so request duoc cho qua trong khoang 24ms (980..1004): 10 / 10
gioi han danh cho MOI cua so 1000ms: 5
```

CẢ mười request đều `true` — trong đúng `24ms` (`980` tới `1004`),
gấp ĐÔI hạn mức `5`/`1000ms` đã lọt qua HOÀN toàn hợp lệ theo góc
nhìn của `choPhepCuaSoCoDinh`: NĂM request đầu thuộc cửa sổ `0` (đếm
`1..5`, ĐÚNG giới hạn), năm request sau thuộc cửa SỔ `1` (đếm lại từ
`1..5`, CŨNG đúng giới hạn) — không có QUY tắc nào nhìn XUYÊN qua
ranh giới.
::::

::::predict{#doan-tang-gioi-han-co-het-loi commitOnce}
Nếu TĂNG `gioiHan` lên gấp đôi (VÍ dụ từ `5` lên `10`) nhưng GIỮ
nguyên cách tính cửa sổ CỐ định, lỗ hổng Ở ranh giới (burst gấp đôi
hạn mức TRONG một khoảng ngắn) có còn XẢY ra không?

:::opt{correct}
CÓ — vẫn có THỂ xảy ra burst gấp đôi HẠN mức MỚI (`20` request trong
khoảng NGẮN quanh ranh giới), vì cơ CHẾ gây lỗ hổng (hai cửa sổ đếm
ĐỘC lập) không hề đổi, chỉ có CON số hạn mức đổi
:::
:::opt
KHÔNG — tăng hạn mức LÀM cửa sổ "rộng rãi" hơn, đủ SỨC hấp thụ mọi
burst quanh ranh GIỚI mà không còn VƯỢT quá giới hạn nào NỮA
::why
Nhầm "tăng hạn MỨC" (thay đổi con SỐ `gioiHan`) VỚI "sửa cơ chế đếm"
(thay đổi CÁCH `choPhepCuaSoCoDinh` xử LÝ ranh giới) — nhưng đây LÀ
hai việc HOÀN toàn khác nhau.

Chỗ lệch: lỗ hổng KHÔNG đến từ việc hạn mức "quá thấp", mà từ VIỆC
mỗi cửa sổ đếm HOÀN toàn độc lập, không hề "nhìn thấy" cửa sổ liền
kề. Dù `gioiHan` LÀ `5` hay `10` hay `1000`, LUÔN có thể xây một kịch
bản gửi ĐÚNG `gioiHan` request sát cuối cửa sổ NÀY cộng đúng
`gioiHan` request sát đầu cửa sổ SAU — cho ra `2 × gioiHan` request
trong một khoảng cực NGẮN, bất kể `gioiHan` LÀ bao nhiêu.
::
:::
::::

::::code{#viet_dem_trong_khoang_thoi_gian}
Hoàn thiện `demTrongKhoangThoiGian` — đếm số timestamp `t` thoả `tuMs
<= t < denMs` (biên dưới BAO gồm, biên trên KHÔNG bao gồm).

```typescript title=starter
function demTrongKhoangThoiGian(cacThoiDiem: number[], tuMs: number, denMs: number): number {
  let dem = 0;
  for (const t of cacThoiDiem) {
    if (___) dem++;
  }
  return dem;
}

console.log(demTrongKhoangThoiGian([100, 200, 300], 0, 250));
```

```typescript title=solution
function demTrongKhoangThoiGian(cacThoiDiem: number[], tuMs: number, denMs: number): number {
  let dem = 0;
  for (const t of cacThoiDiem) {
    if (t >= tuMs && t < denMs) dem++;
  }
  return dem;
}

console.log(demTrongKhoangThoiGian([100, 200, 300], 0, 250));
```

```typescript title=test
const cacThoiDiemGuiT = [980, 985, 990, 995, 999, 1000, 1001, 1002, 1003, 1004];
if (demTrongKhoangThoiGian(cacThoiDiemGuiT, 0, 1000) !== 5) throw new Error("cua so co dinh [0,1000) phai dem dung 5");
if (demTrongKhoangThoiGian(cacThoiDiemGuiT, 1000, 2000) !== 5) throw new Error("cua so co dinh [1000,2000) phai dem dung 5");
if (demTrongKhoangThoiGian(cacThoiDiemGuiT, 5, 1005) !== 10) throw new Error("cua so TRUOT [5,1005) phai dem dung 10 -- gap doi gioi han 5");
if (demTrongKhoangThoiGian(cacThoiDiemGuiT, 999, 1000) !== 1) throw new Error("cua so hep [999,1000) chi chua dung 1 moc (999), khong chua 1000 (bien tren khong bao gom)");
if (demTrongKhoangThoiGian([], 0, 1000) !== 0) throw new Error("mang rong phai dem ra 0");
```

:::hints
- kind: attention
  body: "Bien duoi BAO gom (>=), bien tren KHONG bao gom (<) -- mot dong."
- kind: strategy
  body: "t >= tuMs && t < denMs"
- kind: one-line
  body: "if (t >= tuMs && t < denMs) dem++;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lỗ hổng đã ĐO được bằng số liệu thật: gấp đôi hạn mức trong `24ms`.
Có cách nào giữ được sự CHÍNH XÁC của "cửa sổ trượt thật" mà KHÔNG
cần một hàm quét TOÀN bộ mảng như `demTrongKhoangThoiGian`?
::::

::::reflect{#nghi-lai}
`demTrongKhoangThoiGian` không PHẢI một thuật toán rate limiting —
nó LÀ công cụ ĐO, dùng để CHỨNG minh lỗ hổng bằng số liệu THẬT thay
vì chỉ nói SUÔNG "fixed window có lỗi ở ranh giới". Hai bài TIẾP theo
sẽ xây đúng "cửa sổ trượt" NÀY thành một thuật toán rate limiting
THẬT SỰ — một bản đắt (log đầy đủ) VÀ một bản rẻ (xấp xỉ).
::::

::::checkpoint{mastery=0.8}
::::
