---
id: ky-nghe-phan-mem.kiem-thu.ham-kiem-tra-tinh-chat-tong-quat
title: "Hàm kiemTraTinhChat<A> tổng quát — thay thế fc.assert(fc.property(...))"
summary: "Ghép BỘ SINH (BoSinh/mang) và vòng lặp (chayThuTinhChat) thành MỘT hàm generic duy nhất — CHÍNH LÀ thứ thay thế fc.assert(fc.property(arb, predicate)) của fast-check thật. kiemTraTinhChat<A>(bs, tinhChat, soLan=100) — công cụ trung tâm, dùng lại cho MỌI bài property test sau."
locale: vi
track: ky-nghe-phan-mem
module: kiem-thu
order: 19
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 13
teaches: [kt.generic-property-runner]
requires: [kt.array-generator-composition]
concepts: [kt.generic-property-runner]
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
Có vòng lặp (`chayThuTinhChat`) VÀ bộ sinh (`BoSinh`/`mang`) — ghép
CẢ HAI thành MỘT hàm DUY NHẤT, dễ gọi hơn — trông thế nào?
::::

::::explain{#kiemtratinhchat-ghep-lai}
`kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan
= 100)` — GHÉP bộ sinh (bài 17-18) VÀ vòng lặp (bài 16) thành MỘT hàm
generic DUY NHẤT, NHẬN **THẲNG** `BoSinh<A>` (KHÔNG PHẢI hàm `sinhInput`
rời rạc) VÀ CÓ **GIÁ TRỊ MẶC ĐỊNH** `soLan = 100`:

```typescript
type BoSinh<T> = { generate: () => T };
function soNguyen(min: number, max: number): BoSinh<number> {
  return { generate: () => Math.floor(Math.random() * (max - min + 1)) + min };
}

function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

// KHONG truyen soLan -- dung mac dinh 100
kiemTraTinhChat(soNguyen(-100, 100), (n) => Math.abs(n) >= 0);
```

```text
[PASS] tinh chat dung tren ca 100 lan
```

ĐÂY CHÍNH LÀ thứ **THAY THẾ** `fc.assert(fc.property(arb, predicate))`
của `fast-check` THẬT (KHÔNG dùng được trong sandbox) — MỘT LỜI GỌI
`kiemTraTinhChat(boSinh, tinhChat)` thay THẾ cho việc TỰ viết vòng
lặp + TỰ gọi `.generate()` MỖI lần cần test MỘT tính chất MỚI.
::::

::::example{#tham-so-tuong-minh-va-that-bai}
TRUYỀN `soLan` TƯỜNG MINH khi cần NHIỀU/ÍT vòng LẶP hơn mặc định —
VÀ khi tính chất SAI, thông điệp báo VẪN chi tiết NHƯ `chayThuTinhChat`:

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

// truyen soLan tuong minh: chi 20 lan (thay vi mac dinh 100)
kiemTraTinhChat(soNguyen(1, 6), (n) => n >= 1 && n <= 6, 20);

// bo sinh DAN GIAN, TAI LAP DUOC -- de vi du that bai ON DINH
let dem = 0;
const boSinhDem: BoSinh<number> = { generate: () => dem++ };
try {
  kiemTraTinhChat(boSinhDem, (n) => n < 4, 10);
} catch (e) {
  console.log((e as Error).message);
}
```

```text title=readonly
[PASS] tinh chat dung tren ca 20 lan
that bai o lan thu 4: input = 4
```

`kiemTraTinhChat` HOẠT ĐỘNG với **BẤT KỲ** `BoSinh<A>` nào — KỂ CẢ
MỘT object literal tự viết TAY (`boSinhDem`, KHÔNG dùng `soNguyen`/
`mang`) — MIỄN LÀ nó CÓ field `generate: () => A`. Interface tối
giản LÀ ĐIỂM MẠNH: BẤT KỲ thứ gì "trông giống" `BoSinh<A>` ĐỀU dùng
được.
::::

::::predict{#doan-mac-dinh-100-lan commitOnce}
```typescript
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

kiemTraTinhChat(soNguyen(1, 1), (n) => n === 1);
```

Dòng cuối in ra gì?

:::opt{correct}
`[PASS] tinh chat dung tren ca 100 lan`
:::

:::opt
`[PASS] tinh chat dung tren ca 1 lan` — vì `soNguyen(1, 1)` LUÔN
sinh ĐÚNG MỘT giá trị (`1`), NÊN `kiemTraTinhChat` "THÔNG MINH" nhận
ra CHỈ CẦN thử **MỘT LẦN** LÀ ĐỦ (thử THÊM CŨNG cho KẾT QUẢ giống
hệt), tự động RÚT NGẮN `soLan` xuống `1`
::why
Gần đúng ở việc bạn nhớ ĐÚNG `soNguyen(1, 1)` LUÔN sinh CÙNG MỘT giá
trị — một quan sát ĐÚNG về BOSINH cụ thể NÀY.

Chỗ lệch: `kiemTraTinhChat` KHÔNG "biết" (VÀ KHÔNG CÓ CÁCH biết,
KHÔNG có PHÂN TÍCH nào chạy TRƯỚC) rằng `bs` LUÔN sinh CÙNG MỘT giá
trị — nó CHỈ đơn giản LẶP ĐÚNG `soLan` lần, KHÔNG PHÂN BIỆT bộ sinh
"đa dạng" hay "đơn điệu". Lời gọi NÀY **KHÔNG** truyền THAM SỐ THỨ BA
(`soLan`), NÊN dùng giá trị **MẶC ĐỊNH** khai TRONG chữ ký hàm —
`soLan: number = 100`. Vòng lặp chạy ĐỦ `100` lần (dù MỖI lần ĐỀU
sinh `1`, VÀ MỖI lần ĐỀU `[PASS]` — KHÔNG throw sớm), in
`"tinh chat dung tren ca 100 lan"`.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `kiemTraTinhChat(soNguyen(1, 1), (n) =>
n === 1)` THIẾU đối số THỨ BA (`soLan`), TypeScript đòi TRUYỀN ĐỦ BA
đối số cho MỘT hàm khai BA tham số, KỂ CẢ khi tham số ĐÓ có giá trị
mặc định
::why
Gần đúng ở việc bạn đếm ĐÚNG `kiemTraTinhChat` khai **BA** tham số
(`bs`, `tinhChat`, `soLan`) — một quan sát ĐÚNG về CHỮ KÝ hàm.

Chỗ lệch: tham số CÓ **giá trị MẶC ĐỊNH** (`soLan: number = 100`) LÀ
**TUỲ CHỌN** — người GỌI được PHÉP **BỎ QUA** hoàn toàn, TypeScript
TỰ ĐỘNG dùng giá trị mặc định KHI đối số ĐÓ KHÔNG được truyền (giống
HỆT `emailDaTonTai: string[] = []` Ở `taoFakeDeps`, bài 12). Gọi VỚI
CHỈ hai đối số HOÀN TOÀN hợp lệ. Biên dịch sạch.
::
:::
::::

::::code{#viet_kiemtratinhchat}
Tự viết PHẦN LÕI của `kiemTraTinhChat<A>`.

```typescript title=starter
type BoSinh<T> = { generate: () => T };

function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = ___;
    if (!___(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

const boSinhCoDinh: BoSinh<number> = { generate: () => 7 };
kiemTraTinhChat(boSinhCoDinh, (n) => n === 7, 5);
```

```typescript title=solution
type BoSinh<T> = { generate: () => T };

function kiemTraTinhChat<A>(bs: BoSinh<A>, tinhChat: (a: A) => boolean, soLan: number = 100): void {
  for (let lan = 0; lan < soLan; lan++) {
    const input = bs.generate();
    if (!tinhChat(input)) {
      throw new Error(`that bai o lan thu ${lan}: input = ${JSON.stringify(input)}`);
    }
  }
  console.log(`[PASS] tinh chat dung tren ca ${soLan} lan`);
}

const boSinhCoDinh: BoSinh<number> = { generate: () => 7 };
kiemTraTinhChat(boSinhCoDinh, (n) => n === 7, 5);
```

```typescript title=test
// khong truyen soLan -- phai dung MAC DINH 100, KHONG PHAI mot con so khac
let soLanGoi = 0;
const boSinhDem: BoSinh<number> = { generate: () => { soLanGoi++; return soLanGoi; } };
kiemTraTinhChat(boSinhDem, () => true);
if (soLanGoi !== 100) throw new Error(`mac dinh soLan phai la 100, nhan ${soLanGoi} lan goi`);
console.log("[PASS] mac dinh dung 100 lan khi khong truyen soLan");

// tinh chat SAI phai throw, KHONG duoc im lang
let daNemLoi = false;
try {
  kiemTraTinhChat({ generate: () => 999 }, (n) => n < 5, 3);
} catch {
  daNemLoi = true;
}
if (!daNemLoi) throw new Error("kiemTraTinhChat PHAI nem loi khi tinh chat SAI");

// hoat dong voi BoSinh tu viet tay, khong can soNguyen/mang
let demRieng = 0;
const boSinhRieng: BoSinh<number> = { generate: () => demRieng++ };
try {
  kiemTraTinhChat(boSinhRieng, (n) => n < 3, 10);
} catch (e) {
  if (!(e as Error).message.includes("3")) throw new Error("thong diep loi phai nhac dung input gay that bai (3)");
  console.log("[PASS] hoat dong voi BoSinh tu viet tay, bao dung input that bai");
}
```

:::hints
- kind: attention
  body: "Mỗi lần lặp: gọi generate() của bs để sinh MỘT input mới, rồi kiểm tính chất TRÊN input đó — giống hệt chayThuTinhChat bài 16, chỉ khác nguồn sinh input là bs.generate thay vì sinhInput()."
- kind: strategy
  body: 'bs.generate() : tinhChat — lời gọi sinh input mới từ bộ sinh, hàm kiểm tính chất.'
- kind: one-line
  body: '___ (sinh input) = bs.generate()\n___ (kiem tinh chat) = tinhChat'
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
kiemTraTinhChat<A>: công cụ trung tâm, dùng lại cho MỌI bài property
test sau. Bài chốt cụm: chứng minh ba tính chất của hàm đảo mảng.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`daoNguoc<A>(mang): readonly A[]` — đảo ngược một mảng. Chứng minh
NÓ đúng bằng property test — cần kiểm những tính chất GÌ?
::::

::::checkpoint{mastery=0.8}
::::
