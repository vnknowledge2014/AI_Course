---
id: ky-nghe-phan-mem.ddd.flow-dung-pipeline-tai-su-dung
title: "flow() — dựng pipeline TÁI SỬ DỤNG, chạy SAU"
summary: "flow(f, g, h) KHÔNG nhận giá trị ngay — trả về MỘT HÀM MỚI (x) => h(g(f(x))), gọi SAU, dùng lại NHIỀU lần. \"build now, run later\". Bảng so sánh pipe (run now) vs flow (build reusable)."
locale: vi
track: ky-nghe-phan-mem
module: ddd
order: 21
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 14
teaches: [ddd.flow]
requires: [ddd.pipe]
concepts: [ddd.flow]
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
`pipe` biến đổi MỘT giá trị NGAY. Nếu cần ÁP CÙNG chuỗi biến đổi lên
NHIỀU sản phẩm khác nhau, gọi `pipe` LẶP LẠI mỗi lần hơi PHÍ.
::::

::::explain{#flow-typescript}
`flow(f, g, h)` KHÔNG nhận giá trị NGAY — nó trả về MỘT HÀM MỚI
`(a) => h(g(f(a)))`, gọi SAU, dùng LẠI được NHIỀU lần:

```typescript
function flow<A, B>(f: (a: A) => B): (a: A) => B;
function flow<A, B, C>(f: (a: A) => B, g: (b: B) => C): (a: A) => C;
function flow<A, B, C, D>(f: (a: A) => B, g: (b: B) => C, h: (c: C) => D): (a: A) => D;
function flow(...fns: Array<(x: unknown) => unknown>): (a: unknown) => unknown {
  return (a: unknown) => fns.reduce((acc, fn) => fn(acc), a);
}

type SanPhamTho = { ten: string; gia: number };
function lamSach(sp: SanPhamTho): SanPhamTho {
  return { ten: sp.ten.trim(), gia: sp.gia };
}
function boSung(sp: SanPhamTho): SanPhamTho & { coGiamGia: boolean } {
  return { ...sp, coGiamGia: sp.gia > 100000 };
}
function dinhDangHienThi(sp: SanPhamTho & { coGiamGia: boolean }): string {
  return `${sp.ten} - ${sp.gia}đ${sp.coGiamGia ? " (giảm giá)" : ""}`;
}

const xuLySanPham = flow(lamSach, boSung, dinhDangHienThi);
console.log(xuLySanPham({ ten: "  Dien thoai  ", gia: 5000000 }));
console.log(xuLySanPham({ ten: "  But chi  ", gia: 5000 }));
```

```text
Dien thoai - 5000000đ (giảm giá)
But chi - 5000đ
```

`flow(lamSach, boSung, dinhDangHienThi)` gọi MỘT LẦN, trả về hàm
`xuLySanPham` — TỪ ĐÓ, gọi `xuLySanPham(...)` NHIỀU lần trên NHIỀU sản
phẩm khác nhau, KHÔNG cần liệt kê lại `lamSach`, `boSung`,
`dinhDangHienThi` mỗi lần. Đây LÀ "build now, run later" — ĐỐI LẬP
`pipe`'s "run now".
::::

::::example{#bang-so-sanh-pipe-flow}
```typescript title=readonly
function pipe<A, B>(a: A, f: (a: A) => B): B;
function pipe(a: unknown, ...fns: Array<(x: unknown) => unknown>): unknown {
  return fns.reduce((acc, fn) => fn(acc), a);
}
function flow<A, B>(f: (a: A) => B): (a: A) => B;
function flow(...fns: Array<(x: unknown) => unknown>): (a: unknown) => unknown {
  return (a: unknown) => fns.reduce((acc, fn) => fn(acc), a);
}

const nhanDoi = (x: number) => x * 2;

// pipe: chạy NGAY trên MỘT giá trị cụ thể
console.log(pipe(5, nhanDoi));
console.log(pipe(10, nhanDoi));

// flow: dựng MỘT LẦN, gọi lại NHIỀU lần
const nhanDoiHam = flow(nhanDoi);
console.log(nhanDoiHam(5));
console.log(nhanDoiHam(10));
```

```text title=readonly
10
20
10
20
```

CÙNG kết quả — nhưng `pipe` phải VIẾT LẠI `nhanDoi` ở MỖI lời gọi
(`pipe(5, nhanDoi)`, `pipe(10, nhanDoi)`), còn `flow` chỉ khai `nhanDoi`
MỘT LẦN (`flow(nhanDoi)`), rồi TÁI SỬ DỤNG `nhanDoiHam` bao nhiêu lần
tuỳ ý. Khi chuỗi biến đổi DÀI (nhiều hàm) và được DÙNG LẶP LẠI nhiều
nơi, `flow` tránh được việc GÕ LẠI danh sách hàm mỗi lần.
::::

::::predict{#doan-flow-tai-su-dung commitOnce}
```typescript
function flow<A, B, C>(f: (a: A) => B, g: (b: B) => C): (a: A) => C;
function flow(...fns: Array<(x: unknown) => unknown>): (a: unknown) => unknown {
  return (a: unknown) => fns.reduce((acc, fn) => fn(acc), a);
}

const congNam = (x: number) => x + 5;
const chiaHai = (x: number) => x / 2;

const xuLy = flow(congNam, chiaHai);
console.log(xuLy(3));
console.log(xuLy(11));
```

Hai dòng cuối in ra gì?

:::opt{correct}
`4` rồi `8`
:::

:::opt
`4` rồi `4` — vì `xuLy` được TẠO MỘT LẦN với `3` là đầu vào, các lời
gọi SAU đó dùng lại kết quả CŨ, không tính lại
::why
Gần đúng ở việc bạn nhớ ĐÚNG `xuLy` được TẠO MỘT LẦN (`flow(congNam,
chiaHai)` chỉ gọi MỘT LẦN) — quan sát về việc TẠO một lần đó đúng.

Chỗ lệch: `xuLy` là MỘT HÀM (không phải một giá trị đã TÍNH SẴN) — MỖI
LẦN gọi `xuLy(...)` (dù với `3` hay `11`), nó CHẠY LẠI TOÀN BỘ chuỗi
`congNam` rồi `chiaHai` TRÊN tham số MỚI truyền vào LẦN ĐÓ. `xuLy(11)`
= `(11 + 5) / 2` = `8`, HOÀN TOÀN ĐỘC LẬP với lần gọi `xuLy(3)` trước
đó.
::
:::

:::opt
Máy báo lỗi biên dịch — gọi `xuLy` HAI LẦN với hai giá trị KHÁC NHau
(`3` rồi `11`) không hợp lệ, vì `xuLy` đã "cố định" kiểu từ lần gọi
`flow` đầu tiên
::why
Gần đúng ở việc bạn để ý `flow` CHỈ được gọi MỘT LẦN để tạo `xuLy` —
quan sát về việc CÓ một bước khởi tạo riêng đó đúng.

Chỗ lệch: `xuLy` (kết quả của `flow`) là một HÀM BÌNH THƯỜNG, kiểu
`(a: number) => number` — gọi được BAO NHIÊU LẦN tuỳ ý với BẤT KỲ
`number` nào, không có ràng buộc "chỉ gọi một lần" hay "khoá giá trị
đầu tiên". Biên dịch và chạy hoàn toàn bình thường cho cả hai lời gọi.
::
:::
::::

::::code{#viet_flow}
Tự viết PHẦN THÂN (implementation) của `flow`.

```typescript title=starter
function flow<A, B>(f: (a: A) => B): (a: A) => B;
function flow<A, B, C>(f: (a: A) => B, g: (b: B) => C): (a: A) => C;
function flow(...fns: Array<(x: unknown) => unknown>): (a: unknown) => unknown {
  return ___;
}

const nhanDoi = (x: number) => x * 2;
const congMuoi = (x: number) => x + 10;
const xuLy = flow(nhanDoi, congMuoi);
console.log(xuLy(5));
```

```typescript title=solution
function flow<A, B>(f: (a: A) => B): (a: A) => B;
function flow<A, B, C>(f: (a: A) => B, g: (b: B) => C): (a: A) => C;
function flow(...fns: Array<(x: unknown) => unknown>): (a: unknown) => unknown {
  return (a: unknown) => fns.reduce((acc, fn) => fn(acc), a);
}

const nhanDoi = (x: number) => x * 2;
const congMuoi = (x: number) => x + 10;
const xuLy = flow(nhanDoi, congMuoi);
console.log(xuLy(5));
```

```typescript title=test
const xuLyTest = flow(nhanDoi, congMuoi);
if (xuLyTest(5) !== 20) throw new Error("xuLy(5) phải ra 20 (5*2=10, 10+10=20)");
if (xuLyTest(10) !== 30) throw new Error("xuLy(10) phải ra 30 (10*2=20, 20+10=30) — TÁI SỬ DỤNG được cho input khác");

const chiMotHam = flow(nhanDoi);
if (chiMotHam(7) !== 14) throw new Error("flow với đúng MỘT hàm vẫn phải hoạt động (7*2=14)");
```

:::hints
- kind: attention
  body: "flow phải TRẢ VỀ một HÀM MỚI (không phải một giá trị) — hàm đó, khi được gọi với a, mới chạy reduce trên fns."
- kind: strategy
  body: "(a: unknown) => fns.reduce((acc, fn) => fn(acc), a) — bọc reduce bên trong một arrow function nhận a, TRẢ HÀM đó ra, không gọi ngay."
- kind: one-line
  body: "return (a: unknown) => fns.reduce((acc, fn) => fn(acc), a);"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "20"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
flow(): dựng pipeline MỘT LẦN, gọi lại NHIỀU lần trên input khác nhau.
"Build now, run later" — đối lập pipe()'s "run now".
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Một số bước trong pipeline chỉ cần QUAN SÁT giá trị (ví dụ để debug),
không hề ĐỔI giá trị đó — chèn một bước như vậy vào pipeline thế nào?
::::

::::checkpoint{mastery=0.8}
::::
