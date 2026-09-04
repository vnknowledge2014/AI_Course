---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.mang-lam-tre-va-dao-thu-tu
title: "Mạng làm trễ và đảo thứ tự"
summary: "moPhongMangTre gui 6 goi tin cach nhau 10ms, moi goi tre ngau-nhien-nhung-tat-dinh trong [1,40)ms (soNguyenTrongKhoang), roi dua vao HangDoi (q17, co thuTuChen) de lay dung thu tu DEN NOI. seed=2n: goi0,goi2,goi1,goi4,goi3,goi5 -- goi2 den TRUOC goi1 vi tre it hon, dao nguoc thu tu gui. seed=1n: khong dao thu tu nao ca -- khong phai seed nao cung gay dao thu tu."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.mang-lam-tre-va-dao-thu-tu]
requires: [db.mang-lam-mat-goi-tin]
concepts: [db.mang-lam-tre-va-dao-thu-tu]
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
Mất gói (bài trước) LÀ lỗi mạng rõ ràng nhất — nhưng lỗi mạng phổ biến
HƠN, khó bắt hơn NHIỀU, LÀ: gói không mất, chỉ tới TRỄ khác nhau, và
tới SAI thứ tự lúc gửi.
::::

::::explain{#tre-va-dao-thu-tu}
`moPhongMangTre` gửi `soLuongGoi` gói, mỗi gói cách gói TRƯỚC `10`ms
(thời điểm GỬI tăng đều) — nhưng mỗi gói CHỊU một độ trễ NGẪU-NHIÊN-
NHƯNG-TẤT-ĐỊNH riêng (`soNguyenTrongKhoang`, q17) trước khi "đến nơi".
Dùng lại đúng `HangDoi`/`themSuKien`/`layTiepTheo` (q17) để lấy ĐÚNG
thứ tự ĐẾN, sắp theo thời điểm đến — CÓ thể khác hẳn thứ tự GỬI:

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

function moPhongMangTre(seed: bigint, soLuongGoi: number): string[] {
  const tt = gieoHat(seed);
  const hd = taoHangDoi();
  const dh = taoDongHoAo();
  for (let i = 0; i < soLuongGoi; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 40);
    themSuKien(hd, dh.hienTai + doTre, "goi" + i);
    tienToi(dh, 10);
  }
  const thuTuDen: string[] = [];
  let sk;
  while ((sk = layTiepTheo(hd)) !== undefined) thuTuDen.push(sk.nhan);
  return thuTuDen;
}

console.log("seed=1n:", moPhongMangTre(1n, 6));
console.log("seed=2n:", moPhongMangTre(2n, 6));
```

```text title=readonly
seed=1n: [ 'goi0', 'goi1', 'goi2', 'goi3', 'goi4', 'goi5' ]
seed=2n: [ 'goi0', 'goi2', 'goi1', 'goi4', 'goi3', 'goi5' ]
```

`seed=1n` không đảo thứ tự NÀO — các độ trễ tình cờ đủ "công bằng" để
thứ tự đến TRÙNG thứ tự gửi (KHÔNG phải mọi seed đều gây đảo thứ tự).
`seed=2n` đảo NGAY hai cặp: `goi2` (gửi SAU `goi1` `10`ms) đến TRƯỚC
`goi1` VÌ độ trễ của `goi2` ngắn hơn ĐỦ để bù lại — mạng THẬT gọi đây
LÀ "out-of-order delivery", một trong những nguồn bug khó nhất Ở hệ
phân tán khi code lỡ giả định "gói tới theo đúng thứ tự gửi".
::::

::::example{#vi-sao-nguy-hiem}
Một hệ thống ngây thơ xử lý gói THEO thứ tự NHẬN (không kiểm tra số
thứ tự bên trong gói) sẽ áp dụng nhầm `goi2` TRƯỚC `goi1` — nếu `goi1`
LÀ "mở tài khoản" và `goi2` LÀ "gửi tiền vào tài khoản đó", áp dụng sai
thứ tự LÀM sập cả logic. Đây chính LÀ lý do các giao thức đồng thuận
thật (q19, VSR) LUÔN đánh số thứ tự tường minh, không dựa vào thứ tự
mạng giao gói.
::::

::::predict{#doan-doi-khoang-tre commitOnce}
Nếu đổi khoảng trễ TỪ `[1,40)` xuống `[1,5)` (độ trễ tối đa CHỈ còn
`4`ms, trong khi mỗi gói vẫn gửi cách nhau `10`ms) — VỚI CÙNG seed
bất kỳ, thứ tự đến CÓ còn khả năng đảo so với thứ tự gửi không?
:::opt{correct}
KHÔNG (thực tế) — độ trễ tối đa (`4`ms) nhỏ HƠN khoảng cách gửi
(`10`ms), nên gói `i` LUÔN đến trước thời điểm GỬI của gói `i+1`
(`10i + tối đa 4` < `10(i+1)`), thứ tự đến LUÔN trùng thứ tự gửi
:::
:::opt
Có — độ trễ vẫn "ngẫu nhiên", NÊN vẫn luôn CÓ khả năng đảo thứ tự dù
nhỏ tới đâu
::why
Trực giác NÀY đúng NẾU độ trễ có thể LỚN hơn khoảng cách gửi — nhưng
sai khi biết CHẶN trên của độ trễ (`4`) chắc chắn nhỏ hơn khoảng cách
gửi (`10`).

Chỗ lệch: đảo thứ tự CHỈ xảy ra khi thời điểm ĐẾN của một gói SAU vượt
QUA thời điểm đến của một gói TRƯỚC nó. Nếu độ trễ tối đa < khoảng
cách gửi, gói `i` LUÔN đến (chậm nhất `10i+4`) TRƯỚC khi gói `i+1`
được gửi (`10(i+1)=10i+10`) — không CÒN "cửa sổ" nào để đảo thứ tự cả.
::
:::
::::

::::code{#viet_mo_phong_mang_tre}
Hoàn thiện `moPhongMangTre` — với MỖI gói, thêm nó vào hàng đợi Ở
đúng thời điểm ĐẾN: `dh.hienTai + doTre`.

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

function moPhongMangTre(seed: bigint, soLuongGoi: number): string[] {
  const tt = gieoHat(seed);
  const hd = taoHangDoi();
  const dh = taoDongHoAo();
  for (let i = 0; i < soLuongGoi; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 40);
    ___
    tienToi(dh, 10);
  }
  const thuTuDen: string[] = [];
  let sk;
  while ((sk = layTiepTheo(hd)) !== undefined) thuTuDen.push(sk.nhan);
  return thuTuDen;
}

console.log(moPhongMangTre(2n, 6));
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

function moPhongMangTre(seed: bigint, soLuongGoi: number): string[] {
  const tt = gieoHat(seed);
  const hd = taoHangDoi();
  const dh = taoDongHoAo();
  for (let i = 0; i < soLuongGoi; i++) {
    const doTre = soNguyenTrongKhoang(tt, 1, 40);
    themSuKien(hd, dh.hienTai + doTre, "goi" + i);
    tienToi(dh, 10);
  }
  const thuTuDen: string[] = [];
  let sk;
  while ((sk = layTiepTheo(hd)) !== undefined) thuTuDen.push(sk.nhan);
  return thuTuDen;
}

console.log(moPhongMangTre(2n, 6));
```

