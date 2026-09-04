---
id: thiet-ke-he-thong.ha-tang-du-lieu-quy-mo-lon.metrics-canh-bao-co-do-tre
title: "Cảnh báo có độ trễ: một lần vượt ngưỡng chưa đủ"
summary: "xuLyKhungMoi(tt, trungBinh) chi kich hoat dangCanhBao KHI trungBinh VUOT NGUONG_CANH_BAO=80 trong DUNG SO_KHUNG_LIEN_TIEP=3 khung LIEN TIEP -- mot khung tut xuong duoi nguong RESET bo dem ve 0 ngay lap tuc. Vi du that: 90, 95 (2 lan lien tiep, chua canh bao) roi 60 (tut xuong, reset) -- neu lien tuc tang lai 85, 88, 92 thi DUNG lan thu 3 moi kich hoat canh bao=true."
locale: vi
track: thiet-ke-he-thong
module: ha-tang-du-lieu-quy-mo-lon
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.metrics-canh-bao-co-do-tre]
requires: [sd.metrics-gom-theo-khung-thoi-gian]
concepts: [sd.metrics-canh-bao-co-do-tre]
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
Metric giờ đã được gom gọn theo khung. Nhưng nếu cảnh báo bắn NGAY khi
một khung vượt ngưỡng, một đợt tăng đột biến vài giây sẽ đánh THỨC cả
đội trực lúc `3` giờ sáng — VÌ một con số bất thường, không phải vì hệ
thống thật sự có VẤN đề.
::::

