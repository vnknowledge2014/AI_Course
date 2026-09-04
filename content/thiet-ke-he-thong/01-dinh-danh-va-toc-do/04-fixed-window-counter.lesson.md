---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.fixed-window-counter
title: "Fixed window counter: đếm theo cửa sổ"
summary: "CuaSoCoDinh đếm request trong MỘT cửa sổ thời gian cố định (chiSoCuaSoHienTai = Math.floor(t/kichThuocCuaSoMs)); choPhepCuaSoCoDinh reset đếm về 0 ngay khi chỉ số cửa sổ đổi -- kể cả nhảy qua NHIỀU cửa sổ liên tiếp (không request nào ở giữa) vẫn reset đúng theo chỉ số MỚI, không cộng dồn từ cửa sổ cũ. Giới hạn 3/cửa sổ 60000ms: 4 request đầu cửa sổ 0 cho qua đúng 3, cửa sổ kế tiếp (t=60000) reset về 0, request đầu tiên lại được phép."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 8
teaches: [sd.fixed-window-counter]
requires: [sd.leaky-bucket-hang-doi]
concepts: [sd.fixed-window-counter]
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
Hai bucket (token, leaky) đều cần một CẤU trúc dữ liệu (mảng, hàng
đợi) — VÀ tính toán mỗi lần GỌI. Có cách nào ĐƠN giản hơn: chỉ ĐẾM,
không cần lưu TỪNG request?
::::

