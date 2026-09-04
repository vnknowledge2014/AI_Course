---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.gom-theo-khung-thuan
title: "Windowing thuần: gom sự kiện theo khung thời gian bằng fold, không mutate Map"
summary: "themVaoKhung(cacKhung, sk, kichThuocKhungMs) tra ve MOT Map MOI moi lan goi, khong bao gio Map.set mutate khung cu tai cho -- doi lap truc tiep voi ghiDiemTho kieu OOP cua T7.2 (hienTai.tong += giaTri; bo.khung.set(idx, hienTai)); gomTheoKhung = cacSuKien.reduce(themVaoKhung, new Map()) gom 5 su kien thanh 2 khung (tong 100 va 100), mot bien tham chieu toi khung cu KHONG bi doi khi khung MOI duoc tao, khac han loi mutate tai cho."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.gom-theo-khung-thuan]
requires: [sd.fp.boss-crdt-va-hop-nhat]
concepts: [sd.fp.gom-theo-khung-thuan]
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
Track "Thiết kế hệ thống" vừa khép hai quest: nhật ký bất biến, rồi CRDT
hợp nhất đa bản sao. Quest THỨ BA — và CUỐI CÙNG của T7.3 — quay lại một
bài toán đã gặp Ở "Hạ tầng dữ liệu quy mô lớn": gom hàng nghìn điểm dữ
liệu thô thành từng khung thời gian, để không phải giữ NGUYÊN từng điểm
một. Nhưng lần này, lời giải sẽ không có một dòng `Map.set` mutate tại
chỗ nào cả.
::::

::::explain{#gom-theo-khung-bang-fold}
Ở "Hạ tầng dữ liệu quy mô lớn", `ghiDiemTho` gom điểm thô bằng cách
MUTATE trực tiếp: `hienTai.tong += giaTri; bo.khung.set(idx, hienTai)`.
Bất kỳ biến nào đang giữ tham chiếu tới `hienTai` cũ sẽ thấy giá trị
của nó đổi NGAY khi có điểm mới tới. `themVaoKhung` giải quyết ĐÚNG bài
toán đó theo lối khác hẳn: mỗi lần gộp một sự kiện, nó tạo một
`KhungGom` MỚI VÀ một `Map` MỚI (sao chép từ `Map` cũ bằng
`new Map(cacKhung)`), không bao giờ sửa khung đã tồn tại tại chỗ —
giống hệt tinh thần `apDung` Ở quest "Nhật ký bất biến và fold":

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}

// kieu OOP-mutation (T7.2 "Ha tang du lieu quy mo lon"): mutate Map VA object tai cho
function themVaoKhungKieuCu(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): void {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const hienTai = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  hienTai.tong += sk.soLuong;
  hienTai.soSuKien += 1;
  cacKhung.set(idx, hienTai);
}

// kieu THUAN (T7.3c): moi buoc tra ve MOT Map MOI, khong sua Map cu hay KhungGom cu
function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 };
  const banSaoMoi = new Map(cacKhung);
  banSaoMoi.set(idx, khungMoi);
  return banSaoMoi;
}

function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}

interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

const dh = taoDongHoMoPhong();
const cacSuKien: SuKien[] = [];
function ghiNhan(soLuong: number): void {
  cacSuKien.push({ id: "e" + (cacSuKien.length + 1), nguon: "edge-1", thoiDiem: dh.thoiGianHienTai, soLuong });
}
ghiNhan(10);
tienThoiGian(dh, 2000);
ghiNhan(20);
tienThoiGian(dh, 3000);
ghiNhan(30);
tienThoiGian(dh, 3000);
ghiNhan(40);
tienThoiGian(dh, 2000);
ghiNhan(100);

const banDoKieuCu = new Map<number, KhungGom>();
themVaoKhungKieuCu(banDoKieuCu, cacSuKien[0]!, 10000);
const thamChieuKhung0 = banDoKieuCu.get(0)!;
console.log("KIEU CU: khung 0 ngay sau su kien dau:", JSON.stringify(thamChieuKhung0));
themVaoKhungKieuCu(banDoKieuCu, cacSuKien[1]!, 10000);
console.log("KIEU CU: CUNG bien thamChieuKhung0 sau khi them su kien thu hai:", JSON.stringify(thamChieuKhung0));

