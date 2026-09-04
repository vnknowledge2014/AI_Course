---
id: co-so-du-lieu.vu-tru-tat-dinh.boss-vu-tru-tat-dinh
title: "BOSS — Vũ trụ tất định"
summary: "khopVoiThamChieu chạy chayMoPhong RỒI so sánh dấu vết với một chuỗi tham chiếu đã cho. demSoSeedKhop đếm bao nhiêu seed trong một danh sách cho ra ĐÚNG dấu vết tham chiếu đó. Với thamChieu lấy từ seed=7n VÀ danh sách thử [7n,8n,7n,9n,7n]: đúng 3/5 khớp (ba lần 7n) -- hai seed KHÁC (8n, 9n) cho dấu vết khác hẳn, không khớp. Chứng minh 'cùng seed = cùng vũ trụ' theo cả hai chiều: khớp khi VÀ CHỈ khi cùng seed."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: []
requires: [db.chay-lai-dung-tuyet-doi]
concepts: [db.boss-q17]
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
Không tái lập được (bài 1), xorshift128+ tất định (bài 2-3), đồng
hồ ảo (bài 4), hàng đợi tất định kể cả khi hoà (bài 5-6), ráp lại
chạy đúng tuyệt đối (bài 7). Chứng minh ĐIỀU quan trọng nhất theo cả
hai chiều: CÙNG seed cho cùng vũ trụ — VÀ seed KHÁC cho vũ trụ khác.
::::

