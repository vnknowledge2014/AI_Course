---
id: ky-nghe-phan-mem.kiem-thu.bo-sinh-mang-tong-quat
title: "Bộ sinh MẢNG tổng quát — kết hợp bộ sinh CÓ SẴN"
summary: "Bộ sinh KẾT HỢP được — mang<A>(bs: BoSinh<A>, doDaiToiDa): BoSinh<A[]> xây dựng TỪ BẤT KỲ BoSinh<A> nào, không cần biết A là gì (generic thật). mang(soNguyen(-50, 50)) sinh mảng số nguyên ngẫu nhiên, độ dài NGẪU NHIÊN."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 18
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.array-generator-composition]
requires: [kt.arbitrary-type]
concepts: [kt.array-generator-composition]
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
`soNguyen` sinh MỘT số. Cần sinh CẢ MỘT MẢNG (độ dài ngẫu nhiên, MỖI
phần tử sinh từ bộ sinh CÓ SẴN) — không viết lại công thức từ đầu?
::::

::::explain{#bo-sinh-ket-hop-duoc}
Bộ sinh **KẾT HỢP** được — `mang<A>(bs: BoSinh<A>, doDaiToiDa):
BoSinh<A[]>` xây dựng TỪ **BẤT KỲ** `BoSinh<A>` nào, KHÔNG CẦN biết
`A` LÀ GÌ (generic THẬT, dùng lại kỹ thuật TypeScript generics ĐÃ học
xuyên suốt track FP TRƯỚC):

```typescript
type BoSinh<T> = { generate: () => T };

function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}

function mang<A>(bs: BoSinh<A>, doDaiToiDa: number): BoSinh<A[]> {
  return {
    generate: () => {
      const doDai = Math.floor(Math.random() * (doDaiToiDa + 1));
      const ket: A[] = [];
      for (let i = 0; i < doDai; i++) ket.push(bs.generate());
      return ket;
    },
  };
}

const bsMang = mang(soNguyen(-50, 50), 10);
console.log(Array.isArray(bsMang.generate()));
```

```text
true
```

`mang` **KHÔNG** cần biết `bs` sinh SỐ hay CHUỖI hay bất kỳ gì khác —
nó CHỈ gọi `bs.generate()` LẶP LẠI, đóng gói kết quả VÀO mảng. `A`
LÀ tham số kiểu — `mang(soNguyen(...))` cho `BoSinh<number[]>`, NHƯNG
`mang` HOẠT ĐỘNG y hệt CHO BẤT KỲ `BoSinh<A>` khác Ở TƯƠNG LAI.
::::

::::example{#do-dai-va-phan-tu-deu-ngau-nhien}
MẢNG sinh ra có **ĐỘ DÀI** NGẪU NHIÊN (`0` đến `doDaiToiDa`) VÀ **MỖI
PHẦN TỬ** NGẪU NHIÊN TRONG khoảng CỦA `bs` — CẢ HAI tính chất kiểm
được BẰNG `chayThuTinhChat`:

```typescript title=readonly
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function mang<A>(bs: BoSinh<A>, doDaiToiDa: number): BoSinh<A[]> {
  return {
    generate: () => {
      const doDai = Math.floor(Math.random() * (doDaiToiDa + 1));
      const ket: A[] = [];
      for (let i = 0; i < doDai; i++) ket.push(bs.generate());
      return ket;
    },
  };
}
function chayThuTinhChat<A>(sinhInput: () => A, tinhChat: (a: A) => boolean, soLan: number): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = sinhInput();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

const bsMang = mang(soNguyen(-50, 50), 10);
chayThuTinhChat(bsMang.generate, (m) => m.length <= 10 && m.every((n) => n >= -50 && n <= 50), 100);

// kiem do dai co BIEN THIEN -- khong bi ghim cung mot gia tri
const doDaiThayDoi = new Set<number>();
for (let i = 0; i < 300; i++) doDaiThayDoi.add(bsMang.generate().length);
console.log("do dai co it nhat ba gia tri khac nhau:", doDaiThayDoi.size >= 3);
```

```text title=readonly
[PASS] tinh chat dung tren ca 100 lan
do dai co it nhat ba gia tri khac nhau: true
```

Tính chất "độ dài ≤ `10` VÀ MỌI phần tử nằm trong `[-50, 50]`" ĐÚNG
TRÊN `100` mảng NGẪU NHIÊN — VÀ độ dài thực sự BIẾN THIÊN (KHÔNG bị
"đóng băng" Ở MỘT giá trị) — bằng chứng `mang` sinh RA sự ĐA DẠNG THẬT,
KHÔNG PHẢI TOÀN mảng RỖNG hay TOÀN cùng ĐỘ DÀI.
::::

::::predict{#doan-mang-toan-mot-gia-tri commitOnce}
```typescript
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function mang<A>(bs: BoSinh<A>, doDaiToiDa: number): BoSinh<A[]> {
  return {
    generate: () => {
      const doDai = Math.floor(Math.random() * (doDaiToiDa + 1));
      const ket: A[] = [];
      for (let i = 0; i < doDai; i++) ket.push(bs.generate());
      return ket;
    },
  };
}

// bo sinh phan tu CHI CO MOT gia tri hop le: soNguyen(1, 1)
const bs = mang(soNguyen(1, 1), 5);
let luonToanSo1 = true;
for (let i = 0; i < 100; i++) {
  const m = bs.generate();
  if (!m.every((n) => n === 1)) luonToanSo1 = false;
}
console.log(luonToanSo1);
```

Dòng cuối in ra gì?

:::opt{correct}
`true`
:::

:::opt
KHÔNG chắc chắn — `soNguyen(1, 1)` VẪN gọi `Math.random()` BÊN
TRONG, VÀ `Math.random()` LUÔN có THỂ trả VỀ MỘT giá trị BẤT KỲ trong
`[0, 1)`, nên `generate()` của `bs` (bộ sinh PHẦN TỬ) VẪN CÓ THỂ,
thỉnh thoảng, sinh MỘT số KHÁC `1`
::why
Gần đúng ở việc bạn nhớ `soNguyen(1, 1)` VẪN GỌI `Math.random()` BÊN
TRONG (KHÔNG "tắt" ngẫu nhiên) — một quan sát ĐÚNG rằng cơ chế NGẪU
NHIÊN VẪN CHẠY.

Chỗ lệch: `soNguyen(min, max)` VỚI `min = max = 1` khiến công thức
`Math.floor(Math.random() * (max - min + 1)) + min` = `Math.floor
(Math.random() * 1) + 1` — `Math.random()` LUÔN nằm trong `[0, 1)`,
NHÂN VỚI `1` VẪN nằm trong `[0, 1)`, `Math.floor(...)` CỦA MỘT giá
trị `[0, 1)` LUÔN LÀ `0` (KHÔNG BAO GIỜ khác) — cộng `1` LUÔN ra
`1`. `soNguyen(1, 1).generate()` **LUÔN LUÔN** trả ĐÚNG `1`, KHÔNG
CÓ ngoại lệ — độ NGẪU NHIÊN "bị THU HẸP" hoàn toàn VỀ MỘT điểm khi
`min = max`. MỌI phần tử trong MỌI mảng sinh RA (BẤT KỂ độ dài) ĐỀU
LÀ `1`.
::
:::

:::opt
Máy báo lỗi biên dịch — `mang(soNguyen(1, 1), 5)` không hợp lệ, vì
`soNguyen(1, 1)` (đối số HAI KIỂU BẰNG NHAU) khiến TypeScript SUY
kiểu `T` CHO `BoSinh<T>` LÀ MỘT **literal type** `1` (KHÔNG PHẢI
`number` chung chung), VÀ `mang` đòi `BoSinh<A>` VỚI `A` LÀ kiểu
RỘNG
::why
Gần đúng ở việc bạn nghĩ tới KHẢ NĂNG TypeScript suy kiểu literal
CHẶT (`1` thay vì `number`) TỪ đối số CỤ THỂ — MỘT hiện tượng CÓ THẬT
trong TypeScript ("literal narrowing"), một quan sát tinh TẾ.

Chỗ lệch: `soNguyen` khai RÕ kiểu trả VỀ `BoSinh<number>` (Ở CHỮ KÝ
hàm, KHÔNG PHẢI suy RA TỪ đối số) — chữ ký NÀY **GHI ĐÈ** MỌI suy
luận literal-narrowing, `T` LUÔN LÀ `number` (kiểu RỘNG) BẤT KỂ đối
số CỤ THỂ LÀ `1` hay `100`. `mang(soNguyen(1, 1), 5)` khớp HOÀN TOÀN
`mang<A>(bs: BoSinh<A>, ...)` VỚI `A = number`. Biên dịch sạch.
::
:::
::::

::::code{#viet_mang}
Tự viết `mang<A>(bs, doDaiToiDa): BoSinh<A[]>`.

```typescript title=starter
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}

function mang<A>(bs: BoSinh<A>, doDaiToiDa: number): BoSinh<A[]> {
  return {
    generate: () => {
      const doDai = ___;
      const ket: A[] = [];
      for (let i = 0; i < doDai; i++) ket.push(___);
      return ket;
    },
  };
}

const laMang = Array.isArray(mang(soNguyen(1, 6), 5).generate());
console.log(laMang ? "[PASS] generate() phai tra ve mot MANG" : "[FAIL] generate() phai tra ve mot MANG");
```

```typescript title=solution
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}

function mang<A>(bs: BoSinh<A>, doDaiToiDa: number): BoSinh<A[]> {
  return {
    generate: () => {
      const doDai = Math.floor(Math.random() * (doDaiToiDa + 1));
      const ket: A[] = [];
      for (let i = 0; i < doDai; i++) ket.push(bs.generate());
      return ket;
    },
  };
}

const laMang = Array.isArray(mang(soNguyen(1, 6), 5).generate());
console.log(laMang ? "[PASS] generate() phai tra ve mot MANG" : "[FAIL] generate() phai tra ve mot MANG");
```

```typescript title=test
const bsKiemDoDai = mang(soNguyen(5, 10), 8);
let luonHopLe = true;
const cacDoDai = new Set<number>();
for (let i = 0; i < 500; i++) {
  const m = bsKiemDoDai.generate();
  cacDoDai.add(m.length);
  if (m.length > 8) luonHopLe = false;
  if (!m.every((n) => n >= 5 && n <= 10)) luonHopLe = false;
}
if (!luonHopLe) throw new Error("moi phan tu phai nam trong [5,10] va do dai phai <= 8");
if (cacDoDai.size < 3) throw new Error("do dai phai BIEN THIEN (it nhat ba gia tri khac nhau qua 500 lan), khong duoc ghim cung mot so");
console.log("[PASS] mang sinh dung khoang, dung do dai toi da, va do dai co bien thien");

const bsMotPhanTu = mang(soNguyen(9, 9), 6);
let luonToan9 = true;
for (let i = 0; i < 100; i++) {
  if (!bsMotPhanTu.generate().every((n) => n === 9)) luonToan9 = false;
}
if (!luonToan9) throw new Error("khi bo sinh phan tu chi co MOT gia tri hop le, moi phan tu trong mang phai la gia tri DO");
console.log("[PASS] moi phan tu dung la gia tri bo sinh phan tu sinh ra");

// do dai PHAI DAT DUOC dung gia tri toi da (khong duoc bi thu hep khoang)
const bsBienGioiHan = mang(soNguyen(1, 1), 1);
let coDoDai1 = false;
for (let i = 0; i < 100; i++) {
  if (bsBienGioiHan.generate().length === 1) coDoDai1 = true;
}
if (!coDoDai1) throw new Error("voi doDaiToiDa=1, phai co it nhat mot lan sinh MANG DO DAI DUNG 1 qua 100 lan thu");
console.log("[PASS] do dai dat duoc dung gia tri toi da");
```

:::hints
- kind: attention
  body: "Độ dài mảng: một số nguyên ngẫu nhiên từ 0 đến doDaiToiDa (dùng công thức Math.floor + Math.random giống soNguyen). Mỗi phần tử: gọi generate() của bộ sinh bs được truyền vào."
- kind: strategy
  body: 'Math.floor(Math.random() * (doDaiToiDa + 1)) : bs.generate() — công thức sinh độ dài, lời gọi sinh một phần tử.'
- kind: one-line
  body: '___ (độ dài) = Math.floor(Math.random() * (doDaiToiDa + 1))\n___ (phần tử) = bs.generate()'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "PASS"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bộ sinh kết hợp được — mang<A> xây từ BẤT KỲ BoSinh<A>. Bước tiếp
theo: ghép bộ sinh + vòng lặp thành MỘT hàm tổng quát duy nhất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đã có `chayThuTinhChat` (vòng lặp) VÀ `BoSinh<T>`/`mang` (bộ sinh) —
ghép CẢ HAI thành MỘT hàm DUY NHẤT, thay thế `fc.assert(fc.property
(arb, predicate))` của `fast-check` thật, sẽ trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
