---
id: thiet-ke-he-thong.giao-dich-va-tien.dat-phong-tranh-trung
title: "Đặt phòng tránh trùng: kiểm tra rồi mới đặt"
summary: "datPhong(ht, id, maPhong, khach, ngayNhan, ngayTra) kiem tra CHONG LAP khoang ngay truoc khi ghi -- mai dat P101 ngay 10-13 thanh cong, lan dat TRUNG het 10-13 bi tu choi, hoa dat 12-15 (chi chong lan MOT phan, khac ngay bat dau) cung bi tu choi, nam dat 13-16 (bat dau DUNG luc mai tra phong) lai thanh cong -- kiem tra la CHONG LAP khoang, khong phai trung ngay bat dau."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.dat-phong-tranh-trung]
requires: [sd.boss-ha-tang-du-lieu-quy-mo-lon]
concepts: [sd.dat-phong-tranh-trung]
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
Ba quest T7.2 trước đã ráp xong dịch vụ ĐỌC nội dung lớn, thời gian
thực, VÀ hạ tầng dữ liệu bên dưới chúng. Quest CUỐI của track — "Giao
dịch VÀ tiền" — rẽ sang một câu hỏi khác hẳn: khi một yêu cầu CHẠM vào
tiền, hoặc một chỗ CÓ giới hạn (một phòng khách sạn), đúng đắn quan
trọng HƠN tốc độ. Bắt đầu bằng câu hỏi đơn giản nhất: hai người CÙNG
muốn một phòng, cùng khoảng ngày — ai được giữ?
::::

