---
id: co-so-du-lieu.ke-pha-hoai-co-chu-dich.loi-co-chu-dich-khong-phai-ngau-nhien
title: "Lỗi có chủ đích, không phải ngẫu nhiên"
summary: "chonMucTieu dùng CHÍNH cái tt (trạng thái xorshift128+ từ q17) để chọn MỤC TIÊU sẽ bị phá — không chỉ độ trễ sự kiện như trước. Gọi chonNhieuMucTieu(gieoHat(3n), 5, 6) HAI lần độc lập cho ra ĐÚNG cùng dãy mục tiêu [4,0,2,3,1,0] cả hai lần; seed=9n cho dãy khác hẳn [3,1,0,1,4,4]. Đây LÀ nền tảng của cả q18: seed không chỉ quyết định 'khi nào' (q17) mà còn quyết định 'lỗi nào, ở đâu' — một kẻ phá hoại CÓ CHỦ ĐÍCH, tất định, không phải Math.random()."
locale: vi
track: co-so-du-lieu
module: ke-pha-hoai-co-chu-dich
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 12
teaches: [db.loi-co-chu-dich-khong-phai-ngau-nhien]
requires: [db.chay-lai-dung-tuyet-doi]
concepts: [db.loi-co-chu-dich-khong-phai-ngau-nhien]
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
q17 khép lại: MỘT seed quyết định toàn bộ "khi nào" của một vũ trụ mô
phỏng. Nhưng một kẻ phá hoại thật không chỉ cần biết KHI NÀO — nó cần
biết PHÁ CÁI GÌ. Cùng một `tt` đó có làm được việc thứ hai này không?
::::