::::explain{#boss-that}
`khopVoiThamChieu` chạy `chayMoPhong` RỒI so sánh dấu VẾT với một
chuỗi THAM chiếu ĐÃ cho. `demSoSeedKhop` đếm bao NHIÊU seed trong
MỘT danh sách cho ra ĐÚNG dấu vết THAM chiếu đó:

```typescript title=readonly
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}

interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

interface SuKien { thoiDiem: number; thuTuChen: number; nhan: string; }
interface HangDoi { danhSach: SuKien[]; demChen: number; }
function taoHangDoi(): HangDoi { return { danhSach: [], demChen: 0 }; }
function themSuKien(hd: HangDoi, thoiDiem: number, nhan: string): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, nhan });
  hd.demChen++;
}
function soSanhSuKien(a: SuKien, b: SuKien): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}
function layTiepTheo(hd: HangDoi): SuKien | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKien(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}

interface DiemDauVet { thoiDiem: number; nhan: string; }
function chayMoPhong(seed: bigint, soSuKien: number): DiemDauVet[] {
  const tt = gieoHat(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoi();
  for (let i = 0; i < soSuKien; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 50);
    themSuKien(hd, dh.hienTai + doTre, "sk" + i);
    tienToi(dh, doTre);
  }
  const dauVet: DiemDauVet[] = [];
  let sk;
  while ((sk = layTiepTheo(hd)) !== undefined) dauVet.push({ thoiDiem: sk.thoiDiem, nhan: sk.nhan });
  return dauVet;
}
function dauVetThanhChuoi(dv: DiemDauVet[]): string {
  return dv.map((d) => `${d.thoiDiem}:${d.nhan}`).join(",");
}

function khopVoiThamChieu(seed: bigint, soSuKien: number, thamChieu: string): boolean {
  return dauVetThanhChuoi(chayMoPhong(seed, soSuKien)) === thamChieu;
}

function demSoSeedKhop(danhSachSeed: bigint[], soSuKien: number, thamChieu: string): number {
  let dem = 0;
  for (const seed of danhSachSeed) {
    if (khopVoiThamChieu(seed, soSuKien, thamChieu)) dem++;
  }
  return dem;
}

const thamChieu = dauVetThanhChuoi(chayMoPhong(7n, 5));
console.log("tham chieu (dau vet cua seed=7n):", thamChieu);

const dsSeed = [7n, 8n, 7n, 9n, 7n];
console.log("so seed khop tham chieu:", demSoSeedKhop(dsSeed, 5, thamChieu));
```

```text title=readonly
tham chieu (dau vet cua seed=7n): 4:sk0,22:sk1,55:sk2,88:sk3,123:sk4
so seed khop tham chieu: 3
```

`thamChieu` LÀ dấu vết CỦA `seed=7n`. Danh sách thử `[7n, 8n, 7n,
9n, 7n]` CÓ đúng BA phần tử LÀ `7n` — VÀ `demSoSeedKhop` đếm đúng
`3`, KHÔNG phải `5`: `8n` VÀ `9n` cho dấu VẾT hoàn toàn khác, KHÔNG
khớp `thamChieu`. "Cùng seed = CÙNG vũ trụ" đúng THEO CẢ hai chiều —
khớp KHI VÀ CHỈ KHI cùng seed.
::::

::::example{#khong-phai-may-man}
Việc đúng `3` (không phải `5`, KHÔNG phải `0`) KHÔNG phải may mắn —
nó LÀ hệ quả TẤT yếu của việc mọi thành phần (bài 2-7) ĐỀU là hàm
THUẦN theo đúng seed VÀ trạng thái tường minh, KHÔNG hề chạm tới
`Date.now()`/`Math.random()`/bất kỳ nguồn "THẬT" nào từ hệ điều
hành (bài 1). NẾU dù chỉ MỘT dòng code Ở đâu ĐÓ lỡ gọi `Math.random()`
thật, `demSoSeedKhop([7n, 7n, 7n], 5, thamChieu)` sẽ KHÔNG còn chắc
chắn cho `3` nữa — VÀ đó chính LÀ cách một hệ thống DST thật phát
hiện "rò rỉ" nguồn không tất định trong code.
::::

::::predict{#doan-danh-sach-toan-seed-khop commitOnce}
Gọi `demSoSeedKhop([7n, 7n, 7n], 5, thamChieu)` (danh sách CHỈ toàn
`7n`, ba LẦN). Kết quả trả VỀ LÀ bao nhiêu?
:::opt{correct}
`3` — CẢ ba phần tử ĐỀU là `7n`, VÀ `chayMoPhong(7n, 5)` LUÔN cho ra
đúng CÙNG dấu vết mỗi lần gọi (bài 7) — KHỚP `thamChieu` cả BA lần
:::
:::opt
Có THỂ nhỏ hơn `3` — gọi `chayMoPhong` NHIỀU lần liên tiếp trong
CÙNG script có thể làm trạng thái "trôi", khiến vài LẦN không khớp
::why
Trực giác NÀY lo NGẠI về một dạng "rò rỉ trạng thái" GIỮA các lần
gọi — nhưng `chayMoPhong` (bài 7) LUÔN tạo `tt`, `dh`, `hd` HOÀN
toàn MỚI bên trong THÂN hàm mỗi lần được gọi.

Chỗ lệch: KHÔNG có biến TOÀN cục nào bị chia SẺ giữa các lần gọi
`chayMoPhong` — MỖI lời gọi ĐỘC lập hoàn toàn, chỉ phụ thuộc THAM số
`seed` truyền VÀO. Gọi `chayMoPhong(7n, 5)` bao NHIÊU lần, Ở bất kỳ
thứ tự NÀO, xen kẽ VỚI lời gọi seed khác HAY không, đều cho ĐÚNG
CÙNG một dấu vết — kết quả LUÔN là `3`.
::
:::
::::

::::code{#viet_dem_so_seed_khop}
Hoàn thiện `demSoSeedKhop` — VỚI mỗi seed, nếu `khopVoiThamChieu`
trả về `true`, tăng bộ đếm.

```typescript title=starter
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}

interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

interface SuKien { thoiDiem: number; thuTuChen: number; nhan: string; }
interface HangDoi { danhSach: SuKien[]; demChen: number; }
function taoHangDoi(): HangDoi { return { danhSach: [], demChen: 0 }; }
function themSuKien(hd: HangDoi, thoiDiem: number, nhan: string): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, nhan });
  hd.demChen++;
}
function soSanhSuKien(a: SuKien, b: SuKien): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}
function layTiepTheo(hd: HangDoi): SuKien | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKien(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}

interface DiemDauVet { thoiDiem: number; nhan: string; }
function chayMoPhong(seed: bigint, soSuKien: number): DiemDauVet[] {
  const tt = gieoHat(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoi();
  for (let i = 0; i < soSuKien; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 50);
    themSuKien(hd, dh.hienTai + doTre, "sk" + i);
    tienToi(dh, doTre);
  }
  const dauVet: DiemDauVet[] = [];
  let sk;
  while ((sk = layTiepTheo(hd)) !== undefined) dauVet.push({ thoiDiem: sk.thoiDiem, nhan: sk.nhan });
  return dauVet;
}
function dauVetThanhChuoi(dv: DiemDauVet[]): string {
  return dv.map((d) => `${d.thoiDiem}:${d.nhan}`).join(",");
}

function khopVoiThamChieu(seed: bigint, soSuKien: number, thamChieu: string): boolean {
  return dauVetThanhChuoi(chayMoPhong(seed, soSuKien)) === thamChieu;
}

function demSoSeedKhop(danhSachSeed: bigint[], soSuKien: number, thamChieu: string): number {
  let dem = 0;
  for (const seed of danhSachSeed) {
    ___
  }
  return dem;
}

const thamChieu = dauVetThanhChuoi(chayMoPhong(7n, 5));
console.log(demSoSeedKhop([7n, 8n, 7n, 9n, 7n], 5, thamChieu));
```

```typescript title=solution
const MASK64 = (1n << 64n) - 1n;
interface TrangThai { s0: bigint; s1: bigint; }
function gieoHat(seed: bigint): TrangThai {
  let s0 = seed & MASK64;
  if (s0 === 0n) s0 = 0x9e3779b97f4a7c15n;
  let s1 = (seed * 6364136223846793005n + 1442695040888963407n) & MASK64;
  if (s1 === 0n) s1 = 0xbf58476d1ce4e5b9n;
  return { s0, s1 };
}
function soTiepTheo(tt: TrangThai): bigint {
  let s1 = tt.s0;
  const s0 = tt.s1;
  const ketQua = (s1 + s0) & MASK64;
  tt.s0 = s0;
  s1 ^= (s1 << 23n) & MASK64;
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}
function soNguyenTrongKhoang(tt: TrangThai, min: number, max: number): number {
  return min + Number(soTiepTheo(tt) % BigInt(max - min));
}

interface DongHoAo { hienTai: number; }
function taoDongHoAo(): DongHoAo { return { hienTai: 0 }; }
function tienToi(dh: DongHoAo, soMs: number): void { dh.hienTai += soMs; }

interface SuKien { thoiDiem: number; thuTuChen: number; nhan: string; }
interface HangDoi { danhSach: SuKien[]; demChen: number; }
function taoHangDoi(): HangDoi { return { danhSach: [], demChen: 0 }; }
function themSuKien(hd: HangDoi, thoiDiem: number, nhan: string): void {
  hd.danhSach.push({ thoiDiem, thuTuChen: hd.demChen, nhan });
  hd.demChen++;
}
function soSanhSuKien(a: SuKien, b: SuKien): number {
  if (a.thoiDiem !== b.thoiDiem) return a.thoiDiem - b.thoiDiem;
  return a.thuTuChen - b.thuTuChen;
}
function layTiepTheo(hd: HangDoi): SuKien | undefined {
  if (hd.danhSach.length === 0) return undefined;
  let idxNhoNhat = 0;
  for (let i = 1; i < hd.danhSach.length; i++) {
    if (soSanhSuKien(hd.danhSach[i]!, hd.danhSach[idxNhoNhat]!) < 0) idxNhoNhat = i;
  }
  return hd.danhSach.splice(idxNhoNhat, 1)[0];
}

interface DiemDauVet { thoiDiem: number; nhan: string; }
function chayMoPhong(seed: bigint, soSuKien: number): DiemDauVet[] {
  const tt = gieoHat(seed);
  const dh = taoDongHoAo();
  const hd = taoHangDoi();
  for (let i = 0; i < soSuKien; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 50);
    themSuKien(hd, dh.hienTai + doTre, "sk" + i);
    tienToi(dh, doTre);
  }
  const dauVet: DiemDauVet[] = [];
  let sk;
  while ((sk = layTiepTheo(hd)) !== undefined) dauVet.push({ thoiDiem: sk.thoiDiem, nhan: sk.nhan });
  return dauVet;
}
function dauVetThanhChuoi(dv: DiemDauVet[]): string {
  return dv.map((d) => `${d.thoiDiem}:${d.nhan}`).join(",");
}

function khopVoiThamChieu(seed: bigint, soSuKien: number, thamChieu: string): boolean {
  return dauVetThanhChuoi(chayMoPhong(seed, soSuKien)) === thamChieu;
}

function demSoSeedKhop(danhSachSeed: bigint[], soSuKien: number, thamChieu: string): number {
  let dem = 0;
  for (const seed of danhSachSeed) {
    if (khopVoiThamChieu(seed, soSuKien, thamChieu)) dem++;
  }
  return dem;
}

const thamChieu = dauVetThanhChuoi(chayMoPhong(7n, 5));
console.log(demSoSeedKhop([7n, 8n, 7n, 9n, 7n], 5, thamChieu));
```

```typescript title=test
const thamChieuT = dauVetThanhChuoi(chayMoPhong(7n, 5));

const soKhop = demSoSeedKhop([7n, 8n, 7n, 9n, 7n], 5, thamChieuT);
if (soKhop !== 3) throw new Error("dung 3/5 seed (ba lan 7n) phai khop tham chieu, khong phai 5 hay 0");

const soKhopToanKhop = demSoSeedKhop([7n, 7n, 7n], 5, thamChieuT);
if (soKhopToanKhop !== 3) throw new Error("danh sach toan seed=7n (3 phan tu) phai khop CA 3");

const soKhopKhongCoGi = demSoSeedKhop([8n, 9n, 10n], 5, thamChieuT);
if (soKhopKhongCoGi !== 0) throw new Error("danh sach KHONG chua seed=7n nao thi phai khop dung 0");

if (demSoSeedKhop([], 5, thamChieuT) !== 0) throw new Error("danh sach rong thi phai dem duoc 0");

const thamChieuRong = dauVetThanhChuoi(chayMoPhong(7n, 0));
if (demSoSeedKhop([1n, 2n, 3n], 0, thamChieuRong) !== 3) throw new Error("soSuKien=0 thi MOI seed deu cho dau vet rong, khop voi thamChieuRong ('') -- phai la 3");
```

:::hints
- kind: attention
  body: "Neu khopVoiThamChieu(seed, soSuKien, thamChieu) la true thi tang dem -- mot dong."
- kind: strategy
  body: "if (khopVoiThamChieu(seed, soSuKien, thamChieu)) dem++;"
- kind: one-line
  body: "if (khopVoiThamChieu(seed, soSuKien, thamChieu)) dem++;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "3"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số ngẫu nhiên tất định, đồng hồ ảo, hàng đợi tất định — một vũ trụ
mô phỏng hoàn toàn do MỘT con số quyết định, đúng theo CẢ hai
chiều. q17 khép lại — q18 sẽ tiêm LỖI (mạng mất gói, đĩa hỏng) vào
chính vũ trụ NÀY, và nhờ nền tất định này, một lỗi tìm được LUÔN
tái hiện lại được.
::::

::::reflect{#nghi-lai}
`demSoSeedKhop` không giới THIỆU khái niệm MỚI nào — nó biến "tin
RẰNG mô phỏng tất định" (một câu khẳng ĐỊNH) thành "ĐO được, theo cả
hai chiều" (một con SỐ kiểm chứng được: không phải `0`, KHÔNG phải
`5`, mà đúng `3`), đúng tinh THẦN "đếm thật thay vì TIN" xuyên suốt
khoá học NÀY. Bài học lớn NHẤT của q17: tất định KHÔNG phải một
tính chất TỰ nhiên có sẵn — nó LÀ kết quả của việc LOẠI bỏ HOÀN
toàn mọi nguồn không tất định (`Date.now()`, `Math.random()`) VÀ
thay thế bằng những phiên bản tường MINH, seedable — VÀ chỉ CẦN một
dòng code lỡ dùng LẠI nguồn thật LÀ toàn bộ đảm bảo này sụp đổ ngay.
::::

::::checkpoint{mastery=0.9}
::::
