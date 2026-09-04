---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.boss-nhat-ky-bat-bien-va-fold
title: "BOSS — dịch vụ đếm không có state nào ngoài chính log"
summary: "xuLyLenhDem(kho, lenh) rap DUNG bon manh: sinh id, fold trang thai tu kho.nhatKy (bai 2-3), goi quyetDinhDem THUAN (bai 6, tu choi neu giam AM), append su kien NEU chap nhan (bai 7) -- ap dung cho mot 'dich vu dem' hoan toan khong co state mutable nao ngoai chinh nhat ky; tang 5, tang 3, giam 4 ra gia tri 4, giam 100 (vuot) bi TU CHOI, nhat ky co lap van fold ra DUNG gia tri (idempotent, bai 4-5)."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 10
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [sd.fp.boss-nhat-ky-bat-bien-va-fold]
requires: [sd.fp.sua-loi-bang-su-kien-bu-tru]
concepts: [sd.fp.boss-nhat-ky-bat-bien-va-fold]
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
Chín bài — sự kiện bất biến, trạng thái là fold, snapshot, idempotent,
exactly-once, lõi thuần, vỏ mệnh lệnh, tái tạo từ đầu, sửa lỗi bằng bù
trừ. Tất cả đều dùng VÍ điện tử làm ví dụ. Giờ ráp lại toàn bộ cho một
dịch vụ HOÀN TOÀN khác — một bộ ĐẾM — để chứng minh những khái niệm
này không phải mẹo riêng cho ví tiền, mà là một khung áp dụng ĐƯỢC cho
bất cứ thứ gì có trạng thái.
::::

