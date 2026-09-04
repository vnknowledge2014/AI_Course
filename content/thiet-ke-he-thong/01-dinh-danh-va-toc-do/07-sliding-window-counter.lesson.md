---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.sliding-window-counter
title: "Sliding window counter: xấp xỉ rẻ hơn"
summary: "uocLuongTruot nội suy demCuaSoTruoc × (1 − tỉ lệ vào cửa sổ hiện tại) + demCuaSoHienTai -- xấp xỉ sliding log bằng đúng HAI con số (không phải một mảng). CÙNG kịch bản ranh giới của bài 5 (5 request sát cuối cửa sổ 0 + 5 request sát đầu cửa sổ 1, giới hạn 5/1000ms): sliding window counter chỉ cho qua 6/10 (so với fixed window cho qua 10/10) -- cải thiện rõ rệt dù chưa hoàn hảo như sliding log (sẽ cho đúng ~5)."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.sliding-window-counter]
requires: [sd.sliding-window-log]
concepts: [sd.sliding-window-counter]
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
Sliding window log CHÍNH xác nhưng tốn bộ nhớ theo LƯU lượng. Fixed
window RẺ (hai con số) nhưng có lỗ hổng Ở ranh GIỚI. Sliding window
counter đứng GIỮA: vẫn CHỈ hai con số, nhưng "TRỘN" chúng thông minh
hơn.
::::

