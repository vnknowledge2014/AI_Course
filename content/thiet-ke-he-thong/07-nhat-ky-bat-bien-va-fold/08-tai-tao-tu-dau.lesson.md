---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.tai-tao-tu-dau
title: "Tái tạo từ đầu: xoá sạch cache, log vẫn cho lại đúng trạng thái"
summary: "taiTaoTuNhatKy(nhatKy) fold LAI hoan toan tu nhat ky THO, khong dung bat cu gia tri da tinh truoc do -- xoa SACH mot 'cache' (trangThaiDaTinh) roi tai tao CHI tu nhat ky van ra DUNG gia tri da mat (vi du 160000); xacMinhTaiTao(nhatKy, trangThaiDaLuu) so sanh gia tri tai tao voi gia tri da luu de CHUNG MINH nhat ky moi la nguon su that DUY NHAT, khong phai snapshot hay cache."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.fp.tai-tao-tu-dau]
requires: [sd.fp.vo-menh-lenh-thuc-thi]
concepts: [sd.fp.tai-tao-tu-dau]
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
Snapshot (bài 3) giúp tránh fold lại từ đầu MỖI lần cần trạng thái —
nhưng đó là một tối ưu VỀ TỐC ĐỘ, không phải một nguồn sự thật MỚI.
Nếu snapshot mất — cache sập, process restart, ổ đĩa hỏng — liệu hệ
thống còn cách nào lấy LẠI đúng trạng thái không? Câu trả lời cần
được chứng minh, không chỉ tin tưởng.
::::

