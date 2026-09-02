---
id: ky-nghe-phan-mem.kiem-thu.kiem-tinh-chat-cho-dao-nguoc
title: "Capstone: Kiểm BA tính chất của hàm đảo ngược mảng"
summary: "Bài chốt cụm 4, code có chấm điểm sống: lắp ráp BoSinh/mang/kiemTraTinhChat để chứng minh daoNguoc<A>(mang): readonly A[] thoả BA tính chất độc lập: đảo ngược HAI LẦN = mảng gốc (đi-về), độ dài giữ nguyên, mảng một phần tử không đổi — mỗi tính chất chạy trên hàng chục mảng ngẫu nhiên, không phải một ví dụ tay."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 20
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [kt.gate-boss-pbt-foundations]
requires: [kt.generic-property-runner]
concepts: [kt.gate-boss-pbt-foundations]
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
Bài chốt cụm 4. `daoNguoc<A>(mang): readonly A[]` — đảo ngược một
mảng. Chứng minh nó ĐÚNG bằng property test — kiểm những tính chất gì?
::::

::::explain{#ba-tinh-chat-doc-lap}
BA tính chất **ĐỘC LẬP**, MỖI tính chất bắt MỘT loại bug KHÁC nhau
NẾU `daoNguoc` viết SAI:

1. **Đi-về**: đảo ngược HAI LẦN = mảng GỐC — `daoNguoc(daoNguoc(m))`
   PHẢI bằng `m` (nối bài học sắp tới, mẫu Round-trip).
2. **Độ dài giữ nguyên**: `daoNguoc(m).length === m.length` — đảo
   ngược KHÔNG được LÀM MẤT/THÊM phần tử.
3. **Mảng một phần tử KHÔNG đổi**: `daoNguoc([x])` PHẢI bằng `[x]` —
   trường hợp BIÊN đơn giản NHẤT, KHÔNG có gì để "đảo".

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
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function daoNguoc<A>(mang: readonly A[]): readonly A[] {
  return [...mang].reverse();
}

const bsMang = mang(soNguyen(-50, 50), 12);
kiemTraTinhChat(bsMang, (m) => JSON.stringify(daoNguoc(daoNguoc(m))) === JSON.stringify(m), 100);
```

```text
[PASS] tinh chat dung tren ca 100 lan
```

`[...mang].reverse()` **KHÔNG** làm đột biến `mang` GỐC (`[...mang]`
TẠO **bản sao MỚI** TRƯỚC, `.reverse()` đảo NGƯỢC bản sao ĐÓ) —
QUAN TRỌNG vì `mang` khai kiểu `readonly A[]`, TypeScript CẤM gọi
`.reverse()` TRỰC TIẾP TRÊN mảng `readonly` (`.reverse()` LÀ phương
thức ĐỘT BIẾN, KHÔNG tồn tại TRÊN kiểu `readonly A[]`).
::::

::::example{#do-dai-va-mot-phan-tu}
HAI tính chất CÒN LẠI cần bộ sinh KHÁC NHAU — độ dài GIỮ NGUYÊN dùng
`mang(...)` THÔNG THƯỜNG, mảng MỘT phần tử cần bộ sinh CHUYÊN BIỆT:

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
function mangMotPhanTu<A>(bs: BoSinh<A>): BoSinh<A[]> {
  return { generate: () => [bs.generate()] };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}
function daoNguoc<A>(mang: readonly A[]): readonly A[] {
  return [...mang].reverse();
}

const bsMang = mang(soNguyen(-50, 50), 12);
kiemTraTinhChat(bsMang, (m) => daoNguoc(m).length === m.length, 100);
kiemTraTinhChat(mangMotPhanTu(soNguyen(-50, 50)), (m) => JSON.stringify(daoNguoc(m)) === JSON.stringify(m), 100);
```

```text title=readonly
[PASS] tinh chat dung tren ca 100 lan
[PASS] tinh chat dung tren ca 100 lan
```

`mangMotPhanTu` LÀ MỘT bộ sinh **CHUYÊN BIỆT** — CHỈ sinh mảng độ dài
**ĐÚNG** `1` (`[bs.generate()]`), KHÁC `mang(bs, 1)` (sinh mảng độ
DÀI `0` HOẶC `1`, KHÔNG đảm bảo LUÔN LÀ `1`). Chọn ĐÚNG bộ sinh CHO
TỪNG tính chất LÀ MỘT phần của việc THIẾT KẾ property test.
::::

::::predict{#doan-daonguoc-khong-lam-hong-mang-goc commitOnce}
```typescript
function daoNguoc<A>(mang: readonly A[]): readonly A[] {
  return [...mang].reverse();
}

const goc = [1, 2, 3];
const daoRoi = daoNguoc(goc);
console.log(JSON.stringify(goc), JSON.stringify(daoRoi));
```

Dòng cuối in ra gì?

:::opt{correct}
`[1,2,3] [3,2,1]`
:::

:::opt
`[3,2,1] [3,2,1]` — vì `[...mang]` chỉ TẠO MỘT "tham chiếu KHÁC" tới
CÙNG dữ liệu (KHÔNG PHẢI bản sao THẬT), nên `.reverse()` (MỘT phương
thức đột biến) VẪN đảo NGƯỢC dữ liệu GỐC — `goc` bị ẢNH HƯỞNG THEO
::why
Gần đúng ở việc bạn nhớ ĐÚNG `.reverse()` LÀ MỘT phương thức **ĐỘT
BIẾN** (thay đổi mảng TẠI CHỖ, KHÔNG PHẢI tạo mảng mới) — một quan
sát ĐÚNG về BẢN CHẤT của `.reverse()` KHI gọi TRỰC TIẾP.

Chỗ lệch: `[...mang]` (cú pháp SPREAD, ĐÃ dùng xuyên suốt track NÀY)
**TẠO MỘT MẢNG HOÀN TOÀN MỚI** trong bộ NHỚ, SAO CHÉP từng phần tử
TỪ `mang` VÀO mảng MỚI ĐÓ — KHÔNG PHẢI "tham chiếu khác tới CÙNG dữ
liệu". `.reverse()` gọi SAU ĐÓ đột biến **BẢN SAO MỚI** (KHÔNG PHẢI
`mang` gốc). `goc` **GIỮ NGUYÊN** `[1,2,3]`, CHỈ `daoRoi` (bản SAO đã
đảo) LÀ `[3,2,1]`.
::
:::

:::opt
Máy báo lỗi biên dịch — `daoNguoc(goc)` không hợp lệ, vì `goc` được
khai `const goc = [1, 2, 3]` (kiểu SUY RA LÀ `number[]`, CÓ THỂ ĐỘT
BIẾN), KHÔNG PHẢI `readonly number[]` — tham số `mang: readonly A[]`
đòi HỎI đối số PHẢI khai `readonly` TƯỜNG MINH
::why
Gần đúng ở việc bạn để ý tham số `mang` khai `readonly A[]` — MỘT
quan sát ĐÚNG về CHỮ KÝ hàm.

Chỗ lệch: TypeScript CHO PHÉP truyền MỘT mảng **CÓ THỂ ĐỘT BIẾN**
(`number[]`) VÀO nơi ĐÒI `readonly A[]` — ĐÂY LÀ chiều "AN TOÀN" (một
mảng ĐỘT BIẾN ĐƯỢC VẪN thoả MÃN "tôi CÓ THỂ đọc, dù CÓ THỂ ĐỘT BIẾN"),
CHỈ chiều NGƯỢC LẠI (truyền `readonly A[]` VÀO nơi đòi `A[]` thường)
MỚI bị CẤM. `goc` (kiểu `number[]`) truyền VÀO `daoNguoc` (đòi `readonly
number[]`) HOÀN TOÀN hợp lệ. Biên dịch sạch.
::
:::
::::

::::code{#viet_daonguoc_va_tinh_chat}
Tự viết `daoNguoc` VÀ hai trong ba tính chất (đi-về, độ dài giữ
nguyên).

```typescript title=starter
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
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function daoNguoc<A>(mang: readonly A[]): readonly A[] {
  return ___;
}

const bsMang = mang(soNguyen(-50, 50), 12);
kiemTraTinhChat(bsMang, (m) => JSON.stringify(___) === JSON.stringify(m), 100);
kiemTraTinhChat(bsMang, (m) => ___.length === m.length, 100);
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
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function daoNguoc<A>(mang: readonly A[]): readonly A[] {
  return [...mang].reverse();
}

const bsMang = mang(soNguyen(-50, 50), 12);
kiemTraTinhChat(bsMang, (m) => JSON.stringify(daoNguoc(daoNguoc(m))) === JSON.stringify(m), 100);
kiemTraTinhChat(bsMang, (m) => daoNguoc(m).length === m.length, 100);
```

```typescript title=test
// deterministic: mang co dinh, do dai KHONG PHAI 1 (tranh trung hop ngau nhien)
const coDinh = [10, 20, 30, 40];
const daDao = daoNguoc(coDinh);
if (daDao.length !== 4) throw new Error(`do dai phai giu nguyen la 4, nhan ${daDao.length}`);
if (JSON.stringify(daDao) !== JSON.stringify([40, 30, 20, 10])) throw new Error("dao nguoc SAI thu tu");
if (JSON.stringify(coDinh) !== JSON.stringify([10, 20, 30, 40])) throw new Error("KHONG duoc dot bien mang goc");
console.log("[PASS] dao nguoc dung thu tu, dung do dai, khong dot bien mang goc");

function mangMotPhanTu<A>(bs: BoSinh<A>): BoSinh<A[]> {
  return { generate: () => [bs.generate()] };
}
kiemTraTinhChat(mangMotPhanTu(soNguyen(-50, 50)), (m) => JSON.stringify(daoNguoc(m)) === JSON.stringify(m), 100);
```

:::hints
- kind: attention
  body: "daoNguoc: tạo bản sao bằng spread rồi đảo ngược bản sao (KHÔNG đột biến mảng gốc). Tính chất đi-về: đảo ngược HAI LẦN. Tính chất độ dài: gọi daoNguoc(m) rồi lấy .length."
- kind: strategy
  body: '[...mang].reverse() : daoNguoc(daoNguoc(m)) : daoNguoc(m)'
- kind: one-line
  body: '___ (daoNguoc) = [...mang].reverse()\n___ (di-ve) = daoNguoc(daoNguoc(m))\n___ (do dai) = daoNguoc(m)'
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
Cụm 4 hoàn tất: bộ sinh, vòng lặp, hàm tổng quát — chứng minh ba tính
chất độc lập trên hàng trăm mảng ngẫu nhiên. Cụm tiếp theo: bốn mẫu
tính chất phổ biến — Bất biến, Idempotent, Đi-Về, Oracle.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tính chất "đi-về" vừa dùng LÀ MỘT trong BỐN "mẫu" tính chất phổ biến.
Mẫu ĐẦU TIÊN — Bất Biến (điều PHẢI đúng, VÔ ĐIỀU KIỆN, với MỌI input)
— trông như thế nào?
::::

::::checkpoint{mastery=0.8}
::::
