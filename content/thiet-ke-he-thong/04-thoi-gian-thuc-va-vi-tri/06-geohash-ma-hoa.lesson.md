---
id: thiet-ke-he-thong.thoi-gian-thuc-va-vi-tri.geohash-ma-hoa
title: "Mã hoá ô lưới: điểm gần nhau, mã giống nhau"
summary: "maHoaToaDo don gian hoa lat/lng thanh CHI SO luoi so nguyen o hai muc do phan giai (o THO canh 10, o MIN canh 1), ghep thanh mot chuoi '<oTho>|<oMin>' -- hai diem (12.3,45.6) va (13.7,44.2) cach nhau hon 1 don vi van CHUNG tien to o THO ('1,4'), chi khac o MIN. Diem CACH BIEN mot khoang nho (lat=9.9 va lat=10.1, chi cach 0.2) lai roi vao HAI o THO khac han nhau -- ranh gioi luoi khong phan anh dung khoang cach thuc."
locale: vi
track: thiet-ke-he-thong
module: thoi-gian-thuc-va-vi-tri
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.geohash-ma-hoa]
requires: [sd.gop-thong-bao-trung]
concepts: [sd.geohash-ma-hoa]
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
Notification xong. Mảnh thứ ba — Proximity — đổi hẳn câu hỏi: không phải
"AI online" hay "gửi kênh nào", mà LÀ "ai đang Ở gần TÔI?". Dò từng cặp
toạ độ MỘT là quá chậm — cần một cách gom các điểm GẦN nhau LẠI trước.
::::

