---
id: thiet-ke-he-thong.dinh-danh-va-toc-do.leaky-bucket-hang-doi
title: "Leaky bucket: hàng đợi rò rỉ"
summary: "ThungRi giữ hangDoi FIFO sức chứa succhuaToiDa; xuLyRi rút phần tử khỏi hàng theo tốc độ CỐ định (Math.floor((soMsTroiQua/1000) × tocDoXuLyMoiGiay)), gopYeuCauRi từ chối khi hàng đầy. KHÔNG cho burst (khác token bucket): sức chứa 2, gửi đúng nhịp 1 request/1000ms (khớp tốc độ xử lý 1/giây) không bao giờ bị từ chối, hàng đợi luôn dài 1; nhưng 4 request CÙNG lúc chỉ 2 lọt qua, 2 bị từ chối ngay."
locale: vi
track: thiet-ke-he-thong
module: dinh-danh-va-toc-do
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.leaky-bucket-hang-doi]
requires: [sd.token-bucket-nap-lai-theo-thoi-gian]
concepts: [sd.leaky-bucket-hang-doi]
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
Token bucket cho phép DỒN dùng — hai bài trước đã thấy `8` request
lọt qua CÙNG một lúc chỉ vì token tích luỹ sẵn. Nếu hệ thống PHÍA sau
không chịu được BẤT kỳ đợt dồn dập nào, dù nhỏ, cần một thuật toán
khác hẳn.
::::