::::explain{#rap_dich_vu_dem}
`xuLyLenhDem` ráp đúng các mảnh đã xây: sinh `id` mới, tính trạng thái
HIỆN tại bằng cách fold `kho.nhatKy` qua `apDungDem` (idempotent —
bài 4), hỏi `quyetDinhDem` (lõi thuần — bài 6: từ chối nếu giảm khiến
giá trị ÂM), rồi append sự kiện nếu được chấp nhận (vỏ mệnh lệnh — bài
7). Không một field `giaTri` nào được lưu trực tiếp Ở bất cứ đâu — chỉ
`kho.nhatKy` tồn tại, mọi thứ khác đều được TÍNH từ nó:

```typescript title=readonly
type LoaiSuKienDem = "da_tang" | "da_giam";
interface SuKienDem { id: string; loai: LoaiSuKienDem; buoc: number; }

interface TrangThaiDem { giaTri: number; idDaApDung: Set<string>; }
function trangThaiDemBanDau(): TrangThaiDem { return { giaTri: 0, idDaApDung: new Set() }; }

function apDungDem(trangThai: TrangThaiDem, suKien: SuKienDem): TrangThaiDem {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const giaTriMoi = suKien.loai === "da_tang" ? trangThai.giaTri + suKien.buoc : trangThai.giaTri - suKien.buoc;
  return { giaTri: giaTriMoi, idDaApDung: idMoi };
}

function tinhTrangThaiDem(nhatKy: SuKienDem[]): TrangThaiDem {
  return nhatKy.reduce(apDungDem, trangThaiDemBanDau());
}

type LenhDem =
  | { loai: "tang"; buoc: number }
  | { loai: "giam"; buoc: number };

type KetQuaQuyetDinhDem =
  | { ketQua: "chap_nhan"; suKienMoi: SuKienDem[] }
  | { ketQua: "tu_choi"; lyDo: string };

function quyetDinhDem(trangThai: TrangThaiDem, lenh: LenhDem, idSuKienMoi: string): KetQuaQuyetDinhDem {
  if (lenh.loai === "tang") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tang", buoc: lenh.buoc }] };
  }
  if (trangThai.giaTri - lenh.buoc < 0) {
    return { ketQua: "tu_choi", lyDo: "khong_duoc_am" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_giam", buoc: lenh.buoc }] };
}

interface KhoDem { nhatKy: SuKienDem[]; boDemId: number; }
function taoKhoDem(): KhoDem { return { nhatKy: [], boDemId: 0 }; }

function xuLyLenhDem(kho: KhoDem, lenh: LenhDem): KetQuaQuyetDinhDem {
  kho.boDemId += 1;
  const idMoi = "ev-" + kho.boDemId;
  const trangThaiHienTai = tinhTrangThaiDem(kho.nhatKy);
  const ketQua = quyetDinhDem(trangThaiHienTai, lenh, idMoi);
  if (ketQua.ketQua === "chap_nhan") {
    kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi];
  }
  return ketQua;
}

const kho = taoKhoDem();
console.log("tang 5:", xuLyLenhDem(kho, { loai: "tang", buoc: 5 }).ketQua);
console.log("tang 3:", xuLyLenhDem(kho, { loai: "tang", buoc: 3 }).ketQua);
console.log("giam 4:", xuLyLenhDem(kho, { loai: "giam", buoc: 4 }).ketQua);
console.log("gia tri hien tai (fold tu nhat ky):", tinhTrangThaiDem(kho.nhatKy).giaTri);

console.log("thu giam 100 (vuot gia tri hien tai, se AM):", xuLyLenhDem(kho, { loai: "giam", buoc: 100 }).ketQua);
console.log("nhat ky VAN chi co", kho.nhatKy.length, "su kien (lenh bi tu choi khong duoc ghi)");

const taiTao = tinhTrangThaiDem(kho.nhatKy);
console.log("tai tao tu dau CHI tu nhat ky tho:", taiTao.giaTri);

const suKienDauTien = kho.nhatKy[0]!;
const nhatKyCoLap: SuKienDem[] = [...kho.nhatKy, suKienDauTien];
console.log("nhat ky CO LAP (them 1 ban sao cua su kien dau):", nhatKyCoLap.length, "su kien tho");
console.log("gia tri fold tu nhat ky CO LAP:", tinhTrangThaiDem(nhatKyCoLap).giaTri);
console.log("van giong nhat ky KHONG lap (nho apDungDem idempotent)?", tinhTrangThaiDem(nhatKyCoLap).giaTri === tinhTrangThaiDem(kho.nhatKy).giaTri);
```

```text title=readonly
tang 5: chap_nhan
tang 3: chap_nhan
giam 4: chap_nhan
gia tri hien tai (fold tu nhat ky): 4
thu giam 100 (vuot gia tri hien tai, se AM): tu_choi
nhat ky VAN chi co 3 su kien (lenh bi tu choi khong duoc ghi)
tai tao tu dau CHI tu nhat ky tho: 4
nhat ky CO LAP (them 1 ban sao cua su kien dau): 4 su kien tho
gia tri fold tu nhat ky CO LAP: 4
van giong nhat ky KHONG lap (nho apDungDem idempotent)? true
```

Ba lệnh hợp lệ (`tăng 5`, `tăng 3`, `giảm 4`) đưa giá trị lên `4`,
đúng bằng `5 + 3 - 4`. Lệnh `giảm 100` bị TỪ CHỐI ngay Ở lõi
(`quyetDinhDem`) vì `4 - 100 < 0` — nhật ký giữ nguyên `3` sự kiện.
Tái tạo từ nhật ký thô ra đúng lại `4` (bài 8). VÀ khi một sự kiện bị
LẶP trong nhật ký thô (mô phỏng giao lặp Ở tầng log), fold vẫn ra
ĐÚNG `4` như không hề có bản sao nào — vì `apDungDem` kiểm `id` trước
khi cộng dồn (bài 4-5).
::::

::::example{#loi_test_doc_lap_khoi_vo}
Vì `quyetDinhDem` hoàn toàn thuần, nó có thể được gọi TRỰC TIẾP với dữ
liệu bịa ra — không cần `kho`, không cần vỏ — VÀ vẫn cho ra đúng quyết
định mà vỏ Ở trên đã đưa ra một cách gián tiếp:

```typescript title=readonly
type LoaiSuKienDem = "da_tang" | "da_giam";
interface SuKienDem { id: string; loai: LoaiSuKienDem; buoc: number; }
interface TrangThaiDem { giaTri: number; idDaApDung: Set<string>; }
function trangThaiDemBanDau(): TrangThaiDem { return { giaTri: 0, idDaApDung: new Set() }; }
function apDungDem(trangThai: TrangThaiDem, suKien: SuKienDem): TrangThaiDem {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const giaTriMoi = suKien.loai === "da_tang" ? trangThai.giaTri + suKien.buoc : trangThai.giaTri - suKien.buoc;
  return { giaTri: giaTriMoi, idDaApDung: idMoi };
}
function tinhTrangThaiDem(nhatKy: SuKienDem[]): TrangThaiDem {
  return nhatKy.reduce(apDungDem, trangThaiDemBanDau());
}
type LenhDem =
  | { loai: "tang"; buoc: number }
  | { loai: "giam"; buoc: number };
type KetQuaQuyetDinhDem =
  | { ketQua: "chap_nhan"; suKienMoi: SuKienDem[] }
  | { ketQua: "tu_choi"; lyDo: string };
function quyetDinhDem(trangThai: TrangThaiDem, lenh: LenhDem, idSuKienMoi: string): KetQuaQuyetDinhDem {
  if (lenh.loai === "tang") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tang", buoc: lenh.buoc }] };
  }
  if (trangThai.giaTri - lenh.buoc < 0) {
    return { ketQua: "tu_choi", lyDo: "khong_duoc_am" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_giam", buoc: lenh.buoc }] };
}
interface KhoDem { nhatKy: SuKienDem[]; boDemId: number; }
function taoKhoDem(): KhoDem { return { nhatKy: [], boDemId: 0 }; }
function xuLyLenhDem(kho: KhoDem, lenh: LenhDem): KetQuaQuyetDinhDem {
  kho.boDemId += 1;
  const idMoi = "ev-" + kho.boDemId;
  const trangThaiHienTai = tinhTrangThaiDem(kho.nhatKy);
  const ketQua = quyetDinhDem(trangThaiHienTai, lenh, idMoi);
  if (ketQua.ketQua === "chap_nhan") {
    kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi];
  }
  return ketQua;
}

const kho2 = taoKhoDem();
xuLyLenhDem(kho2, { loai: "tang", buoc: 10 });
xuLyLenhDem(kho2, { loai: "giam", buoc: 3 });
const trangThaiHienTai2 = tinhTrangThaiDem(kho2.nhatKy);
console.log("gia tri sau 2 lenh (qua vo xuLyLenhDem):", trangThaiHienTai2.giaTri);

const ketQuaThangTuLoi = quyetDinhDem(trangThaiHienTai2, { loai: "giam", buoc: 3 }, "test-truc-tiep");
console.log("goi TRUC TIEP quyetDinhDem (khong qua kho, khong qua vo):", ketQuaThangTuLoi.ketQua);

const ketQuaQuaVo = xuLyLenhDem(kho2, { loai: "giam", buoc: 3 });
console.log("qua vo (co ghi that vao kho):", ketQuaQuaVo.ketQua);
console.log("hai duong (truc tiep loi thuan VS qua vo) cho CUNG quyet dinh?", ketQuaThangTuLoi.ketQua === ketQuaQuaVo.ketQua);
```

```text title=readonly
gia tri sau 2 lenh (qua vo xuLyLenhDem): 7
goi TRUC TIEP quyetDinhDem (khong qua kho, khong qua vo): chap_nhan
qua vo (co ghi that vao kho): chap_nhan
hai duong (truc tiep loi thuan VS qua vo) cho CUNG quyet dinh? true
```

Gọi `quyetDinhDem` trực tiếp với `trangThaiHienTai2` (lấy từ fold)
cho ra ĐÚNG cùng quyết định (`chap_nhan`) như gọi qua toàn bộ
`xuLyLenhDem` — vì vỏ không hề thêm hay bớt logic nào, nó chỉ điều
phối tới đúng lõi đó. Kiểm tra một luật nghiệp vụ không cần dựng cả
`kho`; chỉ cần trạng thái VÀ lệnh.
::::

::::predict{#doan-giam-dung-bang-gia-tri commitOnce}
Ở đoạn `explain`, sau ba lệnh hợp lệ, `kho.nhatKy` có `3` sự kiện VÀ
giá trị hiện tại là `4`. Gọi thêm `xuLyLenhDem(kho, { loai: "giam",
buoc: 4 })` (giảm ĐÚNG BẰNG giá trị hiện tại) — `ketQua` VÀ độ dài
`kho.nhatKy` SAU lệnh gọi đó là gì?

:::opt{correct}
`ketQua.ketQua` là `"chap_nhan"`, VÀ `kho.nhatKy.length` trở thành
`4` — điều kiện từ chối trong `quyetDinhDem` là `trangThai.giaTri -
lenh.buoc < 0`, tức `4 - 4 < 0`, là `false`; giảm về đúng `0` là hợp
lệ, không phải một trường hợp bị chặn
:::
:::opt
`ketQua.ketQua` là `"tu_choi"`, VÀ `kho.nhatKy.length` giữ nguyên `3`
— vì giá trị không được phép CHẠM tới `0`, chỉ được phép giảm tới khi
còn dương
::why
Nhầm điều kiện chặn LÀ "không được bằng `0`" thay vì đúng điều kiện đã
viết LÀ "không được ÂM". Hai điều kiện này khác nhau đúng Ở biên: `0`
là giá trị hợp lệ, chỉ có SỐ ÂM mới bị từ chối.

Chỗ lệch: dòng `if (trangThai.giaTri - lenh.buoc < 0)` dùng toán tử
`<`, không phải `<=`. Với `trangThai.giaTri = 4` VÀ `lenh.buoc = 4`,
biểu thức `4 - 4 < 0` là `0 < 0`, tức `false` — điều kiện từ chối
KHÔNG được kích hoạt, nên hàm rơi xuống nhánh chấp nhận Ở cuối. Đây
đúng là tình huống "rút đúng bằng số dư" của bài 6, tái hiện lại
trong dịch vụ đếm: chạm đúng ngưỡng biên vẫn LÀ hợp lệ.
::
:::
::::

::::code{#viet_xu_ly_lenh_dem}
Hoàn thiện `xuLyLenhDem` — sinh `idMoi` bằng cách tăng `kho.boDemId`
rồi ghép `"ev-" + kho.boDemId`, tính trạng thái hiện tại bằng
`tinhTrangThaiDem(kho.nhatKy)`, gọi `quyetDinhDem` với trạng thái đó,
`lenh`, VÀ `idMoi`. Nếu kết quả là `"chap_nhan"`, append toàn bộ
`ketQua.suKienMoi` vào `kho.nhatKy`. Luôn trả về `ketQua`.

```typescript title=starter
type LoaiSuKienDem = "da_tang" | "da_giam";
interface SuKienDem { id: string; loai: LoaiSuKienDem; buoc: number; }
interface TrangThaiDem { giaTri: number; idDaApDung: Set<string>; }
function trangThaiDemBanDau(): TrangThaiDem { return { giaTri: 0, idDaApDung: new Set() }; }
function apDungDem(trangThai: TrangThaiDem, suKien: SuKienDem): TrangThaiDem {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const giaTriMoi = suKien.loai === "da_tang" ? trangThai.giaTri + suKien.buoc : trangThai.giaTri - suKien.buoc;
  return { giaTri: giaTriMoi, idDaApDung: idMoi };
}
function tinhTrangThaiDem(nhatKy: SuKienDem[]): TrangThaiDem {
  return nhatKy.reduce(apDungDem, trangThaiDemBanDau());
}
type LenhDem =
  | { loai: "tang"; buoc: number }
  | { loai: "giam"; buoc: number };
type KetQuaQuyetDinhDem =
  | { ketQua: "chap_nhan"; suKienMoi: SuKienDem[] }
  | { ketQua: "tu_choi"; lyDo: string };
function quyetDinhDem(trangThai: TrangThaiDem, lenh: LenhDem, idSuKienMoi: string): KetQuaQuyetDinhDem {
  if (lenh.loai === "tang") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tang", buoc: lenh.buoc }] };
  }
  if (trangThai.giaTri - lenh.buoc < 0) {
    return { ketQua: "tu_choi", lyDo: "khong_duoc_am" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_giam", buoc: lenh.buoc }] };
}
interface KhoDem { nhatKy: SuKienDem[]; boDemId: number; }
function taoKhoDem(): KhoDem { return { nhatKy: [], boDemId: 0 }; }

function xuLyLenhDem(kho: KhoDem, lenh: LenhDem): KetQuaQuyetDinhDem {
  ___
}

const khoX = taoKhoDem();
xuLyLenhDem(khoX, { loai: "tang", buoc: 7 });
console.log(khoX.nhatKy.length, tinhTrangThaiDem(khoX.nhatKy).giaTri);
```

```typescript title=solution
type LoaiSuKienDem = "da_tang" | "da_giam";
interface SuKienDem { id: string; loai: LoaiSuKienDem; buoc: number; }
interface TrangThaiDem { giaTri: number; idDaApDung: Set<string>; }
function trangThaiDemBanDau(): TrangThaiDem { return { giaTri: 0, idDaApDung: new Set() }; }
function apDungDem(trangThai: TrangThaiDem, suKien: SuKienDem): TrangThaiDem {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const giaTriMoi = suKien.loai === "da_tang" ? trangThai.giaTri + suKien.buoc : trangThai.giaTri - suKien.buoc;
  return { giaTri: giaTriMoi, idDaApDung: idMoi };
}
function tinhTrangThaiDem(nhatKy: SuKienDem[]): TrangThaiDem {
  return nhatKy.reduce(apDungDem, trangThaiDemBanDau());
}
type LenhDem =
  | { loai: "tang"; buoc: number }
  | { loai: "giam"; buoc: number };
type KetQuaQuyetDinhDem =
  | { ketQua: "chap_nhan"; suKienMoi: SuKienDem[] }
  | { ketQua: "tu_choi"; lyDo: string };
function quyetDinhDem(trangThai: TrangThaiDem, lenh: LenhDem, idSuKienMoi: string): KetQuaQuyetDinhDem {
  if (lenh.loai === "tang") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tang", buoc: lenh.buoc }] };
  }
  if (trangThai.giaTri - lenh.buoc < 0) {
    return { ketQua: "tu_choi", lyDo: "khong_duoc_am" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_giam", buoc: lenh.buoc }] };
}
interface KhoDem { nhatKy: SuKienDem[]; boDemId: number; }
function taoKhoDem(): KhoDem { return { nhatKy: [], boDemId: 0 }; }

function xuLyLenhDem(kho: KhoDem, lenh: LenhDem): KetQuaQuyetDinhDem {
  kho.boDemId += 1;
  const idMoi = "ev-" + kho.boDemId;
  const trangThaiHienTai = tinhTrangThaiDem(kho.nhatKy);
  const ketQua = quyetDinhDem(trangThaiHienTai, lenh, idMoi);
  if (ketQua.ketQua === "chap_nhan") {
    kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi];
  }
  return ketQua;
}

const khoX = taoKhoDem();
xuLyLenhDem(khoX, { loai: "tang", buoc: 7 });
console.log(khoX.nhatKy.length, tinhTrangThaiDem(khoX.nhatKy).giaTri);
```

```typescript title=test
const kho = taoKhoDem();
const r1 = xuLyLenhDem(kho, { loai: "tang", buoc: 10 });
if (r1.ketQua !== "chap_nhan") throw new Error("tang LUON phai duoc chap nhan");
const lenSau1 = kho.nhatKy.length;
if (lenSau1 !== 1) throw new Error("lenh tang phai duoc ghi vao nhat ky");

const r2 = xuLyLenhDem(kho, { loai: "giam", buoc: 10 });
if (r2.ketQua !== "chap_nhan") throw new Error("giam DUNG BANG gia tri hien tai (con lai 0) phai duoc chap nhan");
const lenSau2 = kho.nhatKy.length;
if (lenSau2 !== 2) throw new Error("lenh giam hop le phai duoc ghi them");

const giaTriSau2 = tinhTrangThaiDem(kho.nhatKy).giaTri;
if (giaTriSau2 !== 0) throw new Error("gia tri phai la 0 sau tang 10 roi giam 10");

const r3 = xuLyLenhDem(kho, { loai: "giam", buoc: 1 });
if (r3.ketQua !== "tu_choi") throw new Error("giam khi gia tri dang la 0 phai bi TU CHOI (khong duoc am)");
const lenSau3 = kho.nhatKy.length;
if (lenSau3 !== 2) throw new Error("lenh bi tu choi KHONG duoc ghi them vao nhat ky");

const id1 = kho.nhatKy[0]?.id;
const id2 = kho.nhatKy[1]?.id;
if (id1 === undefined || id2 === undefined || id1 === id2) throw new Error("moi su kien duoc ghi phai co id RIENG");
```

:::hints
- kind: attention
  body: "Cau truc GIONG HET xuLyLenh cua bai 7, chi doi ten ham/kieu sang '...Dem': (1) kho.boDemId += 1; (2) const idMoi = 'ev-' + kho.boDemId; (3) tinh trangThaiHienTai = tinhTrangThaiDem(kho.nhatKy), goi ketQua = quyetDinhDem(trangThaiHienTai, lenh, idMoi); (4) neu chap_nhan thi append ketQua.suKienMoi vao kho.nhatKy; luon return ketQua."
- kind: strategy
  body: "kho.boDemId += 1; const idMoi = 'ev-' + kho.boDemId; const trangThaiHienTai = tinhTrangThaiDem(kho.nhatKy); const ketQua = quyetDinhDem(trangThaiHienTai, lenh, idMoi); if (ketQua.ketQua === 'chap_nhan') { kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi]; } return ketQua;"
- kind: one-line
  body: "kho.boDemId += 1; const idMoi = \"ev-\" + kho.boDemId; const trangThaiHienTai = tinhTrangThaiDem(kho.nhatKy); const ketQua = quyetDinhDem(trangThaiHienTai, lenh, idMoi); if (ketQua.ketQua === \"chap_nhan\") { kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi]; } return ketQua;"
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: contains
  expect: "1 7"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ví điện tử, bộ đếm — cùng một khung, hai domain hoàn toàn khác nhau.
`xuLyLenhDem` không hề "biết" nó đang đếm — nó chỉ điều phối lõi
thuần VÀ ghi vào một nhật ký bất biến. Quest "Nhật ký bất biến và
fold" đã xong — nền đã dọn xong cho hai quest tiếp theo: hợp nhất dữ
liệu phân tán (CRDT), và xử lý luồng sự kiện liên tục (streaming).
::::

::::reflect{#nghi-lai}
`xuLyLenhDem` không hề phát minh gì mới — nó ráp đúng thứ tự các mảnh
đã xây riêng lẻ Ở chín bài trước, chỉ đổi domain từ ví sang bộ đếm.
Đây chính LÀ điểm khác biệt cốt lõi với track "Thiết kế thực chiến"
vừa xong: Ở đó, đúng đắn được đảm bảo bằng THỨ TỰ các thao tác mutate
đúng lúc; Ở đây, đúng đắn được đảm bảo bằng việc KHÔNG có state
mutable nào để mutate sai lúc — chỉ có một nhật ký chỉ-được-thêm, VÀ
mọi trạng thái đều LÀ một hàm thuần của nhật ký đó.
::::

::::checkpoint{mastery=0.85}
::::
