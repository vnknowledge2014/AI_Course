---
id: co-so-du-lieu.vu-tru-tat-dinh.dong-ho-ao-khong-goi-datenow
title: "Đồng hồ ảo — không gọi Date.now()"
summary: "DongHoAo giữ đúng một trường hienTai, khởi tạo 0. tienToi(dh, soMs) là CÁCH DUY NHẤT làm nó thay đổi — cộng thêm soMs vào hienTai, không hề chạm tới Date.now()/performance.now() của hệ điều hành. Thời gian mô phỏng hoàn toàn nằm trong tay chương trình: gọi tienToi ba lần (500, 1200, 300) cho hienTai lần lượt 500, 1700, 2000 -- không phụ thuộc máy chạy nhanh hay chậm, không phụ thuộc chạy lúc mấy giờ."
locale: vi
track: co-so-du-lieu
module: vu-tru-tat-dinh
order: 4
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [db.dong-ho-ao-khong-goi-datenow]
requires: [db.gieo-hat-la-tai-lap-duoc]
concepts: [db.dong-ho-ao-khong-goi-datenow]
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
Số ngẫu nhiên đã TẤT định (bài 2-3). Nhưng mô phỏng THƯỜNG cần biết
"lúc nào" một sự KIỆN xảy ra — VÀ `Date.now()` mắc đúng CĂN bệnh của
`Math.random()`: KHÔNG tái lập được.
::::

