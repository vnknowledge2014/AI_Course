---
id: thiet-ke-he-thong.noi-dung-va-kham-pha.cap-nhat-tan-suat-dinh-ky
title: "Cập nhật tần suất định kỳ, không phải mỗi lần gõ"
summary: "chayCapNhatDinhKy chi rebuild bangTanSuat khi (thoiGianHienTai - thoiDiemCapNhatCuoi) >= chuKyMs (1000ms) -- goi tai t=999 tra ve false, bang van rong; tai t=1000 tra ve true, bang co 'may tinh':2, 'meo':1. Goi LAP LAI ngay lap tuc (khong tien dong ho) LUON tra ve false -- ke ca khi log VUA co du lieu moi ('moi') -- vi dieu kien chi xet THOI GIAN da troi, khong xet log co thay doi hay khong."
locale: vi
track: thiet-ke-he-thong
module: noi-dung-va-kham-pha
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 10
teaches: [sd.cap-nhat-tan-suat-dinh-ky]
requires: [sd.trie-top-k-goi-y]
concepts: [sd.cap-nhat-tan-suat-dinh-ky]
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
`xayDungTopK` (bài trước) tính lại TOÀN bộ cây — không hề rẻ. Nếu chạy
nó SAU mỗi lượt tìm kiếm thật, hệ thống sẽ dành phần LỚN thời gian đi
tính LẠI thay vì trả lời người dùng. Cần một nhịp CHẠY khác hẳn.
::::