let banDoThuan = themVaoKhung(new Map<number, KhungGom>(), cacSuKien[0]!, 10000);
const khung0Thuan = banDoThuan.get(0)!;
banDoThuan = themVaoKhung(banDoThuan, cacSuKien[1]!, 10000);
console.log("KIEU THUAN: bien khung0Thuan giu nguyen sau khi them su kien thu hai:", JSON.stringify(khung0Thuan));
console.log("KIEU THUAN: banDoThuan (Map MOI nhat) co khung 0 la:", JSON.stringify(banDoThuan.get(0)));

const ketQua = gomTheoKhung(cacSuKien, 10000);
console.log("gomTheoKhung 5 su kien, khung 0:", JSON.stringify(ketQua.get(0)));
console.log("gomTheoKhung 5 su kien, khung 1:", JSON.stringify(ketQua.get(1)));
console.log("so khung:", ketQua.size);
console.log("cacSuKien van con", cacSuKien.length, "phan tu (khong bi mutate)");
```

```text title=readonly
KIEU CU: khung 0 ngay sau su kien dau: {"chiSoKhung":0,"tong":10,"soSuKien":1}
KIEU CU: CUNG bien thamChieuKhung0 sau khi them su kien thu hai: {"chiSoKhung":0,"tong":30,"soSuKien":2}
KIEU THUAN: bien khung0Thuan giu nguyen sau khi them su kien thu hai: {"chiSoKhung":0,"tong":10,"soSuKien":1}
KIEU THUAN: banDoThuan (Map MOI nhat) co khung 0 la: {"chiSoKhung":0,"tong":30,"soSuKien":2}
gomTheoKhung 5 su kien, khung 0: {"chiSoKhung":0,"tong":100,"soSuKien":4}
gomTheoKhung 5 su kien, khung 1: {"chiSoKhung":1,"tong":100,"soSuKien":1}
so khung: 2
cacSuKien van con 5 phan tu (khong bi mutate)
```

`thamChieuKhung0` (kiểu CŨ) đổi giá trị NGAY khi sự kiện thứ hai tới —
đây chính là cái giá của mutate tại chỗ: bất kỳ ai đang giữ tham chiếu
tới khung `0` đều thấy dữ liệu đổi dưới chân mình. `khung0Thuan` (kiểu
THUẦN) giữ NGUYÊN `{tong:10, soSuKien:1}` — vì `themVaoKhung` không hề
sửa `Map` hay `KhungGom` đã trả về Ở bước trước, nó luôn tạo bản MỚI.
`gomTheoKhung` chỉ là `reduce` gọi `themVaoKhung` liên tiếp — CÙNG một
bài toán windowing đã gặp Ở "Hạ tầng dữ liệu quy mô lớn", nhưng giải
theo lối THUẦN: không `Map.set` mutate tại chỗ, mà fold ra một `Map`
MỚI mỗi bước.
::::

::::example{#goi-lai-nhieu-lan-khong-doi}
Vì `gomTheoKhung` là hàm thuần, gọi nó nhiều lần với ĐÚNG cùng dữ liệu
luôn cho ra cùng nội dung — VÀ một `Map` đã trả về Ở một lần gọi TRƯỚC
không hề bị ảnh hưởng bởi một lần gọi SAU trên dữ liệu khác:

```typescript title=readonly
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}
function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 };
  const banSaoMoi = new Map(cacKhung);
  banSaoMoi.set(idx, khungMoi);
  return banSaoMoi;
}
function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}
function raChuoi(m: Map<number, KhungGom>): string {
  return JSON.stringify(Array.from(m.entries()));
}

const goc: SuKien[] = [
  { id: "e1", nguon: "edge-1", thoiDiem: 1000, soLuong: 7 },
  { id: "e2", nguon: "edge-1", thoiDiem: 4000, soLuong: 3 },
  { id: "e3", nguon: "edge-2", thoiDiem: 12000, soLuong: 9 },
];