::::explain{#nhat-ky-la-nguon-su-that-duy-nhat}
`taiTaoTuNhatKy` không hề dùng bất cứ giá trị nào đã tính trước đó —
nó fold LẠI hoàn toàn từ mảng `SuKien` thô, bắt đầu từ
`trangThaiBanDau()`. Dù một "cache" (`kho.trangThaiDaTinh`) từng được
lưu rồi bị xoá sạch, nhật ký thô vẫn đủ để dựng lại ĐÚNG giá trị đã
mất:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function taiTaoTuNhatKy(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

interface Kho { nhatKy: SuKien[]; trangThaiDaTinh: TrangThai | undefined; }

const kho: Kho = {
  nhatKy: [
    { id: "e1", loai: "da_nap", soTien: 200000 },
    { id: "e2", loai: "da_tru", soTien: 50000 },
    { id: "e3", loai: "da_nap", soTien: 10000 },
  ],
  trangThaiDaTinh: undefined,
};
kho.trangThaiDaTinh = taiTaoTuNhatKy(kho.nhatKy);
console.log("trang thai da TINH VA LUU (cache):", JSON.stringify(kho.trangThaiDaTinh));

kho.trangThaiDaTinh = undefined;
console.log("SAU KHI XOA SACH cache, kho.trangThaiDaTinh =", kho.trangThaiDaTinh);

const taiTao = taiTaoTuNhatKy(kho.nhatKy);
console.log("tai tao LAI tu nhat ky tho:", JSON.stringify(taiTao));
console.log("ket qua tai tao co GIONG HET cache da mat khong?", taiTao.soDu === 160000);
```

```text title=readonly
trang thai da TINH VA LUU (cache): {"soDu":160000}
SAU KHI XOA SACH cache, kho.trangThaiDaTinh = undefined
tai tao LAI tu nhat ky tho: {"soDu":160000}
ket qua tai tao co GIONG HET cache da mat khong? true
```

Sau dòng `kho.trangThaiDaTinh = undefined;`, không còn gì trong `kho`
"nhớ" con số `160000` nữa — ngoại trừ chính `kho.nhatKy`. Gọi
`taiTaoTuNhatKy(kho.nhatKy)` fold LẠI cả ba sự kiện (`+200000`,
`-50000`, `+10000`) VÀ ra ĐÚNG lại `160000` — không sai một đồng.
::::

::::example{#mat-ca-snapshot-lan-cache}
Kết luận tương tự đúng ngay cả khi CẢ snapshot (bài 3) LẪN cache đều
mất — miễn nhật ký THÔ còn nguyên, tái tạo luôn cho lại đúng kết quả:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function taiTaoTuNhatKy(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}
interface Snapshot { trangThai: TrangThai; soSuKienDaFold: number; }

const nhatKyTho: SuKien[] = [
  { id: "e1", loai: "da_nap", soTien: 500 },
  { id: "e2", loai: "da_nap", soTien: 300 },
  { id: "e3", loai: "da_tru", soTien: 200 },
  { id: "e4", loai: "da_nap", soTien: 1000 },
];

let snap: Snapshot | undefined = { trangThai: taiTaoTuNhatKy(nhatKyTho), soSuKienDaFold: nhatKyTho.length };
console.log("snapshot TRUOC khi mat:", JSON.stringify(snap));

snap = undefined;
console.log("snapshot SAU khi mat het:", snap);

const taiTaoLai = taiTaoTuNhatKy(nhatKyTho);
console.log("tai tao lai tu nhat ky THO:", JSON.stringify(taiTaoLai));
console.log("dung bang gia tri da mat (1600)?", taiTaoLai.soDu === 1600);
```

```text title=readonly
snapshot TRUOC khi mat: {"trangThai":{"soDu":1600},"soSuKienDaFold":4}
snapshot SAU khi mat het: undefined
tai tao lai tu nhat ky THO: {"soDu":1600}
dung bang gia tri da mat (1600)? true
```

`snap` bị gán `undefined` — mọi dấu vết của "trạng thái đã tính" biến
mất khỏi bộ nhớ. Nhưng `nhatKyTho` — mảng bốn sự kiện GỐC — chưa từng
bị đụng tới, VÀ `taiTaoTuNhatKy(nhatKyTho)` fold lại ra đúng `1600`.
Snapshot LÀ thứ có thể mất mà không sao; nhật ký thô KHÔNG.
::::

::::predict{#doan-mat-nhat-ky-thay-vi-snapshot commitOnce}
Đảo ngược tình huống Ở trên: giả sử `nhatKyTho` (nhật ký GỐC) bị MẤT
hoàn toàn, nhưng `snap` (snapshot đã tính, `{trangThai: {soDu: 1600},
soSuKienDaFold: 4}`) vẫn còn nguyên. Hệ thống lúc này còn CHỨNG MINH
được `1600` là con số đúng hay không (tái tạo độc lập để đối chiếu)?

:::opt{correct}
Không — mất nhật ký nghĩa là mất NGUỒN duy nhất có thể fold lại để
đối chiếu; `snap.trangThai.soDu = 1600` lúc này chỉ còn LÀ một con số
không ai kiểm chứng lại được, dù bản thân con số đó có thể vẫn đúng
:::
:::opt
Không sao — `snap` CHÍNH LÀ trạng thái, nhật ký thô chỉ là bản ghi phụ
trợ để tối ưu tốc độ tính toán, không phải nguồn sự thật
::why
Đảo ngược đúng vai trò của hai thứ — bài học Ở trên vừa chứng minh
điều ngược lại: nhật ký thô mới LÀ nguồn sự thật (mất snapshot vẫn tái
tạo được), còn snapshot chỉ là một CACHE PHÁI SINH từ nhật ký.

Chỗ lệch: `taiTaoTuNhatKy` nhận `SuKien[]` — nhật ký THÔ — làm đầu
vào DUY nhất; nó không hề nhận `Snapshot` làm nguồn. Nếu nhật ký mất,
không có hàm nào trong bài học này (hay bất kỳ đoạn code nào đã viết)
có thể tính lại `soDu` từ một con số `1600` đơn lẻ để XÁC MINH nó — con
số đó có thể đúng, có thể sai do lỗi tính toán trước đó, và không ai
biết được nữa. Đây chính LÀ điều làm nhật ký khác snapshot: mất một
CACHE là mất tốc độ; mất NGUỒN sự thật là mất khả năng kiểm chứng.
::
:::
::::

::::code{#viet_xac_minh_tai_tao}
Hoàn thiện `xacMinhTaiTao` — tái tạo trạng thái từ `nhatKy` (dùng
`taiTaoTuNhatKy` đã có sẵn), rồi so sánh `soDu` của kết quả tái tạo
với `soDu` của `trangThaiDaLuu`. Trả về `true` nếu khớp, `false` nếu
không.

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function taiTaoTuNhatKy(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

function xacMinhTaiTao(nhatKy: SuKien[], trangThaiDaLuu: TrangThai): boolean {
  ___
}

const nkX: SuKien[] = [{ id: "x1", loai: "da_nap", soTien: 500 }];
console.log(xacMinhTaiTao(nkX, { soDu: 500 }), xacMinhTaiTao(nkX, { soDu: 999 }));
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
function taiTaoTuNhatKy(nhatKy: SuKien[]): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

function xacMinhTaiTao(nhatKy: SuKien[], trangThaiDaLuu: TrangThai): boolean {
  const taiTao = taiTaoTuNhatKy(nhatKy);
  return taiTao.soDu === trangThaiDaLuu.soDu;
}

const nkX: SuKien[] = [{ id: "x1", loai: "da_nap", soTien: 500 }];
console.log(xacMinhTaiTao(nkX, { soDu: 500 }), xacMinhTaiTao(nkX, { soDu: 999 }));
```

```typescript title=test
const nk: SuKien[] = [
  { id: "a", loai: "da_nap", soTien: 700 },
  { id: "b", loai: "da_tru", soTien: 200 },
];
if (xacMinhTaiTao(nk, { soDu: 500 }) !== true) throw new Error("trang thai DUNG (700-200=500) phai duoc xac minh la true");
if (xacMinhTaiTao(nk, { soDu: 500000 }) !== false) throw new Error("trang thai SAI phai duoc xac minh la false");
if (xacMinhTaiTao([], { soDu: 0 }) !== true) throw new Error("nhat ky rong voi trang thai 0 phai la true");
if (xacMinhTaiTao([], { soDu: 1 }) !== false) throw new Error("nhat ky rong voi trang thai KHAC 0 phai la false");

const nkLon: SuKien[] = [];
for (let i = 0; i < 50; i++) nkLon.push({ id: "e" + i, loai: "da_nap", soTien: 2 });
const trangThaiTruocKhiMat = taiTaoTuNhatKy(nkLon);
if (xacMinhTaiTao(nkLon, trangThaiTruocKhiMat) !== true) throw new Error("tai tao lai tu 50 su kien phai KHOP voi trang thai da luu truoc do");
```

:::hints
- kind: attention
  body: "Goi taiTaoTuNhatKy(nhatKy) de tinh lai tu dau, roi so sanh .soDu cua ket qua do voi trangThaiDaLuu.soDu bang ===. Tra ve ket qua so sanh do (mot boolean)."
- kind: strategy
  body: "const taiTao = taiTaoTuNhatKy(nhatKy); return taiTao.soDu === trangThaiDaLuu.soDu;"
- kind: one-line
  body: "const taiTao = taiTaoTuNhatKy(nhatKy); return taiTao.soDu === trangThaiDaLuu.soDu;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true false"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Nhật ký là nguồn sự thật DUY nhất — cache, snapshot, mọi thứ khác đều
có thể xoá VÀ dựng lại. Nhưng "dựng lại" chỉ hoạt động khi nhật ký
đúng NGAY từ đầu. Nếu một sự kiện đã ghi hoá ra SAI — không được sửa,
không được xoá — thì phải làm gì?
::::

::::reflect{#nghi-lai}
`xacMinhTaiTao` không hề tin tưởng bất cứ giá trị nào đã lưu sẵn — nó
LUÔN tính lại từ `nhatKy` rồi mới so sánh. Đây chính LÀ điểm khác biệt
giữa "nguồn sự thật" (source of truth) và "bản sao tối ưu tốc độ"
(cache, snapshot): mất bản sao chỉ làm chậm lại; mất nguồn sự thật là
mất khả năng biết điều gì đã thực sự xảy ra. Nhật ký sự kiện, không
phải trạng thái đã tính, mới LÀ thứ hệ thống PHẢI bảo vệ bằng mọi giá.
::::

::::checkpoint{mastery=0.80}
::::
