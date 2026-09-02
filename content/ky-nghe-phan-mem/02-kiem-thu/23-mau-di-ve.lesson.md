---
id: ky-nghe-phan-mem.kiem-thu.mau-di-ve
title: "Mẫu Đi-Về (Round-trip) — giải mã(mã hoá(x)) = x"
summary: "Mẫu thứ ba: giaiMa(maHoa(x)) === x — mã hoá rồi giải mã phải trả về ĐÚNG dữ liệu gốc. Minh hoạ bằng cipher Caesar-shift tự viết (thay Buffer/base64, module Node không có trong sandbox) — maHoa(chuoi, k)/giaiMa(chuoi, k) qua charCodeAt/fromCharCode."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 23
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [kt.pattern-round-trip]
requires: [kt.pattern-idempotent]
concepts: [kt.pattern-round-trip]
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
Bài 20 đã dùng "đi-về" cho `daoNguoc`. Áp dụng mẫu ĐÓ cho MỘT cipher
Caesar-shift TỰ VIẾT (mã hoá rồi giải mã) — trông thế nào?
::::

::::explain{#di-ve-ma-hoa-giai-ma}
Mẫu tính chất **THỨ BA**: **Đi-Về** (Round-trip) — `giaiMa(maHoa(x))
=== x` — MÃ HOÁ rồi GIẢI MÃ PHẢI trả VỀ **ĐÚNG** dữ liệu GỐC. TỰ VIẾT
MỘT cipher Caesar-shift qua `charCodeAt`/`fromCharCode` (THAY
`Buffer`/base64 — module Node.js, KHÔNG CÓ trong sandbox NÀY):

```typescript
function maHoa(chuoi: string, k: number): string {
  let ket = "";
  for (let i = 0; i < chuoi.length; i++) {
    const ma = chuoi.charCodeAt(i);
    ket += String.fromCharCode(ma + k);
  }
  return ket;
}

function giaiMa(chuoi: string, k: number): string {
  return maHoa(chuoi, -k);
}

console.log(JSON.stringify(maHoa("abc", 3)));
console.log(JSON.stringify(giaiMa(maHoa("abc", 3), 3)));
```

```text
"def"
"abc"
```

`maHoa` DỊCH CHUYỂN từng ký tự `k` VỊ TRÍ trong bảng MÃ (`charCodeAt`
LẤY mã SỐ của ký tự, `String.fromCharCode` CHUYỂN NGƯỢC LẠI thành ký
tự). `giaiMa` **KHÔNG PHẢI** MỘT thuật TOÁN riêng — nó CHỈ gọi LẠI
`maHoa` VỚI ĐỘ DỊCH **NGƯỢC DẤU** (`-k`) — dịch tới RỒI dịch LUI ĐÚNG
BẤY NHIÊU BƯỚC LÀ VỀ lại chỗ CŨ.
::::

::::example{#kiem-tren-nhieu-cap-chuoi-va-do-dich}
Tính chất Đi-Về KIỂM được TRÊN **CẢ CẶP** (chuỗi, độ dịch) SINH NGẪU
NHIÊN — KHÔNG CHỈ MỘT chuỗi/MỘT độ dịch cố định:

```typescript title=readonly
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}
function maHoa(chuoi: string, k: number): string {
  let ket = "";
  for (let i = 0; i < chuoi.length; i++) {
    const ma = chuoi.charCodeAt(i);
    ket += String.fromCharCode(ma + k);
  }
  return ket;
}
function giaiMa(chuoi: string, k: number): string {
  return maHoa(chuoi, -k);
}
function chuoiNgauNhien(doDaiToiDa: number): BoSinh<string> {
  const kyTu = "abcdefgABCDEFG012 !?";
  return {
    generate: () => {
      const doDai = Math.floor(Math.random() * (doDaiToiDa + 1));
      let ket = "";
      for (let i = 0; i < doDai; i++) {
        const idx = Math.floor(Math.random() * kyTu.length);
        const c = kyTu[idx];
        if (c !== undefined) ket += c;
      }
      return ket;
    },
  };
}

type CapMaHoa = { chuoi: string; k: number };
function capMaHoa(doDaiToiDa: number): BoSinh<CapMaHoa> {
  return {
    generate: () => ({
      chuoi: chuoiNgauNhien(doDaiToiDa).generate(),
      k: soNguyen(-20, 20).generate(),
    }),
  };
}

kiemTraTinhChat(capMaHoa(20), (c) => giaiMa(maHoa(c.chuoi, c.k), c.k) === c.chuoi, 150);
```

```text title=readonly
[PASS] tinh chat dung tren ca 150 lan
```

`capMaHoa` sinh MỘT **CẶP** — CHUỖI ngẫu nhiên VÀ độ dịch `k` ngẫu
nhiên TRONG `[-20, 20]` (KỂ CẢ `k` ÂM) — MỖI lần thử LÀ MỘT cặp
KHÁC NHAU. Tính chất "đi-về" ĐÚNG TRÊN `150` cặp NGẪU NHIÊN KHÁC
NHAU, KHÔNG PHỤ THUỘC MỘT chuỗi/MỘT độ dịch CỤ THỂ nào.
::::

::::predict{#doan-mahoa-khong-boc-vong commitOnce}
```typescript
function maHoa(chuoi: string, k: number): string {
  let ket = "";
  for (let i = 0; i < chuoi.length; i++) {
    const ma = chuoi.charCodeAt(i);
    ket += String.fromCharCode(ma + k);
  }
  return ket;
}

console.log(JSON.stringify(maHoa("xyz", 3)));
```

Dòng cuối in ra gì?

:::opt{correct}
`"{|}"`
:::

:::opt
`"abc"` — vì cipher Caesar THẬT (TỪ giáo trình mật mã HỌC cổ điển)
"BỌC VÒNG" TRONG bảng chữ CÁI 26 ký tự — dịch `x` (gần CUỐI bảng
chữ cái) TỚI `3` bước sẽ QUAY VÒNG LẠI TỪ ĐẦU (`x→y→z→a`), giống HỆT
cách đồng hồ 12 GIỜ quay VÒNG
::why
Gần đúng ở việc bạn nhớ ĐÚNG "cipher CAESAR bọc VÒNG trong 26 chữ
cái" — MỘT kiến thức ĐÚNG về cipher Caesar **KINH ĐIỂN** (dạy trong
mật mã HỌC cổ điển).

Chỗ lệch: cài đặt `maHoa` Ở BÀI NÀY **KHÔNG BỌC VÒNG** — nó dịch
TRỰC TIẾP TRÊN **MÃ SỐ** UTF-16 CỦA ký tự (`charCodeAt` trả VỀ MỘT
con SỐ, KHÔNG "biết" gì về "bảng chữ cái 26 ký tự"), `String.fromCharCode
(ma + k)` CỘNG THẲNG `k` VÀO mã SỐ ĐÓ, KHÔNG CÓ phép "MOD 26" nào để
quay VÒNG. `'x'` có mã `120`, `'y'` = `121`, `'z'` = `122` — CỘNG
THÊM `3`: `123, 124, 125` — BA mã SỐ NÀY tương ứng KÝ TỰ `'{'`, `'|'`,
`'}'` (nằm NGAY SAU chữ cái thường trong bảng UTF-16), KHÔNG PHẢI
`'a'`, `'b'`, `'c'`. Việc "KHÔNG bọc vòng" LÀ MỘT lựa chọn THIẾT KẾ
ĐƠN GIẢN HOÁ — tính chất Đi-Về VẪN ĐÚNG (`giaiMa` dịch NGƯỢC LẠI
ĐÚNG BẤY NHIÊU bước), CHỈ KHÁC cipher CỔ ĐIỂN Ở HÀNH VI "bọc vòng".
::
:::

:::opt
Máy báo lỗi biên dịch — `String.fromCharCode(ma + k)` không hợp lệ
khi `ma + k` VƯỢT QUÁ `122` (mã CỦA `'z'`), vì `String.fromCharCode`
CHỈ chấp nhận mã SỐ trong khoảng ký tự CHỮ VÀ SỐ tiêu chuẩn (`0-9`,
`a-z`, `A-Z`)
::why
Gần đúng ở việc bạn nghĩ tới việc CÓ THỂ TỒN TẠI MỘT GIỚI HẠN "hợp
lý" cho mã ký tự — MỘT trực giác dễ hiểu khi NGHĨ về "ký tự thông
thường".

Chỗ lệch: `String.fromCharCode` CHẤP NHẬN **BẤT KỲ** số nguyên nào
biểu diễn được MỘT đơn VỊ mã UTF-16 (từ `0` tới `65535`) — KHÔNG có
RÀNG BUỘC "CHỈ chữ và số". Mã `123, 124, 125` HOÀN TOÀN hợp lệ, tương
ứng CÁC ký tự IN ĐƯỢC (`{`, `|`, `}`) trong bảng ASCII MỞ RỘNG. Biên
dịch VÀ chạy sạch.
::
:::
::::

::::code{#viet_mahoa_giaima}
Tự viết `maHoa` VÀ `giaiMa`.

```typescript title=starter
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function maHoa(chuoi: string, k: number): string {
  let ket = "";
  for (let i = 0; i < chuoi.length; i++) {
    const ma = chuoi.charCodeAt(i);
    ket += String.fromCharCode(___);
  }
  return ket;
}

function giaiMa(chuoi: string, k: number): string {
  return ___;
}

const ketQuaDiVe = giaiMa(maHoa("Byte", 5), 5);
console.log(ketQuaDiVe === "Byte" ? "[PASS] di-ve dung goc" : "[FAIL] di-ve dung goc");
```

```typescript title=solution
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function maHoa(chuoi: string, k: number): string {
  let ket = "";
  for (let i = 0; i < chuoi.length; i++) {
    const ma = chuoi.charCodeAt(i);
    ket += String.fromCharCode(ma + k);
  }
  return ket;
}

function giaiMa(chuoi: string, k: number): string {
  return maHoa(chuoi, -k);
}

const ketQuaDiVe = giaiMa(maHoa("Byte", 5), 5);
console.log(ketQuaDiVe === "Byte" ? "[PASS] di-ve dung goc" : "[FAIL] di-ve dung goc");
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(maHoa("a", 1), "b", "dich mot ky tu dung mot buoc");
assertEqual(giaiMa("b", 1), "a", "giai ma phai dich NGUOC dung mot buoc");
assertEqual(maHoa("a", -1), "`", "dich am cung phai hoat dong dung");

function chuoiNgauNhien(doDaiToiDa: number): BoSinh<string> {
  const kyTu = "abcdefgABCDEFG012 !?";
  return {
    generate: () => {
      const doDai = Math.floor(Math.random() * (doDaiToiDa + 1));
      let ket = "";
      for (let i = 0; i < doDai; i++) {
        const idx = Math.floor(Math.random() * kyTu.length);
        const c = kyTu[idx];
        if (c !== undefined) ket += c;
      }
      return ket;
    },
  };
}
type CapMaHoa = { chuoi: string; k: number };
function capMaHoa(doDaiToiDa: number): BoSinh<CapMaHoa> {
  return {
    generate: () => ({ chuoi: chuoiNgauNhien(doDaiToiDa).generate(), k: soNguyen(-20, 20).generate() }),
  };
}
kiemTraTinhChat(capMaHoa(20), (c) => giaiMa(maHoa(c.chuoi, c.k), c.k) === c.chuoi, 150);
```

:::hints
- kind: attention
  body: "maHoa: cộng độ dịch k vào mã ký tự (ma). giaiMa: gọi lại maHoa với độ dịch NGƯỢC DẤU (-k), không viết thuật toán riêng."
- kind: strategy
  body: 'ma + k : maHoa(chuoi, -k)'
- kind: one-line
  body: '___ (maHoa) = ma + k\n___ (giaiMa) = maHoa(chuoi, -k)'
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
Đi-Về: mã hoá rồi giải mã trả về đúng dữ liệu gốc. Mẫu tiếp theo:
Oracle — so sánh với cài đặt đã tin cậy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Tự viết `canBacHaiCuaToi` (Newton's method) — so sánh nó VỚI `Math.sqrt`
CÓ SẴN (một "oracle" đáng tin) — mẫu tính chất NÀY trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
