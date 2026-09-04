---
id: co-so-du-lieu.vu-tru-tat-dinh.chay-lai-dung-tuyet-doi
title: "Chạy lại đúng tuyệt đối"
summary: "chayMoPhong ráp cả ba mảnh: xorshift128+ sinh độ trễ NGẪU NHIÊN NHƯNG TẤT ĐỊNH cho mỗi sự kiện, DongHoAo cộng dồn thời điểm, HangDoi (có thuTuChen) xử lý đúng thứ tự. Gọi chayMoPhong(7n, 5) HAI lần độc lập cho ra HAI dấu vết thực thi (thoiDiem + nhãn của từng sự kiện, theo đúng thứ tự xử lý) giống hệt nhau — 4:sk0,22:sk1,55:sk2,88:sk3,123:sk4 cả hai lần. Đổi seed thành 8n: dấu vết khác hẳn ngay từ số đầu tiên."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.chay-lai-dung-tuyet-doi]
requires: [db.hoa-thoi-diem-phai-tat-dinh]
concepts: [db.chay-lai-dung-tuyet-doi]
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
Ba mảnh ghép — số ngẫu nhiên tất định (bài 2-3), đồng hồ ảo (bài 4),
hàng đợi tất định kể cả khi hoà (bài 5-6). Ráp cả ba lại, chạy MỘT
kịch bản HAI lần — có THẬT sự khớp tuyệt đối không?
::::