::::explain{#canh-bao-do-tre}
`xuLyKhungMoi` không bắn cảnh báo NGAY khi một khung vượt ngưỡng — nó
đếm số khung VƯỢT ngưỡng LIÊN TIẾP, VÀ chỉ đặt `dangCanhBao = true` khi
bộ đếm đó đạt `SO_KHUNG_LIEN_TIEP`. Một khung tụt xuống dưới ngưỡng
RESET bộ đếm về `0` ngay lập tức — dù trước đó đã gần chạm mốc cỡ nào:

```typescript title=readonly
const NGUONG_CANH_BAO = 80;
const SO_KHUNG_LIEN_TIEP = 3;
interface TrangThaiCanhBao { soLienTiepVuotNguong: number; dangCanhBao: boolean; }
function taoTrangThaiCanhBao(): TrangThaiCanhBao { return { soLienTiepVuotNguong: 0, dangCanhBao: false }; }
function xuLyKhungMoi(tt: TrangThaiCanhBao, trungBinh: number): boolean {
  if (trungBinh > NGUONG_CANH_BAO) {
    tt.soLienTiepVuotNguong += 1;
  } else {
    tt.soLienTiepVuotNguong = 0;
  }
  tt.dangCanhBao = tt.soLienTiepVuotNguong >= SO_KHUNG_LIEN_TIEP;
  return tt.dangCanhBao;
}

const tt = taoTrangThaiCanhBao();
console.log("khung 1, tb=90:", xuLyKhungMoi(tt, 90), "so lien tiep:", tt.soLienTiepVuotNguong);
console.log("khung 2, tb=95:", xuLyKhungMoi(tt, 95), "so lien tiep:", tt.soLienTiepVuotNguong);
console.log("khung 3, tb=60 (tut xuong duoi nguong):", xuLyKhungMoi(tt, 60), "so lien tiep:", tt.soLienTiepVuotNguong);
```

```text title=readonly
khung 1, tb=90: false so lien tiep: 1
khung 2, tb=95: false so lien tiep: 2
khung 3, tb=60 (tut xuong duoi nguong): false so lien tiep: 0
```

Hai khung LIÊN tiếp đã vượt ngưỡng (`90`, `95`) — CHỈ còn cách kích
hoạt đúng MỘT khung nữa. Nhưng khung THỨ ba (`60`) tụt xuống dưới
`NGUONG_CANH_BAO`, VÀ bộ đếm bị đưa VỀ `0` ngay lập tức — công sức của
hai khung trước đó KHÔNG hề được "giữ lại" một phần NÀO.
::::

::::example{#kich-hoat-dung-lan-thu-ba}
Chuỗi liên tiếp phải bắt đầu LẠI từ đầu SAU lần reset — cảnh báo chỉ
bật lên đúng Ở khung thứ BA liên tiếp KỂ TỪ lúc bắt đầu đếm lại:

```typescript title=readonly
const NGUONG_CANH_BAO = 80;
const SO_KHUNG_LIEN_TIEP = 3;
interface TrangThaiCanhBao { soLienTiepVuotNguong: number; dangCanhBao: boolean; }
function taoTrangThaiCanhBao(): TrangThaiCanhBao { return { soLienTiepVuotNguong: 0, dangCanhBao: false }; }
function xuLyKhungMoi(tt: TrangThaiCanhBao, trungBinh: number): boolean {
  if (trungBinh > NGUONG_CANH_BAO) {
    tt.soLienTiepVuotNguong += 1;
  } else {
    tt.soLienTiepVuotNguong = 0;
  }
  tt.dangCanhBao = tt.soLienTiepVuotNguong >= SO_KHUNG_LIEN_TIEP;
  return tt.dangCanhBao;
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: khung 1,2 vuot nguong
// (90, 95), khung 3 tut xuong (60) da RESET bo dem ve 0
const tt = taoTrangThaiCanhBao();
xuLyKhungMoi(tt, 90);
xuLyKhungMoi(tt, 95);
xuLyKhungMoi(tt, 60);

console.log("khung 4, tb=85:", xuLyKhungMoi(tt, 85), "so lien tiep:", tt.soLienTiepVuotNguong);
console.log("khung 5, tb=88:", xuLyKhungMoi(tt, 88), "so lien tiep:", tt.soLienTiepVuotNguong);
console.log("khung 6, tb=92:", xuLyKhungMoi(tt, 92), "so lien tiep:", tt.soLienTiepVuotNguong);
```

```text title=readonly
khung 4, tb=85: false so lien tiep: 1
khung 5, tb=88: false so lien tiep: 2
khung 6, tb=92: true so lien tiep: 3
```

`85`, `88`, `92` LÀ ba khung liên tiếp vượt ngưỡng KỂ TỪ sau lần reset
— khung thứ `6` (thứ ba trong chuỗi mới) LÀ khung đầu tiên trả về
`true`. Hai khung trước đó Ở đợt tăng ĐẦU (khung `1`, `2`) không hề
được cộng dồn CHUNG với đợt này.
::::

::::predict{#doan-bang-nguong-khong-tinh commitOnce}
Bộ đếm đang Ở `0` (mới reset). Ba khung liên tiếp có trung bình LẦN
lượt LÀ `81`, `82`, RỒI ĐÚNG `80` (bằng CHÍNH `NGUONG_CANH_BAO`, không
vượt qua). Ngay SAU khung thứ ba, `dangCanhBao` LÀ gì?

:::opt{correct}
`false` — điều kiện dùng phép so sánh `>` (vượt HẲN), NÊN `80 >
80` LÀ sai; giá trị bằng đúng ngưỡng đi VÀO nhánh `else` VÀ bộ đếm bị
reset về `0` trước khi kịp đạt `3`
:::
:::opt
`true` — `80` đã CHẠM tới ngưỡng cảnh báo RỒI, cộng thêm hai khung
`81`, `82` trước đó LÀ đủ `3` lần liên tiếp "Ở mức đáng lo"
::why
Nhầm "chạm ngưỡng" VỚI "vượt ngưỡng" — nhưng điều kiện `if (trungBinh
> NGUONG_CANH_BAO)` chỉ đúng khi giá trị LỚN hơn ngưỡng THỰC sự, không
tính trường hợp bằng nhau.

Chỗ lệch: `NGUONG_CANH_BAO = 80` VÀ khung thứ ba có `trungBinh = 80`
— biểu thức `80 > 80` cho kết quả `false`, hàm rơi VÀO nhánh `else`
(`tt.soLienTiepVuotNguong = 0`). Chuỗi hai lần vượt ngưỡng trước đó
(`81`, `82`) bị xoá SẠCH ngay tại khung thứ ba, VÀ `dangCanhBao` vẫn LÀ
`false`.
::
:::
::::

::::code{#viet_xu_ly_khung_moi}
Hoàn thiện `xuLyKhungMoi` — nhánh đếm/reset đã có sẵn. Còn thiếu: cập
nhật `dangCanhBao` (đạt hay chưa đạt `SO_KHUNG_LIEN_TIEP`) VÀ trả về
đúng giá trị đó.

```typescript title=starter
const NGUONG_CANH_BAO = 80;
const SO_KHUNG_LIEN_TIEP = 3;
interface TrangThaiCanhBao { soLienTiepVuotNguong: number; dangCanhBao: boolean; }
function taoTrangThaiCanhBao(): TrangThaiCanhBao { return { soLienTiepVuotNguong: 0, dangCanhBao: false }; }
function xuLyKhungMoi(tt: TrangThaiCanhBao, trungBinh: number): boolean {
  if (trungBinh > NGUONG_CANH_BAO) {
    tt.soLienTiepVuotNguong += 1;
  } else {
    tt.soLienTiepVuotNguong = 0;
  }
  ___
}

const ttX = taoTrangThaiCanhBao();
xuLyKhungMoi(ttX, 90);
xuLyKhungMoi(ttX, 95);
console.log(xuLyKhungMoi(ttX, 99));
```

```typescript title=solution
const NGUONG_CANH_BAO = 80;
const SO_KHUNG_LIEN_TIEP = 3;
interface TrangThaiCanhBao { soLienTiepVuotNguong: number; dangCanhBao: boolean; }
function taoTrangThaiCanhBao(): TrangThaiCanhBao { return { soLienTiepVuotNguong: 0, dangCanhBao: false }; }
function xuLyKhungMoi(tt: TrangThaiCanhBao, trungBinh: number): boolean {
  if (trungBinh > NGUONG_CANH_BAO) {
    tt.soLienTiepVuotNguong += 1;
  } else {
    tt.soLienTiepVuotNguong = 0;
  }
  tt.dangCanhBao = tt.soLienTiepVuotNguong >= SO_KHUNG_LIEN_TIEP;
  return tt.dangCanhBao;
}

const ttX = taoTrangThaiCanhBao();
xuLyKhungMoi(ttX, 90);
xuLyKhungMoi(ttX, 95);
console.log(xuLyKhungMoi(ttX, 99));
```

```typescript title=test
const ttT = taoTrangThaiCanhBao();
if (xuLyKhungMoi(ttT, 90) !== false) throw new Error("khung dau tien vuot nguong, chi 1 lan lien tiep, chua canh bao");
if (xuLyKhungMoi(ttT, 95) !== false) throw new Error("khung thu hai vuot nguong, 2 lan lien tiep, chua du 3");
if (xuLyKhungMoi(ttT, 60) !== false) throw new Error("khung thu ba tut duoi nguong, phai reset, khong canh bao");
if (ttT.soLienTiepVuotNguong !== 0) throw new Error("sau khi tut duoi nguong, bo dem lien tiep phai ve 0");

if (xuLyKhungMoi(ttT, 85) !== false) throw new Error("lai bat dau vuot nguong, moi 1 lan lien tiep");
if (xuLyKhungMoi(ttT, 88) !== false) throw new Error("2 lan lien tiep, chua du 3");
if (xuLyKhungMoi(ttT, 92) !== true) throw new Error("dung lan thu 3 lien tiep vuot nguong phai kich hoat canh bao");

if (xuLyKhungMoi(ttT, NGUONG_CANH_BAO) !== false) throw new Error("gia tri DUNG BANG nguong (khong VUOT) phai reset ve khong canh bao");
if (ttT.soLienTiepVuotNguong !== 0) throw new Error("gia tri bang nguong khong tinh la vuot, bo dem phai ve 0");
```

:::hints
- kind: attention
  body: "Con thieu hai buoc cuoi: cap nhat tt.dangCanhBao bang phep so sanh soLienTiepVuotNguong >= SO_KHUNG_LIEN_TIEP, roi return dung gia tri do."
- kind: strategy
  body: "tt.dangCanhBao = tt.soLienTiepVuotNguong >= SO_KHUNG_LIEN_TIEP; return tt.dangCanhBao;"
- kind: one-line
  body: "tt.dangCanhBao = tt.soLienTiepVuotNguong >= SO_KHUNG_LIEN_TIEP; return tt.dangCanhBao;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cảnh báo giờ chờ đủ bằng chứng trước khi bấm chuông. Mảnh HẠ tầng tiếp
theo bỏ hẳn con số Ở lại, chuyển sang giữ FILE — bắt đầu bằng cách chia
nhỏ chúng RA.
::::

::::reflect{#nghi-lai}
`xuLyKhungMoi` không hề LÀM phức tạp phép so sánh ngưỡng — nó chỉ THÊM
một bộ nhớ NGẮN hạn (bộ đếm liên tiếp) giữa phép so sánh VÀ quyết định
cảnh báo. Chính bộ nhớ ngắn hạn đó LÀ thứ biến "một điểm dữ liệu bất
thường" thành "một xu hướng đáng LO", VÀ nó reset dứt khoát ngay khi xu
hướng đó bị GIÁN đoạn.
::::

::::checkpoint{mastery=0.72}
::::
