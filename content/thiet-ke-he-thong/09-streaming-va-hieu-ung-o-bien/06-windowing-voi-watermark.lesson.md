---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.windowing-voi-watermark
title: "Windowing với watermark: sự kiện đến trễ hơn khung đã đóng thì sao?"
summary: "xuLyKhungCoWatermark(cacSuKien, kichThuocKhungMs, watermark) mo rong gomTheoKhung (bai 1): su kien co thoiDiem < watermark bi danh dau 'tre' (suKienTre), TACH RIENG khoi ket qua gom binh thuong -- khong lang le bo qua, khong lang le cong vao khung da 'dong'; van la ham THUAN, tra ve { ketQua, suKienTre } ro rang."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [sd.fp.windowing-voi-watermark]
requires: [sd.fp.effect-chi-o-bien]
concepts: [sd.fp.windowing-voi-watermark]
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
`gomTheoKhung` (bài 1) gom BẤT KỲ sự kiện nào vào ĐÚNG khung của
`thoiDiem` — kể cả một sự kiện có `thoiDiem` rất CŨ, tới rất MUỘN
(mạng chậm, edge server retry). Gộp lặng lẽ vào khung cũ nghĩa là một
khung "đã đóng, đã gửi đi" bỗng dưng đổi giá trị. Bỏ qua lặng lẽ nghĩa
là mất dữ liệu không dấu vết. Cả hai đều tệ — cần một lựa chọn RÕ RÀNG.
::::

::::explain{#watermark-tach-su-kien-tre}
`watermark` LÀ một mốc thời gian: "mọi khung TRƯỚC mốc này coi như đã
ĐÓNG". `xuLyKhungCoWatermark` mở rộng `themVaoKhung` (bài 1) — với mỗi
sự kiện, nếu `thoiDiem` của nó NHỎ HƠN `watermark`, nó bị xếp vào
`suKienTre` (một mảng RIÊNG, không hề bị bỏ qua); ngược lại, nó được
gom vào khung như bình thường. Kết quả trả về TÁCH bạch hai phần:

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

interface KetQuaWatermark { ketQua: Map<number, KhungGom>; suKienTre: SuKien[]; }
function xuLyKhungCoWatermark(cacSuKien: SuKien[], kichThuocKhungMs: number, watermark: number): KetQuaWatermark {
  return cacSuKien.reduce(
    (tich, sk) => {
      if (sk.thoiDiem < watermark) {
        return { ketQua: tich.ketQua, suKienTre: [...tich.suKienTre, sk] };
      }
      return { ketQua: themVaoKhung(tich.ketQua, sk, kichThuocKhungMs), suKienTre: tich.suKienTre };
    },
    { ketQua: new Map<number, KhungGom>(), suKienTre: [] as SuKien[] }
  );
}

const suKien = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong });
const cacSuKien: SuKien[] = [
  suKien("e1", 1000, 5),   // < watermark, TRE
  suKien("e2", 12000, 10), // >= watermark, khung 1
  suKien("e3", 5000, 7),   // < watermark, TRE
  suKien("e4", 15000, 20), // >= watermark, khung 1
  suKien("e5", 9999, 3),   // < watermark, TRE (van thuoc khung 0 ve mat thoi gian, nhung khung 0 DA DONG)
];

