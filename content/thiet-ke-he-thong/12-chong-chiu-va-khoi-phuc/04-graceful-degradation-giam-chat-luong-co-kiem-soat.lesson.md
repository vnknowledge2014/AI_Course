---
id: thiet-ke-he-thong.chong-chiu-va-khoi-phuc.graceful-degradation-giam-chat-luong-co-kiem-soat
title: "Graceful degradation: giảm chất lượng có kiểm soát thay vì sập hoàn toàn"
summary: "layGoiY tai dung choPhepGoi (bai 1): mach mo thi tra ve NGAY dsPhoBien (goi y pho bien chung, khong can downstream), laPhuongAnDuPhong=true; mach dong hoac nua_mo thi tra ve dsCaNhanHoa (thu duong chinh), laPhuongAnDuPhong=false. ThongKeGoiY{soLanCaNhanHoa,soLanDuPhong} MUTATE truc tiep de dem so lan moi loai. Nguoi dung KHONG BAO GIO nhan loi hoan toan -- mach mo van tra ve mot danh sach THAT (khong rong), chi kem ca nhan hoa hon."
locale: vi
track: thiet-ke-he-thong
module: chong-chiu-va-khoi-phuc
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.graceful-degradation-giam-chat-luong-co-kiem-soat]
requires: [sd.bulkhead-co-lap-tai-nguyen-theo-nhom]
concepts: [sd.graceful-degradation-giam-chat-luong-co-kiem-soat]
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
Bulkhead (bài trước) ngăn một nhóm chiếm hết tài nguyên của nhóm khác —
nhưng nếu CHÍNH downstream mà một nhóm gọi tới đang thật sự lỗi (mạch đã
mở, bài 1), cô lập tài nguyên KHÔNG hề giúp nó trả về kết quả. Câu hỏi
tiếp theo: khi KHÔNG thể lấy được câu trả lời TỐT nhất, có nên trả về LỖI
hoàn toàn — hay một câu trả lời KÉM hơn, nhưng vẫn LÀ một câu trả lời?
::::

