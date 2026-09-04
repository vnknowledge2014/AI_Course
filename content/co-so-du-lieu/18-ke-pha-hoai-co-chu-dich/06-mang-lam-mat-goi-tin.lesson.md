---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.mang-lam-mat-goi-tin
title: "Mạng làm mất gói tin"
summary: "matGoiTheoSeed(tt,tyLeMatPhanTram) dùng soNguyenTrongKhoang(tt,0,100) < tyLeMatPhanTram để quyết định MỖI gói tin có bị rớt hay không -- một 'tung xúc xắc trăm mặt' tất định. guiNhieuGoi gửi 20 gói với tỉ lệ mất 30%: seed=1n rớt đúng 5/20 gói Ở đúng những vị trí X......X...X..XX...., LUÔN giống hệt khi chạy lại; seed=42n rớt 7/20 Ở những vị trí khác hẳn. Lỗi mạng đầu tiên của q18 -- không dùng SimDisk, một cấu trúc HOÀN TOÀN mới."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.mang-lam-mat-goi-tin]
requires: [db.crash-tat-dinh-thu-n]
concepts: [db.mang-lam-mat-goi-tin]
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
Đĩa xong (bài 2-5): lost fsync, torn write, latent sector error,
misdirected write, crash tất định thứ N. Nhưng TigerBeetle chạy trên
NHIỀU máy, nói chuyện qua MẠNG — và mạng CŨNG có lỗi riêng của nó.
::::

