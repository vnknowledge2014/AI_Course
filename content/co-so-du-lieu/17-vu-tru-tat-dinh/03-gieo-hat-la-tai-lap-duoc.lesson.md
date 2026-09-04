---
id: co-so-du-lieu.vu-tru-tat-dinh.gieo-hat-la-tai-lap-duoc
title: "Gieo hạt là tái lập được"
summary: "sinhDay tạo MỘT trạng thái mới từ seed rồi gọi soTiepTheo n lần, gom kết quả vào một mảng. sinhDay(42n, 5) gọi HAI lần độc lập (hai gieoHat(42n) riêng biệt, không chia sẻ trạng thái) cho ra HAI mảng giống hệt nhau — trong khi sinhDay(43n, 5) cho một mảng khác hẳn ngay từ phần tử đầu. Cùng seed, dù ở hai máy sinh số hoàn toàn tách biệt, luôn cho cùng một dãy."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [db.gieo-hat-la-tai-lap-duoc]
requires: [db.xorshift128-mot-may-sinh-so-tat-dinh]
concepts: [db.gieo-hat-la-tai-lap-duoc]
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
Bài trước dùng ĐÚNG một `tt` xuyên suốt. Nhưng "tái lập được" phải
CHỨNG minh mạnh hơn: HAI máy sinh số HOÀN toàn tách biệt, cùng seed,
CÓ thật sự cho ra CÙNG một dãy không?
::::

