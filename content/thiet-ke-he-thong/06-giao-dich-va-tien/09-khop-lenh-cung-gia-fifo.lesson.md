---
id: thiet-ke-he-thong.giao-dich-va-tien.khop-lenh-cung-gia-fifo
title: "Cùng giá, ai đến trước: FIFO khi mức giá bằng nhau"
summary: "themLenh(sl, lenh) sap xep theo gia TRUOC, thoiGianDat SAU (b.gia - a.gia || a.thoiGianDat - b.thoiGianDat) -- m1 (t=0), m2 (t=100), m3 (t=200) CUNG gia 50000 xep DUNG thu tu [m1,m2,m3] (den truoc, dung truoc); m4 gia CAO hon (51000) nhung dat MUON hon van len DAU tien (gia thang thoi gian); m5 CUNG gia 50000, thoiGianDat=50 (som hon m2=100) duoc chen VAO giua m1 va m2 du duoc goi SAU m3 -- tieu chi PHU (thoiGianDat) CHI xet khi gia bang nhau."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 9
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.khop-lenh-cung-gia-fifo]
requires: [sd.khop-lenh-gia-tot-nhat]
concepts: [sd.khop-lenh-cung-gia-fifo]
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
Khớp lệnh chọn đúng giá TỐT nhất — nhưng "tốt nhất" không phải LUÔN là
duy nhất. Ba người CÙNG đặt lệnh mua Ở đúng một mức giá, cùng một
giây. Sổ lệnh CHỈ có một vị trí `[0]` — ai được đứng đó?
::::

::::explain{#fifo-khi-gia-bang-nhau}
`themLenh` giờ sắp xếp theo HAI tiêu chí, dùng toán tử `||`: giá
TRƯỚC (`b.gia - a.gia` cho lệnh mua) — nếu hai giá BẰNG nhau, hiệu LÀ
`0` (falsy), biểu thức "rơi" sang tiêu chí THỨ hai: `thoiGianDat`
TĂNG dần — ai đặt lệnh SỚM hơn đứng trước:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; thoiGianDat: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia || a.thoiGianDat - b.thoiGianDat);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia || a.thoiGianDat - b.thoiGianDat);
  }
}

const dh = taoDongHoMoPhong();
const sl = taoSoLenh();

themLenh(sl, { id: "m1", ben: "mua", gia: 50000, soLuong: 5, thoiGianDat: dh.thoiGianHienTai });
tienThoiGian(dh, 100);
themLenh(sl, { id: "m2", ben: "mua", gia: 50000, soLuong: 3, thoiGianDat: dh.thoiGianHienTai });
tienThoiGian(dh, 100);
themLenh(sl, { id: "m3", ben: "mua", gia: 50000, soLuong: 7, thoiGianDat: dh.thoiGianHienTai });

