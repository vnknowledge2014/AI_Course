---
id: thiet-ke-he-thong.nhat-ky-bat-bien-va-fold.ap-dung-idempotent
title: "Áp dụng idempotent: cùng sự kiện, cùng id, chỉ tính một lần"
summary: "apDungIdempotent(trangThai, suKien) kiem tra suKien.id da co trong trangThai.idDaApDung CHUA truoc khi cong don -- ap dung CUNG mot su kien HAI lan cho ra CUNG ket qua nhu ap dung MOT lan; khac han cach 'cache theo khoa' Map<khoa,ketQua> cua T7.2 bai3 -- o day tinh idempotent nam NGAY trong chinh phep fold, khong phai mot lop cache rieng ben ngoai."
locale: vi
track: thiet-ke-he-thong
module: nhat-ky-bat-bien-va-fold
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.fp.ap-dung-idempotent]
requires: [sd.fp.snapshot-tranh-fold-lai-tu-dau]
concepts: [sd.fp.ap-dung-idempotent]
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
Mạng không đáng tin — hệ thống giao sự kiện có thể giao LẶP: cùng một
sự kiện, cùng nội dung, đến hai lần. `apDung` (bài 2) không hề biết
điều đó — gặp sự kiện nào, nó cộng dồn sự kiện đó, không hỏi "cái này
đã tính chưa". Ghi nhớ track T7.2: `thanhToan` từng giải quyết vấn đề
tương tự bằng một `Map<khoá, kết quả>` RIÊNG, đứng ngoài logic tính
toán. Ở đây, cách giải quyết nằm chỗ khác hẳn.
::::

::::explain{#idempotent-nam-trong-chinh-fold}
Mỗi sự kiện mang một `id` DUY nhất. `apDungIdempotent` kiểm tra `id`
đó ĐÃ có trong `trangThai.idDaApDung` (một `Set`) hay chưa — nếu RỒI,
trả về NGUYÊN `trangThai` cũ, không cộng dồn gì thêm. Chỉ khi `id`
hoàn toàn MỚI, hàm mới cộng dồn VÀ ghi `id` đó vào tập đã áp dụng:

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }

function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const soDuMoi = suKien.loai === "da_nap" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien;
  return { soDu: soDuMoi, idDaApDung: idMoi };
}

function apDungKhongKiemTra(trangThai: { soDu: number }, suKien: SuKien): { soDu: number } {
  if (suKien.loai === "da_nap") return { soDu: trangThai.soDu + suKien.soTien };
  return { soDu: trangThai.soDu - suKien.soTien };
}

const suKienLap: SuKien = { id: "e1", loai: "da_nap", soTien: 50000 };
const denHaiLan: SuKien[] = [suKienLap, suKienLap];

const ketQuaKhongKiemTra = denHaiLan.reduce(apDungKhongKiemTra, { soDu: 0 });
console.log("KHONG kiem tra id (bai 2) - ap dung 2 lan:", ketQuaKhongKiemTra.soDu, "(SAI - cong hai lan)");

const ketQuaIdempotent = denHaiLan.reduce(apDungIdempotent, trangThaiBanDau());
console.log("CO kiem tra id (idempotent) - ap dung 2 lan:", ketQuaIdempotent.soDu, "(DUNG - chi tinh mot lan)");
```

```text title=readonly
KHONG kiem tra id (bai 2) - ap dung 2 lan: 100000 (SAI - cong hai lan)
CO kiem tra id (idempotent) - ap dung 2 lan: 50000 (DUNG - chi tinh mot lan)
```

`apDungKhongKiemTra` (giống hệt `apDung` bài 2) không hề biết
`suKienLap` đã xuất hiện trước đó — cộng `50000` HAI lần, ra `100000`
SAI. `apDungIdempotent` kiểm `idDaApDung.has(suKien.id)` NGAY dòng đầu
— lần thứ hai gặp `"e1"`, nó trả về nguyên trạng thái cũ, dừng lại
đúng Ở `50000`. Khác với `Map<khoá, kết quả>` của T7.2 — không có
tầng cache nào đứng NGOÀI phép fold; tính idempotent LÀ một phần của
chính `apDungIdempotent`.
::::

::::example{#luong-su-kien-co-lap-rai-rac}
Idempotency không chỉ xử lý sự kiện lặp NGAY cạnh nhau — nó hoạt động
với bất kỳ vị trí lặp nào trong luồng, vì kiểm tra dựa trên `id`, VÀ
`id` được nhớ trong TOÀN bộ `idDaApDung`, không phải chỉ "phần tử liền
trước":

```typescript title=readonly
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }
function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const soDuMoi = suKien.loai === "da_nap" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien;
  return { soDu: soDuMoi, idDaApDung: idMoi };
}

