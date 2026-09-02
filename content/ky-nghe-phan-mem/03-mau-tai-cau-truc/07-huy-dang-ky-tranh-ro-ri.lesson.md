---
id: ky-nghe-phan-mem.mau-tai-cau-truc.huy-dang-ky-tranh-ro-ri
title: "Hủy đăng ký — tránh rò rỉ bộ nhớ khi Observer không còn cần"
summary: "onEvent(fn) giờ TRẢ VỀ một hàm huỷ đăng ký (đóng gói việc XOÁ fn khỏi nguoiNghe qua filter). Không có cách huỷ đăng ký là rò rỉ bộ nhớ tiềm ẩn — observer sống mãi trong mảng dù phần code tạo ra nó đã không còn cần. Caveat thành thật: filter theo === xoá MỌI lần khớp, không chỉ một."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 7
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.unsubscribe-cleanup]
requires: [mau.event-bus-implementation]
concepts: [mau.unsubscribe-cleanup]
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
Bài 6's `onEvent` không TRẢ VỀ gì — một khi đăng ký, KHÔNG cách nào gỡ
RA. Một component "biến mất" (route đổi trang) nhưng observer VẪN
"sống" trong mảng — vấn đề gì xảy ra?
::::

::::explain{#huy-dang-ky-tra-ve-tu-onevent}
MỘT `EventBus` không có cách HUỶ đăng ký LÀ **rò rỉ bộ nhớ** tiềm ẩn:
observer ĐÃ đăng ký giữ THAM CHIẾU tới closure của nó (VÀ mọi biến nó
đóng gói) — KHÔNG BAO GIỜ được giải phóng nếu KHÔNG huỷ, dù phần code
"chủ" của nó đã KHÔNG còn cần tới nữa.

Fix: `onEvent(fn)` **TRẢ VỀ** một hàm `huyDangKy: () => void` — đóng
gói việc `filter` bỏ ĐÚNG `fn` ra khỏi `nguoiNghe`:

```typescript title=readonly
type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => () => void;
  phatSuKien: (giaTri: T) => void;
};

function taoEventBus<T>(): EventBus<T> {
  let nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      nguoiNghe.push(fn);
      return () => {
        nguoiNghe = nguoiNghe.filter((f) => f !== fn);
      };
    },
    phatSuKien: (giaTri) => {
      for (const f of nguoiNghe) f(giaTri);
    },
  };
}

const bus = taoEventBus<number>();
const nhat: number[] = [];
const huy = bus.onEvent((n) => nhat.push(n));
bus.onEvent((n) => nhat.push(n * 100));

bus.phatSuKien(1);
huy();
bus.phatSuKien(2);

console.log(nhat);
```

```text title=readonly
[1,100,200]
```

`huy()` CHỈ gỡ observer ĐẦU (nhân với `1`) — observer thứ hai (nhân
`100`) VẪN CÒN NGUYÊN, tiếp tục nhận sự kiện `phatSuKien(2)` (đẩy
`200`). `nguoiNghe` giờ khai `let` (KHÔNG PHẢI `const`) vì `filter`
TRẢ VỀ một MẢNG MỚI (bất biến, đúng phong cách track) — cần GÁN LẠI
biến, KHÔNG mutate mảng cũ.
::::

::::example{#filter-so-sanh-tham-chieu}
`filter((f) => f !== fn)` so sánh THEO **THAM CHIẾU** (`!==`) — CHỈ
gỡ đúng cái GIÁ TRỊ HÀM đã được TRUYỀN vào lúc đăng ký, KHÔNG PHẢI
"một hàm trông giống vậy":

```typescript title=readonly
type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => () => void;
  phatSuKien: (giaTri: T) => void;
};
function taoEventBus<T>(): EventBus<T> {
  let nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      nguoiNghe.push(fn);
      return () => { nguoiNghe = nguoiNghe.filter((f) => f !== fn); };
    },
    phatSuKien: (giaTri) => { for (const f of nguoiNghe) f(giaTri); },
  };
}

const bus = taoEventBus<number>();
const ghi: number[] = [];
const inRa = (n: number) => ghi.push(n);

bus.onEvent(inRa);
const huy2 = bus.onEvent(inRa);
bus.phatSuKien(1);
huy2();
bus.phatSuKien(2);
console.log(ghi);
```

```text title=readonly
[1,1]
```

`inRa` được đăng ký **HAI LẦN** (CÙNG một giá trị hàm) — `phatSuKien(1)`
gọi NÓ hai lần, đẩy `[1, 1]`. `huy2()` filter bỏ MỌI phần tử `f !==
inRa` SAI — nghĩa LÀ GIỮ LẠI mọi phần tử KHÁC `inRa`, XOÁ **CẢ HAI**
lần đăng ký CÙNG LÚC (không CHỈ một). `phatSuKien(2)` sau đó KHÔNG
còn observer NÀO — `ghi` DỪNG Ở `[1, 1]`. Đây LÀ một GIỚI HẠN thành
thật của cách huỷ đăng ký ĐƠN GIẢN NÀY: đăng ký CÙNG một hàm NHIỀU
lần rồi huỷ MỘT lần huỷ TẤT CẢ, không chỉ MỘT.
::::

::::predict{#doan-huy-hai-lan-dang-ky commitOnce}
```typescript
type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => () => void;
  phatSuKien: (giaTri: T) => void;
};
function taoEventBus<T>(): EventBus<T> {
  let nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      nguoiNghe.push(fn);
      return () => { nguoiNghe = nguoiNghe.filter((f) => f !== fn); };
    },
    phatSuKien: (giaTri) => { for (const f of nguoiNghe) f(giaTri); },
  };
}

const bus = taoEventBus<number>();
const ghi: number[] = [];
const a = (n: number) => ghi.push(n);
const b = (n: number) => ghi.push(n * 10);

const huyA = bus.onEvent(a);
bus.onEvent(b);
bus.phatSuKien(1);
huyA();
bus.phatSuKien(2);
console.log(ghi);
```

Dòng cuối in ra gì?

:::opt{correct}
`[1,10,20]`
:::

:::opt
`[1,10]` — vì `huyA()` xoá QUAN SÁT VIÊN Ở VỊ TRÍ ĐẦU trong mảng, VÀ
sau khi xoá vị trí ĐẦU, `phatSuKien(2)` KHÔNG còn quan sát viên nào Ở
VỊ TRÍ ĐÓ để gọi NỮA
::why
Gần đúng ở việc bạn nhớ ĐÚNG `huyA` LIÊN QUAN tới quan sát viên ĐẦU
TIÊN (`a`) — quan sát ĐÓ đúng HƯỚNG.

Chỗ lệch: `filter` KHÔNG "xoá theo VỊ TRÍ" — nó XÂY một mảng MỚI, GIỮ
LẠI đúng NHỮNG phần tử THOẢ điều kiện (`f !== a`). `b` (quan sát viên
THỨ HAI, KHÁC `a` VỀ THAM CHIẾU) THOẢ điều kiện `f !== a` (đúng LÀ
`b !== a`), nên `b` **VẪN CÒN** trong mảng SAU `huyA()` — hoàn toàn
KHÔNG liên quan tới "vị trí". `phatSuKien(2)` sau đó VẪN gọi `b`,
đẩy `20` (`2 * 10`).
::
:::

:::opt
Máy báo lỗi biên dịch — `huyA` được khai bằng `const huyA =
bus.onEvent(a)`, nhưng `onEvent` trả về kiểu `() => void` (một HÀM),
TypeScript KHÔNG cho gán kiểu HÀM cho một biến `const` mà KHÔNG khai
kiểu tường minh
::why
Gần đúng ở việc bạn để ý `onEvent` trả về MỘT giá trị có kiểu HÀM
(`() => void`) — một quan sát ĐÚNG về CHỮ KÝ trả về.

Chỗ lệch: TypeScript **SUY LUẬN** kiểu của `huyA` TỪ kiểu trả về của
`onEvent` (ở đây LÀ `() => void`) — hoàn toàn KHÔNG cần khai kiểu
tường minh cho `const`. Gán MỘT giá trị hàm cho `const` (không khai
kiểu) LÀ cú pháp CHUẨN, PHỔ BIẾN NHẤT trong TypeScript — biên dịch
SẠCH.
::
:::
::::

::::code{#viet_huy_dang_ky}
Hoàn thiện `onEvent` — đăng ký `fn`, TRẢ VỀ một hàm huỷ đăng ký ĐÚNG
`fn` đó (dùng `filter`).

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => () => void;
  phatSuKien: (giaTri: T) => void;
};

function taoEventBus<T>(): EventBus<T> {
  let nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      nguoiNghe.push(fn);
      return () => {
        nguoiNghe = ___;
      };
    },
    phatSuKien: (giaTri) => {
      for (const f of nguoiNghe) f(giaTri);
    },
  };
}

const bus1 = taoEventBus<number>();
const a: number[] = [];
const b: number[] = [];
const huyA = bus1.onEvent((n) => a.push(n));
bus1.onEvent((n) => b.push(n));

bus1.phatSuKien(7);
huyA();
bus1.phatSuKien(8);
assertEqual(a.length, 1, "a khong nhan them sau khi huy");
assertEqual(b.length, 2, "b van nhan tiep sau khi huy a");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => () => void;
  phatSuKien: (giaTri: T) => void;
};

