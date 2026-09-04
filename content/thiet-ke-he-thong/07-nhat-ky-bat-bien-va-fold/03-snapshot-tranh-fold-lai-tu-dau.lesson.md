---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.snapshot-tranh-fold-lai-tu-dau
title: "Snapshot: cộng dồn phần mới, không fold lại từ đầu"
summary: "foldTuSnapshot(snap, nhatKyDayDu) chi ap dung apDung len PHAN su kien MOI (nhatKyDayDu.slice(snap.soSuKienDaFold)), cong don LEN TREN snap.trangThai co san -- khong tinh lai tu dau; nhat ky 1000 su kien fold lan dau het 1000 lan goi apDung, them 1 su kien fold lan hai CHI ton 1 lan goi, ket qua giong het fold toan bo tu dau."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [sd.fp.snapshot-tranh-fold-lai-tu-dau]
requires: [sd.fp.trang-thai-la-fold-cua-su-kien]
concepts: [sd.fp.snapshot-tranh-fold-lai-tu-dau]
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
Bài trước để ngỏ một vấn đề: fold lại TOÀN bộ nhật ký mỗi lần cần biết
trạng thái là đắt khi nhật ký dài hàng nghìn sự kiện. Nhưng KHÔNG cần
huỷ bỏ ý tưởng "trạng thái là kết quả của fold" để giải quyết nó —
chỉ cần nhớ LẦN fold trước đã tới đâu, rồi lần sau CHỈ fold phần còn
lại.
::::

