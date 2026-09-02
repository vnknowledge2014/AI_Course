---
id: ky-nghe-phan-mem.kiem-thu.mau-idempotent
title: "Mẫu Idempotent — áp dụng HAI LẦN không khác áp dụng MỘT LẦN"
summary: "Mẫu thứ hai: f(f(x)) === f(x) — áp dụng hàm HAI LẦN KHÔNG làm gì THÊM so với áp dụng MỘT LẦN. Kinh điển cho hàm chuẩn hoá (chuanHoa: trim + lowercase + gộp khoảng trắng) — chuẩn hoá dữ liệu ĐÃ chuẩn hoá phải ra CHÍNH NÓ, KHÔNG BAO GIỜ đổi thêm."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 22
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.pattern-idempotent]
requires: [kt.pattern-invariant]
concepts: [kt.pattern-idempotent]
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
`chuanHoa` (trim + lowercase + gộp khoảng trắng) — chuẩn hoá dữ liệu
ĐÃ chuẩn hoá phải ra CHÍNH NÓ. Mẫu tính chất này trông thế nào?
::::

::::explain{#idempotent-hai-lan-bang-mot-lan}
Mẫu tính chất **THỨ HAI**: **Idempotent** — `f(f(x)) === f(x)` — ÁP
DỤNG hàm **HAI LẦN** KHÔNG làm GÌ THÊM so VỚI áp dụng **MỘT LẦN**.
Kinh điển CHO hàm CHUẨN HOÁ:

```typescript
function chuanHoa(s: string): string {
  return s.trim().toLowerCase().replace(/\s+/g, " ");
}

const goc = "  An   Binh  ";
const lanMot = chuanHoa(goc);
const lanHai = chuanHoa(lanMot);
console.log(JSON.stringify(lanMot));
console.log(JSON.stringify(lanHai));
console.log(lanMot === lanHai);
```

```text
"an binh"
"an binh"
true
```

`chuanHoa` XOÁ khoảng trắng ĐẦU/CUỐI (`.trim()`), CHUYỂN chữ thường
(`.toLowerCase()`), GỘP nhiều khoảng trắng LIÊN TIẾP thành MỘT
(`.replace(/\s+/g, " ")`). Áp dụng `chuanHoa` LẦN THỨ HAI TRÊN chuỗi
ĐÃ chuẩn hoá **KHÔNG THAY ĐỔI GÌ THÊM** — chuỗi ĐÃ "sạch" RỒI, KHÔNG
CÒN khoảng trắng thừa để XOÁ/GỘP, KHÔNG CÒN chữ HOA để chuyển.
::::

::::example{#kiem-tren-nhieu-chuoi-ngau-nhien}
Tính chất Idempotent KIỂM được TRÊN **NHIỀU** chuỗi NGẪU NHIÊN — bộ
sinh CHUỖI TỰ CHẾ (trộn chữ thường/HOA/khoảng trắng):

```typescript title=readonly
type BoSinh<T> = { generate: () => T };
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}
function chuanHoa(s: string): string {
  return s.trim().toLowerCase().replace(/\s+/g, " ");
}

function chuoiNgauNhien(doDaiToiDa: number): BoSinh<string> {
  const kyTu = "abcABC \t\n";
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

kiemTraTinhChat(chuoiNgauNhien(15), (s) => chuanHoa(chuanHoa(s)) === chuanHoa(s), 200);
```

```text title=readonly
[PASS] tinh chat dung tren ca 200 lan
```

`chuoiNgauNhien` sinh chuỗi CHỨA MỚ chữ thường/HOA/khoảng trắng/tab/
xuống dòng **LỘN XỘN** — GẦN như "TỆ NHẤT có thể". Tính chất `chuanHoa
(chuanHoa(s)) === chuanHoa(s)` VẪN đúng TRÊN `200` chuỗi NGẪU NHIÊN —
KHÔNG CẦN nghĩ RA từng CHUỖI cụ thể để test tay.
::::

::::predict{#doan-chuanhoa-tab-va-xuong-dong commitOnce}
```typescript
function chuanHoa(s: string): string {
  return s.trim().toLowerCase().replace(/\s+/g, " ");
}

console.log(JSON.stringify(chuanHoa("  Hello\t\tWorld\n  ")));
```

Dòng cuối in ra gì?

:::opt{correct}
`"hello world"`
:::

:::opt
`"hello\t\tworld"` — vì `.replace(/\s+/g, " ")` CHỈ thay THẾ khoảng
TRẮNG DẠNG **DẤU CÁCH** (space), KHÔNG khớp ký tự TAB (`\t`) hay
XUỐNG DÒNG (`\n`) — HAI ký tự ĐÓ vẫn GIỮ NGUYÊN sau `.replace(...)`
::why
Gần đúng ở việc bạn nhớ `\s` trong biểu thức CHÍNH QUY THƯỜNG được
GIẢI THÍCH LÀ "khoảng TRẮNG" — một khái niệm ĐÚNG NHƯNG bạn thu HẸP
phạm vi CỦA nó VỀ CHỈ dấu CÁCH.

Chỗ lệch: `\s` trong JavaScript/TypeScript regex khớp **MỌI** ký tự
khoảng TRẮNG — dấu CÁCH, TAB (`\t`), XUỐNG dòng (`\n`), CARRIAGE
RETURN (`\r`), VÀ vài ký tự Unicode khoảng trắng KHÁC — KHÔNG CHỈ
dấu cách. `\s+` (một HOẶC nhiều) khớp CẢ cụm `"\t\t"` (hai tab liên
tiếp) THÀNH MỘT LẦN thay thế, ĐỔI thành MỘT dấu cách DUY NHẤT. Sau
`.trim()` (xoá khoảng trắng ĐẦU/CUỐI, GỒM CẢ `\n` VÀ khoảng cách sau
NÓ) VÀ `.toLowerCase()`, kết quả CUỐI LÀ `"hello world"` — MỘT dấu
cách DUY NHẤT GIỮA hai từ.
::
:::

:::opt
Máy báo lỗi biên dịch — chuỗi `"  Hello\t\tWorld\n  "` chứa KÝ TỰ
ĐẶC BIỆT (`\t`, `\n`) TRỘN LẪN VỚI ký tự THƯỜNG trong CÙNG MỘT
literal chuỗi, TypeScript đòi TÁCH RIÊNG bằng phép NỐI CHUỖI (`+`)
::why
Gần đúng ở việc bạn để ý chuỗi NÀY CHỨA CẢ ký tự THƯỜNG LẪN ký tự
ĐẶC BIỆT (escape sequence) — một quan sát ĐÚNG về NỘI DUNG chuỗi.

Chỗ lệch: MỘT chuỗi literal (bọc trong `"..."`) HOÀN TOÀN CHO PHÉP
TRỘN ký tự THƯỜNG VÀ escape sequence (`\t`, `\n`, `\\`, ...) NGAY
TRONG CÙNG MỘT cặp dấu ngoặc kép — KHÔNG CẦN tách RIÊNG BẰNG `+`.
Đây LÀ CÚ PHÁP chuỗi TIÊU CHUẨN, dùng RỘNG RÃI TRONG mọi ngôn ngữ
LẤY cảm hứng TỪ C. Biên dịch sạch.
::
:::
::::

::::code{#viet_chuanhoa}
Tự viết `chuanHoa`.

```typescript title=starter
type BoSinh<T> = { generate: () => T };
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function chuanHoa(s: string): string {
  return ___;
}

function chuoiNgauNhien(doDaiToiDa: number): BoSinh<string> {
  const kyTu = "abcABC \t\n";
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

kiemTraTinhChat(chuoiNgauNhien(15), (s) => chuanHoa(chuanHoa(s)) === chuanHoa(s), 200);
```

```typescript title=solution
type BoSinh<T> = { generate: () => T };
function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

function chuanHoa(s: string): string {
  return s.trim().toLowerCase().replace(/\s+/g, " ");
}

function chuoiNgauNhien(doDaiToiDa: number): BoSinh<string> {
  const kyTu = "abcABC \t\n";
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

kiemTraTinhChat(chuoiNgauNhien(15), (s) => chuanHoa(chuanHoa(s)) === chuanHoa(s), 200);
```

```typescript title=test
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

assertEqual(chuanHoa("  An   Binh  "), "an binh", "trim, lowercase, gop khoang trang");
assertEqual(chuanHoa("DA CHUAN HOA"), "da chuan hoa", "chu hoa phai chuyen thanh chu thuong");
assertEqual(chuanHoa("khong\t\tco\n\ngi"), "khong co gi", "tab va xuong dong deu duoc gop thanh mot dau cach");
assertEqual(chuanHoa(""), "", "chuoi rong van la chuoi rong");
assertEqual(chuanHoa("an binh"), "an binh", "chuoi DA chuan hoa san phai giu nguyen");
```

:::hints
- kind: attention
  body: "Ba bước theo đúng thứ tự: xoá khoảng trắng đầu/cuối, chuyển chữ thường, gộp mọi khoảng trắng liên tiếp (kể cả tab/xuống dòng) thành MỘT dấu cách."
- kind: strategy
  body: 's.trim().toLowerCase().replace(/\\s+/g, " ")'
- kind: one-line
  body: '___ = s.trim().toLowerCase().replace(/\\s+/g, " ")'
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
Idempotent: áp dụng hai lần không khác một lần. Mẫu tiếp theo:
Đi-Về — giải mã(mã hoá(x)) = x.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bài 20 đã dùng "đi-về" cho `daoNguoc`. Áp dụng mẫu ĐÓ cho MỘT cipher
Caesar-shift TỰ VIẾT (mã hoá rồi giải mã) — trông thế nào?
::::

::::checkpoint{mastery=0.8}
::::