::::explain{#phuong-an-du-phong}
`layGoiY` mô phỏng một tính năng gợi ý sản phẩm: đường CHÍNH LÀ cá nhân
hoá (`dsCaNhanHoa`, cần gọi một downstream tính toán riêng cho từng người
dùng), phương án DỰ phòng LÀ danh sách phổ biến CHUNG (`dsPhoBien`, không
cần gọi gì cả). Nó tái dùng `choPhepGoi` — CHÍNH mạch điện Ở bài 1:

```typescript title=readonly
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; }
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
interface ThongKeGoiY { soLanCaNhanHoa: number; soLanDuPhong: number; }
function taoThongKeGoiY(): ThongKeGoiY { return { soLanCaNhanHoa: 0, soLanDuPhong: 0 }; }
interface KetQuaGoiY { danhSach: string[]; laPhuongAnDuPhong: boolean; }
function layGoiY(mach: MachDien, thongKe: ThongKeGoiY, dsCaNhanHoa: string[], dsPhoBien: string[]): KetQuaGoiY {
  if (!choPhepGoi(mach)) {
    thongKe.soLanDuPhong += 1;
    return { danhSach: dsPhoBien, laPhuongAnDuPhong: true };
  }
  thongKe.soLanCaNhanHoa += 1;
  return { danhSach: dsCaNhanHoa, laPhuongAnDuPhong: false };
}

const dsCaNhanHoa = ["giay-chay-bo-X", "ao-thun-Y", "balo-Z"];
const dsPhoBien = ["san-pham-ban-chay-1", "san-pham-ban-chay-2"];
const thongKe = taoThongKeGoiY();

const machDong: MachDien = { trangThai: "dong" };
console.log("mach dong (downstream binh thuong):", JSON.stringify(layGoiY(machDong, thongKe, dsCaNhanHoa, dsPhoBien)));

const machMo: MachDien = { trangThai: "mo" };
console.log("mach mo (downstream dang loi, tu bai 1):", JSON.stringify(layGoiY(machMo, thongKe, dsCaNhanHoa, dsPhoBien)));

console.log("thong ke sau 2 lan goi:", JSON.stringify(thongKe));
```

```text title=readonly
mach dong (downstream binh thuong): {"danhSach":["giay-chay-bo-X","ao-thun-Y","balo-Z"],"laPhuongAnDuPhong":false}
mach mo (downstream dang loi, tu bai 1): {"danhSach":["san-pham-ban-chay-1","san-pham-ban-chay-2"],"laPhuongAnDuPhong":true}
thong ke sau 2 lan goi: {"soLanCaNhanHoa":1,"soLanDuPhong":1}
```

Khi mạch `"mo"`, `layGoiY` KHÔNG hề cố gọi downstream RỒI thất bại — nó
kiểm tra `choPhepGoi` TRƯỚC, biết NGAY mạch đang mở, VÀ trả về
`dsPhoBien` tức THÌ. Người dùng nhận một danh sách THẬT (`san-pham-ban-
chay-1`, `san-pham-ban-chay-2`) — kém cá nhân hoá hơn, nhưng KHÔNG hề LÀ
một trang lỗi trống trơn.
::::

::::example{#nua-mo-van-thu-duong-chinh}
`"nua_mo"` (bài 1) LÀ trạng thái ĐANG thử xem downstream đã hồi phục
chưa — `choPhepGoi` trả về `true` cho trạng thái NÀY, nên `layGoiY` VẪN
thử đường chính, KHÔNG vội dùng dự phòng chỉ vì mạch chưa hoàn toàn
`"dong"`:

```typescript title=readonly
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; }
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
interface ThongKeGoiY { soLanCaNhanHoa: number; soLanDuPhong: number; }
function taoThongKeGoiY(): ThongKeGoiY { return { soLanCaNhanHoa: 0, soLanDuPhong: 0 }; }
interface KetQuaGoiY { danhSach: string[]; laPhuongAnDuPhong: boolean; }
function layGoiY(mach: MachDien, thongKe: ThongKeGoiY, dsCaNhanHoa: string[], dsPhoBien: string[]): KetQuaGoiY {
  if (!choPhepGoi(mach)) {
    thongKe.soLanDuPhong += 1;
    return { danhSach: dsPhoBien, laPhuongAnDuPhong: true };
  }
  thongKe.soLanCaNhanHoa += 1;
  return { danhSach: dsCaNhanHoa, laPhuongAnDuPhong: false };
}

const dsCaNhanHoa = ["giay-chay-bo-X", "ao-thun-Y", "balo-Z"];
const dsPhoBien = ["san-pham-ban-chay-1", "san-pham-ban-chay-2"];
const thongKe = taoThongKeGoiY();

const machNuaMo: MachDien = { trangThai: "nua_mo" };
console.log("mach nua_mo (dang thu lai downstream):", JSON.stringify(layGoiY(machNuaMo, thongKe, dsCaNhanHoa, dsPhoBien)));

// nguoi dung KHONG he thay loi -- ho van nhan duoc mot danh sach, chi la CHUA chac ca nhan hoa
console.log("nguoi dung van THAY mot danh sach (khong rong, khong loi):", layGoiY(machNuaMo, thongKe, dsCaNhanHoa, dsPhoBien).danhSach.length > 0);

console.log("thong ke sau 2 lan (ca hai deu nua_mo, deu tinh la ca nhan hoa):", JSON.stringify(thongKe));
```

```text title=readonly
mach nua_mo (dang thu lai downstream): {"danhSach":["giay-chay-bo-X","ao-thun-Y","balo-Z"],"laPhuongAnDuPhong":false}
nguoi dung van THAY mot danh sach (khong rong, khong loi): true
thong ke sau 2 lan (ca hai deu nua_mo, deu tinh la ca nhan hoa): {"soLanCaNhanHoa":2,"soLanDuPhong":0}
```

`laPhuongAnDuPhong` LÀ `false` — `"nua_mo"` được tính LÀ một lần thử
đường CHÍNH bình thường, giống hệt `"dong"`. Chỉ đúng MỘT trạng thái
(`"mo"`) mới kích hoạt dự phòng. `thongKe` (bị MUTATE trực tiếp — bình
thường trong track NÀY) ghi lại đúng con số ĐÓ, không lẫn lộn giữa hai
loại.
::::

::::predict{#doan-nua-mo-co-du-phong-khong commitOnce}
Mạch đang Ở `"nua_mo"` (không phải `"mo"`, cũng không phải `"dong"`). Gọi
`layGoiY(mach, thongKe, dsCaNhanHoa, dsPhoBien)` — trường `laPhuongAnDuPhong`
trong kết quả trả về LÀ gì?

:::opt{correct}
`false` — `choPhepGoi` chỉ trả về `false` khi `trangThai === "mo"`;
`"nua_mo"` khác `"mo"`, nên `choPhepGoi` trả `true`, VÀ `layGoiY` đi vào
nhánh CÁ nhân hoá (không phải dự phòng)
:::
:::opt
`true` — `"nua_mo"` nghĩa LÀ hệ thống CHƯA chắc chắn downstream đã hồi
phục, nên NÊN thận trọng VÀ dùng phương án dự phòng cho tới khi mạch
hoàn toàn đóng LẠI
::why
Nhầm "chưa chắc chắn hoàn toàn" VỚI "phải dùng dự phòng" — nhưng
`layGoiY` không hề có một nhánh RIÊNG cho `"nua_mo"`; nó chỉ hỏi ĐÚNG một
câu qua `choPhepGoi`: "có LÀ `mo` không".

Chỗ lệch: `choPhepGoi` viết `return mach.trangThai !== "mo";` — với
`trangThai = "nua_mo"`, biểu thức `"nua_mo" !== "mo"` LÀ `true`. Trong
`layGoiY`, điều kiện `if (!choPhepGoi(mach))` trở thành `if (!true)`, tức
`if (false)` — nhánh dự phòng KHÔNG chạy. Việc "thử lại xem downstream đã
ổn chưa" (Ý nghĩa của `nua_mo`) CHÍNH LÀ đi qua đường chính, không phải LÀ
lý do để né nó.
::
:::
::::

::::code{#viet_lay_goi_y}
Hoàn thiện `layGoiY` cho nhánh KHÔNG được phép gọi (`!choPhepGoi(mach)`
LÀ `true`): tăng `thongKe.soLanDuPhong`, RỒI trả về `dsPhoBien` VỚI
`laPhuongAnDuPhong: true`.

```typescript title=starter
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; }
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
interface ThongKeGoiY { soLanCaNhanHoa: number; soLanDuPhong: number; }
function taoThongKeGoiY(): ThongKeGoiY { return { soLanCaNhanHoa: 0, soLanDuPhong: 0 }; }
interface KetQuaGoiY { danhSach: string[]; laPhuongAnDuPhong: boolean; }

function layGoiY(mach: MachDien, thongKe: ThongKeGoiY, dsCaNhanHoa: string[], dsPhoBien: string[]): KetQuaGoiY {
  if (!choPhepGoi(mach)) {
    ___
  }
  thongKe.soLanCaNhanHoa += 1;
  return { danhSach: dsCaNhanHoa, laPhuongAnDuPhong: false };
}

const machX: MachDien = { trangThai: "mo" };
const thongKeX = taoThongKeGoiY();
console.log(JSON.stringify(layGoiY(machX, thongKeX, ["a"], ["b"])), thongKeX.soLanDuPhong);
```

```typescript title=solution
type TrangThaiMach = "dong" | "mo" | "nua_mo";
interface MachDien { trangThai: TrangThaiMach; }
function choPhepGoi(mach: MachDien): boolean {
  return mach.trangThai !== "mo";
}
interface ThongKeGoiY { soLanCaNhanHoa: number; soLanDuPhong: number; }
function taoThongKeGoiY(): ThongKeGoiY { return { soLanCaNhanHoa: 0, soLanDuPhong: 0 }; }
interface KetQuaGoiY { danhSach: string[]; laPhuongAnDuPhong: boolean; }

function layGoiY(mach: MachDien, thongKe: ThongKeGoiY, dsCaNhanHoa: string[], dsPhoBien: string[]): KetQuaGoiY {
  if (!choPhepGoi(mach)) {
    thongKe.soLanDuPhong += 1;
    return { danhSach: dsPhoBien, laPhuongAnDuPhong: true };
  }
  thongKe.soLanCaNhanHoa += 1;
  return { danhSach: dsCaNhanHoa, laPhuongAnDuPhong: false };
}

const machX: MachDien = { trangThai: "mo" };
const thongKeX = taoThongKeGoiY();
console.log(JSON.stringify(layGoiY(machX, thongKeX, ["a"], ["b"])), thongKeX.soLanDuPhong);
```

```typescript title=test
const thongKeT = taoThongKeGoiY();
const dsCaNhanHoaT = ["giay-chay-bo-X", "ao-thun-Y"];
const dsPhoBienT = ["ban-chay-1", "ban-chay-2"];

const machDongT: MachDien = { trangThai: "dong" };
const ketQuaDongT = layGoiY(machDongT, thongKeT, dsCaNhanHoaT, dsPhoBienT);
if (ketQuaDongT.laPhuongAnDuPhong !== false) throw new Error("mach dong phai tra ve danh sach CA NHAN HOA, khong phai du phong");
if (JSON.stringify(ketQuaDongT.danhSach) !== JSON.stringify(dsCaNhanHoaT)) throw new Error("mach dong phai tra dung dsCaNhanHoa");

const machMoT: MachDien = { trangThai: "mo" };
const ketQuaMoT = layGoiY(machMoT, thongKeT, dsCaNhanHoaT, dsPhoBienT);
if (ketQuaMoT.laPhuongAnDuPhong !== true) throw new Error("mach mo phai tra ve PHUONG AN DU PHONG");
if (JSON.stringify(ketQuaMoT.danhSach) !== JSON.stringify(dsPhoBienT)) throw new Error("mach mo phai tra dung dsPhoBien, KHONG goi toi downstream ca nhan hoa");

const machNuaMoT: MachDien = { trangThai: "nua_mo" };
const ketQuaNuaMoT = layGoiY(machNuaMoT, thongKeT, dsCaNhanHoaT, dsPhoBienT);
if (ketQuaNuaMoT.laPhuongAnDuPhong !== false) throw new Error("mach nua_mo van duoc PHEP thu goi chinh, khong phai du phong");

const soLanCaNhanHoaCuoiT = thongKeT.soLanCaNhanHoa;
const soLanDuPhongCuoiT = thongKeT.soLanDuPhong;
if (soLanCaNhanHoaCuoiT !== 2) throw new Error("2 lan (dong, nua_mo) phai duoc dem la ca nhan hoa");
if (soLanDuPhongCuoiT !== 1) throw new Error("dung 1 lan (mo) phai duoc dem la du phong");
```

:::hints
- kind: attention
  body: "Trong nhanh khong duoc goi: tang thongKe.soLanDuPhong len 1, roi return { danhSach: dsPhoBien, laPhuongAnDuPhong: true }."
- kind: strategy
  body: "thongKe.soLanDuPhong += 1; return { danhSach: dsPhoBien, laPhuongAnDuPhong: true };"
- kind: one-line
  body: "thongKe.soLanDuPhong += 1; return { danhSach: dsPhoBien, laPhuongAnDuPhong: true };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "laPhuongAnDuPhong\":true} 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Người dùng luôn thấy MỘT kết quả, không bao giờ LÀ trang trắng. Nhưng
graceful degradation chỉ giải quyết được KHI downstream đã được biết LÀ
đang lỗi (mạch đã mở). Câu hỏi khác: LÀM sao để hệ thống của CHÍNH mình
không PHẢI LÀ nguyên nhân khiến downstream lỗi NGAY từ đầu?
::::

::::reflect{#nghi-lai}
`layGoiY` không hề thêm MỘT khái niệm mới về "chống chịu lỗi" — nó CHỈ
ráp lại đúng MỘT điều kiện đã có sẵn (`choPhepGoi` từ bài 1) VÀO một quyết
định nghiệp vụ (trả về danh sách NÀO). Đây LÀ lý do graceful degradation
thường không cần một thuật toán riêng: nó LÀ nơi TRẠNG thái kỹ thuật (mạch
đang Ở đâu) VÀ trải nghiệm người dùng (thấy gì trên màn hình) gặp nhau.
`ThongKeGoiY` cho thấy đây LÀ một quyết định có thể ĐO được theo thời
gian — bao nhiêu PHẦN trăm request đang phải dùng bản kém cá nhân hoá hơn
— chứ không phải một cảm giác mơ hồ VỀ "đôi khi hệ thống chậm".
::::

::::checkpoint{mastery=0.79}
::::
