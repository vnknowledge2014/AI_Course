---
id: thiet-ke-he-thong.trien-khai-an-toan.canary-ket-hop-rollback-tu-dong
title: "Kết hợp canary + rollback: tự động dừng và lùi lại"
summary: "xuLyMotBuocCanary(trangThai, soLoi, soRequest, nguong, buocTang) rap bai 2 (tang dan) VOI bai 7 (rollback ngay): tang tyLeHienTai neu duoi nguong, nhung ROLLBACK VE 0 (mot buoc, khong giam dan) ngay khi mot buoc BAT KY vuot nguong. chayCanaryTuDong lap qua danh sach du lieu tung buoc, DUNG NGAY khi rollback xay ra -- vi du 4 buoc du lieu (2/1000, 5/1000, 80/1000, 1/1000) voi nguong 5%, buoc tang 20: chi chay 3 buoc (20 -> 40 -> rollback ve 0), buoc 4 KHONG BAO GIO chay toi."
locale: vi
track: thiet-ke-he-thong
module: trien-khai-an-toan
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [sd.canary-ket-hop-rollback-tu-dong]
requires: [sd.rollback-tu-dong-theo-nguong]
concepts: [sd.canary-ket-hop-rollback-tu-dong]
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
Bài 2 dạy tăng canary từng bước, dừng khi vượt ngưỡng. Bài 7 dạy
rollback tức thì khi metric xấu. Nhưng "dừng" (bài 2) VÀ "rollback"
(bài 7) không phải LÀ MỘT — dừng chỉ giữ NGUYÊN tỷ lệ hiện tại, còn
rollback ĐƯA nó VỀ hẳn `0`. Ráp đúng hai mảnh NÀY lại LÀ điều một
pipeline canary THẬT sự cần.
::::

