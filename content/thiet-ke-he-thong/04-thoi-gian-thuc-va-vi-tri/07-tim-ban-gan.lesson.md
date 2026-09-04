---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.tim-ban-gan
title: "Tìm bạn gần: quét ô lưới VÀ tám ô lân cận"
summary: "timDiemLanCan quet O TRUY VAN cung 8 o LAN CAN (dLat, dLng tu -1 den 1) thay vi chi mot O DUY NHAT -- diem 'gan_bien' o o ke ben (idx=2) VAN duoc tim thay khi truy van tu idx=1, trong khi phien ban chi quet DUNG mot o (timDiemChiTrongO) BO SOT no hoan toan. Diem cach 2 O tro len (vuot pham vi -1..+1) van KHONG duoc tim thay du cung mot hang vi do."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.tim-ban-gan]
requires: [sd.geohash-ma-hoa]
concepts: [sd.tim-ban-gan]
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
Bài trước lộ RA một lỗ hổng: hai điểm cách nhau `0.2` đơn vị vẫn có thể
rơi vào hai Ô khác hẳn nhau, chỉ vì nằm HAI bên một đường biên lưới. Tìm
"bạn gần" mà chỉ quét ĐÚNG một ô thì sẽ bỏ SÓT đúng những trường hợp SÁT
biên đó.
::::