::::explain{#sinh-day-tu-seed}
`sinhDay` TẠO một trạng thái MỚI TỪ `seed` (gọi `gieoHat`), rồi gọi
`soTiepTheo` đúng `n` lần, gom kết QUẢ vào một mảng. Gọi `sinhDay`
HAI lần ĐỘC lập (hai `gieoHat(42n)` riêng biệt, KHÔNG chia sẻ trạng
thái):

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

function sinhDay(seed: bigint, n: number): bigint[] {
  const tt = gieoHat(seed);
  const ketQua: bigint[] = [];
  for (let i = 0; i < n; i++) ketQua.push(soTiepTheo(tt));
  return ketQua;
}

function dayThanhChuoi(day: bigint[]): string {
  return day.map((x) => x.toString()).join(",");
}

const dayA = sinhDay(42n, 3);
const dayB = sinhDay(42n, 3);
const dayC = sinhDay(43n, 3);

console.log("dayA:", dayThanhChuoi(dayA));
console.log("dayA khop dayB (cung seed):", dayThanhChuoi(dayA) === dayThanhChuoi(dayB));
console.log("dayA khac dayC (seed khac):", dayThanhChuoi(dayA) !== dayThanhChuoi(dayC));
```

```text title=readonly
dayA: 10481999410520547035,2517254593905905195,593085090057051017
dayA khop dayB (cung seed): true
dayA khac dayC (seed khac): true
```

`dayA` VÀ `dayB` sinh RA TỪ hai lời gọi `gieoHat(42n)` HOÀN toàn
riêng biệt (hai `object` `tt` khác nhau trong bộ NHỚ) — nhưng vì
CÙNG seed, cả hai đều đi qua ĐÚNG cùng một chuỗi phép TOÁN, cho ra
đúng CÙNG dãy số. `dayC` (seed `43n`) khác NGAY từ phần tử ĐẦU tiên.
::::

::::example{#tai-lap-tren-hai-may}
Đây chính LÀ điều `Math.random()` KHÔNG BAO giờ làm được (bài 1):
KHÔNG hề có tham số seed để "khởi động lại đúng chỗ CŨ". VỚI
`xorshift128+`, chỉ CẦN GHI lại đúng một con số (`seed`) LÀ đủ để
TÁI hiện nguyên VẸN cả một dãy dài — dù dãy đó dùng để MÔ phỏng hàng
triệu sự KIỆN. Tìm một BUG hiếm? Ghi LẠI seed đã dùng LÚC tìm ra nó,
chạy LẠI với đúng seed ĐÓ, bug xuất hiện LẠI y hệt.
::::

::::predict{#doan-sinh-day-0-phan-tu commitOnce}
Gọi `sinhDay(42n, 0)` (yêu cầu `0` số). Kết quả trả VỀ LÀ gì?
:::opt{correct}
Một mảng RỖNG `[]` — vòng `for (let i = 0; i < 0; i++)` KHÔNG chạy
lần NÀO, `ketQua` giữ nguyên giá trị khởi TẠO (mảng rỗng)
:::
:::opt
Lỗi runtime — `gieoHat` VẪN chạy nhưng "0 số" LÀ một yêu cầu KHÔNG
hợp lệ
::why
Không CÓ dòng code NÀO trong `sinhDay` kiểm tra `n` PHẢI lớn hơn `0`
— không CÓ điều kiện nào để NÉM lỗi.

Chỗ lệch: `gieoHat(seed)` VẪN chạy bình THƯỜNG (tạo trạng thái, không
liên quan tới `n`) — CHỈ có vòng `for` phụ thuộc `n`. VỚI `n=0`, điều
kiện `i < 0` sai NGAY từ đầu, vòng lặp KHÔNG thực thi thân nó lần
nào, VÀ hàm trả về đúng mảng rỗng `[]` đã khởi tạo — không lỗi.
::
:::
::::

::::code{#viet_sinh_day}
Hoàn thiện `sinhDay` — gọi `soTiepTheo(tt)` đúng `n` lần, đẩy MỖI
kết quả vào `ketQua`.

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

function sinhDay(seed: bigint, n: number): bigint[] {
  const tt = gieoHat(seed);
  const ketQua: bigint[] = [];
  for (let i = 0; i < n; i++) {
    ___
  }
  return ketQua;
}

console.log(sinhDay(42n, 3).length);
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

function sinhDay(seed: bigint, n: number): bigint[] {
  const tt = gieoHat(seed);
  const ketQua: bigint[] = [];
  for (let i = 0; i < n; i++) {
    ketQua.push(soTiepTheo(tt));
  }
  return ketQua;
}

console.log(sinhDay(42n, 3).length);
```

```typescript title=test
function dayThanhChuoiT(day: bigint[]): string {
  return day.map((x) => x.toString()).join(",");
}

const dayA = sinhDay(42n, 3);
const dayB = sinhDay(42n, 3);
if (dayA.length !== 3) throw new Error("sinhDay(42n, 3) phai co dung 3 phan tu");
if (dayThanhChuoiT(dayA) !== dayThanhChuoiT(dayB)) throw new Error("hai lan goi sinhDay CUNG seed phai cho DAY giong het nhau");

const dayC = sinhDay(43n, 3);
if (dayThanhChuoiT(dayA) === dayThanhChuoiT(dayC)) throw new Error("seed KHAC nhau phai cho day KHAC nhau");

const dayRong = sinhDay(42n, 0);
if (dayRong.length !== 0) throw new Error("sinhDay(seed, 0) phai tra ve mang rong");

const day5 = sinhDay(1n, 5);
if (day5.length !== 5) throw new Error("sinhDay(1n, 5) phai co dung 5 phan tu");
const tapHop = new Set(day5.map(String));
if (tapHop.size !== 5) throw new Error("5 so lien tiep sinh ra tu xorshift128+ voi seed nay khong duoc co gia tri TRUNG LAP nao");
```

:::hints
- kind: attention
  body: "Goi soTiepTheo(tt) va day ket qua vao mang ketQua -- mot dong."
- kind: strategy
  body: "ketQua.push(soTiepTheo(tt));"
- kind: one-line
  body: "ketQua.push(soTiepTheo(tt));"
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
Số ngẫu nhiên tất định XONG. Nhưng mô phỏng còn CẦN một trục KHÁC:
thời gian — VÀ `Date.now()` cũng KHÔNG tái lập được y hệt `Math.
random()`.
::::

::::reflect{#nghi-lai}
`sinhDay` chứng minh đúng ĐIỀU quan trọng nhất của bài NÀY: "tất
định" KHÔNG phụ thuộc VÀO việc dùng LẠI đúng một object trong bộ
nhớ — nó phụ thuộc và đúng MỘT con số (`seed`). Hai máy KHÁC nhau,
hai lần chạy khác NHAU, thậm chí hai NGÔN ngữ khác nhau (nếu cùng
CÀI đúng thuật toán) — MIỄN cùng seed, dãy số sinh RA LUÔN khớp
tuyệt đối. Đây LÀ viên gạch đầu tiên của "vũ trụ tất định": biến MỌI
quyết định "ngẫu nhiên" trong mô phỏng thành MỘT lời gọi `soTiepTheo`.
::::

::::checkpoint{mastery=0.85}
::::