const luong: SuKien[] = [
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e2", loai: "da_tru", soTien: 20000 },
  { id: "e1", loai: "da_nap", soTien: 100000 },
  { id: "e3", loai: "da_nap", soTien: 5000 },
  { id: "e2", loai: "da_tru", soTien: 20000 },
];

const ketQua = luong.reduce(apDungIdempotent, trangThaiBanDau());
console.log("so luong su kien trong 'luong' (co lap):", luong.length);
console.log("so id DUY NHAT da ap dung:", ketQua.idDaApDung.size);
console.log("so du cuoi cung:", ketQua.soDu);
```

```text title=readonly
so luong su kien trong 'luong' (co lap): 5
so id DUY NHAT da ap dung: 3
so du cuoi cung: 85000
```

`luong` có `5` phần tử nhưng chỉ `3` `id` PHÂN biệt (`e1`, `e2`,
`e3`) — hai lần lặp lại (`e1` Ở vị trí ba, `e2` Ở vị trí năm) không
nằm cạnh bản gốc của chúng, nhưng vẫn bị nhận ra VÀ bỏ qua đúng, vì
`idDaApDung` nhớ TẤT cả `id` đã gặp, không chỉ id liền kề. Số dư cuối
(`85000`) khớp đúng tổng của ba sự kiện phân biệt: `100000 - 20000 +
5000`.
::::

::::predict{#doan-gui-lai-voi-du-lieu-sai commitOnce}
Tiếp tục từ đoạn `explain`: `ketQuaIdempotent` đã có `soDu = 50000`
VÀ `idDaApDung` chứa `"e1"`. Giả sử hệ thống giao LẠI một bản ghi VỚI
CÙNG `id: "e1"` nhưng `soTien` bị lỗi thành `999999` (một bản ghi bị
hỏng khi truyền lại). Gọi `apDungIdempotent(ketQuaIdempotent, { id:
"e1", loai: "da_nap", soTien: 999999 })` — `soDu` sau lệnh gọi đó là
bao nhiêu?

:::opt{correct}
Vẫn là `50000` — dòng đầu tiên của hàm kiểm tra
`trangThai.idDaApDung.has(suKien.id)`, tìm thấy `"e1"` đã có, trả về
NGUYÊN `trangThai` cũ NGAY, không hề đọc tới `soTien: 999999` của sự
kiện gửi lên
:::
:::opt
`1049999` (`50000 + 999999`) — vì đây LÀ một sự kiện `"da_nap"` hợp
lệ về mặt kiểu dữ liệu, nên nó vẫn được cộng dồn bình thường
::why
Nhầm "kiểm tra idempotent" với "kiểm tra tính hợp lệ của nội dung" —
nhưng `apDungIdempotent` không hề so sánh `soTien` giữa hai lần gửi,
nó CHỈ nhìn vào `id`.

Chỗ lệch: điều kiện `if (trangThai.idDaApDung.has(suKien.id)) return
trangThai;` là dòng ĐẦU tiên trong thân hàm — nó chặn đứng MỌI xử lý
tiếp theo ngay khi `id` đã từng gặp, bất kể `soTien` gửi lên lần này
là gì. Đây chính LÀ điểm mạnh của thiết kế: khoá idempotent bảo vệ
khỏi cả sự kiện lặp NGUYÊN vẹn lẫn sự kiện lặp bị hỏng dữ liệu — miễn
`id` trùng, sự kiện bị bỏ qua hoàn toàn.
::
:::
::::

::::code{#viet_ap_dung_idempotent}
Hoàn thiện `apDungIdempotent` — nếu `suKien.id` đã có trong
`trangThai.idDaApDung`, trả về NGUYÊN `trangThai`. Ngược lại, tạo một
`Set` MỚI (sao chép từ `idDaApDung` cũ, thêm `id` mới), tính `soDu`
mới, VÀ trả về trạng thái MỚI — không sửa `trangThai` truyền vào.

```typescript title=starter
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }

function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  ___
}

const tX0 = trangThaiBanDau();
const tX1 = apDungIdempotent(tX0, { id: "x1", loai: "da_nap", soTien: 900 });
const tX2 = apDungIdempotent(tX1, { id: "x1", loai: "da_nap", soTien: 900 });
console.log(tX2.soDu, tX2.idDaApDung.size);
```

```typescript title=solution
type LoaiSuKien = "da_nap" | "da_tru";
interface SuKien { id: string; loai: LoaiSuKien; soTien: number; }
interface TrangThai { soDu: number; idDaApDung: Set<string>; }
function trangThaiBanDau(): TrangThai { return { soDu: 0, idDaApDung: new Set() }; }

