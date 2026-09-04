---
id: thiet-ke-he-thong.giao-dich-va-tien.dat-phong-huy-va-giu-cho
title: "Huỷ đặt phòng: giải phóng chỗ, không phá trạng thái khác"
summary: "huyDatPhong(ht, id) tim dat phong theo id ROI xoa khoi danh sach -- huy dp-1 (mai, P101 10-13) tra ve 'da_huy' VA giai phong lai khoang do cho lan dat THANH CONG; huy mot id KHONG ton tai ('dp-404') tra ve 'khong_tim_thay' VA khong lam thay doi so dat phong con lai (dat phong cua nam VAN con nguyen); huy CUNG mot id LAN THU HAI tra ve 'khong_tim_thay' chu KHONG phai 'da_huy' -- huy KHONG idempotent."
locale: vi
track: thiet-ke-he-thong
module: giao-dich-va-tien
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.dat-phong-huy-va-giu-cho]
requires: [sd.dat-phong-tranh-trung]
concepts: [sd.dat-phong-huy-va-giu-cho]
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
Đặt phòng chặn CHỖ đúng theo khoảng ngày — bài trước đã xong. Nhưng
khách CÓ thể đổi ý. Huỷ một đặt phòng phải giải phóng lại đúng khoảng
ngày ĐÓ cho người khác — VÀ nếu ai đó lỡ gửi một id sai (hoặc gửi lại
yêu cầu huỷ một cái ĐÃ huỷ rồi), hệ thống không được phép làm hỏng
những đặt phòng KHÁC đang yên ổn.
::::