::::explain{#snapshot-nho-diem-da-fold-toi}
Một `Snapshot` giữ hai thứ: trạng thái ĐÃ tính (`trangThai`) VÀ số sự
kiện ĐÃ được fold vào trạng thái đó (`soSuKienDaFold`). `foldTuSnapshot`
lấy PHẦN sự kiện MỚI (`nhatKyDayDu.slice(snap.soSuKienDaFold)`) — tức
là mọi sự kiện xuất hiện SAU điểm đã fold — rồi chỉ fold đúng phần đó,
cộng dồn LÊN TRÊN `snap.trangThai` có sẵn:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
function tinhTrangThai(nhatKy: NhatKy): TrangThai {
  return nhatKy.reduce(apDung, trangThaiBanDau());
}

interface Snapshot { trangThai: TrangThai; soSuKienDaFold: number; }
function snapshotRong(): Snapshot { return { trangThai: trangThaiBanDau(), soSuKienDaFold: 0 }; }

function foldTuSnapshot(snap: Snapshot, nhatKyDayDu: NhatKy): Snapshot {
  const suKienMoi = nhatKyDayDu.slice(snap.soSuKienDaFold);
  const trangThaiMoi = suKienMoi.reduce(apDung, snap.trangThai);
  return { trangThai: trangThaiMoi, soSuKienDaFold: nhatKyDayDu.length };
}

const nhatKy: NhatKy = [
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e2", loai: "da_tru", soTien: 30000 },
  { id: "e3", loai: "da_nap", soTien: 5000 },
];

const snap1 = foldTuSnapshot(snapshotRong(), nhatKy);
console.log("snapshot sau lan fold DAU (het 3 su kien):", JSON.stringify(snap1));

const nhatKy2: NhatKy = [
  ...nhatKy,
  { id: "e4", loai: "da_tru", soTien: 20000 },
  { id: "e5", loai: "da_nap", soTien: 1000 },
];

const snap2 = foldTuSnapshot(snap1, nhatKy2);
console.log("snapshot sau lan fold THU HAI (chi ap dung 2 su kien moi):", JSON.stringify(snap2));

const tuDau = tinhTrangThai(nhatKy2);
console.log("fold TOAN BO tu dau (de doi chieu):", tuDau.soDu);
console.log("hai cach cho CUNG mot ket qua?", snap2.trangThai.soDu === tuDau.soDu);
```

```text title=readonly
snapshot sau lan fold DAU (het 3 su kien): {"trangThai":{"soDu":75000},"soSuKienDaFold":3}
snapshot sau lan fold THU HAI (chi ap dung 2 su kien moi): {"trangThai":{"soDu":56000},"soSuKienDaFold":5}
fold TOAN BO tu dau (de doi chieu): 56000
hai cach cho CUNG mot ket qua? true
```

Lần fold ĐẦU (từ `snapshotRong()`) phải duyệt cả `3` sự kiện, y hệt
`tinhTrangThai`. Nhưng lần fold THỨ HAI chỉ nhận `snap1` (đã có sẵn
`75000` VÀ mốc `3`) — `.slice(3)` trên `nhatKy2` chỉ trả về ĐÚNG hai
sự kiện mới (`e4`, `e5`), không đụng lại `e1`, `e2`, `e3`. Kết quả
(`56000`) khớp CHÍNH XÁC với fold toàn bộ `nhatKy2` từ đầu — snapshot
không đổi câu trả lời, nó chỉ đổi LƯỢNG công việc cần làm để có câu
trả lời đó.
::::

::::example{#snapshot-tiet-kiem-cong-viec}
Với một nhật ký lớn, khác biệt về LƯỢNG công việc trở nên rõ ràng:
fold từ một snapshot có sẵn chỉ xử lý đúng số sự kiện MỚI, bất kể
nhật ký gốc dài bao nhiêu:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
interface Snapshot { trangThai: TrangThai; soSuKienDaFold: number; }
function snapshotRong(): Snapshot { return { trangThai: trangThaiBanDau(), soSuKienDaFold: 0 }; }

interface KetQuaFold { snapshotMoi: Snapshot; soSuKienMoiDaXuLy: number; }
function foldTuSnapshotCoDem(snap: Snapshot, nhatKyDayDu: NhatKy): KetQuaFold {
  const suKienMoi = nhatKyDayDu.slice(snap.soSuKienDaFold);
  const trangThaiMoi = suKienMoi.reduce(apDung, snap.trangThai);
  return {
    snapshotMoi: { trangThai: trangThaiMoi, soSuKienDaFold: nhatKyDayDu.length },
    soSuKienMoiDaXuLy: suKienMoi.length,
  };
}

const nhatKyLon: NhatKy = [];
for (let i = 0; i < 1000; i++) nhatKyLon.push({ id: "e" + i, loai: "da_nap", soTien: 1 });

const ketQuaA = foldTuSnapshotCoDem(snapshotRong(), nhatKyLon);
console.log("lan dau (nhat ky 1000 su kien), so su kien MOI da xu ly:", ketQuaA.soSuKienMoiDaXuLy);

const nhatKyLon2: NhatKy = [...nhatKyLon, { id: "e1000", loai: "da_tru", soTien: 500 }];
const ketQuaB = foldTuSnapshotCoDem(ketQuaA.snapshotMoi, nhatKyLon2);
console.log("lan hai (nhat ky 1001 su kien, nhung snapshot da co san), so su kien MOI da xu ly:", ketQuaB.soSuKienMoiDaXuLy);
console.log("soDu cuoi cung:", ketQuaB.snapshotMoi.trangThai.soDu);
```

```text title=readonly
lan dau (nhat ky 1000 su kien), so su kien MOI da xu ly: 1000
lan hai (nhat ky 1001 su kien, nhung snapshot da co san), so su kien MOI da xu ly: 1
soDu cuoi cung: 500
```

Lần đầu phải xử lý HẾT `1000` sự kiện — không có snapshot nào để bắt
đầu từ đó. Nhưng lần hai, dù nhật ký đã dài `1001` sự kiện, chỉ đúng
`1` sự kiện MỚI được xử lý — snapshot `ketQuaA.snapshotMoi` đã "nhớ"
`1000` sự kiện trước Ở dạng đã gấp lại thành một con số duy nhất.
::::

::::predict{#doan-fold-tu-snapshot-rong commitOnce}
Ở đoạn `explain`, `snap1` đã có `trangThai.soDu = 75000` VÀ
`soSuKienDaFold = 3`. Thay vì gọi `foldTuSnapshot(snap1, nhatKy2)`,
NẾU gọi `foldTuSnapshot(snapshotRong(), nhatKy2)` — dùng một snapshot
RỖNG thay vì `snap1` — kết quả `trangThai.soDu` cuối cùng có khác
`56000` không?

:::opt{correct}
Không khác — vẫn ra `56000`; fold từ snapshot rỗng nghĩa là
`.slice(0)`, tức là fold lại TOÀN bộ `5` sự kiện của `nhatKy2` từ đầu,
cho CÙNG kết quả cuối như fold từ `snap1`, chỉ tốn công hơn (làm lại
việc `snap1` đã tính sẵn), không hề sai kết quả
:::
:::opt
Có khác — snapshot rỗng sẽ BỎ SÓT ba sự kiện đầu (`e1`, `e2`, `e3`),
vì nó không "biết" gì về chúng
::why
Nhầm ý nghĩa của `soSuKienDaFold` — nó không phải "danh sách sự kiện
được PHÉP đọc", nó chỉ là một MỐC để `.slice` bắt đầu từ đó. Snapshot
rỗng có mốc là `0`, nghĩa là `.slice(0)` LẤY HẾT toàn bộ mảng, không
bỏ sót gì cả.

Chỗ lệch: `foldTuSnapshot` không hề "giới hạn" những sự kiện nào được
nhìn thấy — tham số THỨ HAI (`nhatKyDayDu`) luôn là nhật ký ĐẦY ĐỦ.
`snap.soSuKienDaFold` chỉ quyết định fold bắt đầu TỪ ĐÂU trong mảng
đó. Snapshot rỗng (`soSuKienDaFold: 0`) đơn giản là "chưa fold gì cả",
nên `.slice(0)` trả về nguyên vẹn cả `5` phần tử — snapshot là một
tối ưu VỀ TỐC ĐỘ, không phải một bộ lọc VỀ TÍNH ĐÚNG.
::
:::
::::

::::code{#viet_fold_tu_snapshot}
Hoàn thiện `foldTuSnapshot` — lấy phần sự kiện MỚI bằng
`nhatKyDayDu.slice(snap.soSuKienDaFold)`, fold phần đó qua `apDung`
bắt đầu từ `snap.trangThai` (KHÔNG phải từ `trangThaiBanDau()`), rồi
trả về snapshot MỚI với `soSuKienDaFold` bằng độ dài `nhatKyDayDu`.

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
interface Snapshot { trangThai: TrangThai; soSuKienDaFold: number; }
function snapshotRong(): Snapshot { return { trangThai: trangThaiBanDau(), soSuKienDaFold: 0 }; }

function foldTuSnapshot(snap: Snapshot, nhatKyDayDu: NhatKy): Snapshot {
  ___
}

const nkX: NhatKy = [{ id: "x1", loai: "da_nap", soTien: 700 }];
const snapX = foldTuSnapshot(snapshotRong(), nkX);
console.log(snapX.trangThai.soDu, snapX.soSuKienDaFold);
```

```typescript title=solution
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
type NhatKy = SuKien[];
interface TrangThai { soDu: number; }
function trangThaiBanDau(): TrangThai { return { soDu: 0 }; }
function apDung(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}
interface Snapshot { trangThai: TrangThai; soSuKienDaFold: number; }
function snapshotRong(): Snapshot { return { trangThai: trangThaiBanDau(), soSuKienDaFold: 0 }; }

function foldTuSnapshot(snap: Snapshot, nhatKyDayDu: NhatKy): Snapshot {
  const suKienMoi = nhatKyDayDu.slice(snap.soSuKienDaFold);
  const trangThaiMoi = suKienMoi.reduce(apDung, snap.trangThai);
  return { trangThai: trangThaiMoi, soSuKienDaFold: nhatKyDayDu.length };
}

const nkX: NhatKy = [{ id: "x1", loai: "da_nap", soTien: 700 }];
const snapX = foldTuSnapshot(snapshotRong(), nkX);
console.log(snapX.trangThai.soDu, snapX.soSuKienDaFold);
```

```typescript title=test
const nk: NhatKy = [{ id: "a", loai: "da_nap", soTien: 300 }, { id: "b", loai: "da_tru", soTien: 100 }];
const s1 = foldTuSnapshot(snapshotRong(), nk);
if (s1.trangThai.soDu !== 200) throw new Error("fold tu snapshot rong phai ra dung tong: 300 - 100 = 200");
if (s1.soSuKienDaFold !== 2) throw new Error("soSuKienDaFold phai bang do dai nhat ky da fold");

const nk2: NhatKy = [...nk, { id: "c", loai: "da_nap", soTien: 50 }];
const s2 = foldTuSnapshot(s1, nk2);
if (s2.trangThai.soDu !== 250) throw new Error("fold lan hai phai CONG DON len tren trang thai cu, khong tinh lai tu dau");
if (s2.soSuKienDaFold !== 3) throw new Error("soSuKienDaFold phai cap nhat theo do dai nhat ky MOI");

if (s1.trangThai.soDu !== 200) throw new Error("foldTuSnapshot KHONG duoc sua doi snapshot CU truyen vao");
if (s1.soSuKienDaFold !== 2) throw new Error("foldTuSnapshot KHONG duoc sua doi snapshot CU truyen vao");

const snapGia: Snapshot = { trangThai: { soDu: 100000 }, soSuKienDaFold: 2 };
const sGiu = foldTuSnapshot(snapGia, nk);
if (sGiu.trangThai.soDu !== 100000) throw new Error("khong co su kien MOI thi trangThai phai GIU NGUYEN gia tri cu, khong tinh lai tu dau");
if (sGiu.soSuKienDaFold !== 2) throw new Error("soSuKienDaFold phai giu nguyen khi khong co su kien moi");

const sGiaMoi = foldTuSnapshot(snapGia, nk2);
if (sGiaMoi.trangThai.soDu !== 100050) throw new Error("phai CONG DON su kien MOI len tren gia tri co san trong snapshot (100000 + 50 = 100050), khong tinh lai tu dau");
```

:::hints
- kind: attention
  body: "Ba buoc: (1) lay su kien MOI bang nhatKyDayDu.slice(snap.soSuKienDaFold); (2) fold PHAN do qua apDung, bat dau tu snap.trangThai (khong phai trangThaiBanDau()); (3) tra ve snapshot moi voi soSuKienDaFold = nhatKyDayDu.length."
- kind: strategy
  body: "const suKienMoi = nhatKyDayDu.slice(snap.soSuKienDaFold); const trangThaiMoi = suKienMoi.reduce(apDung, snap.trangThai); return { trangThai: trangThaiMoi, soSuKienDaFold: nhatKyDayDu.length };"
- kind: one-line
  body: "const suKienMoi = nhatKyDayDu.slice(snap.soSuKienDaFold); const trangThaiMoi = suKienMoi.reduce(apDung, snap.trangThai); return { trangThai: trangThaiMoi, soSuKienDaFold: nhatKyDayDu.length };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "700 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Fold từ snapshot, chỉ xử lý phần mới — nhanh, VÀ vẫn ra đúng kết quả
như fold từ đầu. Nhưng nếu MỘT sự kiện lỡ bị fold hai lần — do mạng
gửi lặp, do retry — con số cuối cùng còn đúng không?
::::

::::reflect{#nghi-lai}
Snapshot không thay đổi Ý nghĩa của "trạng thái là fold của sự kiện"
— nó chỉ thay đổi LƯỢNG việc cần làm để có được trạng thái đó.
`foldTuSnapshot(snapshotRong(), nk)` VÀ `tinhTrangThai(nk)` luôn cho
CÙNG một câu trả lời — snapshot chỉ là một điểm khởi đầu khác cho
đúng CÙNG một phép tính, không phải một con đường khác dẫn tới một sự
thật khác.
::::

::::checkpoint{mastery=0.70}
::::
