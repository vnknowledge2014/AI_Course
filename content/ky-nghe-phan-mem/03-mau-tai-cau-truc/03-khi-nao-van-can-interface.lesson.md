---
id: ky-nghe-phan-mem.mau-tai-cau-truc.khi-nao-van-can-interface
title: "Khi nào VẪN cần interface — ranh giới thật giữa FP và OOP"
summary: "FP không phải câu trả lời cho MỌI trường hợp: khi nhiều hành vi PHẢI chia sẻ CÙNG một state (doc/dong CÙNG một kết nối), một OBJECT (nhóm hàm, KHÔNG cần class) tự nhiên hơn một hàm đơn. Object literal + closure vẫn LÀ FP — chỉ khác Strategy ở chỗ nhóm NHIỀU hành vi thay vì MỘT."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.when-interface-still-needed]
requires: [mau.strategy-as-hof]
concepts: [mau.when-interface-still-needed]
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
Strategy (bài 2) chỉ CẦN một hàm. "Kết nối tới server": `doc()` VÀ
`dong()` PHẢI biết CÙNG một trạng thái — kết nối đó CÒN MỞ hay đã
ĐÓNG. Một hàm đơn còn đủ không?
::::

::::explain{#nhom-hanh-vi-chia-se-state}
KHÔNG PHẢI mọi thứ LÀ một Strategy đơn lẻ. Khi NHIỀU hành vi PHẢI
**CHIA SẺ** cùng một trạng thái (kết nối đã đóng CHƯA, dữ liệu ĐÃ
tải CHƯA), tách chúng thành các HÀM ĐỘC LẬP là VÔ NGHĨA — làm sao
`doc()` (một hàm RIÊNG) biết `dong()` (một hàm RIÊNG KHÁC) đã được
gọi HAY CHƯA?

Câu trả lời KHÔNG PHẢI "quay lại OOP class" — VẪN LÀ FP: một **OBJECT
LITERAL** nhóm các hàm liên quan, TẤT CẢ đóng (close over) CÙNG một
biến trạng thái qua closure. KHÔNG `class`, KHÔNG `new`, KHÔNG `this`:

```typescript title=readonly
type KetNoi = {
  doc: () => string;
  dong: () => void;
};

function moKetNoi(diaChi: string): KetNoi {
  let daDong = false;
  const duLieu = `du-lieu-tu-${diaChi}`;
  return {
    doc: () => {
      if (daDong) throw new Error("da dong, khong doc duoc");
      return duLieu;
    },
    dong: () => {
      daDong = true;
    },
  };
}

const kn = moKetNoi("server-a");
console.log(kn.doc());
kn.dong();
try {
  kn.doc();
} catch (e) {
  console.log("loi:", (e as Error).message);
}
```

```text title=readonly
du-lieu-tu-server-a
loi: da dong, khong doc duoc
```

`daDong` KHÔNG PHẢI field của MỘT class — nó LÀ một biến `let` BÊN
TRONG `moKetNoi`, được `doc`/`dong` CÙNG đóng gói (capture) qua
closure. Đây LÀ "interface" (`KetNoi`) THẬT SỰ cần: doc/dong PHẢI đi
CÙNG NHAU, chia sẻ MỘT trạng thái — tách rời chúng SẼ MẤT khả năng
biết "đã đóng CHƯA".
::::

::::example{#moi-loi-goi-doc-lap}
MỖI lời gọi "nhà máy" tạo MỘT object MỚI, với TRẠNG THÁI RIÊNG —
GIỐNG cách `giamCoDinh` (bài 2) tạo strategy RIÊNG, chỉ khác Ở CHỖ
đây LÀ một NHÓM hàm thay vì MỘT hàm:

```typescript title=readonly
type BoDem = { tang: () => number; reset: () => void };
function taoBoDem(): BoDem {
  let dem = 0;
  return {
    tang: () => { dem += 1; return dem; },
    reset: () => { dem = 0; },
  };
}

const b1 = taoBoDem();
const b2 = taoBoDem();
console.log(b1.tang());
console.log(b1.tang());
console.log(b2.tang());
```

```text title=readonly
1
2
1
```

`b1` VÀ `b2` LÀ HAI object ĐỘC LẬP — `dem` của `b1` KHÔNG đụng chạm
`dem` của `b2`. Gọi `b1.tang()` hai lần → `1, 2`; gọi `b2.tang()` lần
ĐẦU (dù SAU b1) → `1` (biến `dem` CỦA b2, KHÔNG PHẢI của b1).
::::

::::predict{#doan-hai-boi-doc-lap commitOnce}
```typescript
type BoDem = { tang: () => number };
function taoBoDem(batDauTu: number): BoDem {
  let dem = batDauTu;
  return {
    tang: () => { dem += 1; return dem; },
  };
}

const a = taoBoDem(10);
const b = taoBoDem(100);
console.log(a.tang());
console.log(b.tang());
console.log(a.tang());
```

Ba dòng in ra gì?

:::opt{correct}
`11` rồi `101` rồi `12`
:::

:::opt
`11` rồi `101` rồi `102` — vì CẢ HAI object CÙNG được tạo TỪ MỘT hàm
`taoBoDem`, nên `dem` LÀ MỘT biến DÙNG CHUNG, `a.tang()` VÀ `b.tang()`
CÙNG cộng dồn vào MỘT nơi
::why
Gần đúng ở việc bạn nhớ ĐÚNG `a` VÀ `b` CÙNG được tạo TỪ MỘT hàm
`taoBoDem` — quan sát ĐÓ đúng.

Chỗ lệch: MỖI **LỜI GỌI** `taoBoDem(...)` chạy thân hàm TỪ ĐẦU, tạo
MỘT biến `dem` **MỚI HOÀN TOÀN** (KHÔNG PHẢI dùng LẠI biến của lần gọi
TRƯỚC) — `taoBoDem(10)` VÀ `taoBoDem(100)` LÀ HAI LẦN GỌI RIÊNG, tạo
HAI closure với HAI biến `dem` **ĐỘC LẬP HOÀN TOÀN** (một khởi tạo từ
`10`, một từ `100`). `a.tang()` CHỈ cộng vào `dem` của `a`
(`11`→`12`), KHÔNG BAO GIỜ chạm tới `dem` của `b` (`101`).
::
:::

:::opt
Máy báo lỗi biên dịch — `taoBoDem` khai kiểu trả về `BoDem` nhưng
`batDauTu` là tham số BẮT BUỘC không có giá trị mặc định, TypeScript
đòi MỌI lời gọi hàm nhà máy trả về MỘT interface PHẢI dùng THAM SỐ
MẶC ĐỊNH
::why
Gần đúng ở việc bạn để ý `taoBoDem` NHẬN một tham số `batDauTu: number`
— một quan sát ĐÚNG về CHỮ KÝ hàm.

Chỗ lệch: TypeScript KHÔNG có quy tắc nào như vậy — MỘT hàm trả về
MỘT kiểu object (`BoDem`) HOÀN TOÀN được PHÉP nhận tham số BẮT BUỘC,
không cần giá trị mặc định. `taoBoDem(10)` VÀ `taoBoDem(100)` ĐỀU
cung cấp ĐỦ tham số bắt buộc — biên dịch SẠCH.
::
:::
::::

::::code{#viet_kho_hang}
Viết `taoKhoHang` — nhóm BA thao tác (`nhapKho`, `xuatKho`, `tonKho`)
CÙNG chia sẻ MỘT biến tồn kho qua closure.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type KhoHang = {
  nhapKho: (soLuong: number) => void;
  xuatKho: (soLuong: number) => boolean;
  tonKho: () => number;
};

function taoKhoHang(soLuongBanDau: number): KhoHang {
  let ton = soLuongBanDau;
  return {
    nhapKho: (soLuong) => {
      ___;
    },
    xuatKho: (soLuong) => {
      if (soLuong > ton) return false;
      ___;
      return true;
    },
    tonKho: () => ton,
  };
}

const kho = taoKhoHang(100);
kho.nhapKho(50);
assertEqual(kho.tonKho(), 150, "sau nhap 50");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type KhoHang = {
  nhapKho: (soLuong: number) => void;
  xuatKho: (soLuong: number) => boolean;
  tonKho: () => number;
};

function taoKhoHang(soLuongBanDau: number): KhoHang {
  let ton = soLuongBanDau;
  return {
    nhapKho: (soLuong) => {
      ton += soLuong;
    },
    xuatKho: (soLuong) => {
      if (soLuong > ton) return false;
      ton -= soLuong;
      return true;
    },
    tonKho: () => ton,
  };
}

const kho = taoKhoHang(100);
kho.nhapKho(50);
assertEqual(kho.tonKho(), 150, "sau nhap 50");
```

```typescript title=test
const kho2 = taoKhoHang(100);
assertEqual(kho2.tonKho(), 100, "ton ban dau");
assertEqual(kho2.xuatKho(30), true, "xuat 30 thanh cong");
assertEqual(kho2.tonKho(), 70, "sau xuat 30");
assertEqual(kho2.xuatKho(1000), false, "xuat vuot ton that bai");
assertEqual(kho2.tonKho(), 70, "ton khong doi sau xuat that bai");
assertEqual(kho2.xuatKho(70), true, "xuat dung bang ton -- bien");
assertEqual(kho2.tonKho(), 0, "ton ve 0 sau xuat het");
```

:::hints
- kind: attention
  body: "nhapKho: cộng DỒN soLuong vào ton. xuatKho (sau khi qua ải kiểm soLuong > ton): trừ soLuong khỏi ton."
- kind: strategy
  body: "ton += soLuong : ton -= soLuong — hai phép gán cộng dồn/trừ dồn NGƯỢC nhau, cùng thao tác trên MỘT biến ton chia sẻ."
- kind: one-line
  body: '___ (nhapKho) = ton += soLuong\n___ (xuatKho) = ton -= soLuong'
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
Object literal + closure = "interface" kiểu FP, khi nhiều hành vi
CHIA SẺ một state. Không cực đoan hoá — chọn đúng hình dạng cho đúng
bài toán. Cụm tiếp theo: Observer.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`taoKhoHang` trả về MỘT object nhóm ba hàm CHIA SẺ state. Cụm tiếp
theo giới thiệu Observer — "nhiều bên CÙNG lắng nghe một sự kiện" —
hình dạng NÀO (hàm đơn hay object nhóm) sẽ phù hợp hơn?
::::

::::checkpoint{mastery=0.8}
::::