::::explain{#huy-giai-phong-cho}
`huyDatPhong` tìm đặt phòng theo `id`, xoá NÓ khỏi danh sách nếu tìm
thấy. Khoảng ngày của đặt phòng bị xoá không còn nằm trong
`ht.dsDatPhong` NỮA — nên `datPhong` cho khoảng ngày đó, gọi SAU, sẽ
không còn thấy chồng lấn:

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

type KetQuaHuy = "da_huy" | "khong_tim_thay";
function huyDatPhong(ht: HeThongDatPhong, id: string): KetQuaHuy {
  const idx = ht.dsDatPhong.findIndex((dp) => dp.id === id);
  if (idx === -1) return "khong_tim_thay";
  ht.dsDatPhong.splice(idx, 1);
  return "da_huy";
}

const ht = taoHeThongDatPhong();
datPhong(ht, "dp-1", "P101", "mai", 10, 13);
datPhong(ht, "dp-4", "P101", "nam", 13, 16);

console.log("huy dp-1 (mai):", huyDatPhong(ht, "dp-1"));
console.log("lan dat LAI P101 10-13 (vua duoc giai phong):", datPhong(ht, "dp-8", "P101", "lan", 10, 13));
console.log("so dat phong con lai:", ht.dsDatPhong.length);
```

```text title=readonly
huy dp-1 (mai): da_huy
lan dat LAI P101 10-13 (vua duoc giai phong): da_dat
so dat phong con lai: 2
```

Sau khi `huyDatPhong` xoá đặt phòng của `mai`, khoảng `10-13` không
còn nằm trong `ht.dsDatPhong` — `lan` đặt LẠI đúng khoảng đó VÀ thành
công NGAY. Số đặt phòng còn lại LÀ `2` (`nam` VÀ `lan`), không phải
`3` — huỷ THẬT sự đã xoá bản ghi cũ, không chỉ đánh dấu.
::::

::::example{#huy-id-sai-khong-pha-trang-thai}
Một `id` KHÔNG tồn tại trong hệ thống — dù chưa từng được tạo, hay ĐÃ
bị huỷ trước đó — không được phép làm thay đổi bất kỳ đặt phòng nào
khác:

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

type KetQuaHuy = "da_huy" | "khong_tim_thay";
function huyDatPhong(ht: HeThongDatPhong, id: string): KetQuaHuy {
  const idx = ht.dsDatPhong.findIndex((dp) => dp.id === id);
  if (idx === -1) return "khong_tim_thay";
  ht.dsDatPhong.splice(idx, 1);
  return "da_huy";
}

// tai lap dung trang thai tu khoi truoc, KHONG in lai: dp-1 (mai) da bi huy,
// dp-4 (nam, P101 13-16) va dp-8 (lan, P101 10-13) dang con hieu luc
const ht = taoHeThongDatPhong();
datPhong(ht, "dp-1", "P101", "mai", 10, 13);
datPhong(ht, "dp-4", "P101", "nam", 13, 16);
huyDatPhong(ht, "dp-1");
datPhong(ht, "dp-8", "P101", "lan", 10, 13);

console.log("huy id SAI 'dp-404' (chua tung ton tai):", huyDatPhong(ht, "dp-404"));
console.log("so dat phong khong doi:", ht.dsDatPhong.length);
console.log("thu dat lai P101 13-16 (nam VAN con giu):", datPhong(ht, "dp-9", "P101", "duc", 13, 16));
```

```text title=readonly
huy id SAI 'dp-404' (chua tung ton tai): khong_tim_thay
so dat phong khong doi: 2
thu dat lai P101 13-16 (nam VAN con giu): phong_da_co_nguoi
```

`huyDatPhong(ht, "dp-404")` trả về `"khong_tim_thay"` VÀ số đặt phòng
GIỮ nguyên LÀ `2` — `findIndex` trả về `-1`, hàm thoát SỚM trước khi
chạm tới `splice`. Đặt phòng của `nam` (`P101 13-16`) hoàn toàn KHÔNG
bị ảnh hưởng — thử đặt lại đúng khoảng đó VẪN bị từ chối, chứng minh
`nam` VẪN đang giữ chỗ.
::::

::::predict{#doan-huy-lan-hai commitOnce}
Đặt phòng `dp-8` (`lan`, `P101 10-13`) VỪA được huỷ thành công. Gọi
`huyDatPhong(ht, "dp-8")` LẦN NỮA, NGAY sau đó, VỚI cùng hệ thống —
kết quả LÀ gì?

:::opt{correct}
`"khong_tim_thay"` — lần huỷ đầu ĐÃ xoá `dp-8` khỏi mảng bằng
`splice`; lần gọi thứ hai, `findIndex` không còn thấy `id` đó NỮA nên
trả về `-1`, hàm trả về `"khong_tim_thay"`
:::
:::opt
`"da_huy"` — huỷ một cái ĐÃ huỷ rồi vẫn nên được coi LÀ thành công, vì
kết quả cuối cùng (đặt phòng không còn tồn tại) LÀ giống nhau
::why
Nhầm "trạng thái CUỐI giống nhau" VỚI "kết quả trả VỀ giống nhau" —
nhưng `huyDatPhong` không hề nhớ nó ĐÃ từng xoá `id` này trước đó.

Chỗ lệch: `huyDatPhong` chỉ dựa vào `ht.dsDatPhong.findIndex(...)` TẠI
thời điểm gọi. Sau lần xoá ĐẦU tiên, `dp-8` không còn trong mảng —
lần gọi THỨ hai tìm thấy `-1` NGAY từ đầu VÀ trả về `"khong_tim_thay"`,
giống hệt như huỷ một `id` chưa từng tồn tại. `huyDatPhong` KHÔNG phải
một thao tác lặp lại vô hại (idempotent) theo nghĩa trả về CÙNG kết
quả mỗi lần.
::
:::
::::

::::code{#viet_huy_dat_phong}
Hoàn thiện `huyDatPhong` — tìm vị trí đặt phòng có `id` khớp bằng
`findIndex`. Nếu KHÔNG tìm thấy (`-1`), trả về `"khong_tim_thay"`
NGAY, không đụng vào mảng. Nếu tìm thấy, xoá đúng phần tử đó bằng
`splice` RỒI trả về `"da_huy"`.

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
  if (timDatPhongChongLap(ht, maPhong, khoang) !== undefined) return "phong_da_co_nguoi";
  ht.dsDatPhong.push({ id, maPhong, khach, khoang });
  return "da_dat";
}

type KetQuaHuy = "da_huy" | "khong_tim_thay";
function huyDatPhong(ht: HeThongDatPhong, id: string): KetQuaHuy {
  ___
}

const htX = taoHeThongDatPhong();
datPhong(htX, "y1", "P500", "an", 1, 3);
console.log(huyDatPhong(htX, "y1"), huyDatPhong(htX, "y1"));
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
  if (timDatPhongChongLap(ht, maPhong, khoang) !== undefined) return "phong_da_co_nguoi";
  ht.dsDatPhong.push({ id, maPhong, khach, khoang });
  return "da_dat";
}

type KetQuaHuy = "da_huy" | "khong_tim_thay";
function huyDatPhong(ht: HeThongDatPhong, id: string): KetQuaHuy {
  const idx = ht.dsDatPhong.findIndex((dp) => dp.id === id);
  if (idx === -1) return "khong_tim_thay";
  ht.dsDatPhong.splice(idx, 1);
  return "da_huy";
}

const htX = taoHeThongDatPhong();
datPhong(htX, "y1", "P500", "an", 1, 3);
console.log(huyDatPhong(htX, "y1"), huyDatPhong(htX, "y1"));
```

```typescript title=test
const htT = taoHeThongDatPhong();
datPhong(htT, "c1", "P400", "an", 1, 5);
datPhong(htT, "c2", "P400", "binh", 5, 9);
datPhong(htT, "c3", "P500", "chi", 1, 5);

const h1 = huyDatPhong(htT, "c1");
if (h1 !== "da_huy") throw new Error("huy dung id dang ton tai phai tra ve da_huy");
const soLuong1 = htT.dsDatPhong.length;
if (soLuong1 !== 2) throw new Error("sau khi huy, chi con 2 dat phong");

const h2 = huyDatPhong(htT, "khong-ton-tai");
if (h2 !== "khong_tim_thay") throw new Error("huy id KHONG ton tai phai tra ve khong_tim_thay");
const soLuong2 = htT.dsDatPhong.length;
if (soLuong2 !== 2) throw new Error("huy id sai KHONG duoc lam thay doi so dat phong con lai");

const r1 = datPhong(htT, "c4", "P400", "duc", 5, 9);
if (r1 !== "phong_da_co_nguoi") throw new Error("c2 (binh, P400 5-9) van con hieu luc, phai bi tu choi");

const r2 = datPhong(htT, "c5", "P400", "hue", 1, 5);
if (r2 !== "da_dat") throw new Error("khoang ngay 1-5 cua c1 da duoc GIAI PHONG sau khi huy, phai dat duoc");

const h3 = huyDatPhong(htT, "c1");
if (h3 !== "khong_tim_thay") throw new Error("huy LAI mot id DA bi huy truoc do phai tra ve khong_tim_thay, khong phai da_huy");

const soLuong3 = htT.dsDatPhong.length;
if (soLuong3 !== 3) throw new Error("cuoi cung con dung 3 dat phong: c2, c3, c5");
```

:::hints
- kind: attention
  body: "Can dung findIndex de tim vi tri: const idx = ht.dsDatPhong.findIndex((dp) => dp.id === id); Neu idx === -1 return 'khong_tim_thay' NGAY. Nguoc lai, ht.dsDatPhong.splice(idx, 1); roi return 'da_huy'."
- kind: strategy
  body: "const idx = ht.dsDatPhong.findIndex((dp) => dp.id === id); if (idx === -1) return 'khong_tim_thay'; ht.dsDatPhong.splice(idx, 1); return 'da_huy';"
- kind: one-line
  body: "const idx = ht.dsDatPhong.findIndex((dp) => dp.id === id); if (idx === -1) return \"khong_tim_thay\"; ht.dsDatPhong.splice(idx, 1); return \"da_huy\";"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "khong_tim_thay"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Đặt rồi huỷ, cả hai chiều đều đúng — mảnh đầu tiên của track "Giao
dịch VÀ tiền" xong. Bước tiếp theo rời khỏi phòng khách sạn, sang một
thứ dễ sai hơn NHIỀU nếu xử lý cẩu thả: tiền.
::::

::::reflect{#nghi-lai}
`huyDatPhong` chỉ làm ĐÚNG một việc — tìm rồi xoá — VÀ thoát SỚM khi
không tìm thấy, trước khi chạm vào mảng. Chính vì thoát sớm đó mà một
`id` sai (dù chưa từng tồn tại, hay đã bị huỷ trước đó) không hề để
lại dấu vết NÀO trên các đặt phòng khác. Cái giá LÀ huỷ không phải một
thao tác "gọi bao nhiêu lần cũng ra cùng kết quả" — gọi lần hai đổi
câu trả lời, dù trạng thái CUỐI của hệ thống không đổi thêm nữa.
::::

::::checkpoint{mastery=0.68}
::::