::::explain{#chon-muc-tieu-bang-seed}
`chonMucTieu` dùng lại ĐÚNG `soNguyenTrongKhoang` (q17) — nhưng lần
này không để chọn ĐỘ TRỄ, mà để chọn MỘT CHỈ SỐ trong danh sách "mục
tiêu" (VÍ dụ: sector nào trên đĩa, gói tin nào trên mạng). Gọi liên
tiếp NHIỀU lần trên CÙNG một `tt` cho ra một DÃY mục tiêu — dãy đó
tất định HỆT như dãy độ trễ ở q17:

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

function chonMucTieu(tt: TrangThai, soLuongMucTieu: number): number {
  return soNguyenTrongKhoang(tt, 0, soLuongMucTieu);
}
function chonNhieuMucTieu(tt: TrangThai, soLuongMucTieu: number, soLan: number): number[] {
  const ketQua: number[] = [];
  for (let i = 0; i < soLan; i++) ketQua.push(chonMucTieu(tt, soLuongMucTieu));
  return ketQua;
}

const ttA = gieoHat(3n);
console.log("seed=3n lan 1:", chonNhieuMucTieu(ttA, 5, 6));
const ttA2 = gieoHat(3n);
console.log("seed=3n lan 2:", chonNhieuMucTieu(ttA2, 5, 6));
const ttB = gieoHat(9n);
console.log("seed=9n:", chonNhieuMucTieu(ttB, 5, 6));
```

```text title=readonly
seed=3n lan 1: [ 4, 0, 2, 3, 1, 0 ]
seed=3n lan 2: [ 4, 0, 2, 3, 1, 0 ]
seed=9n: [ 3, 1, 0, 1, 4, 4 ]
```

`chonMucTieu` KHÔNG phải một khái niệm mới — nó LÀ `soNguyenTrongKhoang`
gọi với `min=0`. Cái mới nằm ở CÁCH dùng: mỗi lần gọi trả về MỘT chỉ
số mục tiêu (0 tới `soLuongMucTieu - 1`), và VÌ nó tiêu thụ CHÍNH `tt`
(cùng trạng thái với `soNguyenTrongKhoang` sinh độ trễ ở q17), một seed
duy nhất giờ quyết định CẢ "khi nào" LẪN "cái gì" bị nhắm tới.
::::

::::example{#khong-phai-may-man-chon-dung}
Trong một hệ thống có 5 sector đĩa, một kẻ tấn công NGẪU NHIÊN thật
(`Math.random()`) chọn sector để phá — chạy lại KHÔNG chắc phá đúng
sector đó nữa, bug "biến mất". Với `chonMucTieu`, seed nào chọn sector
nào là CỐ ĐỊNH — chạy lại `chayKichBan(seedĐó)` LUÔN nhắm đúng lại
sector đã phá lần trước. Đây chính LÀ ý nghĩa "có chủ đích": không
phải kẻ phá hoại "biết trước" cái gì — mà LÀ hành vi của nó tái lập
được, giống một bên chương trình BÌNH thường.
::::

::::predict{#doan-goi-them-mot-lan commitOnce}
Từ `ttA` Ở khối trên (SAU khi đã gọi `chonNhieuMucTieu(ttA, 5, 6)`),
gọi THÊM `chonMucTieu(ttA, 5)` một lần NỮA. Kết quả CÓ chắc chắn LÀ
một trong 5 giá trị `4,0,2,3,1,0` đã thấy Ở TRÊN không?
:::opt{correct}
CÓ — `chonMucTieu` luôn trả về một số trong `[0, soLuongMucTieu)`,
tức `[0,5)`, NÊN kết quả PHẢI là một trong `{0,1,2,3,4}` — nhưng
KHÔNG chắc trùng ĐÚNG một trong sáu giá trị đã liệt kê Ở trên (dãy đã
in RA), vì `ttA` đã bị THAY đổi qua sáu lần gọi trước đó, lần gọi thứ
BẢY tiếp tục từ trạng thái MỚI, không lặp lại từ đầu
:::
:::opt
KHÔNG chắc — phạm vi `[0,5)` chỉ LÀ một RÀNG buộc lỏng, giá trị cụ
thể hoàn toàn có THỂ nằm ngoài `{0,1,2,3,4}`
::why
Trực giác NÀY nhầm "khó đoán trước con SỐ chính xác" VỚI "không có
ràng buộc GÌ về phạm vi" — hai điều khác nhau.

Chỗ lệch: `soNguyenTrongKhoang(tt, 0, 5)` LUÔN trả `min + (... % 5)`,
tức MỘT giá trị nằm chặt trong `[0,5)` bởi chính phép toán `%`, bất kể
`tt` đang Ở trạng thái nào. Cái KHÔNG đoán trước được LÀ giá trị cụ
thể nào trong năm giá trị ĐÓ — không phải liệu nó CÓ nằm trong phạm vi
hay không.
::
:::
::::

::::code{#viet_chon_muc_tieu}
Hoàn thiện `chonMucTieu` — gọi `soNguyenTrongKhoang` với `min=0`,
`max=soLuongMucTieu`.

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

function chonMucTieu(tt: TrangThai, soLuongMucTieu: number): number {
  ___
}
function chonNhieuMucTieu(tt: TrangThai, soLuongMucTieu: number, soLan: number): number[] {
  const ketQua: number[] = [];
  for (let i = 0; i < soLan; i++) ketQua.push(chonMucTieu(tt, soLuongMucTieu));
  return ketQua;
}

console.log(chonNhieuMucTieu(gieoHat(3n), 5, 6));
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

function chonMucTieu(tt: TrangThai, soLuongMucTieu: number): number {
  return soNguyenTrongKhoang(tt, 0, soLuongMucTieu);
}
function chonNhieuMucTieu(tt: TrangThai, soLuongMucTieu: number, soLan: number): number[] {
  const ketQua: number[] = [];
  for (let i = 0; i < soLan; i++) ketQua.push(chonMucTieu(tt, soLuongMucTieu));
  return ketQua;
}

console.log(chonNhieuMucTieu(gieoHat(3n), 5, 6));
```

```typescript title=test
const dayA1 = chonNhieuMucTieu(gieoHat(3n), 5, 6);
if (JSON.stringify(dayA1) !== "[4,0,2,3,1,0]") throw new Error("seed=3n, 6 muc tieu trong [0,5) phai la [4,0,2,3,1,0]");

const dayA2 = chonNhieuMucTieu(gieoHat(3n), 5, 6);
if (JSON.stringify(dayA2) !== JSON.stringify(dayA1)) throw new Error("goi lai CUNG seed=3n phai cho DUNG cung day muc tieu");

const dayB = chonNhieuMucTieu(gieoHat(9n), 5, 6);
if (JSON.stringify(dayB) === JSON.stringify(dayA1)) throw new Error("seed KHAC (9n) phai cho day muc tieu KHAC");

for (const soLuongMucTieu of [3, 5, 10]) {
  const tt = gieoHat(123n);
  for (let i = 0; i < 20; i++) {
    const mt = chonMucTieu(tt, soLuongMucTieu);
    if (mt < 0 || mt >= soLuongMucTieu) throw new Error(`chonMucTieu phai nam trong [0,${soLuongMucTieu}), nhan duoc ${mt}`);
  }
}

const ttMot = gieoHat(3n);
chonNhieuMucTieu(ttMot, 5, 6);
const mtThuBay = chonMucTieu(ttMot, 5);
if (mtThuBay < 0 || mtThuBay >= 5) throw new Error("lan goi thu 7 (tiep tuc tu trang thai da doi) van phai nam trong [0,5)");
```

:::hints
- kind: attention
  body: "Goi soNguyenTrongKhoang voi min=0, max=soLuongMucTieu -- mot dong."
- kind: strategy
  body: "return soNguyenTrongKhoang(tt, 0, soLuongMucTieu);"
- kind: one-line
  body: "return soNguyenTrongKhoang(tt, 0, soLuongMucTieu);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "4,0,2,3,1,0"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một seed giờ quyết định CẢ "khi nào" lẫn "phá gì". Bài tiếp theo: dùng
đúng cách chọn này để quyết định GIỮA hai lỗi đĩa đã học ở R6-1 (lost
fsync, torn write) — không phải người chọn, mà LÀ seed.
::::

::::reflect{#nghi-lai}
`chonMucTieu` không giới thiệu phép toán MỚI nào — nó LÀ đúng
`soNguyenTrongKhoang` của q17, chỉ đổi Ý NGHĨA của con số trả về (từ
"độ trễ" sang "chỉ số mục tiêu"). Đó chính LÀ điểm mạnh của việc tách
trạng thái tất định RA khỏi Ý nghĩa dùng nó: MỘT cơ chế (`tt`, `soTiep
Theo`) phục vụ được NHIỀU vai trò khác nhau, miễn LÀ mọi vai trò đều
đọc TỪ cùng một nguồn tất định, không hề đụng `Math.random()` thật.
::::

::::checkpoint{mastery=0.85}
::::