::::explain{#noi-suy-hai-cua-so}
Ý tưởng: giữ đếm CỦA cửa sổ HIỆN tại (`demCuaSoHienTai`) VÀ cửa sổ
NGAY trước nó (`demCuaSoTruoc`). Ước lượng số request "TRONG khoảng
1000ms gần nhất" bằng cách LẤY toàn bộ đếm hiện tại, CỘNG một PHẦN
của đếm cửa sổ trước — phần ĐÓ tỉ lệ nghịch VỚI việc ta đã đi được
BAO xa vào cửa sổ hiện tại:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface CuaSoTruotBoDem { demCuaSoTruoc: number; demCuaSoHienTai: number; chiSoCuaSoHienTai: number; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoTruotBoDem(gioiHan: number, kichThuocCuaSoMs: number): CuaSoTruotBoDem {
  return { demCuaSoTruoc: 0, demCuaSoHienTai: 0, chiSoCuaSoHienTai: 0, gioiHan, kichThuocCuaSoMs };
}
function capNhatCuaSo(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): void {
  const chiSoHienTai = Math.floor(dh.thoiGianHienTai / cs.kichThuocCuaSoMs);
  if (chiSoHienTai !== cs.chiSoCuaSoHienTai) {
    cs.demCuaSoTruoc = chiSoHienTai === cs.chiSoCuaSoHienTai + 1 ? cs.demCuaSoHienTai : 0;
    cs.demCuaSoHienTai = 0;
    cs.chiSoCuaSoHienTai = chiSoHienTai;
  }
}
function uocLuongTruot(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): number {
  const tiLeVaoCuaSoHienTai = (dh.thoiGianHienTai % cs.kichThuocCuaSoMs) / cs.kichThuocCuaSoMs;
  return cs.demCuaSoTruoc * (1 - tiLeVaoCuaSoHienTai) + cs.demCuaSoHienTai;
}
function choPhepCuaSoTruotBoDem(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): boolean {
  capNhatCuaSo(cs, dh);
  if (uocLuongTruot(cs, dh) >= cs.gioiHan) return false;
  cs.demCuaSoHienTai++;
  return true;
}

// CUNG kich ban bai 5 (5 request sat cuoi cua so 0, 5 request sat dau cua so 1) -- gioi han 5, cua so 1000ms
const dh = taoDongHoMoPhong();
const cs = taoCuaSoTruotBoDem(5, 1000);
const cacThoiDiemGui = [980, 985, 990, 995, 999, 1000, 1001, 1002, 1003, 1004];
const ketQua: { t: number; duocPhep: boolean; uocLuong: number }[] = [];
for (const t of cacThoiDiemGui) {
  dh.thoiGianHienTai = t;
  capNhatCuaSo(cs, dh);
  const uocLuongTruocKhiGoi = Math.round(uocLuongTruot(cs, dh) * 1000) / 1000;
  const duocPhep = choPhepCuaSoTruotBoDem(cs, dh);
  ketQua.push({ t, duocPhep, uocLuong: uocLuongTruocKhiGoi });
}
for (const kq of ketQua) console.log(`t=${kq.t}ms: uoc luong truot=${kq.uocLuong}, duoc phep=${kq.duocPhep}`);
const tongDuocPhep = ketQua.filter((k) => k.duocPhep).length;
console.log("tong so request duoc cho qua (sliding window counter):", tongDuocPhep, "/ 10 -- so voi fixed window (bai 5): 10 / 10");
```

```text title=readonly
t=980ms: uoc luong truot=0, duoc phep=true
t=985ms: uoc luong truot=1, duoc phep=true
t=990ms: uoc luong truot=2, duoc phep=true
t=995ms: uoc luong truot=3, duoc phep=true
t=999ms: uoc luong truot=4, duoc phep=true
t=1000ms: uoc luong truot=5, duoc phep=false
t=1001ms: uoc luong truot=4.995, duoc phep=true
t=1002ms: uoc luong truot=5.99, duoc phep=false
t=1003ms: uoc luong truot=5.985, duoc phep=false
t=1004ms: uoc luong truot=5.98, duoc phep=false
tong so request duoc cho qua (sliding window counter): 6 / 10 -- so voi fixed window (bai 5): 10 / 10
```

Ngay TẠI `t=1000` (vừa sang cửa sổ mới), `tiLeVaoCuaSoHienTai = 0`
NÊN ước lượng vẫn tính GẦN như TOÀN bộ đếm cửa sổ trước (`5 × (1-0) +
0 = 5`) — ĐỦ để chạm hạn mức, request bị CHẶN ngay lập tức, khác hẳn
fixed window (cho qua VÔ điều kiện vì đếm mới reset về `0`). Kết quả
CUỐI: chỉ `6/10` lọt qua — cải thiện RÕ rệt so với `10/10` của fixed
window (bài 5), dù CHƯA hoàn hảo bằng sliding log (LẼ ra đúng phải
chặn Ở khoảng `5`).
::::

::::example{#nhay-qua-cua-so-xa}
Giống fixed window (bài 4), nếu NHẢY qua NHIỀU hơn một cửa sổ liên
tiếp (không request nào Ở giữa), cửa sổ NGAY trước KHÔNG còn LÀ "cửa
sổ liền kề" NỮA — `demCuaSoTruoc` phải VỀ `0`, không được giữ LẠI đếm
cũ ĐÃ quá xa:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface CuaSoTruotBoDem { demCuaSoTruoc: number; demCuaSoHienTai: number; chiSoCuaSoHienTai: number; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoTruotBoDem(gioiHan: number, kichThuocCuaSoMs: number): CuaSoTruotBoDem {
  return { demCuaSoTruoc: 0, demCuaSoHienTai: 0, chiSoCuaSoHienTai: 0, gioiHan, kichThuocCuaSoMs };
}
function capNhatCuaSo(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): void {
  const chiSoHienTai = Math.floor(dh.thoiGianHienTai / cs.kichThuocCuaSoMs);
  if (chiSoHienTai !== cs.chiSoCuaSoHienTai) {
    cs.demCuaSoTruoc = chiSoHienTai === cs.chiSoCuaSoHienTai + 1 ? cs.demCuaSoHienTai : 0;
    cs.demCuaSoHienTai = 0;
    cs.chiSoCuaSoHienTai = chiSoHienTai;
  }
}
function uocLuongTruot(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): number {
  const tiLeVaoCuaSoHienTai = (dh.thoiGianHienTai % cs.kichThuocCuaSoMs) / cs.kichThuocCuaSoMs;
  return cs.demCuaSoTruoc * (1 - tiLeVaoCuaSoHienTai) + cs.demCuaSoHienTai;
}
function choPhepCuaSoTruotBoDem(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): boolean {
  capNhatCuaSo(cs, dh);
  if (uocLuongTruot(cs, dh) >= cs.gioiHan) return false;
  cs.demCuaSoHienTai++;
  return true;
}

const dh = taoDongHoMoPhong();
const cs = taoCuaSoTruotBoDem(4, 1000);
for (const t of [100, 300, 500, 700]) {
  dh.thoiGianHienTai = t;
  choPhepCuaSoTruotBoDem(cs, dh);
}
console.log("sau 4 request trong cua so 0 -- demCuaSoHienTai:", cs.demCuaSoHienTai, ", chiSoCuaSoHienTai:", cs.chiSoCuaSoHienTai);

// NHAY THANG toi cua so 5 (t=5000) -- bo qua cua so 1-4, cua so 0 KHONG con la "ngay truoc" cua so 5
dh.thoiGianHienTai = 5000;
const duocPhepSauNhay = choPhepCuaSoTruotBoDem(cs, dh);
console.log("sau khi nhay toi t=5000 (cua so 5): demCuaSoTruoc =", cs.demCuaSoTruoc, "(PHAI la 0, khong phai 4 -- cua so 0 da qua xa)");
console.log("  duoc phep:", duocPhepSauNhay);
```

```text title=readonly
sau 4 request trong cua so 0 -- demCuaSoHienTai: 4 , chiSoCuaSoHienTai: 0
sau khi nhay toi t=5000 (cua so 5): demCuaSoTruoc = 0 (PHAI la 0, khong phai 4 -- cua so 0 da qua xa)
  duoc phep: true
```

`capNhatCuaSo` chỉ COI cửa sổ VỪA rồi LÀ "cửa sổ trước" (gán VÀO
`demCuaSoTruoc`) khi `chiSoHienTai === cs.chiSoCuaSoHienTai + 1` —
ĐÚNG một bước liền kề. Nhảy TỪ cửa sổ `0` thẳng tới cửa sổ `5` (KHÔNG
liền kề), `demCuaSoTruoc` phải VỀ `0` — cửa sổ `0` đã quá XA để còn
"ảnh hưởng" tới ước lượng hiện TẠI.
::::

::::predict{#doan-uoc-luong-dau-cua-so commitOnce}
`demCuaSoTruoc = 6`, `demCuaSoHienTai = 0`, VỪA đúng lúc sang cửa sổ
mới (`tiLeVaoCuaSoHienTai = 0`, tức `thoiGianHienTai` LÀ bội số CHẴN
của `kichThuocCuaSoMs`). Ước lượng trượt (`uocLuongTruot`) lúc NÀY
bằng bao nhiêu?

:::opt{correct}
`6` — VÌ `tiLeVaoCuaSoHienTai = 0` nên trọng SỐ `(1 - 0) = 1`, giữ
NGUYÊN toàn bộ `demCuaSoTruoc`; cộng `demCuaSoHienTai = 0` vẫn LÀ `6`
:::
:::opt
`0` — VỪA "sang cửa sổ mới" nghĩa LÀ mọi thứ RESET, kể cả ước lượng
trượt CŨNG phải bắt đầu lại TỪ 0 giống fixed window
::why
Nhầm "cửa sổ HIỆN tại reset đếm VỀ 0" (ĐÚNG — `demCuaSoHienTai=0`)
VỚI "ƯỚC lượng trượt cũng reset VỀ 0" (SAI — đó chính LÀ điểm khác
biệt CỐT lõi so VỚI fixed window).

Chỗ lệch: công thức `uocLuongTruot` LÀ `demCuaSoTruoc × (1 -
tiLeVaoCuaSoHienTai) + demCuaSoHienTai`. Tại ĐÚNG ranh giới
(`tiLeVaoCuaSoHienTai = 0`), trọng số CỦA `demCuaSoTruoc` LÀ `(1-0) =
1` — TOÀN bộ đếm cửa sổ trước VẪN được tính, chưa hề "phai" ĐI chút
nào. Đây CHÍNH LÀ lý do sliding window counter KHÔNG mắc lỗ hổng như
fixed window ngay tại ranh giới.
::
:::
::::

::::code{#viet_uoc_luong_truot}
Hoàn thiện `uocLuongTruot` — nội suy giữa đếm cửa sổ trước (nhân
trọng số `1 - tiLeVaoCuaSoHienTai`) VÀ đếm cửa sổ hiện tại (trọng số
đầy đủ).

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface CuaSoTruotBoDem { demCuaSoTruoc: number; demCuaSoHienTai: number; chiSoCuaSoHienTai: number; gioiHan: number; kichThuocCuaSoMs: number; }
function capNhatCuaSo(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): void {
  const chiSoHienTai = Math.floor(dh.thoiGianHienTai / cs.kichThuocCuaSoMs);
  if (chiSoHienTai !== cs.chiSoCuaSoHienTai) {
    cs.demCuaSoTruoc = chiSoHienTai === cs.chiSoCuaSoHienTai + 1 ? cs.demCuaSoHienTai : 0;
    cs.demCuaSoHienTai = 0;
    cs.chiSoCuaSoHienTai = chiSoHienTai;
  }
}
function uocLuongTruot(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): number {
  const tiLeVaoCuaSoHienTai = (dh.thoiGianHienTai % cs.kichThuocCuaSoMs) / cs.kichThuocCuaSoMs;
  ___
}
function choPhepCuaSoTruotBoDem(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): boolean {
  capNhatCuaSo(cs, dh);
  if (uocLuongTruot(cs, dh) >= cs.gioiHan) return false;
  cs.demCuaSoHienTai++;
  return true;
}

const cs: CuaSoTruotBoDem = { demCuaSoTruoc: 8, demCuaSoHienTai: 2, chiSoCuaSoHienTai: 0, gioiHan: 100, kichThuocCuaSoMs: 1000 };
const dh = taoDongHoMoPhong();
dh.thoiGianHienTai = 500;
console.log(uocLuongTruot(cs, dh));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }

interface CuaSoTruotBoDem { demCuaSoTruoc: number; demCuaSoHienTai: number; chiSoCuaSoHienTai: number; gioiHan: number; kichThuocCuaSoMs: number; }
function capNhatCuaSo(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): void {
  const chiSoHienTai = Math.floor(dh.thoiGianHienTai / cs.kichThuocCuaSoMs);
  if (chiSoHienTai !== cs.chiSoCuaSoHienTai) {
    cs.demCuaSoTruoc = chiSoHienTai === cs.chiSoCuaSoHienTai + 1 ? cs.demCuaSoHienTai : 0;
    cs.demCuaSoHienTai = 0;
    cs.chiSoCuaSoHienTai = chiSoHienTai;
  }
}
function uocLuongTruot(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): number {
  const tiLeVaoCuaSoHienTai = (dh.thoiGianHienTai % cs.kichThuocCuaSoMs) / cs.kichThuocCuaSoMs;
  return cs.demCuaSoTruoc * (1 - tiLeVaoCuaSoHienTai) + cs.demCuaSoHienTai;
}
function choPhepCuaSoTruotBoDem(cs: CuaSoTruotBoDem, dh: DongHoMoPhong): boolean {
  capNhatCuaSo(cs, dh);
  if (uocLuongTruot(cs, dh) >= cs.gioiHan) return false;
  cs.demCuaSoHienTai++;
  return true;
}

const cs: CuaSoTruotBoDem = { demCuaSoTruoc: 8, demCuaSoHienTai: 2, chiSoCuaSoHienTai: 0, gioiHan: 100, kichThuocCuaSoMs: 1000 };
const dh = taoDongHoMoPhong();
dh.thoiGianHienTai = 500;
console.log(uocLuongTruot(cs, dh));
```

```typescript title=test
function layDemTruoc(c: CuaSoTruotBoDem): number { return c.demCuaSoTruoc; }
function layDemHienTai(c: CuaSoTruotBoDem): number { return c.demCuaSoHienTai; }

const dhT1 = taoDongHoMoPhong();
const csT1: CuaSoTruotBoDem = { demCuaSoTruoc: 0, demCuaSoHienTai: 3, chiSoCuaSoHienTai: 0, gioiHan: 10, kichThuocCuaSoMs: 1000 };
dhT1.thoiGianHienTai = 500;
if (uocLuongTruot(csT1, dhT1) !== 3) throw new Error("cua so truoc rong (demCuaSoTruoc=0): uoc luong phai dung bang demCuaSoHienTai (3)");

const dhT2 = taoDongHoMoPhong();
const csT2: CuaSoTruotBoDem = { demCuaSoTruoc: 8, demCuaSoHienTai: 2, chiSoCuaSoHienTai: 0, gioiHan: 100, kichThuocCuaSoMs: 1000 };
dhT2.thoiGianHienTai = 500;
if (uocLuongTruot(csT2, dhT2) !== 6) throw new Error("tiLe=0.5: uoc luong phai la 8*(1-0.5)+2 = 6");

const csT3: CuaSoTruotBoDem = { demCuaSoTruoc: 5, demCuaSoHienTai: 0, chiSoCuaSoHienTai: 1, gioiHan: 100, kichThuocCuaSoMs: 1000 };
const dhT3 = taoDongHoMoPhong();
dhT3.thoiGianHienTai = 1000;
if (uocLuongTruot(csT3, dhT3) !== 5) throw new Error("tiLe=0 (dau cua so): uoc luong phai giu nguyen demCuaSoTruoc (5)");

const dhT4 = taoDongHoMoPhong();
const csT4: CuaSoTruotBoDem = { demCuaSoTruoc: 0, demCuaSoHienTai: 0, chiSoCuaSoHienTai: 0, gioiHan: 5, kichThuocCuaSoMs: 1000 };
for (const t of [980, 985, 990, 995, 999]) { dhT4.thoiGianHienTai = t; choPhepCuaSoTruotBoDem(csT4, dhT4); }
if (layDemHienTai(csT4) !== 5) throw new Error("sau 5 request trong cua so 0, demCuaSoHienTai phai la 5");
dhT4.thoiGianHienTai = 1000;
if (choPhepCuaSoTruotBoDem(csT4, dhT4) !== false) throw new Error("t=1000 (dau cua so 1), uoc luong truot phai >= 5, request nay PHAI bi tu choi");
if (layDemTruoc(csT4) !== 5) throw new Error("ngay khi sang cua so 1, demCuaSoTruoc phai duoc gan bang demCuaSoHienTai cu (5)");
```

:::hints
- kind: attention
  body: "Tra ve demCuaSoTruoc nhan trong so (1 - tiLeVaoCuaSoHienTai), cong demCuaSoHienTai -- mot dong."
- kind: strategy
  body: "return cs.demCuaSoTruoc * (1 - tiLeVaoCuaSoHienTai) + cs.demCuaSoHienTai;"
- kind: one-line
  body: "return cs.demCuaSoTruoc * (1 - tiLeVaoCuaSoHienTai) + cs.demCuaSoHienTai;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "6"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bốn thuật toán rate limiting đã xong: token bucket, leaky bucket,
fixed window, sliding window (log VÀ counter). Nửa sau quest chuyển
sang một bài toán khác hẳn — sinh ĐỊNH DANH duy nhất, không trùng,
không cần điều phối.
::::

::::reflect{#nghi-lai}
`uocLuongTruot` chỉ LÀ một phép nội suy TUYẾN tính — nhưng NÓ cho
phép sliding window counter giữ được CÁI RẺ của fixed window (đúng
hai con số) MÀ vẫn tránh được PHẦN lớn lỗ hổng ranh giới. Đánh đổi:
đây LÀ một XẤP XỈ (giả định request rải ĐỀU trong cửa sổ trước), KHÔNG
chính xác tuyệt đối như sliding log.
::::

::::checkpoint{mastery=0.78}
::::
