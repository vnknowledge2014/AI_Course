---
id: ky-nghe-phan-mem.mau-tai-cau-truc.eventbus-tu-viet
title: "EventBus tự viết — đóng gói danh sách observer qua nhà máy"
summary: "\"Đăng ký quan tâm\" = đẩy một HÀM vào mảng; \"thông báo\" = gọi từng hàm trong mảng. taoEventBus<T>(): {onEvent, phatSuKien} đóng gói mảng nguoiNghe qua closure (KHÔNG lộ ra ngoài như bài 5) — generic theo T, mỗi lời gọi tạo một bus ĐỘC LẬP. EventBus thật (Node EventEmitter) dùng Map<string,Function[]> cho NHIỀU loại sự kiện; bản này đơn giản hoá còn MỘT loại."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 6
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.event-bus-implementation]
requires: [mau.observer-oop-shape]
concepts: [mau.event-bus-implementation]
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
Bài 5's `danhSachQuanSat` LỘ hoàn toàn ra ngoài. `taoKhoHang` (bài 3)
đã giải quyết đúng vấn đề NÀY cho tồn kho — áp dụng LẠI kỹ thuật ĐÓ
cho danh sách quan sát.
::::

::::explain{#nha-may-tao-eventbus}
Node.js có sẵn `EventEmitter` — bên trong dùng `Map<string,
Function[]>` để hỗ trợ NHIỀU **LOẠI** sự kiện CÙNG lúc (mỗi TÊN sự
kiện một mảng RIÊNG). Sandbox KHÔNG có module Node — TỰ VIẾT bản ĐƠN
GIẢN HOÁ: CHỈ MỘT loại sự kiện, generic theo KIỂU dữ liệu `T` (đóng
gói qua nhà máy, GIỐNG hệt `taoKhoHang` bài 3 — CHỈ khác nhóm HAI
hành vi thay vì BA):

```typescript title=readonly
type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => void;
  phatSuKien: (giaTri: T) => void;
};

function taoEventBus<T>(): EventBus<T> {
  const nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      nguoiNghe.push(fn);
    },
    phatSuKien: (giaTri) => {
      for (const fn of nguoiNghe) fn(giaTri);
    },
  };
}

const bus = taoEventBus<number>();
bus.onEvent((gia) => console.log("nghe duoc gia:", gia));
bus.phatSuKien(50);
```

```text title=readonly
nghe duoc gia: 50
```

`nguoiNghe` giờ NẰM BÊN TRONG `taoEventBus` — KHÔNG đoạn code NÀO
bên ngoài chạm được TRỰC TIẾP vào nó (không CÓ `bus.nguoiNghe` nào
cả trong kiểu `EventBus<T>`). CHỈ hai cách TƯƠNG TÁC được PHÉP:
`onEvent` (đăng ký) VÀ `phatSuKien` (phát) — ĐÚNG tinh thần
`taoKhoHang` (bài 3): đóng gói state, lộ RA đúng NHỮNG THAO TÁC cần
thiết.
::::

::::example{#hai-bus-doc-lap-generic}
`taoEventBus<T>` LÀ generic — MỖI lời gọi tạo MỘT bus MỚI, với KIỂU
VÀ danh sách `nguoiNghe` **RIÊNG HOÀN TOÀN**, giống `taoBoDem` (bài
3) tạo bộ đếm độc lập:

```typescript title=readonly
type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => void;
  phatSuKien: (giaTri: T) => void;
};
function taoEventBus<T>(): EventBus<T> {
  const nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => { nguoiNghe.push(fn); },
    phatSuKien: (giaTri) => { for (const fn of nguoiNghe) fn(giaTri); },
  };
}

const busSo = taoEventBus<number>();
const busChu = taoEventBus<string>();

busSo.onEvent((n) => console.log("so:", n));
busChu.onEvent((s) => console.log("chu:", s));

busSo.phatSuKien(1);
busChu.phatSuKien("xin chao");
busSo.phatSuKien(2);
```

```text title=readonly
so: 1
chu: xin chao
so: 2
```

`busSo` (kiểu `EventBus<number>`) VÀ `busChu` (kiểu `EventBus<string>`)
KHÔNG đụng chạm nhau — phát sự kiện trên `busSo` KHÔNG BAO GIỜ gọi
observer đã đăng ký Ở `busChu`.
::::

::::predict{#doan-gia-tri-generic commitOnce}
```typescript
type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => void;
  phatSuKien: (giaTri: T) => void;
};
function taoEventBus<T>(): EventBus<T> {
  const nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => { nguoiNghe.push(fn); },
    phatSuKien: (giaTri) => { for (const fn of nguoiNghe) fn(giaTri); },
  };
}

const bus = taoEventBus<number>();
const tong: number[] = [];
bus.onEvent((n) => tong.push(n));
bus.onEvent((n) => tong.push(n * 2));

bus.phatSuKien(5);
console.log(tong);
```

Dòng cuối in ra gì?

:::opt{correct}
`[5,10]`
:::

:::opt
`[5]` — vì `phatSuKien(5)` chỉ phát MỘT LẦN, VÀ một lần phát CHỈ gọi
được MỘT observer, KHÔNG PHẢI TẤT CẢ observer đã đăng ký
::why
Gần đúng ở việc bạn nhớ ĐÚNG `phatSuKien(5)` được gọi ĐÚNG MỘT LẦN —
quan sát ĐÓ về SỐ LẦN gọi `phatSuKien` là chính xác.

Chỗ lệch: "phát MỘT LẦN" nghĩa LÀ gọi HÀM `phatSuKien` MỘT LẦN — BÊN
TRONG hàm ĐÓ có một VÒNG LẶP `for (const fn of nguoiNghe) fn(giaTri)`
chạy qua **TẤT CẢ** observer ĐÃ đăng ký (Ở ĐÂY LÀ HAI: `n => push(n)`
VÀ `n => push(n*2)`), KHÔNG PHẢI chỉ MỘT. Một lần phát → CẢ HAI
observer CÙNG chạy, ĐẨY `5` rồi `10` vào `tong`.
::
:::

:::opt
Máy báo lỗi biên dịch — `taoEventBus<number>()` khai kiểu GENERIC
tường minh (`<number>`), nhưng `bus.onEvent((n) => tong.push(n))`
KHÔNG khai lại kiểu `n`, TypeScript đòi tham số hàm callback PHẢI khai
kiểu TƯỜNG MINH khi generic ĐÃ được chỉ định
::why
Gần đúng ở việc bạn để ý `taoEventBus<number>()` khai `<number>` RÕ
RÀNG — một quan sát ĐÚNG về CÚ PHÁP generic.

Chỗ lệch: TypeScript **SUY LUẬN** (infer) kiểu tham số `n` TỪ kiểu
`bus` đã biết (`EventBus<number>` → `onEvent` đòi `fn: (giaTri:
number) => void`) — KHÔNG cần khai LẠI `(n: number) =>`. Đây LÀ tính
năng "contextual typing" chuẩn của TypeScript, KHÔNG PHẢI lỗi — biên
dịch SẠCH.
::
:::
::::

::::code{#viet_eventbus}
Hoàn thiện `taoEventBus<T>` — `onEvent` đăng ký, `phatSuKien` phát
cho TỪNG observer đã đăng ký.

```typescript title=starter
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => void;
  phatSuKien: (giaTri: T) => void;
};

function taoEventBus<T>(): EventBus<T> {
  const nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      ___;
    },
    phatSuKien: (giaTri) => {
      for (const fn of nguoiNghe) ___;
    },
  };
}

const bus1 = taoEventBus<number>();
const nhanDuoc1: number[] = [];
bus1.onEvent((n) => nhanDuoc1.push(n));
bus1.phatSuKien(99);
assertEqual(nhanDuoc1.length, 1, "mot nguoi nghe nhan mot lan phat");
assertEqual(nhanDuoc1[0], 99, "gia tri nhan dung");
```

```typescript title=solution
function assertEqual<T>(actual: T, expected: T, label: string): void {
  if (actual !== expected) throw new Error(`[FAIL] ${label}: mong ${expected}, nhan ${actual}`);
  console.log(`[PASS] ${label}`);
}

type EventBus<T> = {
  onEvent: (fn: (giaTri: T) => void) => void;
  phatSuKien: (giaTri: T) => void;
};

function taoEventBus<T>(): EventBus<T> {
  const nguoiNghe: Array<(giaTri: T) => void> = [];
  return {
    onEvent: (fn) => {
      nguoiNghe.push(fn);
    },
    phatSuKien: (giaTri) => {
      for (const fn of nguoiNghe) fn(giaTri);
    },
  };
}

const bus1 = taoEventBus<number>();
const nhanDuoc1: number[] = [];
bus1.onEvent((n) => nhanDuoc1.push(n));
bus1.phatSuKien(99);
assertEqual(nhanDuoc1.length, 1, "mot nguoi nghe nhan mot lan phat");
assertEqual(nhanDuoc1[0], 99, "gia tri nhan dung");
```

```typescript title=test
const bus2 = taoEventBus<string>();
const nhanDuoc2a: string[] = [];
const nhanDuoc2b: string[] = [];
bus2.onEvent((s) => nhanDuoc2a.push(s));
bus2.onEvent((s) => nhanDuoc2b.push(s + "!"));
bus2.phatSuKien("hi");
assertEqual(nhanDuoc2a[0], "hi", "nguoi nghe thu nhat nhan dung");
assertEqual(nhanDuoc2b[0], "hi!", "nguoi nghe thu hai bien doi dung");

const bus3 = taoEventBus<number>();
bus3.phatSuKien(123);
assertEqual(nhanDuoc1.length, 1, "bus rieng biet khong anh huong bus khac");
```

:::hints
- kind: attention
  body: "onEvent: THÊM fn (tham số của onEvent) vào nguoiNghe. phatSuKien: GỌI fn (biến vòng lặp) với giaTri, cho TỪNG phần tử."
- kind: strategy
  body: "nguoiNghe.push(fn) : fn(giaTri) — thêm vào mảng nội bộ, và gọi hàm nghe."
- kind: one-line
  body: '___ (onEvent) = nguoiNghe.push(fn)\n___ (phatSuKien) = fn(giaTri)'
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
EventBus = nhà máy đóng gói mảng observer, generic theo kiểu dữ liệu.
Còn một lỗ hổng: đăng ký RỒI, làm sao HUỶ đăng ký?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`onEvent` hiện KHÔNG trả về gì (`void`) — MỘT khi đã đăng ký, KHÔNG
CÓ cách nào gỡ observer đó RA. Vấn đề gì xảy ra nếu một observer
"sống" MÃI MÃI trong `nguoiNghe`, dù phần code TẠO ra nó đã KHÔNG
còn cần nữa?
::::

::::checkpoint{mastery=0.8}
::::
