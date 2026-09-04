---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.vo-menh-lenh-thuc-thi
title: "Vỏ mệnh lệnh: điều phối và ghi, không tự quyết logic"
summary: "xuLyLenh(kho, lenh) la vo (shell): sinh id MOI, fold trang thai HIEN TAI tu kho.nhatKy, GOI quyetDinh (bai 6) de quyet dinh, roi APPEND su kien vao kho.nhatKy NEU duoc chap nhan -- vo CHI dieu phoi (goi loi, roi ghi), KHONG tu quyet logic nghiep vu nao; doi CHINH SACH quyet dinh (vd cho thau chi) ma KHONG can sua xuLyLenh, chi can doi ham duoc goi ben trong."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.vo-menh-lenh-thuc-thi]
requires: [sd.fp.loi-quyet-dinh-thuan]
concepts: [sd.fp.vo-menh-lenh-thuc-thi]
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
`quyetDinh` (bài trước) trả lời "điều gì NÊN xảy ra" — nhưng nó không
hề ghi gì cả. Không ai gọi nó, không ai gán `id`, không ai đưa kết quả
vào nhật ký. Cần một lớp NGOÀI đứng ra làm đúng những việc đó — và chỉ
đúng những việc đó.
::::

::::explain{#vo-dieu-phoi-khong-tu-quyet}
`xuLyLenh` là "vỏ mệnh lệnh" (imperative shell): nó sinh `id` mới
(`kho.boDemId`), tính trạng thái HIỆN tại bằng cách fold `kho.nhatKy`,
gọi `quyetDinh` (lõi thuần) để quyết định, rồi CHỈ append sự kiện vào
`kho.nhatKy` khi kết quả LÀ chấp nhận. Vỏ không hề tự kiểm tra số dư
hay tự quyết định logic nào — nó CHỈ điều phối:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };
type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };
function quyetDinh(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  if (lenh.loai === "nap") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_nap", soTien: lenh.soTien }] };
  }
  if (trangThai.soDu < lenh.soTien) {
    return { ketQua: "tu_choi", lyDo: "khong_du_so_du" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tru", soTien: lenh.soTien }] };
}

interface Kho { nhatKy: SuKien[]; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }

function xuLyLenh(kho: Kho, lenh: Lenh): KetQuaQuyetDinh {
  kho.boDemId += 1;
  const idMoi = "ev-" + kho.boDemId;
  const trangThaiHienTai = tinhTrangThai(kho.nhatKy);
  const ketQua = quyetDinh(trangThaiHienTai, lenh, idMoi);
  if (ketQua.ketQua === "chap_nhan") {
    kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi];
  }
  return ketQua;
}

const kho = taoKho();
console.log("nap 100000:", xuLyLenh(kho, { loai: "nap", soTien: 100000 }).ketQua);
console.log("rut 30000:", xuLyLenh(kho, { loai: "rut", soTien: 30000 }).ketQua);
console.log("nhat ky co", kho.nhatKy.length, "su kien");
console.log("so du hien tai (tinh boi fold):", tinhTrangThai(kho.nhatKy).soDu);