::::explain{#dong-ho-ao}
`DongHoAo` giữ đúng MỘT trường `hienTai`, khởi tạo `0`. `tienToi(dh,
soMs)` LÀ cách DUY nhất làm nó thay đổi — CỘNG thêm `soMs` vào
`hienTai`, KHÔNG hề chạm tới `Date.now()`/`performance.now()` của hệ
điều hành:

```typescript title=readonly
interface DongHoAo { hienTai: number; }

function taoDongHoAo(): DongHoAo {
  return { hienTai: 0 };
}

function tienToi(dh: DongHoAo, soMs: number): void {
  dh.hienTai += soMs;
}

const dh = taoDongHoAo();
console.log("luc bat dau:", dh.hienTai);
tienToi(dh, 500);
console.log("sau khi tien 500ms:", dh.hienTai);
tienToi(dh, 1200);
console.log("sau khi tien them 1200ms:", dh.hienTai);
tienToi(dh, 300);
console.log("sau khi tien them 300ms:", dh.hienTai);
```

```text title=readonly
luc bat dau: 0
sau khi tien 500ms: 500
sau khi tien them 1200ms: 1700
sau khi tien them 300ms: 2000
```

`hienTai` tăng ĐÚNG theo tổng CÁC lần gọi `tienToi` — `500`, RỒI
`500+1200=1700`, RỒI `1700+300=2000`. KHÔNG hề CÓ độ trễ THẬT nào
giữa các dòng `console.log` (script chạy trong VÀI mili-giây thật)
— nhưng "thời gian mô phỏng" ĐÃ trôi qua đúng `2` giây.
::::

::::example{#nen-thoi-gian}
Đây LÀ lý do `DongHoAo` mạnh: mô phỏng `10000` giờ hoạt ĐỘNG của một
hệ thống CHỈ cần gọi `tienToi` đủ số LẦN — chạy THẬT trong vài giây
(hoặc ÍT hơn) trên máy THẬT, KHÔNG cần chờ `10000` giờ thật trôi
qua. Tách "thời gian mô PHỎNG" khỏi "thời gian CHẠY mô phỏng" LÀ
điều `Date.now()` không BAO giờ cho phép — nó LUÔN gắn chặt VỚI đồng
hồ vật lý CỦA máy đang chạy.
::::

::::predict{#doan-tien-am commitOnce}
Gọi `tienToi(dh, -100)` (SỐ mili-giây ÂM) trên MỘT đồng hồ đang Ở
`hienTai = 500`. `hienTai` SAU đó LÀ bao nhiêu?
:::opt{correct}
`400` — `tienToi` chỉ đơn giản CỘNG `soMs` VÀO `hienTai`, KHÔNG hề
kiểm tra dấu; `500 + (-100) = 400`, đồng hồ "lùi" LẠI dù về mặt Ý
nghĩa đó KHÔNG hợp lý cho một mô phỏng THẬT
:::
:::opt
Lỗi runtime — thời GIAN không thể "tiến" một lượng ÂM, hàm PHẢI từ
chối
::why
Trực giác NÀY hợp LÝ về mặt Ý NGHĨA (thời gian không lùi) — nhưng
`tienToi` (đúng NHƯ code Ở trên) không hề CÓ dòng kiểm tra dấu NÀO.

Chỗ lệch: `dh.hienTai += soMs;` LÀ phép cộng số HỌC trần trụi —
JavaScript KHÔNG tự chặn số ÂM. `tienToi(dh, -100)` chạy BÌNH thường,
KHÔNG ném lỗi, VÀ `hienTai` giảm ĐÚNG `100`. (Trong THỰC tế, code
GỌI `tienToi` PHẢI tự đảm bảo `soMs >= 0` — bài NÀY chưa thêm kiểm
tra đó.)
::
:::
::::

::::code{#viet_tien_toi}
Hoàn thiện `tienToi` — cộng `soMs` VÀO `dh.hienTai`.

```typescript title=starter
interface DongHoAo { hienTai: number; }

function taoDongHoAo(): DongHoAo {
  return { hienTai: 0 };
}

function tienToi(dh: DongHoAo, soMs: number): void {
  ___
}

const dh = taoDongHoAo();
tienToi(dh, 500);
console.log(dh.hienTai);
```

```typescript title=solution
interface DongHoAo { hienTai: number; }

function taoDongHoAo(): DongHoAo {
  return { hienTai: 0 };
}

function tienToi(dh: DongHoAo, soMs: number): void {
  dh.hienTai += soMs;
}

const dh = taoDongHoAo();
tienToi(dh, 500);
console.log(dh.hienTai);
```

```typescript title=test
const dh2 = taoDongHoAo();
const tBanDau = dh2.hienTai;
if (tBanDau !== 0) throw new Error("dong ho moi tao phai bat dau tu 0");
tienToi(dh2, 500);
const t1 = dh2.hienTai;
if (t1 !== 500) throw new Error("sau tienToi(500), hienTai phai la 500");
tienToi(dh2, 1200);
const t2 = dh2.hienTai;
if (t2 !== 1700) throw new Error("sau tienToi(1200) tiep theo, hienTai phai la 1700");
tienToi(dh2, 0);
const t3 = dh2.hienTai;
if (t3 !== 1700) throw new Error("tienToi(0) khong duoc doi hienTai");
tienToi(dh2, 300);
const t4 = dh2.hienTai;
if (t4 !== 2000) throw new Error("sau tienToi(300) tiep theo, hienTai phai la 2000");

const dhRieng1 = taoDongHoAo();
const dhRieng2 = taoDongHoAo();
tienToi(dhRieng1, 999);
const tRieng2 = dhRieng2.hienTai;
if (tRieng2 !== 0) throw new Error("tienToi tren MOT dong ho khong duoc anh huong dong ho KHAC");
```

:::hints
- kind: attention
  body: "Cong soMs vao dh.hienTai -- mot dong."
- kind: strategy
  body: "dh.hienTai += soMs;"
- kind: one-line
  body: "dh.hienTai += soMs;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "500"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Thời gian ảo, số ngẫu nhiên tất định — còn thiếu một mảnh: THỨ tự
xử lý SỰ kiện khi nhiều thứ CÙNG xảy ra Ở các thời điểm KHÁC nhau.
::::

::::reflect{#nghi-lai}
`DongHoAo` LÀ một `interface` bé nhỏ — MỘT trường số, một hàm cộng.
Nhưng ý NGHĨA của nó lớn: TÁCH hoàn toàn "thời gian mô phỏng" khỏi
đồng hồ VẬT lý của máy đang chạy chương trình. Kết HỢP VỚI
`soTiepTheo` (bài 2-3), giờ CẢ hai trục "khi NÀO" VÀ "cái gì xảy ra"
đều nằm HOÀN toàn trong tay chương trình — không CÒN phụ thuộc hệ
điều hành. Chỉ CÒN thiếu một cấu trúc để SẮP xếp các sự kiện theo
đúng thời điểm ẢO đó.
::::

::::checkpoint{mastery=0.8}
::::