const lan1 = gomTheoKhung(goc, 10000);
console.log("lan goi 1:", raChuoi(lan1));
console.log("goc van con", goc.length, "phan tu sau lan goi 1");

const lan2 = gomTheoKhung(goc, 10000);
console.log("lan goi 2 (cung du lieu):", raChuoi(lan2));
console.log("hai lan goi cho CUNG noi dung?", raChuoi(lan1) === raChuoi(lan2));
console.log("hai lan goi co phai CUNG mot Map object khong?", (lan1 as unknown) === (lan2 as unknown));

const noiDungLan1TruocDo = raChuoi(lan1);
const gocMoRong: SuKien[] = [...goc, { id: "e4", nguon: "edge-2", thoiDiem: 15000, soLuong: 50 }];
const lan3 = gomTheoKhung(gocMoRong, 10000);
console.log("lan goi 3 (them 1 su kien vao du lieu MOI):", raChuoi(lan3));
console.log("lan1 (Map cua lan goi DAU) khong doi sau khi lan3 chay?", raChuoi(lan1) === noiDungLan1TruocDo);
```

```text title=readonly
lan goi 1: [[0,{"chiSoKhung":0,"tong":10,"soSuKien":2}],[1,{"chiSoKhung":1,"tong":9,"soSuKien":1}]]
goc van con 3 phan tu sau lan goi 1
lan goi 2 (cung du lieu): [[0,{"chiSoKhung":0,"tong":10,"soSuKien":2}],[1,{"chiSoKhung":1,"tong":9,"soSuKien":1}]]
hai lan goi cho CUNG noi dung? true
hai lan goi co phai CUNG mot Map object khong? false
lan goi 3 (them 1 su kien vao du lieu MOI): [[0,{"chiSoKhung":0,"tong":10,"soSuKien":2}],[1,{"chiSoKhung":1,"tong":59,"soSuKien":2}]]
lan1 (Map cua lan goi DAU) khong doi sau khi lan3 chay? true
```

Hai lần gọi `gomTheoKhung(goc, 10000)` cho ra nội dung GIỐNG hệt nhau
(`raChuoi(lan1) === raChuoi(lan2)` là `true`) nhưng là HAI object `Map`
khác nhau trong bộ nhớ (`false`). Thêm một sự kiện vào `gocMoRong` rồi
gọi lại `gomTheoKhung` tạo ra `lan3` — nhưng `lan1` (đã trả về TRƯỚC
đó) giữ nguyên nội dung, không hề bị `lan3` chạm tới.
::::

::::predict{#doan-tham-chieu-cu-sau-buoc-ba commitOnce}
Tiếp tục đúng luồng trong `explain`, sau bước gán `khung0Thuan =
banDoThuan.get(0)` (giá trị `{tong:10, soSuKien:1}`, dùng sự kiện đầu)
VÀ bước gán lại `banDoThuan = themVaoKhung(banDoThuan, cacSuKien[1],
10000)` (dùng sự kiện thứ hai, kết quả Ở `banDoThuan.get(0)` LÀ
`{tong:30, soSuKien:2}`). Gọi tiếp MỘT bước nữa: `banDoThuan =
themVaoKhung(banDoThuan, cacSuKien[2], 10000)`, dùng sự kiện THỨ BA
(`thoiDiem: 5000, soLuong: 30`, vẫn rơi vào khung `0`). Giá trị của
`khung0Thuan` SAU lệnh gọi thứ ba này là gì?

:::opt{correct}
Vẫn là `{"chiSoKhung":0,"tong":10,"soSuKien":1}` — không đổi;
`khung0Thuan` được gán MỘT lần duy nhất, ngay sau bước đầu tiên, và
`themVaoKhung` không bao giờ sửa một `KhungGom` đã trả về trước đó, nó
luôn tạo bản ghi MỚI cho `banDoThuan` MỚI
:::
:::opt
`{"chiSoKhung":0,"tong":60,"soSuKien":3}` — vì `khung0Thuan` tham
chiếu tới cùng entry Ở `banDoThuan.get(0)`, và `banDoThuan` vừa được
cập nhật thêm một lần nữa qua phép gán lại
::why
Nhầm "biến `banDoThuan` được gán lại nhiều lần" với "giá trị mà
`khung0Thuan` đang giữ cũng đổi theo" — nhưng `khung0Thuan` không hề
được gán LẠI Ở bất kỳ dòng nào sau lần gán đầu tiên; nó là một biến
`const` giữ tham chiếu tới đúng MỘT object đã tạo Ở bước đầu.

Chỗ lệch: `themVaoKhung` LUÔN kết thúc bằng `const banSaoMoi = new
Map(cacKhung); banSaoMoi.set(idx, khungMoi); return banSaoMoi;` — nó
không bao giờ gọi `.set` lên `Map` cũ (`cacKhung`), và `khungMoi` LUÔN
là một object VỪA tạo bằng `{ ..., tong: khungCu.tong + sk.soLuong,
... }`, không phải object đã tồn tại bị sửa field. Giá trị
`{"tong":60,"soSuKien":3}` đúng LÀ nội dung của `banDoThuan.get(0)`
SAU bước ba — nhưng đó là một entry hoàn toàn MỚI trong một `Map`
hoàn toàn MỚI, không phải cùng object mà `khung0Thuan` đang giữ.
::
:::
::::

::::code{#viet_gom_theo_khung}
Hoàn thiện `themVaoKhung` — tính `idx` bằng `chiSoKhung` (đã cho sẵn),
lấy khung hiện có tại `idx` trong `cacKhung` (hoặc một khung rỗng
`{chiSoKhung: idx, tong: 0, soSuKien: 0}` nếu chưa có), tạo một
`KhungGom` MỚI với `tong` VÀ `soSuKien` đã cộng thêm sự kiện `sk`, rồi
trả về một `Map` MỚI (sao chép từ `cacKhung` bằng `new Map(cacKhung)`)
có khung đó Ở đúng `idx`. KHÔNG được gọi `.set` lên `cacKhung` truyền
vào, VÀ KHÔNG được sửa field của khung cũ tại chỗ.

```typescript title=starter
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}