```typescript title=test
const thuTu1 = moPhongMangTre(1n, 6);
if (JSON.stringify(thuTu1) !== JSON.stringify(["goi0","goi1","goi2","goi3","goi4","goi5"])) {
  throw new Error("seed=1n phai cho DUNG thu tu goi0..goi5, khong dao");
}

const thuTu2 = moPhongMangTre(2n, 6);
if (JSON.stringify(thuTu2) !== JSON.stringify(["goi0","goi2","goi1","goi4","goi3","goi5"])) {
  throw new Error("seed=2n phai cho DUNG thu tu goi0,goi2,goi1,goi4,goi3,goi5 (co dao)");
}

const thuTu2Lai = moPhongMangTre(2n, 6);
if (JSON.stringify(thuTu2Lai) !== JSON.stringify(thuTu2)) throw new Error("goi lai CUNG seed=2n phai cho DUNG cung thu tu");

if (moPhongMangTre(1n, 0).length !== 0) throw new Error("soLuongGoi=0 phai cho mang rong");

const guiThuTuGoc = ["goi0","goi1","goi2","goi3","goi4","goi5"];
if (JSON.stringify(thuTu2.slice().sort()) !== JSON.stringify(guiThuTuGoc.slice().sort())) {
  throw new Error("dao thu tu KHONG duoc lam mat goi nao -- tap hop nhan phai giong het tap hop gui");
}
```

:::hints
- kind: attention
  body: "Them su kien vao hang doi o dung thoi diem den: dh.hienTai + doTre -- mot dong."
- kind: strategy
  body: "themSuKien(hd, dh.hienTai + doTre, \"goi\" + i);"
- kind: one-line
  body: "themSuKien(hd, dh.hienTai + doTre, \"goi\" + i);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "goi2"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Năm lỗi tách rời xong: ba lỗi đĩa, một cách kích crash tất định, hai
lỗi mạng. Bài tiếp theo: gộp TẤT cả lại dưới đúng MỘT seed.
::::

::::reflect{#nghi-lai}
`moPhongMangTre` không giới thiệu cấu trúc MỚI — nó ráp lại đúng
`HangDoi`/`DongHoAo` (q17) theo một Ý nghĩa khác: không phải "sự kiện
mô phỏng nội bộ" mà LÀ "gói tin trên mạng". Bài học lớn nhất: hạ tầng
tất định (bài 1-7) không quan tâm nó đang mô phỏng ĐĨA hay MẠNG — nó
chỉ quan tâm "cùng trạng thái đầu vào, cùng kết quả", và đó LÀ tất cả
những gì cần để một bug tái hiện được.
::::

::::checkpoint{mastery=0.85}
::::
