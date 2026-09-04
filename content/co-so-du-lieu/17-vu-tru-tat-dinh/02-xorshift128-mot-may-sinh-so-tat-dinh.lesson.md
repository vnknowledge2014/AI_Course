---
id: co-so-du-lieu.vu-tru-tat-dinh.xorshift128-mot-may-sinh-so-tat-dinh
title: "xorshift128+ — máy sinh số tất định"
summary: "soTiepTheo cài thuật toán xorshift128+ THẬT (dùng trong V8, Firefox) — trạng thái 128-bit (hai số 64-bit s0/s1, biểu diễn bằng BigInt), mỗi lần gọi trộn bit bằng xor+shift RỒI trả về một số 64-bit mới. Khác Math.random(): soTiepTheo là một HÀM THUẦN nhận trạng thái làm tham số — cùng trạng thái đầu vào LUÔN cho ra cùng số kế tiếp, mọi lần gọi, trên mọi máy."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [db.xorshift128-mot-may-sinh-so-tat-dinh]
requires: [db.kiem-thu-khong-tai-lap-duoc]
concepts: [db.xorshift128-mot-may-sinh-so-tat-dinh]
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
`Math.random()` không tái lập được (bài trước) VÌ nó tự giấu TRẠNG
thái bên trong, không cho code nào đụng TỚI. Nếu trạng thái ĐÓ là
một GIÁ trị tường minh — MỘT tham số bình thường — thì SAO?
::::