const ketQua = xuLyKhungCoWatermark(cacSuKien, 10000, 10000);
console.log("khung 0 (phai KHONG ton tai, vi watermark = 10000 da dong khung 0):", JSON.stringify(ketQua.ketQua.get(0)));
console.log("khung 1:", JSON.stringify(ketQua.ketQua.get(1)));
console.log("so khung trong ket qua:", ketQua.ketQua.size);
console.log("so su kien TRE:", ketQua.suKienTre.length);
console.log("id cac su kien tre:", JSON.stringify(ketQua.suKienTre.map((sk) => sk.id)));
```

```text title=readonly
khung 0 (phai KHONG ton tai, vi watermark = 10000 da dong khung 0): undefined
khung 1: {"chiSoKhung":1,"tong":30,"soSuKien":2}
so khung trong ket qua: 1
so su kien TRE: 3
id cac su kien tre: ["e1","e3","e5"]
```

`e5` có `thoiDiem: 9999` — về mặt TÍNH TOÁN, nó thuộc khung `0` y hệt
`e1` VÀ `e3`. Nhưng vì `watermark = 10000` đã tuyên bố "khung `0` đóng
rồi", `e5` KHÔNG được lặng lẽ cộng vào một khung đã đóng — nó bị đưa
vào `suKienTre`, RÕ RÀNG, để một lớp khác quyết định phải làm gì với
nó (bỏ, gộp bù, cảnh báo). `ketQua.ketQua` (các khung "sống") chỉ chứa
khung `1`, được gom từ `e2` VÀ `e4`.
::::

::::example{#doi-watermark-khong-sua-ham}
Vì `xuLyKhungCoWatermark` chỉ phụ thuộc vào tham số `watermark` truyền
vào, đổi giá trị đó thay đổi HOÀN TOÀN việc phân loại — mà không cần
sửa một dòng code nào của hàm:

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
interface KetQuaWatermark { ketQua: Map<number, KhungGom>; suKienTre: SuKien[]; }
function xuLyKhungCoWatermark(cacSuKien: SuKien[], kichThuocKhungMs: number, watermark: number): KetQuaWatermark {
  return cacSuKien.reduce(
    (tich, sk) => {
      if (sk.thoiDiem < watermark) {
        return { ketQua: tich.ketQua, suKienTre: [...tich.suKienTre, sk] };
      }
      return { ketQua: themVaoKhung(tich.ketQua, sk, kichThuocKhungMs), suKienTre: tich.suKienTre };
    },
    { ketQua: new Map<number, KhungGom>(), suKienTre: [] as SuKien[] }
  );
}

const suKien = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "edge-1", thoiDiem, soLuong });
const cacSuKien: SuKien[] = [
  suKien("e1", 1000, 5),
  suKien("e2", 12000, 10),
  suKien("e3", 5000, 7),
];

// cung mot loi, watermark KHAC nhau -- khong sua xuLyKhungCoWatermark
const khongTre = xuLyKhungCoWatermark(cacSuKien, 10000, 0);
console.log("watermark=0 (khong gi tre ca): so su kien tre =", khongTre.suKienTre.length, ", so khung =", khongTre.ketQua.size);

const tatCaTre = xuLyKhungCoWatermark(cacSuKien, 10000, 999999);
console.log("watermark rat lon (tat ca deu tre): so su kien tre =", tatCaTre.suKienTre.length, ", so khung =", tatCaTre.ketQua.size);

console.log("cacSuKien goc van con", cacSuKien.length, "phan tu (khong bi mutate)");
console.log("goi lai voi CUNG tham so cho CUNG ket qua?", JSON.stringify(Array.from(khongTre.ketQua.entries())) === JSON.stringify(Array.from(xuLyKhungCoWatermark(cacSuKien, 10000, 0).ketQua.entries())));
```

```text title=readonly
watermark=0 (khong gi tre ca): so su kien tre = 0 , so khung = 2
watermark rat lon (tat ca deu tre): so su kien tre = 3 , so khung = 0
cacSuKien goc van con 3 phan tu (khong bi mutate)
goi lai voi CUNG tham so cho CUNG ket qua? true
```

`watermark: 0` không loại BẤT KỲ sự kiện nào (mọi `thoiDiem` đều
`>= 0`) — kết quả giống hệt `gomTheoKhung` thường. `watermark: 999999`
khiến TẤT CẢ ba sự kiện bị coi LÀ trễ — `ketQua.ketQua` rỗng hoàn
toàn. Cùng một hàm, cùng một logic gom khung — chỉ đổi MỘT tham số đã
thay đổi hoàn toàn kết quả phân loại.
::::