::::explain{#quet-o-va-lan-can}
Mỗi điểm được lưu vào MỘT ô lưới, theo đúng chỉ số `chiSoO` của bài
trước. Để tìm mọi điểm "gần" một toạ độ truy vấn, KHÔNG thể chỉ quét ô
CHỨA toạ độ đó — phải quét CẢ tám ô lân cận (lệch `-1`, `0`, hoặc `1`
theo cả hai trục), vì một điểm sát biên có thể nằm Ở ô KẾ bên:

```typescript title=readonly
const KICH_THUOC_O = 1;
function chiSoO(toaDo: number): number { return Math.floor(toaDo / KICH_THUOC_O); }
function khoaO(latIdx: number, lngIdx: number): string { return `${latIdx},${lngIdx}`; }

interface DiemVN { ten: string; lat: number; lng: number; }
type LuoiDiem = Map<string, DiemVN[]>;
function taoLuoiDiem(): LuoiDiem { return new Map(); }

function themDiemVaoLuoi(luoi: LuoiDiem, diem: DiemVN): void {
  const khoa = khoaO(chiSoO(diem.lat), chiSoO(diem.lng));
  const ds = luoi.get(khoa) ?? [];
  ds.push(diem);
  luoi.set(khoa, ds);
}

function timDiemLanCan(luoi: LuoiDiem, latTruyVan: number, lngTruyVan: number): DiemVN[] {
  const latIdx = chiSoO(latTruyVan);
  const lngIdx = chiSoO(lngTruyVan);
  const ketQua: DiemVN[] = [];
  for (let dLat = -1; dLat <= 1; dLat++) {
    for (let dLng = -1; dLng <= 1; dLng++) {
      const khoa = khoaO(latIdx + dLat, lngIdx + dLng);
      const ds = luoi.get(khoa) ?? [];
      ketQua.push(...ds);
    }
  }
  return ketQua;
}

const luoi = taoLuoiDiem();
themDiemVaoLuoi(luoi, { ten: "gan_bien", lat: 2.05, lng: 50.5 });
themDiemVaoLuoi(luoi, { ten: "cung_o", lat: 1.5, lng: 50.5 });
themDiemVaoLuoi(luoi, { ten: "xa", lat: 10.5, lng: 50.5 });

const ketQua = timDiemLanCan(luoi, 1.95, 50.5);
console.log("truy van tai (1.95, 50.5), o truy van idx:", chiSoO(1.95), chiSoO(50.5));
console.log("cac diem tim duoc:", ketQua.map((d) => d.ten).sort());
```

```text title=readonly
truy van tai (1.95, 50.5), o truy van idx: 1 50
cac diem tim duoc: [ 'cung_o', 'gan_bien' ]
```

Truy vấn rơi vào ô `(1, 50)`. `"cung_o"` (lat=1.5) nằm ĐÚNG trong ô đó.
`"gan_bien"` (lat=2.05) nằm Ở ô `(2, 50)` — LÂN cận theo trục lat (lệch
`+1`) — vẫn được tìm thấy nhờ vòng lặp `dLat` chạy từ `-1` tới `1`.
`"xa"` (lat=10.5, ô `(10, 50)`) cách quá xa để nằm trong CHÍN ô được
quét, nên không xuất hiện trong kết quả.
::::

::::example{#chi-quet-mot-o-bo-sot}
So sánh trực tiếp với một phiên bản CHƯA đủ — chỉ quét đúng ô chứa toạ
độ truy vấn, không xét ô lân cận — để thấy RÕ nó bỏ sót điều gì:

```typescript title=readonly
const KICH_THUOC_O = 1;
function chiSoO(toaDo: number): number { return Math.floor(toaDo / KICH_THUOC_O); }
function khoaO(latIdx: number, lngIdx: number): string { return `${latIdx},${lngIdx}`; }

interface DiemVN { ten: string; lat: number; lng: number; }
type LuoiDiem = Map<string, DiemVN[]>;
function taoLuoiDiem(): LuoiDiem { return new Map(); }

function themDiemVaoLuoi(luoi: LuoiDiem, diem: DiemVN): void {
  const khoa = khoaO(chiSoO(diem.lat), chiSoO(diem.lng));
  const ds = luoi.get(khoa) ?? [];
  ds.push(diem);
  luoi.set(khoa, ds);
}

// phien ban CHUA du: chi quet DUNG o chua diem truy van, khong xet o lan can
function timDiemChiTrongO(luoi: LuoiDiem, latTruyVan: number, lngTruyVan: number): DiemVN[] {
  const khoa = khoaO(chiSoO(latTruyVan), chiSoO(lngTruyVan));
  return luoi.get(khoa) ?? [];
}

function timDiemLanCan(luoi: LuoiDiem, latTruyVan: number, lngTruyVan: number): DiemVN[] {
  const latIdx = chiSoO(latTruyVan);
  const lngIdx = chiSoO(lngTruyVan);
  const ketQua: DiemVN[] = [];
  for (let dLat = -1; dLat <= 1; dLat++) {
    for (let dLng = -1; dLng <= 1; dLng++) {
      const khoa = khoaO(latIdx + dLat, lngIdx + dLng);
      const ds = luoi.get(khoa) ?? [];
      ketQua.push(...ds);
    }
  }
  return ketQua;
}

// tai lap dung trang thai: cung mot luoi nhu khoi truoc
const luoi = taoLuoiDiem();
themDiemVaoLuoi(luoi, { ten: "gan_bien", lat: 2.05, lng: 50.5 });
themDiemVaoLuoi(luoi, { ten: "cung_o", lat: 1.5, lng: 50.5 });
themDiemVaoLuoi(luoi, { ten: "xa", lat: 10.5, lng: 50.5 });

const chiTrongO = timDiemChiTrongO(luoi, 1.95, 50.5);
console.log("chi quet O CHUA truy van, tim duoc:", chiTrongO.map((d) => d.ten).sort());

const lanCan = timDiemLanCan(luoi, 1.95, 50.5);
console.log("quet O truy van VA lan can, tim duoc:", lanCan.map((d) => d.ten).sort());
```

```text title=readonly
chi quet O CHUA truy van, tim duoc: [ 'cung_o' ]
quet O truy van VA lan can, tim duoc: [ 'cung_o', 'gan_bien' ]
```

`timDiemChiTrongO` chỉ nhìn ĐÚNG một ô — nó bỏ SÓT `"gan_bien"` hoàn
toàn, dù điểm đó chỉ cách ranh giới ô truy vấn `0.05` đơn vị. Đây chính
LÀ hậu quả thực tế của "lỗi biên" đã thấy Ở bài trước: một điểm SÁT biên
có hàng xóm THẬT sự gần lại nằm Ở ô kế bên.
::::

::::predict{#doan-vuot-pham-vi-lan-can commitOnce}
Một điểm `"khanh"` nằm Ở `(lat=5.5, lng=32.5)`. Truy vấn thực hiện tại
`(lat=5.5, lng=30.5)` — CÙNG một hàng vĩ độ (`latIdx` giống hệt nhau,
đều LÀ `5`), chỉ khác kinh độ. `"khanh"` có được `timDiemLanCan` tìm thấy
không?

:::opt{correct}
Không — `lngIdx` của truy vấn LÀ `30`, của `"khanh"` LÀ `32`, cách nhau
`2` ô; vòng lặp `dLng` chỉ quét từ `-1` tới `1` (tức LÀ `lngIdx` từ `29`
tới `31`), nên ô `32` nằm NGOÀI phạm vi được quét
:::
:::opt
Có — hai điểm nằm CÙNG một hàng vĩ độ (`latIdx` trùng khớp), nên chắc
chắn được coi LÀ lân cận, bất kể chênh lệch kinh độ LÀ bao nhiêu
::why
Nhầm "cùng một trục" VỚI "lân cận" — nhưng `timDiemLanCan` đòi hỏi CẢ hai
trục (`dLat` LẪN `dLng`) đều nằm trong phạm vi `-1` tới `1`, không phải
chỉ một trục khớp LÀ đủ.

Chỗ lệch: vòng lặp `dLng` chỉ tạo ra ba giá trị `lngIdx`: `29`, `30`,
`31` (`lngIdx truy vấn + dLng`, với `dLng` từ `-1` đến `1`). `"khanh"`
nằm Ở `lngIdx = 32` — CÁCH `2` ô so với truy vấn — không hề khớp bất kỳ
khoá nào trong ba khoá được quét, dù `latIdx` của cả hai trùng NHAU
hoàn toàn.
::
:::
::::

::::code{#viet_tim_diem_lan_can}
Hoàn thiện `timDiemLanCan` — dùng hai vòng `for` lồng nhau (`dLat` VÀ
`dLng`, mỗi cái chạy từ `-1` tới `1`) để quét ô truy vấn CÙNG tám ô lân
cận, gom mọi điểm tìm được vào `ketQua`.

```typescript title=starter
const KICH_THUOC_O = 1;
function chiSoO(toaDo: number): number { return Math.floor(toaDo / KICH_THUOC_O); }
function khoaO(latIdx: number, lngIdx: number): string { return `${latIdx},${lngIdx}`; }

interface DiemVN { ten: string; lat: number; lng: number; }
type LuoiDiem = Map<string, DiemVN[]>;
function taoLuoiDiem(): LuoiDiem { return new Map(); }

function themDiemVaoLuoi(luoi: LuoiDiem, diem: DiemVN): void {
  const khoa = khoaO(chiSoO(diem.lat), chiSoO(diem.lng));
  const ds = luoi.get(khoa) ?? [];
  ds.push(diem);
  luoi.set(khoa, ds);
}

function timDiemLanCan(luoi: LuoiDiem, latTruyVan: number, lngTruyVan: number): DiemVN[] {
  const latIdx = chiSoO(latTruyVan);
  const lngIdx = chiSoO(lngTruyVan);
  const ketQua: DiemVN[] = [];
  ___
  return ketQua;
}

const luoiX = taoLuoiDiem();
themDiemVaoLuoi(luoiX, { ten: "p1", lat: 4.5, lng: 4.5 });
console.log(timDiemLanCan(luoiX, 5.5, 5.5).map((d) => d.ten));
```

```typescript title=solution
const KICH_THUOC_O = 1;
function chiSoO(toaDo: number): number { return Math.floor(toaDo / KICH_THUOC_O); }
function khoaO(latIdx: number, lngIdx: number): string { return `${latIdx},${lngIdx}`; }

interface DiemVN { ten: string; lat: number; lng: number; }
type LuoiDiem = Map<string, DiemVN[]>;
function taoLuoiDiem(): LuoiDiem { return new Map(); }

function themDiemVaoLuoi(luoi: LuoiDiem, diem: DiemVN): void {
  const khoa = khoaO(chiSoO(diem.lat), chiSoO(diem.lng));
  const ds = luoi.get(khoa) ?? [];
  ds.push(diem);
  luoi.set(khoa, ds);
}

function timDiemLanCan(luoi: LuoiDiem, latTruyVan: number, lngTruyVan: number): DiemVN[] {
  const latIdx = chiSoO(latTruyVan);
  const lngIdx = chiSoO(lngTruyVan);
  const ketQua: DiemVN[] = [];
  for (let dLat = -1; dLat <= 1; dLat++) {
    for (let dLng = -1; dLng <= 1; dLng++) {
      const khoa = khoaO(latIdx + dLat, lngIdx + dLng);
      const ds = luoi.get(khoa) ?? [];
      ketQua.push(...ds);
    }
  }
  return ketQua;
}

const luoiX = taoLuoiDiem();
themDiemVaoLuoi(luoiX, { ten: "p1", lat: 4.5, lng: 4.5 });
console.log(timDiemLanCan(luoiX, 5.5, 5.5).map((d) => d.ten));
```

```typescript title=test
const luoi = taoLuoiDiem();
themDiemVaoLuoi(luoi, { ten: "gan_bien", lat: 2.05, lng: 50.5 });
themDiemVaoLuoi(luoi, { ten: "cung_o", lat: 1.5, lng: 50.5 });
themDiemVaoLuoi(luoi, { ten: "xa", lat: 10.5, lng: 50.5 });

const kq1 = timDiemLanCan(luoi, 1.95, 50.5).map((d) => d.ten).sort();
if (kq1.join(",") !== "cung_o,gan_bien") throw new Error("phai tim thay CA 'cung_o' (cung o) VA 'gan_bien' (o lan can), KHONG duoc co 'xa'");

const luoi2 = taoLuoiDiem();
themDiemVaoLuoi(luoi2, { ten: "khanh", lat: 5.5, lng: 32.5 });
const kq2 = timDiemLanCan(luoi2, 5.5, 30.5).map((d) => d.ten);
if (kq2.length !== 0) throw new Error("lngIdx cach nhau 2 o (vuot pham vi lan can -1..+1) thi KHONG duoc tim thay");

const luoi3 = taoLuoiDiem();
const kqRong = timDiemLanCan(luoi3, 0, 0);
if (kqRong.length !== 0) throw new Error("luoi rong phai tra ve mang rong, khong duoc nem loi");

const luoi4 = taoLuoiDiem();
themDiemVaoLuoi(luoi4, { ten: "a", lat: 0.5, lng: 0.5 });
themDiemVaoLuoi(luoi4, { ten: "b", lat: 0.5, lng: 0.5 });
const kq4 = timDiemLanCan(luoi4, 0.5, 0.5);
if (kq4.length !== 2) throw new Error("hai diem cung mot o phai deu duoc tra ve, khong duoc mat diem nao");
```

:::hints
- kind: attention
  body: "Can hai vong for LONG nhau: dLat chay tu -1 den 1, ben trong lai co dLng chay tu -1 den 1 -- tong cong quet DUNG 9 o (1 o chinh + 8 o lan can)."
- kind: strategy
  body: "Voi moi cap (dLat, dLng), tinh khoa = khoaO(latIdx + dLat, lngIdx + dLng), lay danh sach diem tai khoa do (hoac mang rong neu chua co), roi ketQua.push(...ds)."
- kind: one-line
  body: "for (let dLat = -1; dLat <= 1; dLat++) { for (let dLng = -1; dLng <= 1; dLng++) { const khoa = khoaO(latIdx + dLat, lngIdx + dLng); const ds = luoi.get(khoa) ?? []; ketQua.push(...ds); } }"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "p1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ô lưới VÀ lân cận giải quyết đúng lỗi biên vừa thấy. Mảnh kế tiếp rời
bản đồ, sang một danh sách LUÔN phải giữ thứ tự: bảng xếp hạng.
::::

::::reflect{#nghi-lai}
`timDiemLanCan` chỉ thêm đúng HAI vòng lặp so với "chỉ quét một ô" —
nhưng chính hai vòng lặp đó vá đúng lỗ hổng đã thấy Ở bài trước: một
điểm sát biên vẫn có "hàng xóm" THẬT sự gần, chỉ LÀ hàng xóm đó sống Ở ô
kế bên.
::::

::::checkpoint{mastery=0.78}
::::