::::explain{#xorshift128-that}
`soTiepTheo` cài đúng thuật TOÁN "xorshift128+" (dùng THẬT trong V8,
Firefox trước ĐÂY) — trạng thái LÀ hai số 64-bit (`s0`, `s1`, biểu
diễn bằng `bigint` VÌ JavaScript's `number` không đủ CHÍNH xác cho
số nguyên 64-bit). Mỗi lần gọi, hàm TRỘN bit bằng xor VÀ shift, RỒI
trả VỀ một số MỚI — VÀ chính trạng thái CŨNG đổi, sẵn sàng CHO lần
gọi sau:

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

const tt = gieoHat(42n);
console.log("so 1:", soTiepTheo(tt));
console.log("so 2:", soTiepTheo(tt));
console.log("so 3:", soTiepTheo(tt));
```

```text title=readonly
so 1: 10481999410520547035n
so 2: 2517254593905905195n
so 3: 593085090057051017n
```

MỖI lần gọi `soTiepTheo(tt)` CHO một số 64-bit MỚI — VÀ tham số `tt`
BỊ thay đổi (`tt.s0`, `tt.s1` được ghi ĐÈ) để lần gọi SAU tiếp tục
đúng CHỖ. Không CÓ `Date.now()`, không CÓ nguồn "ngẫu nhiên THẬT" nào
từ hệ điều HÀNH — MỌI con số Ở đây chỉ LÀ phép toán bit TRÊN trạng
thái đã CHO.
::::

::::example{#khong-phai-ngau-nhien-that}
`soTiepTheo` KHÔNG hề "ngẫu nhiên" theo nghĩa CHẶT — nó LÀ một hàm
TẤT định: đưa VÀO đúng CÙNG một `tt` (cùng `s0`, `s1`), LUÔN nhận
LẠI đúng CÙNG một số. Dãy số SINH ra "trông" ngẫu nhiên (khó đoán
NẾU chưa biết trạng thái), nhưng BẢN chất LÀ một phép biến đổi bit
LẶP đi lặp lại — CHÍNH tính CHẤT "tất định nhưng trông ngẫu nhiên"
đó LÀ điều bài trước (`Math.random()`) KHÔNG có.
::::

::::predict{#doan-hai-lan-goi-cung-mot-tt commitOnce}
Tạo MỘT `tt` DUY nhất bằng `gieoHat(5n)`. Gọi `soTiepTheo(tt)` HAI
lần LIÊN tiếp (không tạo `tt` mới GIỮA hai lần gọi). Hai kết QUẢ trả
về CÓ giống nhau KHÔNG?
:::opt{correct}
KHÔNG — MỖI lần gọi `soTiepTheo` đều THAY đổi `tt.s0`/`tt.s1` (dòng
`tt.s0 = s0;` VÀ `tt.s1 = s1`), NÊN lần gọi THỨ hai nhận một trạng
THÁI đã khác lần đầu, cho RA một số khác
:::
:::opt
CÓ — CÙNG một `tt` (không tạo LẠI) thì hàm TẤT định phải cho CÙNG
kết quả mỗi LẦN gọi
::why
Trực giác "hàm TẤT định thì luôn cho CÙNG kết quả" đúng khi ĐẦU vào
GIỐNG hệt — nhưng `tt` KHÔNG còn giống hệt SAU lần gọi đầu, vì
`soTiepTheo` tự SỬA nó.

Chỗ lệch: `soTiepTheo` NHẬN `tt` LÀM tham số, nhưng KHÔNG chỉ đọc —
nó GHI đè `tt.s0` VÀ `tt.s1` NGAY bên trong thân hàm (đột biến trạng
thái, KHÔNG phải hàm THUẦN theo nghĩa CHẶT). Lần gọi thứ HAI nhận
đúng trạng thái ĐÃ bị đổi bởi lần gọi ĐẦU — "tất định" Ở ĐÂY nghĩa
LÀ CÙNG một CHUỖI trạng thái LUÔN sinh ra CÙNG một CHUỖI số, không
phải MỘT trạng thái CỐ định sinh MÃI một SỐ.
::
:::
::::

::::code{#viet_so_tiep_theo}
Hoàn thiện `soTiepTheo` — bước trộn bit THỨ nhất: `s1` xor VỚI
`s1` dịch TRÁI `23` bit (nhớ `& MASK64` để giữ ĐÚNG 64-bit).

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
  ___
  s1 ^= s1 >> 17n;
  s1 ^= s0 ^ (s0 >> 26n);
  tt.s1 = s1 & MASK64;
  return ketQua;
}

const tt = gieoHat(42n);
console.log(soTiepTheo(tt));
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

const tt = gieoHat(42n);
console.log(soTiepTheo(tt));
```

```typescript title=test
const tt2 = gieoHat(42n);
const s1 = soTiepTheo(tt2);
const s2 = soTiepTheo(tt2);
if (s1 === s2) throw new Error("hai lan goi lien tiep CUNG mot tt phai cho hai so KHAC nhau (trang thai da doi)");
if (s1 !== 10481999410520547035n) throw new Error("soTiepTheo dau tien tu seed=42 phai la 10481999410520547035n");
if (s2 !== 2517254593905905195n) throw new Error("so thu hai (cung tt, da doi trang thai) phai la 2517254593905905195n");

const tt3 = gieoHat(42n);
const s1Lai = soTiepTheo(tt3);
if (s1Lai !== 10481999410520547035n) throw new Error("gieoHat(42n) MOI, goi soTiepTheo lan dau, phai cho DUNG lai so dau tien 10481999410520547035n");

if (s1 < 0n || s1 > MASK64) throw new Error("ket qua phai nam trong pham vi 64-bit khong dau [0, 2^64-1]");
```

:::hints
- kind: attention
  body: "Xor s1 voi (s1 dich trai 23 bit), nho & MASK64 de giu dung 64-bit -- mot dong."
- kind: strategy
  body: "s1 ^= (s1 << 23n) & MASK64;"
- kind: one-line
  body: "s1 ^= (s1 << 23n) & MASK64;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "10481999410520547035"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng trạng thái ĐẦU vào (`gieoHat(42n)`) LUÔN cho ra ĐÚNG dãy số đó —
nhưng con SỐ `42n` (seed) mới thật SỰ là chìa khoá. Đổi seed thì
SAO?
::::

::::reflect{#nghi-lai}
`soTiepTheo` chứng minh MỘT ý tưởng đơn giản NHƯNG mạnh: "ngẫu nhiên"
VÀ "tất định" không hề LOẠI trừ nhau — chỉ CẦN trạng thái LÀ một giá
trị TƯỜNG minh (`tt`, một `object` BÌNH thường) thay VÌ giấu Ở đâu
đó BÊN trong hệ điều hành. Bước tiếp THEO: xác nhận điều NÀY trên
HAI máy sinh số ĐỘC lập, không CHỈ một `tt` duy nhất.
::::

::::checkpoint{mastery=0.85}
::::