::::explain{#mang-mat-goi}
`matGoiTheoSeed` mô phỏng lỗi mạng đơn giản NHẤT: một gói tin CÓ thể
"rớt" (không bao giờ tới nơi). Dùng đúng `soNguyenTrongKhoang` (q17) —
lần này KHÔNG chọn mục tiêu hay độ trễ, mà LÀM một "tung xúc xắc trăm
mặt": rớt nếu số ra NHỎ hơn tỉ lệ mất phần trăm:

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

function matGoiTheoSeed(tt: TrangThai, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;
}
function guiNhieuGoi(tt: TrangThai, soLuongGoi: number, tyLeMatPhanTram: number): boolean[] {
  const ketQua: boolean[] = [];
  for (let i = 0; i < soLuongGoi; i++) ketQua.push(matGoiTheoSeed(tt, tyLeMatPhanTram));
  return ketQua;
}

const tt1 = gieoHat(1n);
const ket1 = guiNhieuGoi(tt1, 20, 30);
console.log(`seed=1n: ${ket1.filter(Boolean).length}/20 mat -- ${ket1.map((b) => (b ? "X" : ".")).join("")}`);

const tt42 = gieoHat(42n);
const ket42 = guiNhieuGoi(tt42, 20, 30);
console.log(`seed=42n: ${ket42.filter(Boolean).length}/20 mat -- ${ket42.map((b) => (b ? "X" : ".")).join("")}`);
```

```text title=readonly
seed=1n: 5/20 mat -- X......X...X..XX....
seed=42n: 7/20 mat -- ..X..X.....X.X.XX..X
```

Tỉ lệ ĐẶT là `30%` nhưng số THỰC TẾ rớt (`5/20=25%`, `7/20=35%`) không
khớp CHÍNH XÁC `30%` — ĐÚNG như tung một đồng xu thiên vị 20 lần không
ra chính xác 30% mặt "X". Cái QUAN TRỌNG không phải tỉ lệ trung bình
khớp tuyệt đối, mà LÀ: gọi lại `guiNhieuGoi(gieoHat(1n), 20, 30)` LUÔN
cho ĐÚNG lại chuỗi `X......X...X..XX....`, không đổi.
::::

::::example{#khong-phai-mang-that}
Mạng THẬT rớt gói theo hàng NGÀN nguyên nhân vật lý (nhiễu, tắc nghẽn,
switch lỗi) — không hề tất định. `matGoiTheoSeed` KHÔNG mô phỏng "tại
sao" gói rớt, chỉ mô phỏng "gói NÀO rớt, theo một quy luật tái lập
được" — đủ để kiểm tra hệ thống XỬ LÝ mất gói đúng cách, không cần một
mạng thật để tạo ra tình huống đó.
::::

::::predict{#doan-ty-le-0-phan-tram commitOnce}
Gọi `guiNhieuGoi(gieoHat(5n), 20, 0)` (tỉ lệ mất LÀ `0`). Kết quả CÓ
thể chứa `true` (một gói bị rớt) không?
:::opt{correct}
KHÔNG — `matGoiTheoSeed` trả `true` khi `soNguyenTrongKhoang(tt,0,100)
< 0`, nhưng `soNguyenTrongKhoang(tt,0,100)` LUÔN nằm trong `[0,100)`,
tức LUÔN `>= 0` — không GIÁ trị nào nhỏ hơn `0` được, nên KHÔNG gói
nào rớt
:::
:::opt
Có thể — dù tỉ lệ LÀ `0`, vẫn CÓ một xác suất nhỏ ngẫu nhiên gây rớt,
giống mạng THẬT không bao giờ đạt "0% lỗi" tuyệt đối
::why
Trực giác NÀY đúng cho mạng THẬT (không hệ thống vật lý nào đạt đúng
`0%` lỗi tuyệt đối) — nhưng SAI cho MÔ hình toán học Ở đây, VÌ đây LÀ
một phép SO SÁNH số nguyên, không phải một quá trình vật lý.

Chỗ lệch: `tyLeMatPhanTram=0` làm điều kiện `soNguyenTrongKhoang(...)
< 0` trở thành KHÔNG THỂ đúng — miền giá trị `[0,100)` không hề chứa
số ÂM. Đây LÀ một trong những lý do mô phỏng tất định HỮU ích: nó cho
phép biểu diễn "chính XÁC 0% lỗi", một điều mạng thật không BAO GIỜ
đảm bảo được.
::
:::
::::

::::code{#viet_mat_goi_theo_seed}
Hoàn thiện `matGoiTheoSeed` — gói rớt khi `soNguyenTrongKhoang(tt,0,100)`
NHỎ hơn `tyLeMatPhanTram`.

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

function matGoiTheoSeed(tt: TrangThai, tyLeMatPhanTram: number): boolean {
  ___
}
function guiNhieuGoi(tt: TrangThai, soLuongGoi: number, tyLeMatPhanTram: number): boolean[] {
  const ketQua: boolean[] = [];
  for (let i = 0; i < soLuongGoi; i++) ketQua.push(matGoiTheoSeed(tt, tyLeMatPhanTram));
  return ketQua;
}

const ket = guiNhieuGoi(gieoHat(1n), 20, 30);
console.log(ket.filter(Boolean).length);
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

function matGoiTheoSeed(tt: TrangThai, tyLeMatPhanTram: number): boolean {
  return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;
}
function guiNhieuGoi(tt: TrangThai, soLuongGoi: number, tyLeMatPhanTram: number): boolean[] {
  const ketQua: boolean[] = [];
  for (let i = 0; i < soLuongGoi; i++) ketQua.push(matGoiTheoSeed(tt, tyLeMatPhanTram));
  return ketQua;
}

const ket = guiNhieuGoi(gieoHat(1n), 20, 30);
console.log(ket.filter(Boolean).length);
```

```typescript title=test
const ket1 = guiNhieuGoi(gieoHat(1n), 20, 30);
if (JSON.stringify(ket1) !== "[true,false,false,false,false,false,false,true,false,false,false,true,false,false,true,true,false,false,false,false]") {
  throw new Error("seed=1n, 20 goi, ty le 30% phai cho DUNG chuoi X......X...X..XX....");
}
if (ket1.filter(Boolean).length !== 5) throw new Error("seed=1n phai co dung 5/20 goi mat");

const ket1Lai = guiNhieuGoi(gieoHat(1n), 20, 30);
if (JSON.stringify(ket1Lai) !== JSON.stringify(ket1)) throw new Error("goi lai CUNG seed=1n phai cho DUNG cung ket qua");

const ket0PhanTram = guiNhieuGoi(gieoHat(5n), 20, 0);
if (ket0PhanTram.some(Boolean)) throw new Error("ty le mat 0% thi KHONG goi nao duoc phep mat");

const ket100PhanTram = guiNhieuGoi(gieoHat(5n), 20, 100);
if (!ket100PhanTram.every(Boolean)) throw new Error("ty le mat 100% thi MOI goi deu phai mat");

const ket42 = guiNhieuGoi(gieoHat(42n), 20, 30);
if (JSON.stringify(ket42) === JSON.stringify(ket1)) throw new Error("seed KHAC (42n) phai cho ket qua KHAC seed=1n");

const ttBienGioi1 = gieoHat(7n);
const rawBienGioi = soNguyenTrongKhoang(ttBienGioi1, 0, 100);
const ttBienGioi2 = gieoHat(7n);
if (matGoiTheoSeed(ttBienGioi2, rawBienGioi) !== false) {
  throw new Error(`bien gioi: ty le mat DUNG BANG so vua rut ra (${rawBienGioi}) phai KHONG mat goi -- phep so sanh phai la '<' nghiem ngat, khong phai '<='`);
}
```

:::hints
- kind: attention
  body: "Rot khi soNguyenTrongKhoang(tt,0,100) < tyLeMatPhanTram -- mot dong."
- kind: strategy
  body: "return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;"
- kind: one-line
  body: "return soNguyenTrongKhoang(tt, 0, 100) < tyLeMatPhanTram;"
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
Mất gói xong. Nhưng mạng còn tệ hơn: gói không mất, chỉ TỚI TRỄ — và
trễ khác nhau làm chúng tới SAI thứ tự.
::::

::::reflect{#nghi-lai}
`matGoiTheoSeed` không giới thiệu phép toán MỚI — nó tái sử dụng đúng
`soNguyenTrongKhoang` theo một cách khác: một "tung xúc xắc" thay VÌ
một khoảng giá trị liên tục. Bài học: MỘT khối xây dựng tất định
(`soNguyenTrongKhoang`) đủ LINH HOẠT để mô phỏng nhiều LOẠI quyết định
ngẫu-nhiên-nhưng-tất-định khác nhau — chọn mục tiêu (bài 1), chọn kiểu
lỗi (bài 2), VÀ giờ LÀ rớt/không rớt một gói tin.
::::

::::checkpoint{mastery=0.85}
::::