::::explain{#cua-so-co-dinh}
Fixed window counter chia trục thời gian THÀNH các cửa sổ (window)
kích thước CỐ định, liên tiếp không CHỒNG lấn — VÍ dụ mỗi `60000ms`.
Mỗi cửa sổ chỉ CẦN một con SỐ: đếm bao nhiêu request đã đến TRONG cửa
sổ đó. Sang cửa sổ MỚI, đếm reset VỀ 0:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

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

const dh = taoDongHoMoPhong();
const cs = taoCuaSoCoDinh(3, 60000); // toi da 3 request / cua so 60 giay

const dot1: boolean[] = [];
for (let i = 0; i < 4; i++) dot1.push(choPhepCuaSoCoDinh(cs, dh));
console.log("t=0, 4 request lien tiep (gioi han 3):", dot1.join(","));
console.log("dem trong cua so hien tai:", cs.soRequestTrongCuaSo);

tienThoiGian(dh, 60000);
console.log("sang cua so moi (t=60000), dem TRUOC khi goi:", cs.soRequestTrongCuaSo);
const duocPhepCuaSoMoi = choPhepCuaSoCoDinh(cs, dh);
console.log("request dau tien cua so moi:", duocPhepCuaSoMoi, "-- dem SAU khi goi:", cs.soRequestTrongCuaSo);
```

```text title=readonly
t=0, 4 request lien tiep (gioi han 3): true,true,true,false
dem trong cua so hien tai: 3
sang cua so moi (t=60000), dem TRUOC khi goi: 3
request dau tien cua so moi: true -- dem SAU khi goi: 1
```

`chiSoCuaSoHienTai = Math.floor(t / 60000)` XÁC định cửa sổ NÀO ta
đang đứng. Ở `t=0`, chỉ số LÀ `0`; ba request đầu cho qua (đếm LÊN
`1,2,3`), request thứ tư gặp `soRequestTrongCuaSo >= gioiHan`, bị
chặn. Tại `t=60000`, chỉ số cửa SỔ đổi thành `1` (KHÁC `0` cũ) — điều
kiện `chiSoHienTai !== cs.chiSoCuaSoHienTai` đúng, đếm RESET về `0`
NGAY trước khi kiểm tra giới hạn, nên request MỚI lại được phép.
::::

::::example{#nhay-qua-nhieu-cua-so}
Nếu KHÔNG request nào đến TRONG suốt vài cửa sổ, bộ đếm KHÔNG hề
"cộng dồn" hay "nhớ" các cửa sổ đã bỏ qua — nó chỉ so SÁNH chỉ số cửa
sổ HIỆN tại với chỉ số đã LƯU, dù chênh lệch BAO nhiêu:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

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

// cua so 1000ms, gioi han 2 -- nhay THANG qua nhieu cua so cung mot luc (khong request nao o giua)
const dh = taoDongHoMoPhong();
const cs = taoCuaSoCoDinh(2, 1000);
choPhepCuaSoCoDinh(cs, dh);
choPhepCuaSoCoDinh(cs, dh);
console.log("cua so 0 (t=0): chiSoCuaSoHienTai =", cs.chiSoCuaSoHienTai, ", dem =", cs.soRequestTrongCuaSo);

tienThoiGian(dh, 5500); // nhay thang toi cua so thu 5 (khong co request nao o cua so 1-4)
const duocPhep = choPhepCuaSoCoDinh(cs, dh);
console.log("sau khi nhay 5500ms (cua so 5, bo qua cua so 1-4):");
console.log("  duoc phep:", duocPhep, "-- chiSoCuaSoHienTai:", cs.chiSoCuaSoHienTai, "-- dem:", cs.soRequestTrongCuaSo);
```

```text title=readonly
cua so 0 (t=0): chiSoCuaSoHienTai = 0 , dem = 2
sau khi nhay 5500ms (cua so 5, bo qua cua so 1-4):
  duoc phep: true -- chiSoCuaSoHienTai: 5 -- dem: 1
```

`Math.floor(5500/1000) = 5` — dù cửa sổ `1,2,3,4` chưa hề CÓ request
nào, `choPhepCuaSoCoDinh` không hề "biết" hay QUAN tâm tới CHÚNG; nó
chỉ so SÁNH `5 !== 0` (chỉ số CŨ), thấy khác, reset đếm VỀ `0` rồi
cho request NÀY qua. Bộ đếm hoàn toàn KHÔNG có khái niệm "cửa sổ bị
bỏ lỡ".
::::

::::predict{#doan-hai-cua-so-lien-tiep commitOnce}
Giới hạn `2` request/cửa sổ `1000ms`. Gửi đúng `2` request tại
`t=999` (cuối cửa sổ `0`, cả hai đều được phép), rồi gửi thêm `2`
request NGAY tại `t=1000` (đầu cửa sổ `1`). BỐN request NÀY, trong
khoảng `1ms`, có TỔNG cộng bao nhiêu request được CHO qua?

:::opt{correct}
CẢ `4` — hai cửa sổ KHÁC nhau (`0` và `1`) đều có giới hạn RIÊNG, mỗi
cửa sổ tự đếm từ `0`, không hề "chia sẻ" hạn mức VỚI nhau
:::
:::opt
Chỉ `2` — dù chỉ số cửa sổ có đổi, HAI request quá gần nhau (`1ms`)
vẫn tính CHUNG một đợt, hạn mức TỔNG vẫn LÀ 2
::why
Nhầm "khoảng cách THỜI gian giữa hai request" VỚI "cửa sổ chúng thuộc
VỀ" — nhưng `choPhepCuaSoCoDinh` không hề nhìn VÀO khoảng cách thời
gian GIỮA các request, nó chỉ nhìn `Math.floor(t / kichThuocCuaSoMs)`.

Chỗ lệch: `t=999` cho `chiSoHienTai = Math.floor(999/1000) = 0`,
CÒN `t=1000` cho `Math.floor(1000/1000) = 1` — HAI chỉ số KHÁC nhau,
dù CHỈ cách nhau `1ms`. Cửa sổ `1` reset đếm VỀ `0`, nên `2` request
tại `t=1000` lại được tính TỪ đầu, không hề bị "hạn mức cửa sổ 0" cản
trở. Đây CHÍNH LÀ lỗ hổng Ở ranh giới — bài SAU sẽ đo nó CỤ thể hơn.
::
:::
::::

::::code{#viet_cho_phep_cua_so_co_dinh}
Hoàn thiện điều kiện phát hiện "đã sang cửa sổ MỚI" trong
`choPhepCuaSoCoDinh` — so sánh chỉ số cửa sổ vừa TÍNH với chỉ số ĐÃ
lưu.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface CuaSoCoDinh { soRequestTrongCuaSo: number; chiSoCuaSoHienTai: number; gioiHan: number; kichThuocCuaSoMs: number; }
function taoCuaSoCoDinh(gioiHan: number, kichThuocCuaSoMs: number): CuaSoCoDinh {
  return { soRequestTrongCuaSo: 0, chiSoCuaSoHienTai: 0, gioiHan, kichThuocCuaSoMs };
}
function choPhepCuaSoCoDinh(cs: CuaSoCoDinh, dh: DongHoMoPhong): boolean {
  const chiSoHienTai = Math.floor(dh.thoiGianHienTai / cs.kichThuocCuaSoMs);
  if (___) {
    cs.chiSoCuaSoHienTai = chiSoHienTai;
    cs.soRequestTrongCuaSo = 0;
  }
  if (cs.soRequestTrongCuaSo >= cs.gioiHan) return false;
  cs.soRequestTrongCuaSo++;
  return true;
}

const dh = taoDongHoMoPhong();
const cs = taoCuaSoCoDinh(1, 1000);
console.log(choPhepCuaSoCoDinh(cs, dh), choPhepCuaSoCoDinh(cs, dh));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

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

const dh = taoDongHoMoPhong();
const cs = taoCuaSoCoDinh(1, 1000);
console.log(choPhepCuaSoCoDinh(cs, dh), choPhepCuaSoCoDinh(cs, dh));
```

```typescript title=test
function layDemCuaSo(cs: CuaSoCoDinh): number { return cs.soRequestTrongCuaSo; }
function layChiSoCuaSo(cs: CuaSoCoDinh): number { return cs.chiSoCuaSoHienTai; }

const dhT = taoDongHoMoPhong();
const csT = taoCuaSoCoDinh(2, 1000);
const kqT: boolean[] = [];
for (let i = 0; i < 3; i++) kqT.push(choPhepCuaSoCoDinh(csT, dhT));
if (kqT.join(",") !== "true,true,false") throw new Error("gioi han 2, cua so 0: 2 dau duoc, thu 3 bi tu choi");
if (layDemCuaSo(csT) !== 2) throw new Error("dem trong cua so 0 phai dung la 2");

tienThoiGian(dhT, 1000);
if (choPhepCuaSoCoDinh(csT, dhT) !== true) throw new Error("sang cua so moi (t=1000) phai duoc phep lai (da reset)");
if (layDemCuaSo(csT) !== 1) throw new Error("dem sau request dau tien cua cua so moi phai la 1, khong duoc cong don tu cua so cu");
if (layChiSoCuaSo(csT) !== 1) throw new Error("chiSoCuaSoHienTai phai cap nhat thanh 1");

const dhT2 = taoDongHoMoPhong();
const csT2 = taoCuaSoCoDinh(5, 1000);
choPhepCuaSoCoDinh(csT2, dhT2);
tienThoiGian(dhT2, 3500);
choPhepCuaSoCoDinh(csT2, dhT2);
if (layChiSoCuaSo(csT2) !== 3) throw new Error("nhay thang qua nhieu cua so (t=3500) phai cap nhat chiSoCuaSoHienTai thanh 3");
if (layDemCuaSo(csT2) !== 1) throw new Error("cua so moi (sau khi nhay xa) phai bat dau tu dem 1, khong cong don");
```

:::hints
- kind: attention
  body: "So sanh chiSoHienTai (vua tinh) voi cs.chiSoCuaSoHienTai (da luu) -- khac nhau nghia la sang cua so moi, mot dong."
- kind: strategy
  body: "chiSoHienTai !== cs.chiSoCuaSoHienTai"
- kind: one-line
  body: "if (chiSoHienTai !== cs.chiSoCuaSoHienTai) {"
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
Fixed window counter gọn VÀ nhanh — chỉ hai con SỐ, không mảng
không hàng đợi. Nhưng bài dự đoán vừa hé LỘ một lỗ hổng: ranh giới
GIỮA hai cửa sổ. Bài sau sẽ ĐO nó bằng số liệu THẬT.
::::

::::reflect{#nghi-lai}
`choPhepCuaSoCoDinh` chỉ cần đúng HAI trường trạng thái
(`chiSoCuaSoHienTai`, `soRequestTrongCuaSo`) — rẻ hơn HẲN một mảng
timestamp hay một hàng đợi. Nhưng "rẻ" ở ĐÂY đánh đổi bằng một điểm
YẾU: cửa sổ chỉ quan tâm CHỈ SỐ, hoàn toàn không "nhớ" điều GÌ đã
xảy ra Ở cửa sổ liền trước.
::::

::::checkpoint{mastery=0.75}
::::