::::explain{#kiem-tra-roi-dat}
`datPhong` không chỉ so khớp ngày BẮT đầu — nó kiểm tra khoảng ngày
MỚI có CHỒNG LẤN với bất kỳ đặt phòng nào ĐÃ có của cùng một phòng hay
không, TRƯỚC khi ghi thêm bất cứ gì. Hai khoảng `[ngayNhan, ngayTra)`
chồng lấn khi VÀ chỉ khi mỗi khoảng bắt đầu TRƯỚC lúc khoảng kia kết
thúc:

```typescript title=readonly
interface KhoangNgay { ngayNhan: number; ngayTra: number; }
interface DatPhong { id: string; maPhong: string; khach: string; khoang: KhoangNgay; }
interface HeThongDatPhong { dsDatPhong: DatPhong[]; }
function taoHeThongDatPhong(): HeThongDatPhong { return { dsDatPhong: [] }; }

function coChongLap(a: KhoangNgay, b: KhoangNgay): boolean {
  return a.ngayNhan < b.ngayTra && b.ngayNhan < a.ngayTra;
}

function timDatPhongChongLap(ht: HeThongDatPhong, maPhong: string, khoang: KhoangNgay): DatPhong | undefined {
  return ht.dsDatPhong.find((dp) => dp.maPhong === maPhong && coChongLap(dp.khoang, khoang));
}

type KetQuaDatPhong = "da_dat" | "phong_da_co_nguoi";
function datPhong(ht: HeThongDatPhong, id: string, maPhong: string, khach: string, ngayNhan: number, ngayTra: number): KetQuaDatPhong {
  const khoang: KhoangNgay = { ngayNhan, ngayTra };
  if (timDatPhongChongLap(ht, maPhong, khoang) !== undefined) return "phong_da_co_nguoi";
  ht.dsDatPhong.push({ id, maPhong, khach, khoang });
  return "da_dat";
}

const ht = taoHeThongDatPhong();
console.log("mai dat 10-13:", datPhong(ht, "dp-1", "P101", "mai", 10, 13));
console.log("lan dat TRUNG 10-13:", datPhong(ht, "dp-2", "P101", "lan", 10, 13));
console.log("hoa dat 12-15 (chong lan mot phan):", datPhong(ht, "dp-3", "P101", "hoa", 12, 15));
console.log("nam dat 13-16 (bat dau dung luc mai tra phong):", datPhong(ht, "dp-4", "P101", "nam", 13, 16));
console.log("so dat phong thanh cong:", ht.dsDatPhong.length);
```

```text title=readonly
mai dat 10-13: da_dat
lan dat TRUNG 10-13: phong_da_co_nguoi
hoa dat 12-15 (chong lan mot phan): phong_da_co_nguoi
nam dat 13-16 (bat dau dung luc mai tra phong): da_dat
so dat phong thanh cong: 2
```

`lan` bị từ chối vì trùng HẾT khoảng ngày của `mai`. `hoa` cũng bị từ
chối dù ngày BẮT đầu (`12`) khác hẳn ngày bắt đầu của `mai` (`10`) —
`coChongLap` không so ngày bắt đầu, nó so hai khoảng CÓ giao nhau hay
không. `nam` lại thành công dù bắt đầu ĐÚNG ngày `mai` kết thúc — nửa
khoảng mở `[10, 13)` nghĩa LÀ phòng trống KỂ từ ngày `13`.
::::

::::example{#phong-khac-khong-anh-huong}
Việc kiểm tra chồng lấn CHỈ so trong CÙNG một `maPhong` — hai phòng
khác nhau không hề biết gì VỀ nhau, dù đặt trùng ngày TUYỆT đối:

```typescript title=readonly
interface KhoangNgay { ngayNhan: number; ngayTra: number; }
interface DatPhong { id: string; maPhong: string; khach: string; khoang: KhoangNgay; }
interface HeThongDatPhong { dsDatPhong: DatPhong[]; }
function taoHeThongDatPhong(): HeThongDatPhong { return { dsDatPhong: [] }; }

function coChongLap(a: KhoangNgay, b: KhoangNgay): boolean {
  return a.ngayNhan < b.ngayTra && b.ngayNhan < a.ngayTra;
}

function timDatPhongChongLap(ht: HeThongDatPhong, maPhong: string, khoang: KhoangNgay): DatPhong | undefined {
  return ht.dsDatPhong.find((dp) => dp.maPhong === maPhong && coChongLap(dp.khoang, khoang));
}

type KetQuaDatPhong = "da_dat" | "phong_da_co_nguoi";
function datPhong(ht: HeThongDatPhong, id: string, maPhong: string, khach: string, ngayNhan: number, ngayTra: number): KetQuaDatPhong {
  const khoang: KhoangNgay = { ngayNhan, ngayTra };
  if (timDatPhongChongLap(ht, maPhong, khoang) !== undefined) return "phong_da_co_nguoi";
  ht.dsDatPhong.push({ id, maPhong, khach, khoang });
  return "da_dat";
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: mai da dat P101 10-13,
// nam da dat P101 13-16 (lan va hoa da bi tu choi, khong nam trong danh sach)
const ht = taoHeThongDatPhong();
datPhong(ht, "dp-1", "P101", "mai", 10, 13);
datPhong(ht, "dp-2", "P101", "lan", 10, 13);
datPhong(ht, "dp-3", "P101", "hoa", 12, 15);
datPhong(ht, "dp-4", "P101", "nam", 13, 16);

console.log("binh dat P102 10-13 (phong KHAC, trung ngay voi mai):", datPhong(ht, "dp-5", "P102", "binh", 10, 13));
console.log("chi dat P101 11-12 (nam TRON trong khoang cua mai):", datPhong(ht, "dp-6", "P101", "chi", 11, 12));
console.log("tong so dat phong thanh cong:", ht.dsDatPhong.length);
```

```text title=readonly
binh dat P102 10-13 (phong KHAC, trung ngay voi mai): da_dat
chi dat P101 11-12 (nam TRON trong khoang cua mai): phong_da_co_nguoi
tong so dat phong thanh cong: 3
```

`binh` đặt CÙNG khoảng ngày với `mai` nhưng Ở phòng `P102` — thành
công NGAY, vì `timDatPhongChongLap` chỉ tìm trong các đặt phòng CÓ
`maPhong` trùng khớp. `chi` bị từ chối dù khoảng `11-12` của cô ấy nằm
TRỌN vẹn bên trong khoảng `10-13` của `mai` — "nằm trong" cũng LÀ một
dạng chồng lấn.
::::

::::predict{#doan-cham-bien commitOnce}
Phòng `P101` hiện có `mai` giữ `10-13` VÀ `nam` giữ `13-16`. Một khách
MỚI, `"duc"`, đặt `P101` với khoảng `16-18` — bắt đầu ĐÚNG lúc `nam`
trả phòng (ngày `16`). Gọi `datPhong(ht, "dp-7", "P101", "duc", 16,
18)` — kết quả LÀ gì?

:::opt{correct}
`"da_dat"` — `coChongLap` dùng nửa khoảng mở, `nam.ngayTra` LÀ `16` VÀ
`duc.ngayNhan` cũng LÀ `16`; điều kiện `b.ngayNhan < a.ngayTra` LÀ
`16 < 16`, sai — không chồng lấn, phòng đã trống kể từ ngày `16`
:::
:::opt
`"phong_da_co_nguoi"` — hai đặt phòng CÙNG chạm vào ngày `16` (một kết
thúc, một bắt đầu), nên tính LÀ trùng ngày, phải bị từ chối
::why
Nhầm "hai khoảng CHẠM nhau tại một mốc ngày" VỚI "hai khoảng chồng
lấn" — nhưng `coChongLap` dùng phép so sánh NGHIÊM ngặt (`<`), không
phải `<=`.

Chỗ lệch: với `a = nam (13, 16)` VÀ `b = duc (16, 18)`, điều kiện thứ
hai LÀ `b.ngayNhan < a.ngayTra`, tức `16 < 16` — SAI. Vì một điều kiện
trong phép `&&` sai, `coChongLap` trả về `false` — không tìm thấy đặt
phòng chồng lấn NÀO, VÀ `datPhong` ghi thành công, trả về `"da_dat"`.
::
:::
::::

::::code{#viet_dat_phong}
Hoàn thiện `datPhong` — khoảng ngày MỚI đã được dựng (`khoang`). Còn
thiếu: tìm đặt phòng chồng lấn CÙNG phòng bằng `timDatPhongChongLap`;
nếu CÓ, trả về `"phong_da_co_nguoi"`; nếu KHÔNG, ghi đặt phòng mới vào
`ht.dsDatPhong` RỒI trả về `"da_dat"`.

```typescript title=starter
interface KhoangNgay { ngayNhan: number; ngayTra: number; }
interface DatPhong { id: string; maPhong: string; khach: string; khoang: KhoangNgay; }
interface HeThongDatPhong { dsDatPhong: DatPhong[]; }
function taoHeThongDatPhong(): HeThongDatPhong { return { dsDatPhong: [] }; }

function coChongLap(a: KhoangNgay, b: KhoangNgay): boolean {
  return a.ngayNhan < b.ngayTra && b.ngayNhan < a.ngayTra;
}

function timDatPhongChongLap(ht: HeThongDatPhong, maPhong: string, khoang: KhoangNgay): DatPhong | undefined {
  return ht.dsDatPhong.find((dp) => dp.maPhong === maPhong && coChongLap(dp.khoang, khoang));
}

type KetQuaDatPhong = "da_dat" | "phong_da_co_nguoi";
function datPhong(ht: HeThongDatPhong, id: string, maPhong: string, khach: string, ngayNhan: number, ngayTra: number): KetQuaDatPhong {
  const khoang: KhoangNgay = { ngayNhan, ngayTra };
  ___
}

const htX = taoHeThongDatPhong();
console.log(datPhong(htX, "x1", "P900", "duc", 5, 7), datPhong(htX, "x2", "P900", "hue", 6, 9));
```

```typescript title=solution
interface KhoangNgay { ngayNhan: number; ngayTra: number; }
interface DatPhong { id: string; maPhong: string; khach: string; khoang: KhoangNgay; }
interface HeThongDatPhong { dsDatPhong: DatPhong[]; }
function taoHeThongDatPhong(): HeThongDatPhong { return { dsDatPhong: [] }; }

function coChongLap(a: KhoangNgay, b: KhoangNgay): boolean {
  return a.ngayNhan < b.ngayTra && b.ngayNhan < a.ngayTra;
}

function timDatPhongChongLap(ht: HeThongDatPhong, maPhong: string, khoang: KhoangNgay): DatPhong | undefined {
  return ht.dsDatPhong.find((dp) => dp.maPhong === maPhong && coChongLap(dp.khoang, khoang));
}

type KetQuaDatPhong = "da_dat" | "phong_da_co_nguoi";
function datPhong(ht: HeThongDatPhong, id: string, maPhong: string, khach: string, ngayNhan: number, ngayTra: number): KetQuaDatPhong {
  const khoang: KhoangNgay = { ngayNhan, ngayTra };
  const trung = timDatPhongChongLap(ht, maPhong, khoang);
  if (trung !== undefined) return "phong_da_co_nguoi";
  ht.dsDatPhong.push({ id, maPhong, khach, khoang });
  return "da_dat";
}

const htX = taoHeThongDatPhong();
console.log(datPhong(htX, "x1", "P900", "duc", 5, 7), datPhong(htX, "x2", "P900", "hue", 6, 9));
```

```typescript title=test
const htT = taoHeThongDatPhong();
const r1 = datPhong(htT, "t1", "P200", "an", 1, 5);
if (r1 !== "da_dat") throw new Error("dat phong dau tien vao phong trong phai thanh cong");

const r2 = datPhong(htT, "t2", "P200", "binh", 1, 5);
if (r2 !== "phong_da_co_nguoi") throw new Error("trung HET khoang ngay voi dat phong da co phai bi tu choi");

const r3 = datPhong(htT, "t3", "P200", "chi", 3, 4);
if (r3 !== "phong_da_co_nguoi") throw new Error("khoang ngay nam TRON ben trong (3-4 trong 1-5) van la chong lap, phai bi tu choi");

const r4 = datPhong(htT, "t4", "P200", "dung", 5, 8);
if (r4 !== "da_dat") throw new Error("bat dau DUNG luc phong cu duoc tra (ngay 5) khong duoc tinh la chong lap");

const r5 = datPhong(htT, "t5", "P300", "an", 1, 5);
if (r5 !== "da_dat") throw new Error("phong KHAC nhau khong duoc anh huong lan nhau du trung ngay");

if (htT.dsDatPhong.length !== 3) throw new Error("chi 3 dat phong thanh cong duoc luu: t1, t4, t5");
```

:::hints
- kind: attention
  body: "Con thieu hai buoc trong than ham: goi timDatPhongChongLap(ht, maPhong, khoang); neu ket qua khac undefined thi return 'phong_da_co_nguoi'; nguoc lai push dat phong moi roi return 'da_dat'."
- kind: strategy
  body: "const trung = timDatPhongChongLap(ht, maPhong, khoang); if (trung !== undefined) return 'phong_da_co_nguoi'; ht.dsDatPhong.push({ id, maPhong, khach, khoang }); return 'da_dat';"
- kind: one-line
  body: "const trung = timDatPhongChongLap(ht, maPhong, khoang); if (trung !== undefined) return \"phong_da_co_nguoi\"; ht.dsDatPhong.push({ id, maPhong, khach, khoang }); return \"da_dat\";"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "phong_da_co_nguoi"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Chồng lấn khoảng ngày, không phải trùng ngày bắt đầu — nền móng ĐẦU
tiên của "đúng đắn quan trọng hơn tốc độ" đã xong. Nhưng đặt phòng còn
có thể bị HUỶ — huỷ một cái có làm hỏng những cái còn lại không?
::::

::::reflect{#nghi-lai}
`datPhong` không hề đơn giản hoá bài toán thành "trùng ngày bắt đầu" —
`coChongLap` so sánh HAI khoảng theo đúng ngữ nghĩa nửa khoảng mở
`[ngayNhan, ngayTra)`, nên một đặt phòng nằm TRỌN bên trong đặt phòng
khác vẫn bị bắt, VÀ một đặt phòng bắt đầu ĐÚNG lúc cái trước kết thúc
vẫn được chấp nhận. Việc kiểm tra CHỈ so trong cùng `maPhong` cũng LÀ
một quyết định có chủ đích — không gộp chung trạng thái của những
phòng không liên quan.
::::

::::checkpoint{mastery=0.66}
::::
