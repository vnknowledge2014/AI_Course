---
id: co-so-du-lieu.mo-hinh-hoa-theo-cau-hoi.clustering-column-sap-xep-trong-partition
title: "Clustering column — sắp xếp trong partition"
summary: "layTrongKhoangThoiGian quét MỘT partition đã sắp xếp theo clustering column (thoiGian), dừng NGAY khi vượt quá đầu khoảng — không cần chạm hết partition. Khoảng [0,9] (ở đầu, 500 hàng) chỉ chạm 11 hàng; khoảng [490,499] (ở cuối) phải chạm cả 500 vì không còn gì để bỏ qua sau đó. Vị trí khoảng cần tìm bên trong partition quyết định chi phí quét — nhưng kết quả (số hàng tìm thấy) luôn đúng, bất kể vị trí."
locale: vi
track: co-so-du-lieu
module: mo-hinh-hoa-theo-cau-hoi
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.clustering-column-sap-xep-trong-partition]
requires: [db.partition-key-quyet-dinh-khoa]
concepts: [db.clustering-column-sap-xep-trong-partition]
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
Partition key (bài trước) gom hết tin nhắn của MỘT phòng vào MỘT
chỗ. Nhưng một phòng có THỂ có hàng trăm tin nhắn — câu hỏi thường
gặp LÀ "tin nhắn TRONG khoảng thời gian [a,b]", không phải "tất cả".
::::