console.log("thu rut 999999 (vuot so du):", xuLyLenh(kho, { loai: "rut", soTien: 999999 }).ketQua);
console.log("nhat ky VAN chi co", kho.nhatKy.length, "su kien (lenh bi tu choi KHONG duoc ghi)");
```

```text title=readonly
nap 100000: chap_nhan
rut 30000: chap_nhan
nhat ky co 2 su kien
so du hien tai (tinh boi fold): 70000
thu rut 999999 (vuot so du): tu_choi
nhat ky VAN chi co 2 su kien (lenh bi tu choi KHONG duoc ghi)
```

`xuLyLenh` không hề chứa dòng nào SO SÁNH `soDu` với `soTien` — toàn
bộ logic đó nằm Ở `quyetDinh`. Việc của `xuLyLenh` chỉ là: sinh `id`,
tính trạng thái, HỎI lõi, VÀ ghi nếu được phép. Khi lệnh bị từ chối
(rút `999999`), nhật ký vẫn dừng đúng Ở `2` sự kiện — vỏ không hề
"lỡ" ghi gì thêm.
::::

::::example{#doi-chinh-sach-khong-sua-vo}
Vì vỏ không chứa logic nghiệp vụ, đổi CHÍNH SÁCH quyết định (ví dụ:
cho phép thấu chi tới một hạn mức) chỉ cần viết một `quyetDinh` KHÁC —
phần điều phối (sinh id, fold, append) giữ nguyên y hệt:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}
type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };
type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };
interface Kho { nhatKy: SuKien[]; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }

function quyetDinhChoThauChi(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  if (lenh.loai === "nap") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_nap", soTien: lenh.soTien }] };
  }
  if (trangThai.soDu - lenh.soTien < -50000) {
    return { ketQua: "tu_choi", lyDo: "vuot han muc thau chi" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tru", soTien: lenh.soTien }] };
}

function xuLyLenhThauChi(kho: Kho, lenh: Lenh): KetQuaQuyetDinh {
  kho.boDemId += 1;
  const idMoi = "ev-" + kho.boDemId;
  const trangThaiHienTai = tinhTrangThai(kho.nhatKy);
  const ketQua = quyetDinhChoThauChi(trangThaiHienTai, lenh, idMoi);
  if (ketQua.ketQua === "chap_nhan") {
    kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi];
  }
  return ketQua;
}

const khoThauChi = taoKho();
console.log("rut 30000 tu so du 0 (chinh sach MOI cho thau chi):", xuLyLenhThauChi(khoThauChi, { loai: "rut", soTien: 30000 }).ketQua);
console.log("so du sau khi thau chi:", tinhTrangThai(khoThauChi.nhatKy).soDu);
console.log("rut them 25000 nua (tong am 55000, VUOT han muc 50000):", xuLyLenhThauChi(khoThauChi, { loai: "rut", soTien: 25000 }).ketQua);
```

```text title=readonly
rut 30000 tu so du 0 (chinh sach MOI cho thau chi): chap_nhan
so du sau khi thau chi: -30000
rut them 25000 nua (tong am 55000, VUOT han muc 50000): tu_choi
```

`xuLyLenhThauChi` giống HỆT `xuLyLenh` về CẤU TRÚC (sinh id, fold, hỏi
lõi, append nếu chấp nhận) — điểm khác biệt DUY nhất là nó gọi
`quyetDinhChoThauChi` thay vì `quyetDinh`. Chính sách "không được âm"
đổi thành "được âm tới `-50000`" mà KHÔNG một dòng nào trong phần
điều phối cần sửa.
::::