function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  ___
}

function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}

const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 100, soLuong: 4 },
  { id: "x2", nguon: "e", thoiDiem: 200, soLuong: 6 },
];
const ketQuaX = gomTheoKhung(dsX, 1000);
console.log(ketQuaX.get(0)?.tong, ketQuaX.size);
```

```typescript title=solution
interface SuKien { id: string; nguon: string; thoiDiem: number; soLuong: number; }
interface KhungGom { chiSoKhung: number; tong: number; soSuKien: number; }
function chiSoKhung(thoiDiem: number, kichThuocKhungMs: number): number {
  return Math.floor(thoiDiem / kichThuocKhungMs);
}

function themVaoKhung(cacKhung: Map<number, KhungGom>, sk: SuKien, kichThuocKhungMs: number): Map<number, KhungGom> {
  const idx = chiSoKhung(sk.thoiDiem, kichThuocKhungMs);
  const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 };
  const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 };
  const banSaoMoi = new Map(cacKhung);
  banSaoMoi.set(idx, khungMoi);
  return banSaoMoi;
}

function gomTheoKhung(cacSuKien: SuKien[], kichThuocKhungMs: number): Map<number, KhungGom> {
  return cacSuKien.reduce((khung, sk) => themVaoKhung(khung, sk, kichThuocKhungMs), new Map<number, KhungGom>());
}

const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 100, soLuong: 4 },
  { id: "x2", nguon: "e", thoiDiem: 200, soLuong: 6 },
];
const ketQuaX = gomTheoKhung(dsX, 1000);
console.log(ketQuaX.get(0)?.tong, ketQuaX.size);
```

```typescript title=test
const tKhungRong = new Map<number, KhungGom>();
const tSk1: SuKien = { id: "t1", nguon: "s", thoiDiem: 500, soLuong: 5 };
const tKhungSau1 = themVaoKhung(tKhungRong, tSk1, 1000);
if (tKhungRong.size !== 0) throw new Error("Map goc truyen vao KHONG duoc bi doi (immutable)");
if (tKhungSau1.size !== 1) throw new Error("Map moi phai co dung 1 khung");
const tKhung0 = tKhungSau1.get(0);
if (tKhung0 === undefined || tKhung0.tong !== 5 || tKhung0.soSuKien !== 1) throw new Error("khung 0 phai co tong=5, soSuKien=1");