::::explain{#clustering-column}
Bên TRONG một partition, các hàng được giữ theo THỨ tự của một
"clustering column" (Ở đây LÀ `thoiGian`) — GIỐNG hệt memtable (q04
bài 2) LUÔN giữ thứ tự khoá. `layTrongKhoangThoiGian` tận DỤNG thứ tự
đó: quét TUẦN tự, dừng NGAY khi vượt quá đầu khoảng cần tìm, KHÔNG
cần chạm hết partition:

```typescript title=readonly
interface TinNhan { id: string; phongId: string; thoiGian: number; }

function sapXepTheoThoiGian(hang: TinNhan[]): TinNhan[] {
  return [...hang].sort((a, b) => a.thoiGian - b.thoiGian);
}

function layTrongKhoangThoiGian(hangDaSapXep: TinNhan[], tu: number, den: number): { ketQua: TinNhan[]; soLuotCham: number } {
  const ketQua: TinNhan[] = [];
  let soLuotCham = 0;
  for (const h of hangDaSapXep) {
    soLuotCham++;
    if (h.thoiGian > den) break;
    if (h.thoiGian >= tu) ketQua.push(h);
  }
  return { ketQua, soLuotCham };
}

const hang: TinNhan[] = [];
for (let i = 0; i < 500; i++) hang.push({ id: "tn" + i, phongId: "phong5", thoiGian: i });
const daSapXep = sapXepTheoThoiGian(hang);

const rDau = layTrongKhoangThoiGian(daSapXep, 0, 9);
console.log("khoang [0,9] (o dau): soKetQua=" + rDau.ketQua.length + " soLuotCham=" + rDau.soLuotCham);
const rCuoi = layTrongKhoangThoiGian(daSapXep, 490, 499);
console.log("khoang [490,499] (o cuoi): soKetQua=" + rCuoi.ketQua.length + " soLuotCham=" + rCuoi.soLuotCham);
```

```text title=readonly
khoang [0,9] (o dau): soKetQua=10 soLuotCham=11
khoang [490,499] (o cuoi): soKetQua=10 soLuotCham=500
```

Cả hai khoảng ĐỀU tìm đúng `10` hàng. NHƯNG khoảng `[0,9]` (Ở đầu
partition `500` hàng) chỉ CẦN chạm `11` hàng — dừng NGAY sau khi vượt
`9`. Khoảng `[490,499]` (Ở CUỐI) phải chạm HẾT cả `500` hàng, vì
KHÔNG còn gì phía sau để biết dừng SỚM — vị trí khoảng cần tìm BÊN
TRONG partition quyết định chi phí, KHÔNG phải kết quả.
::::

::::example{#thu-tu-la-dieu-kien-can}
`layTrongKhoangThoiGian` CHỈ dừng sớm được VÌ `hangDaSapXep` ĐÃ có
thứ tự — nếu hàng KHÔNG được sắp xếp theo `thoiGian`, dòng `if
(h.thoiGian > den) break;` sẽ dừng SAI (bỏ lỡ những hàng nằm trong
khoảng nhưng đứng SAU một hàng "có vẻ" đã vượt quá). Clustering
column KHÔNG chỉ LÀ một tiện ích — nó LÀ điều kiện CẦN để range query
đúng VÀ nhanh cùng lúc.
::::

::::predict{#doan-khoang-giua commitOnce}
Cùng partition `500` hàng, tìm khoảng `[250, 259]` (chính GIỮA). So
với khoảng `[0,9]` (`soLuotCham=11`) VÀ `[490,499]` (`soLuotCham=500`),
`soLuotCham` của khoảng GIỮA gần với con SỐ nào hơn?

:::opt{correct}
Gần `250` (khoảng GIỮA `11` và `500`) — quét TUẦN tự phải đi qua
đúng SỐ hàng đứng TRƯỚC khoảng cần tìm (`250` hàng CÓ `thoiGian` từ
`0` đến `249`) RỒI mới chạm tới VÀ dừng lại
:::

:::opt
Gần `11` — clustering column LUÔN giữ chi phí THẤP dù khoảng cần
tìm nằm Ở đâu, đó chính LÀ Ý nghĩa của việc "được sắp xếp"
::why
Trực giác NÀY đúng RẰNG sắp xếp GIÚP — nhưng nhầm "giúp" VỚI "MIỄN
phí bất kể vị trí". `layTrongKhoangThoiGian` LÀ quét TUYẾN tính
(khôn phải tìm kiếm nhị phân), nên chi phí VẪN tỉ lệ THUẬN với
khoảng cách TỪ đầu partition tới điểm bắt đầu khoảng cần tìm.

Chỗ lệch: để chạm tới `thoiGian=250`, vòng `for` PHẢI đi qua đúng
`250` hàng đứng TRƯỚC nó (`thoiGian` từ `0` tới `249`) — dù MỖI hàng
đó bị bỏ qua (không đẩy VÀO `ketQua`), chúng VẪN được "chạm" VÀ tính
VÀO `soLuotCham`. Kết quả CHÍNH xác LÀ `261` (`250` hàng bỏ qua +
`10` hàng khớp + `1` hàng để phát hiện vượt khoảng) — gần `250`,
không gần `11`.
::
:::
::::

::::code{#viet_lay_trong_khoang}
Hoàn thiện `layTrongKhoangThoiGian` — dừng NGAY khi gặp một hàng có
`thoiGian` vượt quá đầu KHOẢNG (`den`).

```typescript title=starter
interface TinNhan { id: string; phongId: string; thoiGian: number; }

function sapXepTheoThoiGian(hang: TinNhan[]): TinNhan[] {
  return [...hang].sort((a, b) => a.thoiGian - b.thoiGian);
}

function layTrongKhoangThoiGian(hangDaSapXep: TinNhan[], tu: number, den: number): { ketQua: TinNhan[]; soLuotCham: number } {
  const ketQua: TinNhan[] = [];
  let soLuotCham = 0;
  for (const h of hangDaSapXep) {
    soLuotCham++;
    ___
    if (h.thoiGian >= tu) ketQua.push(h);
  }
  return { ketQua, soLuotCham };
}

const hang: TinNhan[] = [];
for (let i = 0; i < 500; i++) hang.push({ id: "tn" + i, phongId: "phong5", thoiGian: i });
console.log(layTrongKhoangThoiGian(sapXepTheoThoiGian(hang), 0, 9).soLuotCham);
```

```typescript title=solution
interface TinNhan { id: string; phongId: string; thoiGian: number; }

function sapXepTheoThoiGian(hang: TinNhan[]): TinNhan[] {
  return [...hang].sort((a, b) => a.thoiGian - b.thoiGian);
}

function layTrongKhoangThoiGian(hangDaSapXep: TinNhan[], tu: number, den: number): { ketQua: TinNhan[]; soLuotCham: number } {
  const ketQua: TinNhan[] = [];
  let soLuotCham = 0;
  for (const h of hangDaSapXep) {
    soLuotCham++;
    if (h.thoiGian > den) break;
    if (h.thoiGian >= tu) ketQua.push(h);
  }
  return { ketQua, soLuotCham };
}

const hang: TinNhan[] = [];
for (let i = 0; i < 500; i++) hang.push({ id: "tn" + i, phongId: "phong5", thoiGian: i });
console.log(layTrongKhoangThoiGian(sapXepTheoThoiGian(hang), 0, 9).soLuotCham);
```

```typescript title=test
const hang2: TinNhan[] = [];
for (let i = 0; i < 500; i++) hang2.push({ id: "tn" + i, phongId: "phong5", thoiGian: i });
const daSapXep2 = sapXepTheoThoiGian(hang2);

const rDau = layTrongKhoangThoiGian(daSapXep2, 0, 9);
if (rDau.ketQua.length !== 10) throw new Error("khoang [0,9] phai tim dung 10 hang");
if (rDau.soLuotCham !== 11) throw new Error("khoang [0,9] (o dau) phai dung SOM, chi cham 11 hang, khong phai het ca 500");

const rCuoi = layTrongKhoangThoiGian(daSapXep2, 490, 499);
if (rCuoi.ketQua.length !== 10) throw new Error("khoang [490,499] phai tim dung 10 hang");
if (rCuoi.soLuotCham !== 500) throw new Error("khoang [490,499] (o cuoi) khong con gi de dung som -- phai cham het 500 hang");

const rGiua = layTrongKhoangThoiGian(daSapXep2, 250, 259);
if (rGiua.ketQua.length !== 10) throw new Error("khoang [250,259] phai tim dung 10 hang");
if (rGiua.soLuotCham !== 261) throw new Error("khoang [250,259] phai cham dung 261 hang (250 bo qua + 10 khop + 1 phat hien vuot)");

const rRong = layTrongKhoangThoiGian(daSapXep2, 10000, 20000);
if (rRong.ketQua.length !== 0) throw new Error("khoang khong khop hang nao thi ket qua phai rong");
```

:::hints
- kind: attention
  body: "Neu thoiGian cua hang vuot qua dau khoang (den) thi dung vong lap ngay -- mot dong."
- kind: strategy
  body: "if (h.thoiGian > den) break;"
- kind: one-line
  body: "if (h.thoiGian > den) break;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "11"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Partition key CHỌN đúng chỗ, clustering column CHO thứ tự BÊN trong.
Nhưng nếu partition key CHỌN sai, chuyện GÌ xảy ra?
::::

::::reflect{#nghi-lai}
`layTrongKhoangThoiGian` cho thấy LÝ do wide-column tách RIÊNG hai
khái niệm: partition key TRẢ lời "đi TỚI máy nào" (bài 2, chi phí
GẦN như hằng số nhờ hashing), clustering column TRẢ lời "trong máy
đó, hàng nào ĐỨNG trước hàng nào" (bài NÀY, chi phí phụ thuộc VỊ trí
NHƯNG luôn tận dụng được thứ tự CÓ sẵn). Hai quyết định thiết kế
NÀY — chọn partition key VÀ clustering column NÀO — CHÍNH LÀ "mô
hình hoá theo câu hỏi": phải biết câu hỏi thường gặp TRƯỚC khi chọn.
::::

::::checkpoint{mastery=0.85}
::::
