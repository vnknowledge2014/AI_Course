---
id: ky-nghe-phan-mem.mau-tai-cau-truc.capstone-theo-doi-gia-co-phieu
title: "Capstone: Theo dõi giá cổ phiếu qua EventBus"
summary: "Bài chốt cụm 2: taoEventBus<SuKienGia>(), đăng ký HAI observer độc lập (capNhatTongGiaTri tính tổng danh mục, ghiLichSu ghi lịch sử), phát nhiều sự kiện giá khác nhau, kiểm cả hai nhận đúng — rồi huỷ MỘT observer, xác nhận nó KHÔNG còn nhận sự kiện tiếp theo trong khi observer kia vẫn hoạt động."
locale: vi
track: ky-nghe-phan-mem
module: mau-tai-cau-truc
order: 8
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 15
teaches: [mau.gate-boss-observer]
requires: [mau.unsubscribe-cleanup]
concepts: [mau.gate-boss-observer]
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
Bài chốt cụm 2. Một danh mục đầu tư nắm giữ HAI mã cổ phiếu — TÍNH
tổng giá trị VÀ GHI lịch sử thay đổi, CÙNG LÚC, qua MỘT bus.
::::

::::explain{#hai-observer-doc-lap-tren-mot-bus}
Ghép TOÀN BỘ cụm 2: `taoEventBus<SuKienGia>()` (bài 6) mang HAI
observer HOÀN TOÀN ĐỘC LẬP — `capNhatTongGiaTri` (cập nhật giá MỚI
NHẤT VÀ tính LẠI tổng giá trị danh mục) VÀ `ghiLichSu` (chỉ ĐẨY sự
kiện vào một mảng). MỘT bus, HAI mục đích KHÁC NHAU:

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

type SuKienGia = { maCoPhieu: string; giaMoi: number };

const soLuongNamGiu = new Map<string, number>([["AAA", 10], ["BBB", 5]]);
const giaHienTai = new Map<string, number>([["AAA", 0], ["BBB", 0]]);
let tongGiaTri = 0;

function capNhatTongGiaTri(su: SuKienGia): void {
  giaHienTai.set(su.maCoPhieu, su.giaMoi);
  let tong = 0;
  for (const [ma, soLuong] of soLuongNamGiu) {
    tong += soLuong * giaHienTai.get(ma)!;
  }
  tongGiaTri = tong;
}

const lichSu: SuKienGia[] = [];
function ghiLichSu(su: SuKienGia): void {
  lichSu.push(su);
}

const bus = taoEventBus<SuKienGia>();
const huyCapNhat = bus.onEvent(capNhatTongGiaTri);
bus.onEvent(ghiLichSu);

bus.phatSuKien({ maCoPhieu: "AAA", giaMoi: 100 });
bus.phatSuKien({ maCoPhieu: "BBB", giaMoi: 50 });

huyCapNhat();
bus.phatSuKien({ maCoPhieu: "AAA", giaMoi: 200 });

console.log(tongGiaTri);
console.log(lichSu.length);
```

```text title=readonly
1250
3
```

Sau hai sự kiện ĐẦU: `tongGiaTri = 10*100 + 5*50 = 1250`. `huyCapNhat()`
gỡ `capNhatTongGiaTri` — sự kiện THỨ BA (`AAA` → `200`) VẪN được
`ghiLichSu` ghi nhận (`lichSu.length = 3`), NHƯNG `tongGiaTri` **KHÔNG
ĐỔI** (dừng Ở `1250`, KHÔNG tính lại thành `10*200+5*50=2250`) — ĐÚNG
Ý NGHĨA huỷ đăng ký (bài 7): observer ĐÃ huỷ KHÔNG còn nhận sự kiện
MỚI, observer CÒN LẠI hoàn toàn KHÔNG bị ảnh hưởng.
::::

::::example{#huy-dang-ky-khong-anh-huong-tuong-lai}
Huỷ đăng ký CHỈ ảnh hưởng sự kiện **TƯƠNG LAI** — kết quả ĐÃ tính
TRƯỚC đó (dựa trên sự kiện ĐÃ nhận) KHÔNG bị "tính LẠI" hay "xoá":

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

type SuKienGia = { maCoPhieu: string; giaMoi: number };
const soLuongNamGiu = new Map<string, number>([["AAA", 10], ["BBB", 5]]);
const giaHienTai = new Map<string, number>([["AAA", 0], ["BBB", 0]]);
let tongGiaTri = 0;
function capNhatTongGiaTri(su: SuKienGia): void {
  giaHienTai.set(su.maCoPhieu, su.giaMoi);
  let tong = 0;
  for (const [ma, soLuong] of soLuongNamGiu) {
    tong += soLuong * giaHienTai.get(ma)!;
  }
  tongGiaTri = tong;
}

const bus = taoEventBus<SuKienGia>();
const huyCapNhat = bus.onEvent(capNhatTongGiaTri);

bus.phatSuKien({ maCoPhieu: "AAA", giaMoi: 10 });
huyCapNhat();
bus.phatSuKien({ maCoPhieu: "BBB", giaMoi: 1000 });
console.log(tongGiaTri);
```

```text title=readonly
100
```

Sự kiện `AAA=10` tính `tongGiaTri = 10*10 + 5*0 = 100` TRƯỚC khi huỷ.
Sự kiện `BBB=1000` (SAU khi huỷ) KHÔNG chạm tới `capNhatTongGiaTri`
NỮA — `tongGiaTri` GIỮ NGUYÊN `100`, KHÔNG nhảy lên `5100`.
::::

::::predict{#doan-huy-truoc-su-kien-lon commitOnce}
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

type SuKienGia = { maCoPhieu: string; giaMoi: number };
const soLuongNamGiu = new Map<string, number>([["AAA", 10], ["BBB", 5]]);
const giaHienTai = new Map<string, number>([["AAA", 0], ["BBB", 0]]);
let tongGiaTri = 0;
function capNhatTongGiaTri(su: SuKienGia): void {
  giaHienTai.set(su.maCoPhieu, su.giaMoi);
  let tong = 0;
  for (const [ma, soLuong] of soLuongNamGiu) {
    tong += soLuong * giaHienTai.get(ma)!;
  }
  tongGiaTri = tong;
}

const bus = taoEventBus<SuKienGia>();
const huyCapNhat = bus.onEvent(capNhatTongGiaTri);

bus.phatSuKien({ maCoPhieu: "AAA", giaMoi: 10 });
huyCapNhat();
bus.phatSuKien({ maCoPhieu: "BBB", giaMoi: 1000 });
console.log(tongGiaTri);
```

Dòng cuối in ra gì?

:::opt{correct}
`100`
:::

:::opt
`5100` — vì `phatSuKien({ maCoPhieu: "BBB", giaMoi: 1000 })` VẪN được
GỌI (dòng code ĐÓ THỰC THI BÌNH THƯỜNG), nên `capNhatTongGiaTri`
VẪN chạy VÀ tính LẠI `tongGiaTri` dựa trên `giaMoi = 1000`
::why
Gần đúng ở việc bạn nhớ ĐÚNG dòng `bus.phatSuKien({...BBB, 1000})`
VẪN được THỰC THI (KHÔNG bị bỏ qua, KHÔNG lỗi) — quan sát ĐÓ đúng.

Chỗ lệch: `phatSuKien` THỰC THI, NHƯNG nó chỉ LẶP qua `nguoiNghe`
**LÚC ĐÓ** — VÀ `huyCapNhat()` (gọi TRƯỚC ĐÓ) đã `filter` XOÁ
`capNhatTongGiaTri` khỏi `nguoiNghe` **RỒI**. `phatSuKien` VẪN chạy,
NHƯNG danh sách nó LẶP qua giờ KHÔNG còn `capNhatTongGiaTri` NỮA — SO
với chương trình tổng thể, `phatSuKien` "chạy" KHÔNG đồng nghĩa "MỌI
observer TỪNG đăng ký ĐỀU chạy". `tongGiaTri` GIỮ NGUYÊN `100`.
::
:::

:::opt
Máy báo lỗi lúc chạy — `huyCapNhat()` được gọi TRƯỚC `bus.phatSuKien`
lần THỨ HAI, TypeScript/JavaScript coi việc "huỷ MỘT observer RỒI VẪN
gọi `phatSuKien`" LÀ thao tác KHÔNG hợp lệ, ném lỗi NGAY LÚC CHẠY
::why
Gần đúng ở việc bạn để ý THỨ TỰ gọi (`huyCapNhat()` TRƯỚC lần
`phatSuKien` thứ hai) — một quan sát ĐÚNG về TRÌNH TỰ code.

Chỗ lệch: KHÔNG có quy tắc "huỷ RỒI gọi `phatSuKien` LÀ lỗi" — `bus`
VẪN LÀ một `EventBus` HOÀN TOÀN HỢP LỆ SAU khi một (hay TẤT CẢ)
observer bị huỷ, `phatSuKien` VẪN gọi được BÌNH THƯỜNG (chỉ đơn giản
LÀ lặp qua một mảng `nguoiNghe` NGẮN HƠN, có THỂ RỖNG). Chạy SẠCH,
không lỗi nào cả.
::
:::
::::

::::code{#viet_capstone_observer}
Hoàn thiện `capNhatTongGiaTri` (công thức tổng giá trị) VÀ đăng ký
`ghiLichSu` vào bus.

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
      return () => { nguoiNghe = nguoiNghe.filter((f) => f !== fn); };
    },
    phatSuKien: (giaTri) => { for (const f of nguoiNghe) f(giaTri); },
  };
}

type SuKienGia = { maCoPhieu: string; giaMoi: number };

const soLuongNamGiu = new Map<string, number>([["AAA", 10], ["BBB", 5]]);
const giaHienTai = new Map<string, number>([["AAA", 0], ["BBB", 0]]);
let tongGiaTri = 0;

function capNhatTongGiaTri(su: SuKienGia): void {
  giaHienTai.set(su.maCoPhieu, su.giaMoi);
  let tong = 0;
  for (const [ma, soLuong] of soLuongNamGiu) {
    tong += ___;
  }
  tongGiaTri = tong;
}

const lichSu: SuKienGia[] = [];
function ghiLichSu(su: SuKienGia): void {
  lichSu.push(su);
}

const bus = taoEventBus<SuKienGia>();
const huyCapNhat = bus.onEvent(capNhatTongGiaTri);
___;

bus.phatSuKien({ maCoPhieu: "AAA", giaMoi: 100 });
bus.phatSuKien({ maCoPhieu: "BBB", giaMoi: 50 });
assertEqual(tongGiaTri, 1250, "tong gia tri sau hai su kien");
assertEqual(lichSu.length, 2, "lich su ghi du hai su kien");
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
      return () => { nguoiNghe = nguoiNghe.filter((f) => f !== fn); };
    },
    phatSuKien: (giaTri) => { for (const f of nguoiNghe) f(giaTri); },
  };
}

type SuKienGia = { maCoPhieu: string; giaMoi: number };

const soLuongNamGiu = new Map<string, number>([["AAA", 10], ["BBB", 5]]);
const giaHienTai = new Map<string, number>([["AAA", 0], ["BBB", 0]]);
let tongGiaTri = 0;

function capNhatTongGiaTri(su: SuKienGia): void {
  giaHienTai.set(su.maCoPhieu, su.giaMoi);
  let tong = 0;
  for (const [ma, soLuong] of soLuongNamGiu) {
    tong += soLuong * giaHienTai.get(ma)!;
  }
  tongGiaTri = tong;
}

const lichSu: SuKienGia[] = [];
function ghiLichSu(su: SuKienGia): void {
  lichSu.push(su);
}

const bus = taoEventBus<SuKienGia>();
const huyCapNhat = bus.onEvent(capNhatTongGiaTri);
bus.onEvent(ghiLichSu);

bus.phatSuKien({ maCoPhieu: "AAA", giaMoi: 100 });
bus.phatSuKien({ maCoPhieu: "BBB", giaMoi: 50 });
assertEqual(tongGiaTri, 1250, "tong gia tri sau hai su kien");
assertEqual(lichSu.length, 2, "lich su ghi du hai su kien");
```

```typescript title=test
huyCapNhat();
bus.phatSuKien({ maCoPhieu: "AAA", giaMoi: 200 });
assertEqual(tongGiaTri, 1250, "tong gia tri KHONG doi sau khi huy dang ky capNhat");
assertEqual(lichSu.length, 3, "lich su VAN ghi tiep vi ghiLichSu chua bi huy");
```

:::hints
- kind: attention
  body: "capNhatTongGiaTri: nhân soLuong (biến vòng lặp) với giá HIỆN TẠI của đúng mã đó. Wiring: đăng ký ghiLichSu vào CÙNG bus, giống cách capNhatTongGiaTri đã đăng ký."
- kind: strategy
  body: "soLuong * giaHienTai.get(ma)! : bus.onEvent(ghiLichSu) — nhân số lượng với giá, và đăng ký observer thứ hai."
- kind: one-line
  body: '___ (trong vong lap) = soLuong * giaHienTai.get(ma)!\n___ (dang ky) = bus.onEvent(ghiLichSu)'
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
Cụm 2 hoàn tất: Observer OOP shape, EventBus tự viết, huỷ đăng ký,
capstone nhiều observer độc lập. Cụm tiếp theo: Command = Discriminated
Union.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Observer PHẢN ỨNG khi sự kiện xảy ra — nó KHÔNG "hoàn tác" được gì cả.
Nếu một ứng dụng ghi chú cần UNDO/REDO (quay lại thao tác TRƯỚC), một
mảng HÀM observer có đủ không?
::::

::::checkpoint{mastery=0.8}
::::