const tSk2: SuKien = { id: "t2", nguon: "s", thoiDiem: 700, soLuong: 3 };
const tKhungSau2 = themVaoKhung(tKhungSau1, tSk2, 1000);
if (tKhungSau1.get(0)?.tong !== 5) throw new Error("Map CU (tKhungSau1) khong duoc bi doi boi buoc SAU");
const tKhung0Sau2 = tKhungSau2.get(0);
if (tKhung0Sau2 === undefined || tKhung0Sau2.tong !== 8 || tKhung0Sau2.soSuKien !== 2) throw new Error("khung 0 sau buoc 2 phai co tong=8, soSuKien=2");

const tSk3: SuKien = { id: "t3", nguon: "s", thoiDiem: 1500, soLuong: 9 };
const tKhungSau3 = themVaoKhung(tKhungSau2, tSk3, 1000);
if (tKhungSau3.size !== 2) throw new Error("su kien roi vao khung MOI phai tao them 1 khung, tong 2");
if (tKhungSau3.get(1)?.tong !== 9) throw new Error("khung 1 phai co tong=9");

const tDsLon: SuKien[] = [];
for (let i = 0; i < 20; i++) tDsLon.push({ id: "e" + i, nguon: "s", thoiDiem: i * 100, soLuong: 1 });
const tGomLon = gomTheoKhung(tDsLon, 1000);
if (tGomLon.size !== 2) throw new Error("20 su kien cach nhau 100ms, khung 1000ms, phai gom thanh 2 khung");
if (tGomLon.get(0)?.soSuKien !== 10) throw new Error("khung 0 phai co 10 su kien (t=0..900)");
if (tDsLon.length !== 20) throw new Error("mang goc KHONG duoc bi doi sau khi goi gomTheoKhung");
```

:::hints
- kind: attention
  body: "Lay khungCu bang cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 }. Tao khungMoi la mot OBJECT MOI voi tong: khungCu.tong + sk.soLuong va soSuKien: khungCu.soSuKien + 1. Tao banSaoMoi bang new Map(cacKhung), goi banSaoMoi.set(idx, khungMoi), roi return banSaoMoi -- TUYET DOI khong goi cacKhung.set(...) hay sua khungCu tai cho."
- kind: strategy
  body: "const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 }; const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 }; const banSaoMoi = new Map(cacKhung); banSaoMoi.set(idx, khungMoi); return banSaoMoi;"
- kind: one-line
  body: "const khungCu = cacKhung.get(idx) ?? { chiSoKhung: idx, tong: 0, soSuKien: 0 }; const khungMoi: KhungGom = { chiSoKhung: idx, tong: khungCu.tong + sk.soLuong, soSuKien: khungCu.soSuKien + 1 }; const banSaoMoi = new Map(cacKhung); banSaoMoi.set(idx, khungMoi); return banSaoMoi;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "10 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng bài toán gom-theo-khung, nhưng giờ không còn một dòng mutate nào —
mỗi bước fold ra một `Map` mới, mọi tham chiếu cũ đứng yên. Nhưng gom
sự kiện thành khung mới chỉ là nửa đầu của streaming — khi nào một
khung đã "đủ" để gửi đi?
::::

::::reflect{#nghi-lai}
`themVaoKhung` không hề phát minh phép toán MỚI — `chiSoKhung`, `tong`,
`soSuKien` giống hệt bài windowing kiểu OOP đã học. Khác biệt DUY nhất
nằm Ở nơi phép cộng dồn xảy ra: kiểu cũ cộng dồn VÀO một ô nhớ đã tồn
tại; kiểu thuần tạo một ô nhớ MỚI mang giá trị đã cộng dồn, để nguyên ô
cũ cho bất kỳ ai còn đang giữ tham chiếu tới nó. Đây chính là chi phí
VÀ lợi ích của lối viết thuần: tốn thêm một `new Map` mỗi bước, đổi lấy
việc không ai bị "giật mình" bởi dữ liệu đổi dưới chân.
::::

::::checkpoint{mastery=0.72}
::::