::::explain{#hang-doi-ri}
Leaky bucket (xô rò rỉ) đảo NGƯỢC cách nghĩ: thay VÌ một kho token,
đây LÀ một hàng đợi FIFO sức chứa GIỚI hạn. Request MỚI xếp hàng NẾU
còn chỗ; hàng ĐƯỢC "rò" ra (xử lý) Ở một tốc độ CỐ định, không phụ
thuộc CÓ bao nhiêu request đang chờ:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungRi { hangDoi: number[]; succhuaToiDa: number; tocDoXuLyMoiGiay: number; thoiDiemXuLyCuoi: number; dinhDanhTiep: number; }
function taoThungRi(succhuaToiDa: number, tocDoXuLyMoiGiay: number, dh: DongHoMoPhong): ThungRi {
  return { hangDoi: [], succhuaToiDa, tocDoXuLyMoiGiay, thoiDiemXuLyCuoi: dh.thoiGianHienTai, dinhDanhTiep: 1 };
}
function xuLyRi(thung: ThungRi, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemXuLyCuoi;
  if (soMsTroiQua <= 0) return;
  const soLuongXuLyDuoc = Math.floor((soMsTroiQua / 1000) * thung.tocDoXuLyMoiGiay);
  for (let i = 0; i < soLuongXuLyDuoc && thung.hangDoi.length > 0; i++) thung.hangDoi.shift();
  if (soLuongXuLyDuoc > 0) {
    thung.thoiDiemXuLyCuoi += (soLuongXuLyDuoc / thung.tocDoXuLyMoiGiay) * 1000;
  }
}
function gopYeuCauRi(thung: ThungRi, dh: DongHoMoPhong): boolean {
  xuLyRi(thung, dh);
  if (thung.hangDoi.length >= thung.succhuaToiDa) return false;
  thung.hangDoi.push(thung.dinhDanhTiep++);
  return true;
}

const dh = taoDongHoMoPhong();
const thung = taoThungRi(3, 1, dh); // suc chua 3, xu ly 1 item/giay

const dot1: boolean[] = [];
for (let i = 0; i < 5; i++) dot1.push(gopYeuCauRi(thung, dh));
console.log("5 request lien tiep (hang doi suc chua 3):", dot1.join(","));
console.log("do dai hang doi:", thung.hangDoi.length, "-- noi dung:", thung.hangDoi.join(","));

tienThoiGian(dh, 1000);
const duocNhanSauKhiXuLy = gopYeuCauRi(thung, dh);
console.log("sau 1000ms (xu ly duoc 1 item), request moi:", duocNhanSauKhiXuLy);
console.log("do dai hang doi:", thung.hangDoi.length, "-- noi dung:", thung.hangDoi.join(","));
```

```text title=readonly
5 request lien tiep (hang doi suc chua 3): true,true,true,false,false
do dai hang doi: 3 -- noi dung: 1,2,3
sau 1000ms (xu ly duoc 1 item), request moi: true
do dai hang doi: 3 -- noi dung: 2,3,4
```

Ba request ĐẦU lấp đầy hàng đợi (`succhuaToiDa=3`); HAI request cuối
gặp hàng ĐẦY, bị từ chối NGAY — KHÔNG hề "chờ chỗ trống" như một hàng
đợi thông thường. Sau `1000ms` (xử lý ĐƯỢC đúng `1` item Ở tốc độ
`1/giây`), item CŨ nhất (`1`) bị rút ra, MỞ đúng một chỗ cho request
MỚI (`4`) — hàng đợi LUÔN giữ đúng "cỡ", chưa bao giờ vượt sức chứa.
::::

::::example{#khong-cho-burst}
Điểm khác biệt CỐT lõi với token bucket: nếu request đến ĐÚNG bằng
tốc độ xử LÝ, hàng đợi KHÔNG bao giờ đầy VÀ không bao giờ bị từ chối
— nhưng NẾU nhiều request đến CÙNG một lúc, chỉ đúng SỨC chứa được
nhận, phần CÒN lại mất NGAY, không CÓ "vốn tích luỹ" nào để dùng bù:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungRi { hangDoi: number[]; succhuaToiDa: number; tocDoXuLyMoiGiay: number; thoiDiemXuLyCuoi: number; dinhDanhTiep: number; }
function taoThungRi(succhuaToiDa: number, tocDoXuLyMoiGiay: number, dh: DongHoMoPhong): ThungRi {
  return { hangDoi: [], succhuaToiDa, tocDoXuLyMoiGiay, thoiDiemXuLyCuoi: dh.thoiGianHienTai, dinhDanhTiep: 1 };
}
function xuLyRi(thung: ThungRi, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemXuLyCuoi;
  if (soMsTroiQua <= 0) return;
  const soLuongXuLyDuoc = Math.floor((soMsTroiQua / 1000) * thung.tocDoXuLyMoiGiay);
  for (let i = 0; i < soLuongXuLyDuoc && thung.hangDoi.length > 0; i++) thung.hangDoi.shift();
  if (soLuongXuLyDuoc > 0) {
    thung.thoiDiemXuLyCuoi += (soLuongXuLyDuoc / thung.tocDoXuLyMoiGiay) * 1000;
  }
}
function gopYeuCauRi(thung: ThungRi, dh: DongHoMoPhong): boolean {
  xuLyRi(thung, dh);
  if (thung.hangDoi.length >= thung.succhuaToiDa) return false;
  thung.hangDoi.push(thung.dinhDanhTiep++);
  return true;
}

// suc chua 2, toc do xu ly 1 item/giay -- gui DUNG 1 request moi 1000ms (khop toc do xu ly)
const dh = taoDongHoMoPhong();
const thung = taoThungRi(2, 1, dh);

for (let vong = 0; vong < 5; vong++) {
  const duocNhan = gopYeuCauRi(thung, dh);
  console.log(`t=${dh.thoiGianHienTai}ms: duoc nhan=${duocNhan}, do dai hang doi=${thung.hangDoi.length}`);
  tienThoiGian(dh, 1000);
}
console.log("--- so sanh: gui 4 request DON mot luc (t khong doi) ---");
const dh2 = taoDongHoMoPhong();
const thung2 = taoThungRi(2, 1, dh2);
const donDot: boolean[] = [];
for (let i = 0; i < 4; i++) donDot.push(gopYeuCauRi(thung2, dh2));
console.log("4 request cung luc, suc chua 2:", donDot.join(","));
```

```text title=readonly
t=0ms: duoc nhan=true, do dai hang doi=1
t=1000ms: duoc nhan=true, do dai hang doi=1
t=2000ms: duoc nhan=true, do dai hang doi=1
t=3000ms: duoc nhan=true, do dai hang doi=1
t=4000ms: duoc nhan=true, do dai hang doi=1
--- so sanh: gui 4 request DON mot luc (t khong doi) ---
4 request cung luc, suc chua 2: true,true,false,false
```

Gửi ĐÚNG nhịp `1000ms` một request (khớp tốc độ xử lý `1/giây`): NĂM
request LIÊN tiếp đều được nhận, hàng đợi LUÔN dài `1` — không hề
tăng dần. Nhưng `4` request CÙNG lúc (không thời gian trôi qua Ở
giữa) chỉ `2` lọt QUA — sức chứa `2` LÀ giới hạn TUYỆT đối cho một
"đợt", dù tốc độ xử lý VẪN còn dư (chưa dùng gì).
::::

::::predict{#doan-gui-dung-nhip commitOnce}
Hàng đợi sức chứa `1`, tốc độ xử lý `2` item/giây (nhanh HƠN tốc độ
gửi). Nếu gửi ĐÚNG một request MỖI `1000ms` (chậm hơn NHIỀU so với
tốc độ xử lý), request NÀO có bị từ chối KHÔNG?

:::opt{correct}
KHÔNG — mỗi request MỚI đến khi hàng đợi ĐàXử lý xong item TRƯỚC đó
từ LÂU (`2` item/giây xử lý nhanh hơn nhịp gửi `1`/giây RẤT nhiều),
hàng đợi luôn RỖNG trước khi request tiếp theo tới
:::
:::opt
CÓ — sức chứa `1` quá nhỏ, sớm HAY muộn cũng CÓ một request đến ĐÚNG
lúc hàng còn giữ item cũ, gây từ chối
::why
Nhầm "sức chứa NHỎ luôn dẫn tới từ chối" VỚI "sức chứa nhỏ CHỈ gây từ
chối khi tốc độ ĐẾN vượt tốc độ xử lý". Ở đây tốc độ xử lý (`2/giây`)
NHANH hơn hẳn nhịp gửi (`1/giây`).

Chỗ lệch: MỖI khi `gopYeuCauRi` được gọi, nó LUÔN gọi `xuLyRi` TRƯỚC
— và VÌ khoảng cách giữa hai request LÀ `1000ms` trong khi tốc độ xử
lý cho phép RÚT tới `2` item MỖI giây, hàng đợi LUÔN kịp rỗng lại
(dư sức) TRƯỚC khi request kế tiếp tới, dù sức chứa CHỈ là `1`.
::
:::
::::

::::code{#viet_gop_yeu_cau_ri}
Hoàn thiện `gopYeuCauRi` — sau khi ĐÃ xử lý (rút bớt) hàng đợi, kiểm
tra hàng ĐÃ đầy (`>= succhuaToiDa`) thì từ chối.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungRi { hangDoi: number[]; succhuaToiDa: number; tocDoXuLyMoiGiay: number; thoiDiemXuLyCuoi: number; dinhDanhTiep: number; }
function taoThungRi(succhuaToiDa: number, tocDoXuLyMoiGiay: number, dh: DongHoMoPhong): ThungRi {
  return { hangDoi: [], succhuaToiDa, tocDoXuLyMoiGiay, thoiDiemXuLyCuoi: dh.thoiGianHienTai, dinhDanhTiep: 1 };
}
function xuLyRi(thung: ThungRi, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemXuLyCuoi;
  if (soMsTroiQua <= 0) return;
  const soLuongXuLyDuoc = Math.floor((soMsTroiQua / 1000) * thung.tocDoXuLyMoiGiay);
  for (let i = 0; i < soLuongXuLyDuoc && thung.hangDoi.length > 0; i++) thung.hangDoi.shift();
  if (soLuongXuLyDuoc > 0) {
    thung.thoiDiemXuLyCuoi += (soLuongXuLyDuoc / thung.tocDoXuLyMoiGiay) * 1000;
  }
}
function gopYeuCauRi(thung: ThungRi, dh: DongHoMoPhong): boolean {
  xuLyRi(thung, dh);
  ___
  thung.hangDoi.push(thung.dinhDanhTiep++);
  return true;
}

const dh = taoDongHoMoPhong();
const thung = taoThungRi(1, 1, dh);
console.log(gopYeuCauRi(thung, dh), gopYeuCauRi(thung, dh));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface ThungRi { hangDoi: number[]; succhuaToiDa: number; tocDoXuLyMoiGiay: number; thoiDiemXuLyCuoi: number; dinhDanhTiep: number; }
function taoThungRi(succhuaToiDa: number, tocDoXuLyMoiGiay: number, dh: DongHoMoPhong): ThungRi {
  return { hangDoi: [], succhuaToiDa, tocDoXuLyMoiGiay, thoiDiemXuLyCuoi: dh.thoiGianHienTai, dinhDanhTiep: 1 };
}
function xuLyRi(thung: ThungRi, dh: DongHoMoPhong): void {
  const soMsTroiQua = dh.thoiGianHienTai - thung.thoiDiemXuLyCuoi;
  if (soMsTroiQua <= 0) return;
  const soLuongXuLyDuoc = Math.floor((soMsTroiQua / 1000) * thung.tocDoXuLyMoiGiay);
  for (let i = 0; i < soLuongXuLyDuoc && thung.hangDoi.length > 0; i++) thung.hangDoi.shift();
  if (soLuongXuLyDuoc > 0) {
    thung.thoiDiemXuLyCuoi += (soLuongXuLyDuoc / thung.tocDoXuLyMoiGiay) * 1000;
  }
}
function gopYeuCauRi(thung: ThungRi, dh: DongHoMoPhong): boolean {
  xuLyRi(thung, dh);
  if (thung.hangDoi.length >= thung.succhuaToiDa) return false;
  thung.hangDoi.push(thung.dinhDanhTiep++);
  return true;
}

const dh = taoDongHoMoPhong();
const thung = taoThungRi(1, 1, dh);
console.log(gopYeuCauRi(thung, dh), gopYeuCauRi(thung, dh));
```

```typescript title=test
function layDoDaiHangDoi(t: ThungRi): number { return t.hangDoi.length; }

const dhT = taoDongHoMoPhong();
const thungT = taoThungRi(2, 1, dhT);
const kqT: boolean[] = [];
for (let i = 0; i < 3; i++) kqT.push(gopYeuCauRi(thungT, dhT));
if (kqT.join(",") !== "true,true,false") throw new Error("suc chua 2: 2 request dau duoc nhan, request 3 bi tu choi");
if (layDoDaiHangDoi(thungT) !== 2) throw new Error("hang doi phai dung 2 (bang suc chua), khong duoc vuot qua");

const dhT2 = taoDongHoMoPhong();
const thungT2 = taoThungRi(1, 1, dhT2);
gopYeuCauRi(thungT2, dhT2);
if (gopYeuCauRi(thungT2, dhT2) !== false) throw new Error("suc chua 1, da co 1 item, request thu 2 phai bi tu choi");
if (layDoDaiHangDoi(thungT2) !== 1) throw new Error("hang doi phai dung 1 (khong bi tu choi lam mat item da co san)");

const dhT3 = taoDongHoMoPhong();
const thungT3 = taoThungRi(20, 3, dhT3);
for (let i = 0; i < 10; i++) gopYeuCauRi(thungT3, dhT3);
for (let i = 0; i < 5; i++) { tienThoiGian(dhT3, 400); gopYeuCauRi(thungT3, dhT3); }
if (layDoDaiHangDoi(thungT3) !== 9) throw new Error("thoiDiemXuLyCuoi phai CONG DON chinh xac (soLuongXuLyDuoc/tocDo)*1000, khong duoc GAN THANG bang dh.thoiGianHienTai -- neu gan thang se lam MAT phan du thoi gian chua du xu ly 1 item, khien tong so item xu ly duoc SAU 5 lan goi (moi lan cach 400ms, toc do 3/giay) chi la 5 thay vi 6 dung (hang doi con lai phai la 9, khong phai 10)");
```

:::hints
- kind: attention
  body: "Neu hang doi da day (do dai >= suc chua toi da), tu choi ngay -- mot dong."
- kind: strategy
  body: "if (thung.hangDoi.length >= thung.succhuaToiDa) return false;"
- kind: one-line
  body: "if (thung.hangDoi.length >= thung.succhuaToiDa) return false;"
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
Ba thuật toán rồi: token bucket (cho burst), token bucket có nạp
theo thời gian, leaky bucket (không burst). Còn một họ thuật toán
khác hẳn — đếm request theo CỬA SỔ thời gian cố định.
::::

::::reflect{#nghi-lai}
`gopYeuCauRi` VÀ `tieuThuTokenTG` (bài trước) đều gọi "cập nhật trạng
thái theo THỜI gian" TRƯỚC khi quyết định — nhưng Ý nghĩa hoàn toàn
khác nhau: token bucket NẠP thêm phần được PHÉP làm; leaky bucket RÚT
bớt phần ĐÃ xử lý xong. Cùng một kỹ thuật (tính THEO mili-giây trôi
qua), hai MỤC đích đối lập.
::::

::::checkpoint{mastery=0.75}
::::