::::explain{#rap-ba-manh}
`chayMoPhong` ráp CẢ ba mảnh: `xorshift128+` sinh ĐỘ TRỄ "ngẫu
nhiên NHƯNG tất định" CHO mỗi sự kiện, `DongHoAo` cộng DỒN thời
điểm, `HangDoi` (CÓ `thuTuChen`, bài 6) xử LÝ đúng thứ tự:

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

const traceA = chayMoPhong(7n, 5);
const traceB = chayMoPhong(7n, 5);
console.log("traceA:", dauVetThanhChuoi(traceA));
console.log("traceA khop traceB:", dauVetThanhChuoi(traceA) === dauVetThanhChuoi(traceB));

const traceC = chayMoPhong(8n, 5);
console.log("traceC (seed khac):", dauVetThanhChuoi(traceC));
console.log("traceA khac traceC:", dauVetThanhChuoi(traceA) !== dauVetThanhChuoi(traceC));
```

```text title=readonly
traceA: 4:sk0,22:sk1,55:sk2,88:sk3,123:sk4
traceA khop traceB: true
traceC (seed khac): 3:sk0,33:sk1,35:sk2,51:sk3,98:sk4
traceA khac traceC: true
```

`chayMoPhong(7n, 5)` gọi HAI lần ĐỘC lập (hai lần TẠO `tt`, `dh`,
`hd` hoàn TOÀN mới) cho RA đúng CÙNG dấu vết — MỖI sự kiện `skI`
CHỜ đúng cùng ĐỘ trễ, xảy RA đúng cùng thời ĐIỂM, xử lý đúng cùng
thứ TỰ. Đổi seed thành `8n`: dấu VẾT khác NGAY từ `skI` đầu tiên
(`4` → `3`).
::::

::::example{#tim-bug-roi-tai-hien-bug}
Đây chính LÀ "Deterministic Simulation Testing" (DST) — kỹ thuật
TigerBeetle nổi tiếng VÌ nó: chạy HÀNG triệu kịch bản MÔ phỏng ngẫu
nhiên (mỗi kịch bản MỘT seed), một kịch bản LÀM lộ bug — CHỈ cần
GHI lại đúng SEED đó (một con SỐ), chạy LẠI `chayMoPhong(seedĐó, ...)`
LÀ tái hiện chính XÁC bug, byte-for-byte, để GỠ lỗi thoải mái. So
với hệ thống dùng `Math.random()`/`Date.now()` THẬT (bài 1) — nơi
bug "biến MẤT" khi thử chạy LẠI — đây LÀ khác biệt SỐNG còn.
::::

::::predict{#doan-doi-so-su-kien commitOnce}
Gọi `chayMoPhong(7n, 3)` (CÙNG seed `7n`, nhưng CHỈ `3` sự kiện thay
VÌ `5`). Ba PHẦN tử ĐẦU của dấu vết CÓ giống ĐÚNG ba phần tử đầu của
`traceA` (`4:sk0,22:sk1,55:sk2`) không?
:::opt{correct}
CÓ — `tt` được TẠO lại TỪ ĐẦU bằng `gieoHat(7n)` MỖI lần gọi
`chayMoPhong`, nên `soNguyenTrongKhoang` sinh RA đúng cùng CHUỖI độ
trễ cho `3` sự kiện đầu TIÊN, bất kể tổng số sự kiện LÀ `3` hay `5`
:::
:::opt
KHÔNG chắc — thuật toán xorshift128+ CÓ thể sinh chuỗi số KHÁC nếu
tổng số LẦN gọi khác nhau
::why
Trực giác NÀY nhầm "số LẦN gọi TRONG một lần chạy" VỚI "chuỗi số
sinh RA" — hai điều KHÔNG liên quan tới nhau THEO CÁCH này.

Chỗ lệch: `soTiepTheo` LÀ một hàm THUẦN tuý phụ thuộc và trạng THÁI
HIỆN tại — nó KHÔNG hề "biết trước" TỔNG cộng sẽ có bao NHIÊU lần
gọi. `gieoHat(7n)` LUÔN cho đúng CÙNG trạng thái BAN đầu, và LẦN gọi
`soTiepTheo` thứ NHẤT, thứ HAI, thứ BA LUÔN cho đúng CÙNG ba số, bất
kể SAU đó có gọi THÊM lần thứ tư hay KHÔNG.
::
:::
::::

::::code{#viet_chay_mo_phong}
Hoàn thiện `chayMoPhong` — với MỖI sự kiện, thêm nó VÀO hàng đợi Ở
đúng thời điểm `dh.hienTai + doTre`.

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
    ___
    tienToi(dh, doTre);
  }
  const dauVet: DiemDauVet[] = [];
  let sk;
  while ((sk = layTiepTheo(hd)) !== undefined) dauVet.push({ thoiDiem: sk.thoiDiem, nhan: sk.nhan });
  return dauVet;
}

console.log(chayMoPhong(7n, 5).length);
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

console.log(chayMoPhong(7n, 5).length);
```

```typescript title=test
function dauVetThanhChuoiT(dv: DiemDauVet[]): string {
  return dv.map((d) => `${d.thoiDiem}:${d.nhan}`).join(",");
}

const traceA = chayMoPhong(7n, 5);
if (traceA.length !== 5) throw new Error("chayMoPhong(7n, 5) phai co dung 5 diem trong dau vet");
const chuoiA = dauVetThanhChuoiT(traceA);
if (chuoiA !== "4:sk0,22:sk1,55:sk2,88:sk3,123:sk4") throw new Error("dau vet chayMoPhong(7n, 5) phai dung 4:sk0,22:sk1,55:sk2,88:sk3,123:sk4");

const traceB = chayMoPhong(7n, 5);
if (dauVetThanhChuoiT(traceB) !== chuoiA) throw new Error("goi lai chayMoPhong CUNG seed=7n phai cho dau vet giong het lan truoc");

const traceC = chayMoPhong(8n, 5);
if (dauVetThanhChuoiT(traceC) === chuoiA) throw new Error("seed KHAC (8n) phai cho dau vet KHAC");

const traceRong = chayMoPhong(7n, 0);
if (traceRong.length !== 0) throw new Error("chayMoPhong(seed, 0) phai tra ve dau vet rong");

for (let i = 1; i < traceA.length; i++) {
  const truoc = traceA[i - 1]!;
  const sau = traceA[i]!;
  if (sau.thoiDiem < truoc.thoiDiem) throw new Error("dau vet PHAI theo dung thu tu thoiDiem TANG dan");
}
```

:::hints
- kind: attention
  body: "Them su kien vao hang doi o dung thoi diem dh.hienTai + doTre -- mot dong."
- kind: strategy
  body: "themSuKien(hd, dh.hienTai + doTre, \"sk\" + i);"
- kind: one-line
  body: "themSuKien(hd, dh.hienTai + doTre, \"sk\" + i);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "5"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một mô phỏng nhỏ CHẠY lại đúng tuyệt đối. Ráp một "vũ trụ" ĐẦY đủ
hơn — nhiều "tin nhắn", nhiều seed — trông ra SAO?
::::

::::reflect{#nghi-lai}
`chayMoPhong` KHÔNG giới thiệu MỘT khái niệm mới nào — nó XẾP đúng
thứ tự BA mảnh ghép đã học: `gieoHat`/`soTiepTheo` (bài 2-3) quyết
định "BAO lâu tới sự kiện tiếp theo", `DongHoAo` (bài 4) cộng dồn
thành "thời ĐIỂM thật", `HangDoi` (bài 5-6) xử lý ĐÚNG thứ tự, kể cả
khi hoà. Kết QUẢ: một "vũ trụ" nhỏ mà TOÀN bộ diễn biến của nó chỉ
phụ THUỘC vào đúng MỘT con số — `seed`. Đây LÀ nền tảng bắt buộc để
q18 (tiêm lỗi mạng/đĩa) VÀ q19 (đồng thuận VSR) tìm bug RỒI tái hiện
lại được CHÍNH XÁC bug đó.
::::

::::checkpoint{mastery=0.85}
::::