::::explain{#ma-hoa-thanh-chuoi-o-luoi}
Thay vì so khoảng cách THẬT giữa từng cặp điểm (chậm khi có hàng triệu
điểm), ta CHIA mặt phẳng lat/lng thành lưới Ô vuông, RỒI mã hoá mỗi toạ
độ thành CHỈ SỐ của ô chứa nó. Dùng HAI mức độ phân giải — ô THÔ (cạnh
`10`) VÀ ô MỊN (cạnh `1`) — VÀ ghép thành một chuỗi: hai điểm càng gần
nhau càng có nhiều phần chuỗi TRÙNG nhau:

```typescript title=readonly
const KICH_THUOC_O_THO = 10;
const KICH_THUOC_O_MIN = 1;

function chiSoO(toaDo: number, kichThuocO: number): number {
  return Math.floor(toaDo / kichThuocO);
}

function maHoaToaDo(lat: number, lng: number): string {
  const oTho = `${chiSoO(lat, KICH_THUOC_O_THO)},${chiSoO(lng, KICH_THUOC_O_THO)}`;
  const oMin = `${chiSoO(lat, KICH_THUOC_O_MIN)},${chiSoO(lng, KICH_THUOC_O_MIN)}`;
  return `${oTho}|${oMin}`;
}

function oThoCuaMa(ma: string): string {
  return ma.split("|")[0]!;
}

const diemA = maHoaToaDo(12.3, 45.6);
const diemB = maHoaToaDo(13.7, 44.2);

console.log("ma cua A (lat=12.3, lng=45.6):", diemA);
console.log("ma cua B (lat=13.7, lng=44.2):", diemB);
console.log("A va B cung o THO (tien to giong nhau) khong:", oThoCuaMa(diemA) === oThoCuaMa(diemB));
```

```text title=readonly
ma cua A (lat=12.3, lng=45.6): 1,4|12,45
ma cua B (lat=13.7, lng=44.2): 1,4|13,44
A va B cung o THO (tien to giong nhau) khong: true
```

`chiSoO` chỉ LÀ `Math.floor(toaDo / kichThuocO)` — chia toạ độ cho kích
thước ô rồi làm TRÒN xuống. A VÀ B cách nhau hơn `1` đơn vị lat NÊN mã ô
MỊN của chúng khác nhau (`"12,45"` với `"13,44"`) — NHƯNG cả hai vẫn nằm
TRONG cùng một ô THÔ cạnh `10` (`"1,4"`), nên tiền tố trước dấu `|` khớp
NHAU. Đây chính LÀ tính chất cốt lõi: điểm gần nhau CHIA sẻ tiền tố.
::::

::::example{#diem-xa-khac-tien-to-diem-gan-trung-ma}
Một điểm THẬT xa có mã ô THÔ khác hẳn — không chia sẻ tiền tố nào. Một
điểm RẤT gần (trong cùng cả hai mức độ phân giải) có thể ra ĐÚNG cùng
một chuỗi mã, dù toạ độ thập phân không hề trùng khớp:

```typescript title=readonly
const KICH_THUOC_O_THO = 10;
const KICH_THUOC_O_MIN = 1;

function chiSoO(toaDo: number, kichThuocO: number): number {
  return Math.floor(toaDo / kichThuocO);
}

function maHoaToaDo(lat: number, lng: number): string {
  const oTho = `${chiSoO(lat, KICH_THUOC_O_THO)},${chiSoO(lng, KICH_THUOC_O_THO)}`;
  const oMin = `${chiSoO(lat, KICH_THUOC_O_MIN)},${chiSoO(lng, KICH_THUOC_O_MIN)}`;
  return `${oTho}|${oMin}`;
}

function oThoCuaMa(ma: string): string {
  return ma.split("|")[0]!;
}

const diemA = maHoaToaDo(12.3, 45.6);
const diemC = maHoaToaDo(-50, -100);
const diemD = maHoaToaDo(12.35, 45.65);

console.log("ma cua C (lat=-50, lng=-100, rat xa A):", diemC);
console.log("C va A cung o THO khong:", oThoCuaMa(diemC) === oThoCuaMa(diemA));
console.log("ma cua D (lat=12.35, lng=45.65, rat gan A):", diemD);
console.log("ma cua D co GIONG HET ma cua A khong:", diemD === diemA);
```

```text title=readonly
ma cua C (lat=-50, lng=-100, rat xa A): -5,-10|-50,-100
C va A cung o THO khong: false
ma cua D (lat=12.35, lng=45.65, rat gan A): 1,4|12,45
ma cua D co GIONG HET ma cua A khong: true
```

`C` cách xa `A` hàng chục đơn vị NÊN cả `oTho` LẪN `oMin` đều khác — mã
hai điểm không chia sẻ TIỀN tố nào cả. `D` chỉ lệch `A` `0.05` đơn vị —
`chiSoO` làm TRÒN xuống nên cả hai vẫn rơi vào ĐÚNG một ô mịn, VÀ mã của
`D` trùng KHỚP hoàn toàn với mã của `A`.
::::

::::predict{#doan-bien-luoi-diem-gan-van-khac-o commitOnce}
Hai điểm CÓ cùng `lng=50`, chỉ khác `lat`: điểm `X` Ở `lat=9.9`, điểm `Y`
Ở `lat=10.1`. Khoảng cách thực Giữa hai điểm CHỈ LÀ `0.2` đơn vị — rất
gần. `X` VÀ `Y` có CÙNG mã ô THÔ không?

:::opt{correct}
Không — `chiSoO(9.9, 10) = 0` NHƯNG `chiSoO(10.1, 10) = 1`; hai điểm nằm
NGAY hai bên của một đường biên lưới (mốc `10`), nên dù khoảng cách thực
rất nhỏ, chỉ số ô THÔ của chúng vẫn khác nhau HOÀN toàn
:::
:::opt
Có — khoảng cách thực giữa hai điểm chỉ LÀ `0.2`, quá nhỏ để rơi vào hai
ô KHÁC nhau
::why
Nhầm "khoảng cách thực nhỏ" VỚI "cùng một ô lưới" — nhưng lưới chia theo
MỐC cố định (bội số của `kichThuocO`), không hề quan tâm khoảng cách
tương đối giữa hai điểm cụ thể.

Chỗ lệch: `chiSoO` dùng `Math.floor(toaDo / kichThuocO)` — `9.9 / 10 =
0.99`, làm tròn xuống LÀ `0`; còn `10.1 / 10 = 1.01`, làm tròn xuống LÀ
`1`. Mốc `10` nằm NGAY GIỮA `X` và `Y`, nên dù chúng gần nhau trong thực
tế, lưới vẫn xếp chúng vào hai ô khác nhau — đây chính LÀ "lỗi biên"
khiến bài SAU (tìm bạn gần) phải quét CẢ ô lân cận, không chỉ ô của
chính điểm truy vấn.
::
:::
::::

::::code{#viet_ma_hoa_toa_do}
Hoàn thiện `maHoaToaDo` — dòng tính `oTho` đã có sẵn; viết dòng tính
`oMin` theo ĐÚNG khuôn mẫu đó, chỉ đổi `KICH_THUOC_O_THO` thành
`KICH_THUOC_O_MIN`.

```typescript title=starter
const KICH_THUOC_O_THO = 10;
const KICH_THUOC_O_MIN = 1;

function chiSoO(toaDo: number, kichThuocO: number): number {
  return Math.floor(toaDo / kichThuocO);
}

function maHoaToaDo(lat: number, lng: number): string {
  const oTho = `${chiSoO(lat, KICH_THUOC_O_THO)},${chiSoO(lng, KICH_THUOC_O_THO)}`;
  ___
  return `${oTho}|${oMin}`;
}

console.log(maHoaToaDo(3.2, 7.8));
```

```typescript title=solution
const KICH_THUOC_O_THO = 10;
const KICH_THUOC_O_MIN = 1;

function chiSoO(toaDo: number, kichThuocO: number): number {
  return Math.floor(toaDo / kichThuocO);
}

function maHoaToaDo(lat: number, lng: number): string {
  const oTho = `${chiSoO(lat, KICH_THUOC_O_THO)},${chiSoO(lng, KICH_THUOC_O_THO)}`;
  const oMin = `${chiSoO(lat, KICH_THUOC_O_MIN)},${chiSoO(lng, KICH_THUOC_O_MIN)}`;
  return `${oTho}|${oMin}`;
}

console.log(maHoaToaDo(3.2, 7.8));
```

```typescript title=test
if (maHoaToaDo(12.3, 45.6) !== "1,4|12,45") throw new Error("ma cua (12.3, 45.6) phai la '1,4|12,45'");
if (maHoaToaDo(13.7, 44.2) !== "1,4|13,44") throw new Error("ma cua (13.7, 44.2) phai la '1,4|13,44'");
if (maHoaToaDo(-50, -100) !== "-5,-10|-50,-100") throw new Error("toa do am phai lam tron XUONG (Math.floor), '-5,-10|-50,-100'");
if (maHoaToaDo(0, 0) !== "0,0|0,0") throw new Error("goc toa do (0,0) phai ma hoa thanh '0,0|0,0'");
if (maHoaToaDo(9.9, 50) !== "0,5|9,50") throw new Error("lat=9.9 phai o o THO idx 0 (chua toi 10)");
if (maHoaToaDo(10.1, 50) !== "1,5|10,50") throw new Error("lat=10.1 phai o o THO idx 1 (da vuot 10), khac han voi lat=9.9");
```

:::hints
- kind: attention
  body: "Dong con thieu phai khai bao oMin, cung cau truc voi dong oTho ngay ben tren nhung dung KICH_THUOC_O_MIN thay vi KICH_THUOC_O_THO."
- kind: strategy
  body: "oMin la mot chuoi ghep chiSoO(lat, KICH_THUOC_O_MIN) va chiSoO(lng, KICH_THUOC_O_MIN), cach nhau boi dau phay, dung template string giong het dong tren."
- kind: one-line
  body: "const oMin = `${chiSoO(lat, KICH_THUOC_O_MIN)},${chiSoO(lng, KICH_THUOC_O_MIN)}`;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 5000
- tier: output
  match: contains
  expect: "0,0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mã ô cho biết điểm nào GẦN điểm nào — trừ đúng cái biên vừa thấy. Mảnh
kế tiếp xử lý đúng chỗ hổng đó: quét không chỉ MỘT ô, mà cả các ô lân
cận.
::::

::::reflect{#nghi-lai}
`maHoaToaDo` chỉ LÀ hai lần `Math.floor` VÀ một phép ghép chuỗi — nhưng
điều đáng nhớ LÀ tính chất nó tạo ra: điểm gần nhau THƯỜNG chia sẻ tiền
tố, giúp gom nhóm nhanh mà không cần so khoảng cách từng cặp. "THƯỜNG"
chứ không phải "LUÔN" — đường biên lưới LÀ ngoại lệ cần nhớ.
::::

::::checkpoint{mastery=0.76}
::::