::::predict{#doan-vo-tu-kiem-tra-rieng commitOnce}
Giả sử `xuLyLenh` được viết LẠI để tự thêm một điều kiện RIÊNG NGAY
bên trong nó — ví dụ `if (lenh.soTien > 1000000) return { ketQua:
"tu_choi", lyDo: "qua_lon" };` — TRƯỚC khi gọi `quyetDinh`, thay vì để
`quyetDinh` xử lý toàn bộ việc quyết định. Vấn đề kiến trúc Ở đây là
gì?

:::opt{correct}
Vỏ giờ đã TỰ quyết định một phần logic nghiệp vụ (giới hạn `1000000`)
— muốn kiểm tra luật đó phải chạy qua TOÀN bộ `xuLyLenh` (cần `kho`,
cần setup), mất khả năng test luật đó tách biệt chỉ bằng `quyetDinh`
với dữ liệu thuần
:::
:::opt
Không sao — miễn kết quả CUỐI cùng đúng (lệnh quá lớn vẫn bị từ chối),
logic nằm Ở lớp nào trong code không quan trọng
::why
Nhầm "kết quả đúng" với "kiến trúc đúng" — bài học Ở trên vừa chứng
minh giá trị THẬT của việc tách lõi/vỏ: đổi chính sách (`quyetDinh`
→ `quyetDinhChoThauChi`) mà không sửa phần điều phối. Nếu logic rải
rác Ở cả hai lớp, thay đổi chính sách đòi hỏi sửa CẢ vỏ lẫn lõi.

Chỗ lệch: mục đích của việc tách "lõi quyết định thuần" khỏi "vỏ mệnh
lệnh" không chỉ là để lõi test được không cần mock (bài 6) — nó còn
đảm bảo MỌI luật nghiệp vụ tập trung Ở đúng MỘT chỗ. Một điều kiện
nghiệp vụ nằm lẫn trong vỏ nghĩa là kiểm tra nó giờ cần setup `Kho`
đầy đủ, đúng thứ vỏ được sinh ra để TRÁNH.
::
:::
::::

::::code{#viet_xu_ly_lenh}
Hoàn thiện `xuLyLenh` — sinh `idMoi` bằng cách tăng `kho.boDemId`
trước rồi ghép chuỗi `"ev-" + kho.boDemId`, tính trạng thái hiện tại
bằng `tinhTrangThai(kho.nhatKy)`, gọi `quyetDinh` với trạng thái đó,
lệnh, VÀ `idMoi`. Nếu kết quả là `"chap_nhan"`, append toàn bộ
`ketQua.suKienMoi` vào `kho.nhatKy`. Luôn trả về `ketQua`.

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}
type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };
type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };
function quyetDinh(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  if (lenh.loai === "nap") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_nap", soTien: lenh.soTien }] };
  }
  if (trangThai.soDu < lenh.soTien) {
    return { ketQua: "tu_choi", lyDo: "khong_du_so_du" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tru", soTien: lenh.soTien }] };
}
interface Kho { nhatKy: SuKien[]; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }

function xuLyLenh(kho: Kho, lenh: Lenh): KetQuaQuyetDinh {
  ___
}

const khoX = taoKho();
xuLyLenh(khoX, { loai: "nap", soTien: 1000 });
console.log(khoX.nhatKy.length, tinhTrangThai(khoX.nhatKy).soDu);
```

```typescript title=solution
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function tinhTrangThai(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}
type Lenh =
  | { loai: "nap"; soTien: number }
  | { loai: "rut"; soTien: number };
type KetQuaQuyetDinh =
  | { ketQua: "chap_nhan"; suKienMoi: SuKien[] }
  | { ketQua: "tu_choi"; lyDo: string };
function quyetDinh(trangThai: TrangThai, lenh: Lenh, idSuKienMoi: string): KetQuaQuyetDinh {
  if (lenh.loai === "nap") {
    return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_nap", soTien: lenh.soTien }] };
  }
  if (trangThai.soDu < lenh.soTien) {
    return { ketQua: "tu_choi", lyDo: "khong_du_so_du" };
  }
  return { ketQua: "chap_nhan", suKienMoi: [{ id: idSuKienMoi, loai: "da_tru", soTien: lenh.soTien }] };
}
interface Kho { nhatKy: SuKien[]; boDemId: number; }
function taoKho(): Kho { return { nhatKy: [], boDemId: 0 }; }

function xuLyLenh(kho: Kho, lenh: Lenh): KetQuaQuyetDinh {
  kho.boDemId += 1;
  const idMoi = "ev-" + kho.boDemId;
  const trangThaiHienTai = tinhTrangThai(kho.nhatKy);
  const ketQua = quyetDinh(trangThaiHienTai, lenh, idMoi);
  if (ketQua.ketQua === "chap_nhan") {
    kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi];
  }
  return ketQua;
}