function taoEventBus<T>(): EventBus<T> {
  let nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      nguoiNghe.push(fn);
      return () => {
        nguoiNghe = nguoiNghe.filter((f) => f !== fn);
      };
    },
    phatSuKien: (giaTri) => {
      for (const f of nguoiNghe) f(giaTri);
    },
  };
}

const bus1 = taoEventBus<number>();
const a: number[] = [];
const b: number[] = [];
const huyA = bus1.onEvent((n) => a.push(n));
bus1.onEvent((n) => b.push(n));

bus1.phatSuKien(7);
huyA();
bus1.phatSuKien(8);
assertEqual(a.length, 1, "a khong nhan them sau khi huy");
assertEqual(b.length, 2, "b van nhan tiep sau khi huy a");
```

```typescript title=test
const bus2 = taoEventBus<string>();
const ghi: string[] = [];
const huyOnly = bus2.onEvent((s) => ghi.push(s));
bus2.phatSuKien("truoc");
huyOnly();
huyOnly();
bus2.phatSuKien("sau");
assertEqual(ghi.length, 1, "goi huy hai lan van an toan khong them gi sau do");
assertEqual(ghi[0], "truoc", "gia tri duy nhat dung");
```

:::hints
- kind: attention
  body: "nguoiNghe = ...: gán LẠI (không mutate) bằng kết quả filter — giữ lại NHỮNG f KHÁC fn (so sánh tham chiếu, dùng !==)."
- kind: strategy
  body: "nguoiNghe.filter((f) => f !== fn) — mảng mới, loại bỏ đúng fn."
- kind: one-line
  body: '___ = nguoiNghe.filter((f) => f !== fn)'
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
onEvent trả về hàm huỷ đăng ký — filter theo tham chiếu, gán lại mảng
(bất biến). Cụm này gần xong: ghép TẤT CẢ vào một ví dụ thực.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`EventBus<T>` (bài 6-7) giờ có ĐỦ `onEvent` (đăng ký + trả huỷ) VÀ
`phatSuKien` (thông báo). Bài chốt cụm sẽ ghép chúng vào MỘT kịch bản
thực — theo dõi giá cổ phiếu, NHIỀU observer, một observer huỷ giữa
chừng. Bạn hình dung được luồng đó chưa?
::::

::::checkpoint{mastery=0.8}
::::