::::explain{#dinh-ky-thay-vi-thuc-thoi}
Tần suất tìm kiếm THẬT không cần cập nhật NGAY tức thì — gộp lại theo
CHU kỳ (mỗi vài phút, mỗi GIỜ) vẫn đủ chính xác cho việc gợi ý. Ta tách
LÀM hai phần: ghi nhận (rẻ, LUÔN chạy) VÀ tổng hợp định kỳ (đắt, chỉ
chạy khi ĐỦ thời gian đã trôi — đo bằng đồng hồ MÔ phỏng, không phải
`Date.now()`):

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NhatKyTimKiem { cacTu: string[]; }
function taoNhatKyTimKiem(): NhatKyTimKiem { return { cacTu: [] }; }
function ghiNhanTimKiem(nk: NhatKyTimKiem, tu: string): void { nk.cacTu.push(tu); }

interface TrangThaiCapNhatDinhKy {
  bangTanSuat: Map<string, number>;
  thoiDiemCapNhatCuoi: number;
}
function taoTrangThaiCapNhatDinhKy(): TrangThaiCapNhatDinhKy {
  return { bangTanSuat: new Map(), thoiDiemCapNhatCuoi: 0 };
}

function chayCapNhatDinhKy(
  tt: TrangThaiCapNhatDinhKy,
  nk: NhatKyTimKiem,
  dh: DongHoMoPhong,
  chuKyMs: number,
): boolean {
  if (dh.thoiGianHienTai - tt.thoiDiemCapNhatCuoi < chuKyMs) return false;
  const bangMoi = new Map<string, number>();
  for (const tu of nk.cacTu) {
    bangMoi.set(tu, (bangMoi.get(tu) ?? 0) + 1);
  }
  tt.bangTanSuat = bangMoi;
  tt.thoiDiemCapNhatCuoi = dh.thoiGianHienTai;
  return true;
}

const dh = taoDongHoMoPhong();
const nk = taoNhatKyTimKiem();
const tt = taoTrangThaiCapNhatDinhKy();
const CHU_KY_MS = 1000;

ghiNhanTimKiem(nk, "may tinh");
ghiNhanTimKiem(nk, "may tinh");
ghiNhanTimKiem(nk, "meo");
tienThoiGian(dh, 500);
console.log("t=500, chay dinh ky (chua du chu ky 1000):", chayCapNhatDinhKy(tt, nk, dh, CHU_KY_MS));

tienThoiGian(dh, 500);
console.log("t=1000 (dung chu ky), chay dinh ky:", chayCapNhatDinhKy(tt, nk, dh, CHU_KY_MS));
console.log("bang tan suat sau khi cap nhat:", JSON.stringify([...tt.bangTanSuat.entries()]));
```

```text title=readonly
t=500, chay dinh ky (chua du chu ky 1000): false
t=1000 (dung chu ky), chay dinh ky: true
bang tan suat sau khi cap nhat: [["may tinh",2],["meo",1]]
```

`ghiNhanTimKiem` chỉ LÀ một lần `push` — RẺ, chạy được HÀNG triệu lần
mỗi giây. `chayCapNhatDinhKy` mới LÀ phần đắt (duyệt LẠI toàn bộ nhật
ký), VÀ nó tự CHẶN mình lại bằng điều kiện thời gian: tại `t=500` (chưa
đủ `1000ms` kể từ `thoiDiemCapNhatCuoi = 0`) nó KHÔNG chạy, bảng vẫn
rỗng. Đúng `t=1000` mới thật sự rebuild.
::::

::::example{#goi-lai-ngay-khong-lam-lai}
Gọi `chayCapNhatDinhKy` NGAY sau khi nó VỪA chạy thành công (không hề
tiến đồng hồ) LUÔN trả về `false` — NGAY cả khi nhật ký vừa CÓ thêm dữ
liệu mới, vì điều kiện chỉ xét thời GIAN đã trôi qua, không xét nhật ký
có ĐỔI hay không:

```typescript title=readonly
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NhatKyTimKiem { cacTu: string[]; }
function taoNhatKyTimKiem(): NhatKyTimKiem { return { cacTu: [] }; }
function ghiNhanTimKiem(nk: NhatKyTimKiem, tu: string): void { nk.cacTu.push(tu); }

interface TrangThaiCapNhatDinhKy {
  bangTanSuat: Map<string, number>;
  thoiDiemCapNhatCuoi: number;
}
function taoTrangThaiCapNhatDinhKy(): TrangThaiCapNhatDinhKy {
  return { bangTanSuat: new Map(), thoiDiemCapNhatCuoi: 0 };
}

function chayCapNhatDinhKy(
  tt: TrangThaiCapNhatDinhKy,
  nk: NhatKyTimKiem,
  dh: DongHoMoPhong,
  chuKyMs: number,
): boolean {
  if (dh.thoiGianHienTai - tt.thoiDiemCapNhatCuoi < chuKyMs) return false;
  const bangMoi = new Map<string, number>();
  for (const tu of nk.cacTu) {
    bangMoi.set(tu, (bangMoi.get(tu) ?? 0) + 1);
  }
  tt.bangTanSuat = bangMoi;
  tt.thoiDiemCapNhatCuoi = dh.thoiGianHienTai;
  return true;
}

const CHU_KY_MS = 1000;

// tai lap dung trang thai da co o khoi truoc: 3 tim kiem, tien toi t=1000, chay lan dau thanh cong
const dh = taoDongHoMoPhong();
const nk = taoNhatKyTimKiem();
const tt = taoTrangThaiCapNhatDinhKy();
ghiNhanTimKiem(nk, "may tinh");
ghiNhanTimKiem(nk, "may tinh");
ghiNhanTimKiem(nk, "meo");
tienThoiGian(dh, 1000);
chayCapNhatDinhKy(tt, nk, dh, CHU_KY_MS);

ghiNhanTimKiem(nk, "may anh"); // co du lieu MOI trong log
console.log("da co tu moi trong log, nhung goi lai NGAY (khong tien dong ho):", chayCapNhatDinhKy(tt, nk, dh, CHU_KY_MS));
console.log("bang tan suat co 'may anh' chua:", tt.bangTanSuat.has("may anh"));

tienThoiGian(dh, 1000); // du them 1 chu ky nua
console.log("sau khi tien du 1 chu ky nua, chay lai:", chayCapNhatDinhKy(tt, nk, dh, CHU_KY_MS));
console.log("gio bang tan suat co 'may anh' chua:", tt.bangTanSuat.has("may anh"));
```

```text title=readonly
da co tu moi trong log, nhung goi lai NGAY (khong tien dong ho): false
bang tan suat co 'may anh' chua: false
sau khi tien du 1 chu ky nua, chay lai: true
gio bang tan suat co 'may anh' chua: true
```

`"may anh"` đã NẰM trong nhật ký NGAY sau lần gọi đầu tiên — NHƯNG chỉ
khi đồng hồ THẬT sự tiến đủ `chuKyMs` tính TỪ `thoiDiemCapNhatCuoi`, bảng
tần suất MỚI phản ánh nó. "Có dữ liệu MỚI" và "đã tới lúc rebuild" LÀ
hai điều kiện hoàn TOÀN tách biệt.
::::

::::predict{#doan-goi-lai-ngay-khong-chay commitOnce}
`chuKyMs = 1000`. Tại `t=1000`, gọi `chayCapNhatDinhKy` VÀ nó chạy
THÀNH công (trả `true`). Ngay SAU đó, thêm một từ MỚI vào nhật ký bằng
`ghiNhanTimKiem`, RỒI gọi `chayCapNhatDinhKy` một lần NỮA — vẫn tại
`t=1000`, không hề gọi `tienThoiGian`. Lần gọi THỨ hai này trả về gì?

:::opt{correct}
`false` — hiệu `dh.thoiGianHienTai - tt.thoiDiemCapNhatCuoi` LÀ `1000 -
1000 = 0`, nhỏ hơn `chuKyMs`, NÊN hàm dừng ngay Ở dòng kiểm tra đầu
tiên, dù nhật ký vừa CÓ thêm dữ liệu
:::
:::opt
`true` — nhật ký vừa THAY đổi (có từ mới), nên lần gọi kế tiếp PHẢI
rebuild lại để phản ánh đúng dữ liệu mới NHẤT
::why
Nhầm "nhật ký có thay ĐỔI" với "đã đủ điều kiện thời gian để chạy LẠI"
— nhưng `chayCapNhatDinhKy` không hề ĐỌC `nk.cacTu.length` trong điều
kiện quyết ĐỊNH có chạy hay không.

Chỗ lệch: dòng đầu TIÊN của hàm chỉ so sánh `dh.thoiGianHienTai` VỚI
`tt.thoiDiemCapNhatCuoi` — sau lần chạy thành công tại `t=1000`,
`tt.thoiDiemCapNhatCuoi` được CẬP nhật thành `1000`. Gọi lại NGAY (vẫn
`t=1000`) cho hiệu số bằng `0`, nhỏ hơn `chuKyMs=1000`, NÊN hàm `return
false` ngay LẬP tức — nội dung `nk.cacTu` có thay đổi HAY không hoàn
toàn không được XÉT tới trong điều kiện này.
::
:::
::::

::::code{#viet_chay_cap_nhat_dinh_ky}
Hoàn thiện `chayCapNhatDinhKy` — dòng đầu TIÊN phải kiểm tra đã đủ
`chuKyMs` kể từ lần cập nhật CUỐI hay chưa; nếu chưa, dừng ngay VÀ trả
`false`.

```typescript title=starter
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NhatKyTimKiem { cacTu: string[]; }
function taoNhatKyTimKiem(): NhatKyTimKiem { return { cacTu: [] }; }
function ghiNhanTimKiem(nk: NhatKyTimKiem, tu: string): void { nk.cacTu.push(tu); }

interface TrangThaiCapNhatDinhKy {
  bangTanSuat: Map<string, number>;
  thoiDiemCapNhatCuoi: number;
}
function taoTrangThaiCapNhatDinhKy(): TrangThaiCapNhatDinhKy {
  return { bangTanSuat: new Map(), thoiDiemCapNhatCuoi: 0 };
}

function chayCapNhatDinhKy(
  tt: TrangThaiCapNhatDinhKy,
  nk: NhatKyTimKiem,
  dh: DongHoMoPhong,
  chuKyMs: number,
): boolean {
  ___
  const bangMoi = new Map<string, number>();
  for (const tu of nk.cacTu) {
    bangMoi.set(tu, (bangMoi.get(tu) ?? 0) + 1);
  }
  tt.bangTanSuat = bangMoi;
  tt.thoiDiemCapNhatCuoi = dh.thoiGianHienTai;
  return true;
}

const dh = taoDongHoMoPhong();
const nk = taoNhatKyTimKiem();
const tt = taoTrangThaiCapNhatDinhKy();
ghiNhanTimKiem(nk, "meo");
tienThoiGian(dh, 1000);
console.log(chayCapNhatDinhKy(tt, nk, dh, 1000));
```

```typescript title=solution
interface DongHoMoPhong { thoiGianHienTai: number; }
function taoDongHoMoPhong(): DongHoMoPhong { return { thoiGianHienTai: 0 }; }
function tienThoiGian(dh: DongHoMoPhong, soMs: number): void { dh.thoiGianHienTai += soMs; }

interface NhatKyTimKiem { cacTu: string[]; }
function taoNhatKyTimKiem(): NhatKyTimKiem { return { cacTu: [] }; }
function ghiNhanTimKiem(nk: NhatKyTimKiem, tu: string): void { nk.cacTu.push(tu); }

interface TrangThaiCapNhatDinhKy {
  bangTanSuat: Map<string, number>;
  thoiDiemCapNhatCuoi: number;
}
function taoTrangThaiCapNhatDinhKy(): TrangThaiCapNhatDinhKy {
  return { bangTanSuat: new Map(), thoiDiemCapNhatCuoi: 0 };
}

function chayCapNhatDinhKy(
  tt: TrangThaiCapNhatDinhKy,
  nk: NhatKyTimKiem,
  dh: DongHoMoPhong,
  chuKyMs: number,
): boolean {
  if (dh.thoiGianHienTai - tt.thoiDiemCapNhatCuoi < chuKyMs) return false;
  const bangMoi = new Map<string, number>();
  for (const tu of nk.cacTu) {
    bangMoi.set(tu, (bangMoi.get(tu) ?? 0) + 1);
  }
  tt.bangTanSuat = bangMoi;
  tt.thoiDiemCapNhatCuoi = dh.thoiGianHienTai;
  return true;
}

const dh = taoDongHoMoPhong();
const nk = taoNhatKyTimKiem();
const tt = taoTrangThaiCapNhatDinhKy();
ghiNhanTimKiem(nk, "meo");
tienThoiGian(dh, 1000);
console.log(chayCapNhatDinhKy(tt, nk, dh, 1000));
```

```typescript title=test
const dhT = taoDongHoMoPhong();
const nkT = taoNhatKyTimKiem();
const ttT = taoTrangThaiCapNhatDinhKy();
const chuKy = 1000;

ghiNhanTimKiem(nkT, "may tinh");
ghiNhanTimKiem(nkT, "may tinh");
ghiNhanTimKiem(nkT, "meo");

tienThoiGian(dhT, 999);
if (chayCapNhatDinhKy(ttT, nkT, dhT, chuKy) !== false) throw new Error("t=999 (< chu ky 1000) KHONG duoc chay cap nhat");
if (ttT.bangTanSuat.size !== 0) throw new Error("truoc lan cap nhat dau tien, bang tan suat phai rong");

tienThoiGian(dhT, 1);
if (chayCapNhatDinhKy(ttT, nkT, dhT, chuKy) !== true) throw new Error("t=1000 (dung chu ky) PHAI chay cap nhat");
if (ttT.bangTanSuat.get("may tinh") !== 2) throw new Error("'may tinh' xuat hien 2 lan trong log, tan suat phai la 2");
if (ttT.bangTanSuat.get("meo") !== 1) throw new Error("'meo' xuat hien 1 lan, tan suat phai la 1");

ghiNhanTimKiem(nkT, "moi");
if (chayCapNhatDinhKy(ttT, nkT, dhT, chuKy) !== false) throw new Error("goi lai NGAY (dong ho chua tien) KHONG duoc chay lai, bat ke log co du lieu moi hay khong");
if (ttT.bangTanSuat.has("moi")) throw new Error("'moi' chua duoc dua vao bang vi job chua chay lai");

tienThoiGian(dhT, 999);
if (chayCapNhatDinhKy(ttT, nkT, dhT, chuKy) !== false) throw new Error("van chua du 1000ms ke tu lan cap nhat cuoi");
tienThoiGian(dhT, 1);
if (chayCapNhatDinhKy(ttT, nkT, dhT, chuKy) !== true) throw new Error("du 1000ms ke tu lan cap nhat cuoi, PHAI chay lai");
if (ttT.bangTanSuat.get("moi") !== 1) throw new Error("sau lan chay thu hai, 'moi' phai xuat hien trong bang voi tan suat 1");
```

:::hints
- kind: attention
  body: "Dong dau tien phai SO SANH thoi gian da troi (dh.thoiGianHienTai - tt.thoiDiemCapNhatCuoi) voi chuKyMs -- neu CHUA du, return false ngay, khong dung toi phan rebuild ben duoi."
- kind: strategy
  body: "Neu (dh.thoiGianHienTai - tt.thoiDiemCapNhatCuoi) nho hon chuKyMs thi return false; nguoc lai moi cho phep chay tiep xuong duoi."
- kind: one-line
  body: "if (dh.thoiGianHienTai - tt.thoiDiemCapNhatCuoi < chuKyMs) return false;"
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
Search Autocomplete xong: trie, top-K cache, cập nhật định kỳ. Sang
YouTube — nơi một video KHÔNG hề sẵn sàng ngay lúc TẢI lên.
::::

::::reflect{#nghi-lai}
`chayCapNhatDinhKy` chỉ THÊM đúng một điều kiện Ở đầu hàm — NHƯNG điều
kiện đó chuyển hẳn một thao TÁC đắt (duyệt lại toàn bộ log) từ "chạy MỖI
lần gọi" thành "chạy MỖI khi đủ thời gian". `DongHoMoPhong` cho phép
kiểm chứng quyết định NÀY mà không cần chờ hàng giờ ĐỒNG hồ thật trôi
qua.
::::

::::checkpoint{mastery=0.75}
::::