console.log("ba lenh CUNG gia 50000, dat theo thu tu m1, m2, m3:", JSON.stringify(sl.lenhMua.map((l) => l.id)));
console.log("lenh mua TOT NHAT (dat SOM nhat, gia bang nhau):", JSON.stringify(sl.lenhMua[0]));
```

```text title=readonly
ba lenh CUNG gia 50000, dat theo thu tu m1, m2, m3: ["m1","m2","m3"]
lenh mua TOT NHAT (dat SOM nhat, gia bang nhau): {"id":"m1","ben":"mua","gia":50000,"soLuong":5,"thoiGianDat":0}
```

Cả ba lệnh cùng giá `50000` — `b.gia - a.gia` LUÔN LÀ `0` giữa chúng,
nên `themLenh` dựa hoàn toàn VÀO `thoiGianDat` để xếp thứ tự. `m1`
(đặt Ở `thoiGianDat: 0`, sớm nhất) đứng Ở `[0]` — người đến TRƯỚC được
phục vụ trước, đúng nghĩa FIFO (first in, first out).
::::

::::example{#gia-van-la-tieu-chi-chinh}
`||` chỉ RƠI sang `thoiGianDat` khi hiệu giá LÀ `0` — một lệnh giá tốt
HƠN, dù đặt MUỘN hơn bao nhiêu, vẫn đứng TRƯỚC mọi lệnh giá kém hơn:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; thoiGianDat: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia || a.thoiGianDat - b.thoiGianDat);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia || a.thoiGianDat - b.thoiGianDat);
  }
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: m1 (t=0), m2 (t=100),
// m3 (t=200), cung gia 50000, dang sap xep theo thu tu thoi gian [m1,m2,m3]
const dh = taoDongHoMoPhong();
const sl = taoSoLenh();
themLenh(sl, { id: "m1", ben: "mua", gia: 50000, soLuong: 5, thoiGianDat: dh.thoiGianHienTai });
tienThoiGian(dh, 100);
themLenh(sl, { id: "m2", ben: "mua", gia: 50000, soLuong: 3, thoiGianDat: dh.thoiGianHienTai });
tienThoiGian(dh, 100);
themLenh(sl, { id: "m3", ben: "mua", gia: 50000, soLuong: 7, thoiGianDat: dh.thoiGianHienTai });
tienThoiGian(dh, 100);

themLenh(sl, { id: "m4", ben: "mua", gia: 51000, soLuong: 2, thoiGianDat: dh.thoiGianHienTai });
console.log("m4 gia CAO hon (51000) nhung dat MUON hon -- van len DAU:", JSON.stringify(sl.lenhMua.map((l) => l.id)));

themLenh(sl, { id: "m5", ben: "mua", gia: 50000, soLuong: 1, thoiGianDat: 50 });
console.log("m5 CUNG gia 50000, thoiGianDat=50 (SOM hon m2=100, du duoc GOI sau m3):", JSON.stringify(sl.lenhMua.map((l) => l.id)));
```

```text title=readonly
m4 gia CAO hon (51000) nhung dat MUON hon -- van len DAU: ["m4","m1","m2","m3"]
m5 CUNG gia 50000, thoiGianDat=50 (SOM hon m2=100, du duoc GOI sau m3): ["m4","m1","m5","m2","m3"]
```

`m4` (`51000`, đặt SAU cùng theo thời gian thực gọi hàm) vẫn nhảy lên
ĐẦU — giá quyết định TRƯỚC tiên. `m5` (`50000`, `thoiGianDat: 50`)
được `themLenh` GỌI sau `m3`, nhưng `thoiGianDat` của nó (`50`) SỚM
hơn `m2` (`100`) — nó được CHÈN đúng vào giữa `m1` VÀ `m2`, chứng minh
thứ tự dựa VÀO `thoiGianDat` thật sự, không phải thứ tự GỌI hàm.
::::