::::explain{#xu-ly-mot-buoc-canary}
`xuLyMotBuocCanary` gộp quyết định tăng (bài 2) VÀ hành động rollback
(bài 7) VÀO một hàm DUY nhất: tỷ lệ lỗi dưới ngưỡng thì TĂNG dần như
bình thường; vượt ngưỡng thì KHÔNG dừng ở tỷ lệ hiện tại — LÙI thẳng
VỀ `0`:

```typescript title=readonly
interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
function taoTrangThaiCanaryRollback(): TrangThaiCanaryRollback {
  return { tyLeHienTai: 0, daRollback: false };
}

type KetQuaBuoc = "da_tang" | "rollback_ve_0";

function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

const ts = taoTrangThaiCanaryRollback();
console.log("buoc 1 (2 loi/1000, nguong 5%):", xuLyMotBuocCanary(ts, 2, 1000, 0.05, 20));
console.log("ty le sau buoc 1:", ts.tyLeHienTai);
console.log("buoc 2 (5 loi/1000, van duoi nguong):", xuLyMotBuocCanary(ts, 5, 1000, 0.05, 20));
console.log("ty le sau buoc 2:", ts.tyLeHienTai);
console.log("buoc 3 (80 loi/1000, VUOT nguong):", xuLyMotBuocCanary(ts, 80, 1000, 0.05, 20));
console.log("ty le sau buoc 3 (rollback VE 0):", ts.tyLeHienTai);
console.log("da rollback:", ts.daRollback);
```

```text title=readonly
buoc 1 (2 loi/1000, nguong 5%): da_tang
ty le sau buoc 1: 20
buoc 2 (5 loi/1000, van duoi nguong): da_tang
ty le sau buoc 2: 40
buoc 3 (80 loi/1000, VUOT nguong): rollback_ve_0
ty le sau buoc 3 (rollback VE 0): 0
da rollback: true
```

Hai bước ĐẦU tăng đều đặn: `20`, RỒI `40`. Bước BA, tỷ lệ lỗi
`80/1000 = 8%` vượt ngưỡng `5%` — thay vì "dừng Ở `40`" (như bài 2 làm
riêng lẻ), hàm ĐƯA tỷ lệ VỀ `0` NGAY LẬP TỨC, giống hệt cách
`quyetDinhRollback` của bài 7 hành động.
::::

::::example{#chay-canary-tu-dong}
`chayCanaryTuDong` chạy qua một danh sách dữ liệu QUAN sát theo từng
bước, gọi `xuLyMotBuocCanary` cho MỖI bước, VÀ dừng LẠI ngay khi có
rollback xảy ra — các bước dữ liệu CÒN LẠI, dù có sẵn, không bao giờ
được xử lý TỚI:

```typescript title=readonly
interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
function taoTrangThaiCanaryRollback(): TrangThaiCanaryRollback {
  return { tyLeHienTai: 0, daRollback: false };
}
type KetQuaBuoc = "da_tang" | "rollback_ve_0";
function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

interface DuLieuBuoc { soLoi: number; soRequest: number; }

function chayCanaryTuDong(
  trangThai: TrangThaiCanaryRollback,
  cacBuoc: DuLieuBuoc[],
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc[] {
  const cacKetQua: KetQuaBuoc[] = [];
  for (const buoc of cacBuoc) {
    const kq = xuLyMotBuocCanary(trangThai, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang);
    cacKetQua.push(kq);
    if (kq === "rollback_ve_0") break;
  }
  return cacKetQua;
}

// bon buoc DU LIEU da chuan bi san -- buoc thu 3 co ty le loi cao bat thuong
const cacBuoc: DuLieuBuoc[] = [
  { soLoi: 2, soRequest: 1000 },
  { soLoi: 5, soRequest: 1000 },
  { soLoi: 80, soRequest: 1000 },
  { soLoi: 1, soRequest: 1000 },
];

const ts = taoTrangThaiCanaryRollback();
const ketQua = chayCanaryTuDong(ts, cacBuoc, 0.05, 20);

console.log("cac ket qua tung buoc (DUNG NGAY o buoc vo, KHONG chay buoc 4):", JSON.stringify(ketQua));
console.log("so buoc THAT su da chay:", ketQua.length);
console.log("ty le canary cuoi cung:", ts.tyLeHienTai);
console.log("da rollback:", ts.daRollback);
```

```text title=readonly
cac ket qua tung buoc (DUNG NGAY o buoc vo, KHONG chay buoc 4): ["da_tang","da_tang","rollback_ve_0"]
so buoc THAT su da chay: 3
ty le canary cuoi cung: 0
da rollback: true
```

Bốn bước dữ liệu ĐÃ chuẩn bị sẵn, nhưng chỉ `3` bước THẬT sự được xử
lý — bước THỨ tư (`1` lỗi trên `1000`, hoàn toàn AN toàn) không bao
giờ được xét TỚI, vì vòng lặp đã `break` NGAY khi bước ba trả về
`"rollback_ve_0"`. Đây LÀ đúng tinh thần "dừng VÀ lùi lại Ở BẤT KỲ
bước nào" — không tiếp tục tăng dù dữ liệu sau đó có tốt trở LẠI.
::::

::::predict{#doan-bien-ket-hop commitOnce}
Trạng thái đang Ở `tyLeHienTai=40`. Gọi `xuLyMotBuocCanary(ts, 3, 100,
0.03, 20)` — `3` lỗi trên `100` request LÀ tỷ lệ ĐÚNG BẰNG ngưỡng
`0.03`. Kết quả LÀ gì, VÀ `tyLeHienTai` sau đó bằng bao nhiêu?

:::opt{correct}
`"da_tang"`, `tyLeHienTai` thành `60` — điều kiện rollback dùng `>`
(nghiêm ngặt); `0.03 > 0.03` LÀ `false`, nên tỷ lệ lỗi bằng đúng
ngưỡng KHÔNG kích hoạt rollback, hàm rơi xuống nhánh tăng bình thường
:::
:::opt
`"rollback_ve_0"`, `tyLeHienTai` thành `0` — khi ĐÃ kết hợp cả canary
LẪN rollback vào một hàm, ngưỡng nên được kiểm tra NGHIÊM ngặt hơn
(bao gồm cả trường hợp bằng) để an toàn tối đa
::why
Nhầm "phiên bản kết hợp nên thận trọng hơn" VỚI "điều kiện code THẬT
sự không đổi khi ráp hai hàm lại" — `xuLyMotBuocCanary` giữ NGUYÊN
đúng điều kiện `>` từ `quyetDinhTangCanary` (bài 2) VÀ
`quyetDinhRollback` (bài 7), không siết chặt thêm.

Chỗ lệch: dòng `if (tyLeLoi > nguongTyLeLoi)` dùng phép so sánh nghiêm
ngặt. Với `tyLeLoi = 3/100 = 0.03` VÀ `nguongTyLeLoi = 0.03`, biểu
thức LÀ `0.03 > 0.03`, cho `false` — nhánh rollback KHÔNG chạy. Hàm đi
tiếp xuống dòng tăng, cộng `buocTang=20` vào `40` thành `60`, VÀ trả
về `"da_tang"`.
::
:::
::::

::::code{#viet_chay_canary_tu_dong}
Hoàn thiện `chayCanaryTuDong` — với MỖI bước dữ liệu, gọi
`xuLyMotBuocCanary`, đẩy kết quả vào mảng, VÀ dừng vòng lặp NGAY nếu
kết quả LÀ `"rollback_ve_0"`.

```typescript title=starter
interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
function taoTrangThaiCanaryRollback(): TrangThaiCanaryRollback {
  return { tyLeHienTai: 0, daRollback: false };
}
type KetQuaBuoc = "da_tang" | "rollback_ve_0";
function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

interface DuLieuBuoc { soLoi: number; soRequest: number; }

function chayCanaryTuDong(
  trangThai: TrangThaiCanaryRollback,
  cacBuoc: DuLieuBuoc[],
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc[] {
  const cacKetQua: KetQuaBuoc[] = [];
  for (const buoc of cacBuoc) {
    ___
  }
  return cacKetQua;
}

const tsX = taoTrangThaiCanaryRollback();
const ketQuaX = chayCanaryTuDong(tsX, [{ soLoi: 1, soRequest: 100 }, { soLoi: 1, soRequest: 100 }], 0.05, 25);
console.log(ketQuaX.length, tsX.tyLeHienTai);
```

```typescript title=solution
interface TrangThaiCanaryRollback { tyLeHienTai: number; daRollback: boolean; }
function taoTrangThaiCanaryRollback(): TrangThaiCanaryRollback {
  return { tyLeHienTai: 0, daRollback: false };
}
type KetQuaBuoc = "da_tang" | "rollback_ve_0";
function xuLyMotBuocCanary(
  trangThai: TrangThaiCanaryRollback,
  soLoiQuanSat: number,
  soRequestQuanSat: number,
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc {
  const tyLeLoi = soRequestQuanSat === 0 ? 0 : soLoiQuanSat / soRequestQuanSat;
  if (tyLeLoi > nguongTyLeLoi) {
    trangThai.tyLeHienTai = 0;
    trangThai.daRollback = true;
    return "rollback_ve_0";
  }
  trangThai.tyLeHienTai = Math.min(100, trangThai.tyLeHienTai + buocTang);
  return "da_tang";
}

interface DuLieuBuoc { soLoi: number; soRequest: number; }

function chayCanaryTuDong(
  trangThai: TrangThaiCanaryRollback,
  cacBuoc: DuLieuBuoc[],
  nguongTyLeLoi: number,
  buocTang: number
): KetQuaBuoc[] {
  const cacKetQua: KetQuaBuoc[] = [];
  for (const buoc of cacBuoc) {
    const kq = xuLyMotBuocCanary(trangThai, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang);
    cacKetQua.push(kq);
    if (kq === "rollback_ve_0") break;
  }
  return cacKetQua;
}

const tsX = taoTrangThaiCanaryRollback();
const ketQuaX = chayCanaryTuDong(tsX, [{ soLoi: 1, soRequest: 100 }, { soLoi: 1, soRequest: 100 }], 0.05, 25);
console.log(ketQuaX.length, tsX.tyLeHienTai);
```

```typescript title=test
const tsKhongVoT = taoTrangThaiCanaryRollback();
const ketQuaKhongVoT = chayCanaryTuDong(
  tsKhongVoT,
  [{ soLoi: 1, soRequest: 1000 }, { soLoi: 1, soRequest: 1000 }, { soLoi: 1, soRequest: 1000 }],
  0.05,
  30
);
if (ketQuaKhongVoT.length !== 3) throw new Error("khong buoc nao vuot nguong thi phai chay HET ca 3 buoc");
const tyLeKhongVoT = tsKhongVoT.tyLeHienTai;
if (tyLeKhongVoT !== 90) throw new Error("tang dan 30 moi buoc, sau 3 buoc phai la 90");
const rollbackKhongVoT = tsKhongVoT.daRollback;
if (rollbackKhongVoT !== false) throw new Error("khong vo thi daRollback phai la false");

const tsVoT = taoTrangThaiCanaryRollback();
const ketQuaVoT = chayCanaryTuDong(
  tsVoT,
  [
    { soLoi: 2, soRequest: 1000 },
    { soLoi: 5, soRequest: 1000 },
    { soLoi: 80, soRequest: 1000 },
    { soLoi: 1, soRequest: 1000 },
  ],
  0.05,
  20
);
if (ketQuaVoT.length !== 3) throw new Error("phai DUNG NGAY tai buoc vo (buoc 3), KHONG chay buoc 4 (chi con 3 phan tu)");
const buocCuoiVoT = ketQuaVoT[ketQuaVoT.length - 1];
if (buocCuoiVoT !== "rollback_ve_0") throw new Error("phan tu cuoi cung phai la rollback_ve_0");
const tyLeVoT = tsVoT.tyLeHienTai;
if (tyLeVoT !== 0) throw new Error("sau khi vo, ty le canary phai VE DUNG 0");
const rollbackVoT = tsVoT.daRollback;
if (rollbackVoT !== true) throw new Error("sau khi vo, daRollback phai la true");
```

:::hints
- kind: attention
  body: "Trong than for: const kq = xuLyMotBuocCanary(trangThai, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang); cacKetQua.push(kq); neu kq === 'rollback_ve_0' thi break."
- kind: strategy
  body: "const kq = xuLyMotBuocCanary(trangThai, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang); cacKetQua.push(kq); if (kq === 'rollback_ve_0') break;"
- kind: one-line
  body: "const kq = xuLyMotBuocCanary(trangThai, buoc.soLoi, buoc.soRequest, nguongTyLeLoi, buocTang); cacKetQua.push(kq); if (kq === 'rollback_ve_0') break;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "2 50"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Canary tăng dần, rollback lùi ngay — ráp thành MỘT quy trình duy nhất,
tự vận hành mà không cần ai can thiệp giữa chừng. Bài CUỐI của track
ráp thêm hai mảnh còn lại — health check VÀ feature flag — thành một
pipeline triển khai đầu-cuối hoàn chỉnh.
::::

::::reflect{#nghi-lai}
`xuLyMotBuocCanary` không PHÁT minh logic mới — nó LẤY nguyên quyết
định tăng của bài 2 VÀ hành động rollback của bài 7, đặt chúng CẠNH
nhau trong đúng MỘT hàm. `chayCanaryTuDong` cũng vậy: vòng lặp đơn
giản, dừng bằng `break` NGAY khi có tín hiệu xấu. Cái mới không nằm Ở
kỹ thuật, mà Ở CHỖ hai mảnh RIÊNG lẻ giờ PHẢI cùng chia sẻ một trạng
thái — VÀ trạng thái đó phản ánh ĐÚNG những gì đã thật sự xảy ra, kể
cả khi điều đó có nghĩa LÀ huỷ bỏ mọi tiến độ đã đạt được.
::::

::::checkpoint{mastery=0.83}
::::