function apDungIdempotent(trangThai: TrangThai, suKien: SuKien): TrangThai {
  if (trangThai.idDaApDung.has(suKien.id)) return trangThai;
  const idMoi = new Set(trangThai.idDaApDung);
  idMoi.add(suKien.id);
  const soDuMoi = suKien.loai === "da_nap" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien;
  return { soDu: soDuMoi, idDaApDung: idMoi };
}

const tX0 = trangThaiBanDau();
const tX1 = apDungIdempotent(tX0, { id: "x1", loai: "da_nap", soTien: 900 });
const tX2 = apDungIdempotent(tX1, { id: "x1", loai: "da_nap", soTien: 900 });
console.log(tX2.soDu, tX2.idDaApDung.size);
```

```typescript title=test
const t0 = trangThaiBanDau();
const e1: SuKien = { id: "e1", loai: "da_nap", soTien: 1000 };
const t1 = apDungIdempotent(t0, e1);
if (t1.soDu !== 1000) throw new Error("su kien MOI (id chua gap) phai duoc AP DUNG binh thuong");
if (!t1.idDaApDung.has("e1")) throw new Error("id cua su kien vua ap dung phai duoc GHI vao idDaApDung");

const t2 = apDungIdempotent(t1, e1);
if (t2.soDu !== 1000) throw new Error("gui lai CUNG id KHONG duoc cong them lan nua");
if (t2.idDaApDung.size !== 1) throw new Error("idDaApDung KHONG duoc them ban ghi trung");

const e1Sai: SuKien = { id: "e1", loai: "da_nap", soTien: 999999 };
const t3 = apDungIdempotent(t2, e1Sai);
if (t3.soDu !== 1000) throw new Error("cung id du soTien gui len KHAC van phai BI BO QUA hoan toan (chi nhin ID)");

const e2: SuKien = { id: "e2", loai: "da_tru", soTien: 300 };
const t4 = apDungIdempotent(t1, e2);
if (t4.soDu !== 700) throw new Error("su kien MOI (id khac) phai duoc tru dung so tien: 1000 - 300 = 700");
if (t4.idDaApDung.size !== 2) throw new Error("idDaApDung phai co 2 phan tu sau khi ap dung 2 su kien khac id");

if (t1.idDaApDung.size !== 1) throw new Error("apDungIdempotent KHONG duoc mutate Set cua trangThai TRUOC do -- t1 phai giu nguyen 1 phan tu sau khi tao t4 tu no");
if (t1.idDaApDung.has("e2")) throw new Error("Set cua t1 KHONG duoc chua id cua su kien ap dung SAU do (chung to bi mutate chung, khong tao Set moi)");
```

:::hints
- kind: attention
  body: "Kiem tra trangThai.idDaApDung.has(suKien.id) TRUOC. Neu true, return NGUYEN trangThai. Neu false, tao Set MOI (new Set(trangThai.idDaApDung)), them id vao Set moi, tinh soDu moi, roi tra ve object MOI voi Set moi do -- KHONG duoc goi .add() truc tiep tren trangThai.idDaApDung."
- kind: strategy
  body: "if (trangThai.idDaApDung.has(suKien.id)) return trangThai; const idMoi = new Set(trangThai.idDaApDung); idMoi.add(suKien.id); const soDuMoi = suKien.loai === 'da_nap' ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien; return { soDu: soDuMoi, idDaApDung: idMoi };"
- kind: one-line
  body: "if (trangThai.idDaApDung.has(suKien.id)) return trangThai; const idMoi = new Set(trangThai.idDaApDung); idMoi.add(suKien.id); const soDuMoi = suKien.loai === \"da_nap\" ? trangThai.soDu + suKien.soTien : trangThai.soDu - suKien.soTien; return { soDu: soDuMoi, idDaApDung: idMoi };"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "900 1"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Áp dụng một sự kiện hai lần, ba lần, mười lần — kết quả không đổi.
Nhưng đây chỉ là MỘT nửa của lời hứa "exactly-once". Nửa còn lại: liệu
một hệ thống giao lặp THẬT SỰ (at-least-once) có thể dựa vào tính chất
này để đạt hiệu ứng đúng-một-lần không? Cần chứng minh bằng số liệu.
::::

::::reflect{#nghi-lai}
`apDungIdempotent` không hề tách "kiểm tra trùng lặp" ra thành một
bước RIÊNG đứng trước phép tính, kiểu `Map<khoá, kết quả>` của T7.2 —
nó gộp CẢ hai vào đúng MỘT hàm, MỘT lần fold. Tính idempotent Ở đây
không phải một tính năng THÊM vào, nó LÀ một phần định nghĩa của
chính phép biến đổi trạng thái: áp dụng một sự kiện đã áp dụng rồi
luôn LÀ một phép không-làm-gì (no-op).
::::

::::checkpoint{mastery=0.72}
::::