const khoX = taoKho();
xuLyLenh(khoX, { loai: "nap", soTien: 1000 });
console.log(khoX.nhatKy.length, tinhTrangThai(khoX.nhatKy).soDu);
```

```typescript title=test
const kho = taoKho();
const r1 = xuLyLenh(kho, { loai: "nap", soTien: 1000 });
if (r1.ketQua !== "chap_nhan") throw new Error("nap tien phai duoc chap nhan");
const lenSau1 = kho.nhatKy.length;
if (lenSau1 !== 1) throw new Error("lenh duoc chap nhan phai duoc GHI vao nhat ky");

const r2 = xuLyLenh(kho, { loai: "rut", soTien: 999999 });
if (r2.ketQua !== "tu_choi") throw new Error("rut vuot so du phai bi TU CHOI");
const lenSau2 = kho.nhatKy.length;
if (lenSau2 !== 1) throw new Error("lenh bi TU CHOI KHONG duoc ghi them vao nhat ky");

const r3 = xuLyLenh(kho, { loai: "rut", soTien: 400 });
if (r3.ketQua !== "chap_nhan") throw new Error("rut trong han muc phai duoc chap nhan");
const lenSau3 = kho.nhatKy.length;
if (lenSau3 !== 2) throw new Error("lenh chap nhan thu hai phai duoc ghi them, nang tong len 2");

const trangThaiCuoi = tinhTrangThai(kho.nhatKy);
if (trangThaiCuoi.soDu !== 600) throw new Error("so du cuoi phai la 1000 - 400 = 600 (lenh bi tu choi khong anh huong)");

const id1 = kho.nhatKy[0]?.id;
const id2 = kho.nhatKy[1]?.id;
if (id1 === undefined || id2 === undefined || id1 === id2) throw new Error("moi su kien duoc ghi phai co id RIENG, khong trung nhau");
```

:::hints
- kind: attention
  body: "Bon buoc theo dung thu tu: (1) kho.boDemId += 1; (2) const idMoi = 'ev-' + kho.boDemId; (3) tinh trangThaiHienTai = tinhTrangThai(kho.nhatKy), goi ketQua = quyetDinh(trangThaiHienTai, lenh, idMoi); (4) neu ketQua.ketQua === 'chap_nhan' thi kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi]; luon return ketQua."
- kind: strategy
  body: "kho.boDemId += 1; const idMoi = 'ev-' + kho.boDemId; const trangThaiHienTai = tinhTrangThai(kho.nhatKy); const ketQua = quyetDinh(trangThaiHienTai, lenh, idMoi); if (ketQua.ketQua === 'chap_nhan') { kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi]; } return ketQua;"
- kind: one-line
  body: "kho.boDemId += 1; const idMoi = \"ev-\" + kho.boDemId; const trangThaiHienTai = tinhTrangThai(kho.nhatKy); const ketQua = quyetDinh(trangThaiHienTai, lenh, idMoi); if (ketQua.ketQua === \"chap_nhan\") { kho.nhatKy = [...kho.nhatKy, ...ketQua.suKienMoi]; } return ketQua;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "1 1000"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Lõi thuần quyết định, vỏ điều phối và ghi — mỗi lệnh chấp nhận đều tạo
ra trạng thái ĐÚNG. Nhưng nếu nhật ký bị hỏng, hay cache bị xoá sạch —
liệu có tin tưởng được là mọi thứ có thể dựng LẠI, chỉ từ chính nhật
ký?
::::

::::reflect{#nghi-lai}
`xuLyLenh` là nơi DUY NHẤT trong hệ thống được phép mutate `kho` — và
nó chỉ làm đúng việc đó: sinh id, đọc trạng thái, hỏi lõi, ghi nếu
được phép. Không một dòng nào trong nó biết "số dư âm là sai" — kiến
thức đó nằm TRỌN Ở `quyetDinh`. Tách bạch này chính là "functional
core, imperative shell": lõi mang toàn bộ trí tuệ nghiệp vụ VÀ không
đụng thế giới bên ngoài; vỏ đụng thế giới bên ngoài VÀ không mang trí
tuệ nghiệp vụ nào.
::::

::::checkpoint{mastery=0.78}
::::