::::predict{#doan-thoi-diem-dung-bang-watermark commitOnce}
Một sự kiện DUY nhất có `thoiDiem: 10000`. Gọi
`xuLyKhungCoWatermark([suKienDo], 1000, 10000)` (`watermark` cũng LÀ
`10000`, ĐÚNG bằng `thoiDiem` của sự kiện). Sự kiện này rơi vào
`suKienTre` hay được gom vào khung bình thường?

:::opt{correct}
Được gom vào khung BÌNH THƯỜNG (khung `10`, vì `chiSoKhung(10000,
1000) = 10`) — điều kiện đánh dấu trễ dùng `sk.thoiDiem < watermark`,
tức `10000 < 10000`, là `false`; chạm ĐÚNG mốc watermark KHÔNG bị coi
LÀ trễ
:::
:::opt
Rơi vào `suKienTre` — `watermark` đại diện cho "mốc mà mọi thứ TỪ đó
trở về trước đã đóng", nên một sự kiện ĐÚNG tại mốc đó cũng nên bị coi
LÀ thuộc về phần đã đóng
::why
Nhầm ranh giới `<` với `<=` — dòng code kiểm tra chính xác LÀ
`sk.thoiDiem < watermark`, không phải `sk.thoiDiem <= watermark`.

Chỗ lệch: `10000 < 10000` tính ra `false`, nên nhánh `suKienTre` KHÔNG
được kích hoạt — sự kiện rơi vào nhánh `themVaoKhung` bình thường. Đây
là lựa chọn THIẾT KẾ có chủ đích, không phải ngẫu nhiên: `watermark`
được định nghĩa LÀ "mọi khung mà đường biên TRÊN của nó nhỏ hơn hoặc
bằng watermark thì đã đóng" — một sự kiện đến ĐÚNG tại watermark vẫn
còn kịp, giống hệt cách `quyetDinhGomLo` (bài 2) VÀ `quyetDinhNhanSuKien`
(bài 3) đều coi việc CHẠM đúng một ngưỡng LÀ còn hợp lệ, không phải đã
vượt quá.
::
:::
::::

::::code{#viet_xu_ly_khung_co_watermark}
Hoàn thiện `xuLyKhungCoWatermark` — dùng `reduce` trên `cacSuKien`,
bắt đầu từ `{ ketQua: new Map(), suKienTre: [] }`. Với mỗi sự kiện: nếu
`sk.thoiDiem < watermark`, trả về tích luỹ MỚI với `sk` thêm vào
`suKienTre` (giữ nguyên `ketQua`); ngược lại, trả về tích luỹ MỚI với
`ketQua` được cập nhật bằng `themVaoKhung(tich.ketQua, sk,
kichThuocKhungMs)` (giữ nguyên `suKienTre`).

```typescript title=starter
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
interface KetQuaWatermark { ketQua: Map<number, KhungGom>; suKienTre: SuKien[]; }

function xuLyKhungCoWatermark(cacSuKien: SuKien[], kichThuocKhungMs: number, watermark: number): KetQuaWatermark {
  ___
}

const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 500, soLuong: 3 },
  { id: "x2", nguon: "e", thoiDiem: 2000, soLuong: 4 },
];
const kqX = xuLyKhungCoWatermark(dsX, 1000, 1000);
console.log(kqX.suKienTre.length, kqX.ketQua.size);
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
interface KetQuaWatermark { ketQua: Map<number, KhungGom>; suKienTre: SuKien[]; }

function xuLyKhungCoWatermark(cacSuKien: SuKien[], kichThuocKhungMs: number, watermark: number): KetQuaWatermark {
  return cacSuKien.reduce(
    (tich, sk) => {
      if (sk.thoiDiem < watermark) {
        return { ketQua: tich.ketQua, suKienTre: [...tich.suKienTre, sk] };
      }
      return { ketQua: themVaoKhung(tich.ketQua, sk, kichThuocKhungMs), suKienTre: tich.suKienTre };
    },
    { ketQua: new Map<number, KhungGom>(), suKienTre: [] as SuKien[] }
  );
}

const dsX: SuKien[] = [
  { id: "x1", nguon: "e", thoiDiem: 500, soLuong: 3 },
  { id: "x2", nguon: "e", thoiDiem: 2000, soLuong: 4 },
];
const kqX = xuLyKhungCoWatermark(dsX, 1000, 1000);
console.log(kqX.suKienTre.length, kqX.ketQua.size);
```

```typescript title=test
const tSk = (id: string, thoiDiem: number, soLuong: number): SuKien => ({ id, nguon: "s", thoiDiem, soLuong });

const tDs: SuKien[] = [tSk("a", 500, 10), tSk("b", 1500, 20), tSk("c", 999, 5)];
const tKq = xuLyKhungCoWatermark(tDs, 1000, 1000);
const tSoTre = tKq.suKienTre.length;
if (tSoTre !== 2) throw new Error("hai su kien co thoiDiem < 1000 (a: 500, c: 999) phai duoc danh dau TRE");
const tIdTre = tKq.suKienTre.map((sk) => sk.id).sort().join(",");
if (tIdTre !== "a,c") throw new Error("id cac su kien tre phai la a va c");
const tKhung1 = tKq.ketQua.get(1);
if (tKhung1 === undefined || tKhung1.tong !== 20 || tKhung1.soSuKien !== 1) throw new Error("khung 1 chi duoc chua su kien b (thoiDiem=1500)");
const tKhung0 = tKq.ketQua.get(0);
if (tKhung0 !== undefined) throw new Error("khung 0 KHONG duoc ton tai -- moi su kien thuoc khung 0 deu bi coi la tre");

const tDsBienGioi: SuKien[] = [tSk("d", 1000, 7)];
const tKqBienGioi = xuLyKhungCoWatermark(tDsBienGioi, 1000, 1000);
if (tKqBienGioi.suKienTre.length !== 0) throw new Error("thoiDiem DUNG BANG watermark KHONG duoc coi la tre");
if (tKqBienGioi.ketQua.get(1)?.tong !== 7) throw new Error("su kien dung watermark phai duoc gom vao khung 1 binh thuong");

const tDsGoc: SuKien[] = [tSk("e", 100, 1)];
const tChuoiTruoc = JSON.stringify(tDsGoc);
xuLyKhungCoWatermark(tDsGoc, 1000, 1000);
if (JSON.stringify(tDsGoc) !== tChuoiTruoc) throw new Error("xuLyKhungCoWatermark KHONG duoc mutate cacSuKien truyen vao");
```

:::hints
- kind: attention
  body: "Dung cacSuKien.reduce voi gia tri khoi tao { ketQua: new Map(), suKienTre: [] as SuKien[] }. Trong ham gop: neu sk.thoiDiem < watermark, tra ve { ketQua: tich.ketQua, suKienTre: [...tich.suKienTre, sk] }; nguoc lai tra ve { ketQua: themVaoKhung(tich.ketQua, sk, kichThuocKhungMs), suKienTre: tich.suKienTre }."
- kind: strategy
  body: "return cacSuKien.reduce((tich, sk) => { if (sk.thoiDiem < watermark) { return { ketQua: tich.ketQua, suKienTre: [...tich.suKienTre, sk] }; } return { ketQua: themVaoKhung(tich.ketQua, sk, kichThuocKhungMs), suKienTre: tich.suKienTre }; }, { ketQua: new Map<number, KhungGom>(), suKienTre: [] as SuKien[] });"
- kind: one-line
  body: "return cacSuKien.reduce((tich, sk) => sk.thoiDiem < watermark ? { ketQua: tich.ketQua, suKienTre: [...tich.suKienTre, sk] } : { ketQua: themVaoKhung(tich.ketQua, sk, kichThuocKhungMs), suKienTre: tich.suKienTre }, { ketQua: new Map<number, KhungGom>(), suKienTre: [] as SuKien[] });"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sự kiện trễ giờ được đánh dấu RÕ RÀNG, không lặng lẽ biến mất hay lặng
lẽ làm sai một khung đã đóng. Nhưng khi nhiều nguồn dữ liệu ĐỘC LẬP
cùng gom vào MỘT khung, làm sao hợp nhất chúng mà không đếm trùng?
::::

::::reflect{#nghi-lai}
`xuLyKhungCoWatermark` không hề "sửa" `themVaoKhung` — nó BỌC thêm một
lớp quyết định (trễ hay không trễ) TRƯỚC khi gọi lại đúng hàm gom
khung đã có. Đây là cách mở rộng một hàm thuần mà không viết lại nó:
thêm một điều kiện rẽ nhánh Ở TẦNG gọi, giữ nguyên hàm gốc, VÀ trả về
một cấu trúc kết quả rộng hơn (`{ ketQua, suKienTre }`) để không thứ
gì — kể cả một sự kiện bị từ chối xử lý — biến mất khỏi tầm nhìn của
người gọi.
::::

::::checkpoint{mastery=0.81}
::::