::::predict{#doan-gia-thang-thoi-gian commitOnce}
Sổ lệnh đang có `m1` (`50000`, `thoiGianDat: 0`), `m2` (`50000`,
`thoiGianDat: 100`), `m3` (`50000`, `thoiGianDat: 200`). Thêm `m7`:
`gia: 49000` (THẤP hơn cả ba), `thoiGianDat: 0` — TRÙNG hệt thời gian
đặt VỚI `m1`. `m7` đứng Ở vị trí NÀO trong `sl.lenhMua` sau khi thêm?

:::opt{correct}
Đứng CUỐI cùng (sau `m3`) — `b.gia - a.gia` giữa `m7` VÀ bất kỳ lệnh
nào Ở `50000` LÀ `50000 - 49000 = 1000`, khác `0`; toán tử `||` không
bao giờ RƠI sang `thoiGianDat` khi giá đã khác nhau, nên việc `m7`
trùng thời gian VỚI `m1` không có Ý nghĩa gì Ở đây
:::
:::opt
Đứng NGAY ĐẦU, cạnh `m1` — vì cả hai CÙNG có `thoiGianDat: 0`, chúng
phải được xem LÀ "đến cùng lúc" VÀ xếp gần nhau
::why
Nhầm "trùng thời gian đặt" VỚI "đủ điều kiện để so theo thời gian" —
nhưng `thoiGianDat` CHỈ được dùng LÀM tiêu chí phân xử khi tiêu chí
giá đã KHÔNG phân xử được (hiệu giá bằng `0`).

Chỗ lệch: biểu thức `b.gia - a.gia || a.thoiGianDat - b.thoiGianDat`
đánh giá `b.gia - a.gia` TRƯỚC. Với `m7` (giá `49000`) so VỚI `m1`
(giá `50000`), hiệu LÀ `1000` — một số KHÁC `0`, tức LÀ TRUTHY. Toán
tử `||` trả về NGAY giá trị bên trái (`1000`) VÀ dừng lại — nhánh
`thoiGianDat` không bao giờ được tính tới. `m7` xếp Ở cuối, đúng theo
thứ tự GIÁ giảm dần, bất kể `thoiGianDat` trùng khớp cỡ nào.
::
:::
::::

::::code{#viet_them_lenh_fifo}
Hoàn thiện `themLenh` — thêm lệnh vào đúng mảng theo `lenh.ben`, RỒI
sắp xếp VỚI hai tiêu chí nối bằng `||`: giá TRƯỚC (giảm dần cho lệnh
mua, tăng dần cho lệnh bán), `thoiGianDat` TĂNG dần khi giá bằng
nhau.

```typescript title=starter
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; thoiGianDat: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  ___
}

const slX = taoSoLenh();
themLenh(slX, { id: "y1", ben: "mua", gia: 10, soLuong: 1, thoiGianDat: 5 });
themLenh(slX, { id: "y2", ben: "mua", gia: 10, soLuong: 1, thoiGianDat: 2 });
console.log(JSON.stringify(slX.lenhMua.map((l) => l.id)));
```

```typescript title=solution
type BenLenh = "mua" | "ban";
interface Lenh { id: string; ben: BenLenh; gia: number; soLuong: number; thoiGianDat: number; }
interface SoLenh { lenhMua: Lenh[]; lenhBan: Lenh[]; }
function taoSoLenh(): SoLenh { return { lenhMua: [], lenhBan: [] }; }

function themLenh(sl: SoLenh, lenh: Lenh): void {
  if (lenh.ben === "mua") {
    sl.lenhMua.push(lenh);
    sl.lenhMua.sort((a, b) => b.gia - a.gia || a.thoiGianDat - b.thoiGianDat);
  } else {
    sl.lenhBan.push(lenh);
    sl.lenhBan.sort((a, b) => a.gia - b.gia || a.thoiGianDat - b.thoiGianDat);
  }
}

const slX = taoSoLenh();
themLenh(slX, { id: "y1", ben: "mua", gia: 10, soLuong: 1, thoiGianDat: 5 });
themLenh(slX, { id: "y2", ben: "mua", gia: 10, soLuong: 1, thoiGianDat: 2 });
console.log(JSON.stringify(slX.lenhMua.map((l) => l.id)));
```

```typescript title=test
const slT = taoSoLenh();
themLenh(slT, { id: "a1", ben: "mua", gia: 100, soLuong: 1, thoiGianDat: 10 });
themLenh(slT, { id: "a2", ben: "mua", gia: 300, soLuong: 1, thoiGianDat: 20 });
themLenh(slT, { id: "a3", ben: "mua", gia: 200, soLuong: 1, thoiGianDat: 30 });
const giaMua = slT.lenhMua.map((l) => l.gia);
if (JSON.stringify(giaMua) !== JSON.stringify([300, 200, 100])) throw new Error("gia van phai la tieu chi CHINH, sap xep GIAM dan cho lenh mua");

const slT2 = taoSoLenh();
themLenh(slT2, { id: "b1", ben: "mua", gia: 500, soLuong: 1, thoiGianDat: 30 });
themLenh(slT2, { id: "b2", ben: "mua", gia: 500, soLuong: 1, thoiGianDat: 10 });
themLenh(slT2, { id: "b3", ben: "mua", gia: 500, soLuong: 1, thoiGianDat: 20 });
const idMua = slT2.lenhMua.map((l) => l.id);
if (JSON.stringify(idMua) !== JSON.stringify(["b2", "b3", "b1"])) throw new Error("CUNG gia phai sap xep theo thoiGianDat TANG dan (den truoc, khop truoc)");

const slT3 = taoSoLenh();
themLenh(slT3, { id: "c1", ben: "ban", gia: 90, soLuong: 1, thoiGianDat: 30 });
themLenh(slT3, { id: "c2", ben: "ban", gia: 90, soLuong: 1, thoiGianDat: 10 });
const idBan = slT3.lenhBan.map((l) => l.id);
if (JSON.stringify(idBan) !== JSON.stringify(["c2", "c1"])) throw new Error("lenh BAN cung gia cung phai uu tien thoiGianDat SOM hon");

const slT4 = taoSoLenh();
themLenh(slT4, { id: "d1", ben: "mua", gia: 100, soLuong: 1, thoiGianDat: 0 });
themLenh(slT4, { id: "d2", ben: "mua", gia: 100, soLuong: 1, thoiGianDat: 5 });
themLenh(slT4, { id: "d3", ben: "mua", gia: 99, soLuong: 1, thoiGianDat: 0 });
const idMua4 = slT4.lenhMua.map((l) => l.id);
if (JSON.stringify(idMua4) !== JSON.stringify(["d1", "d2", "d3"])) throw new Error("gia CAO hon phai luon dung TRUOC du thoiGianDat som hon bao nhieu");
```

:::hints
- kind: attention
  body: "Van push vao dung mang theo lenh.ben nhu truoc. Doi cham la ham so sanh trong sort: noi them '|| a.thoiGianDat - b.thoiGianDat' NGAY sau phep tru gia, cho CA lenhMua lan lenhBan."
- kind: strategy
  body: "if (lenh.ben === 'mua') { sl.lenhMua.push(lenh); sl.lenhMua.sort((a, b) => b.gia - a.gia || a.thoiGianDat - b.thoiGianDat); } else { sl.lenhBan.push(lenh); sl.lenhBan.sort((a, b) => a.gia - b.gia || a.thoiGianDat - b.thoiGianDat); }"
- kind: one-line
  body: "if (lenh.ben === \"mua\") { sl.lenhMua.push(lenh); sl.lenhMua.sort((a, b) => b.gia - a.gia || a.thoiGianDat - b.thoiGianDat); } else { sl.lenhBan.push(lenh); sl.lenhBan.sort((a, b) => a.gia - b.gia || a.thoiGianDat - b.thoiGianDat); }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 7000
- tier: output
  match: contains
  expect: "[\"y2\",\"y1\"]"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giá quyết định trước, thời gian phân xử khi giá bằng nhau — sổ lệnh
giờ công bằng ĐÚNG nghĩa FIFO. Chín mảnh đã xây xong: đặt phòng, huỷ
phòng, idempotency, sổ cái, ví không âm, chuyển nguyên tử, sổ lệnh,
khớp lệnh, FIFO. Giờ ráp TẤT cả lại.
::::

::::reflect{#nghi-lai}
`||` trong một hàm so sánh không phải MẸO cú pháp ngẫu nhiên — nó
diễn đạt CHÍNH XÁC ý nghĩa "tiêu chí phụ chỉ có Ý nghĩa khi tiêu chí
chính KHÔNG phân xử được". `0` (falsy) nghĩa LÀ "hai giá bằng nhau,
chưa biết ai trước ai" — ĐÚNG lúc đó, VÀ chỉ lúc đó, `thoiGianDat` mới
được hỏi tới. Một khi giá đã khác nhau, thời gian đặt lệnh — dù chênh
lệch bao nhiêu — không còn ý nghĩa gì với thứ tự trong sổ lệnh.
::::

::::checkpoint{mastery=0.82}
::::
