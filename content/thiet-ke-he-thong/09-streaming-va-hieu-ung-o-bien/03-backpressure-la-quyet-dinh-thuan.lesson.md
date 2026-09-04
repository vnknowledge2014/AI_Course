---
id: thiet-ke-he-thong.streaming-va-hieu-ung-o-bien.backpressure-la-quyet-dinh-thuan
title: "Backpressure như một phép quyết định thuần: buffer đầy → tín hiệu chậm lại"
summary: "quyetDinhNhanSuKien(kichThuocBufferHienTai, gioiHanBuffer) tra ve 'nhan' | 'tu_choi_tam' | 'chan_lai' -- ham THUAN, khong goi I/O nao, khong tu dung producer -- no CHI tra ve mot TIN HIEU; buffer duoi 80% gioi han la 'nhan', tu 80% den duoi 100% la 'tu_choi_tam', du 100% la 'chan_lai'. Ai goi ham nay (vo menh lenh, bai sau) moi la nguoi quyet dinh THUC SU tam dung nhan du lieu."
locale: vi
track: thiet-ke-he-thong
module: streaming-va-hieu-ung-o-bien
order: 3
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 11
teaches: [sd.fp.backpressure-la-quyet-dinh-thuan]
requires: [sd.fp.quyet-dinh-gom-lo]
concepts: [sd.fp.backpressure-la-quyet-dinh-thuan]
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
`quyetDinhGomLo` (bài trước) giả định buffer luôn có chỗ để nhận thêm
sự kiện. Nhưng nếu bên gửi (producer) bơm dữ liệu NHANH hơn bên nhận
(consumer) kịp gom lô VÀ gửi đi, buffer sẽ phình to KHÔNG giới hạn.
Cần một tín hiệu — "chậm lại" — trước khi bộ nhớ cạn kiệt. Câu hỏi
Ở đây: tín hiệu đó nên LÀ gì, VÀ ai chịu trách nhiệm tạo ra nó?
::::

::::explain{#tin-hieu-thuan-khong-tu-chan}
`quyetDinhNhanSuKien` nhận kích thước buffer HIỆN tại VÀ giới hạn cho
phép, trả về MỘT trong ba tín hiệu: `"nhan"` (buffer còn nhiều chỗ),
`"tu_choi_tam"` (buffer đã đầy Ít NHẤT `80%`, nên bắt đầu từ chối tạm
để giảm áp lực), hoặc `"chan_lai"` (buffer đã CHẠM hoặc VƯỢT giới hạn).
Hàm không hề gọi bất kỳ I/O nào — không dừng producer, không ném lỗi,
không ghi log — nó CHỈ trả về một chuỗi:

```typescript title=readonly
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";

function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}

const gioiHan = 10;
for (const kichThuoc of [0, 3, 7, 8, 9, 10, 11]) {
  console.log("buffer =", kichThuoc, "/", gioiHan, "->", quyetDinhNhanSuKien(kichThuoc, gioiHan));
}

console.log("--- ham THUAN: khong doc/ghi gi ben ngoai ---");
const kq1 = quyetDinhNhanSuKien(8, 10);
const kq2 = quyetDinhNhanSuKien(8, 10);
console.log("goi 2 lan voi cung tham so:", kq1, kq2, kq1 === kq2);
```

```text title=readonly
buffer = 0 / 10 -> nhan
buffer = 3 / 10 -> nhan
buffer = 7 / 10 -> nhan
buffer = 8 / 10 -> tu_choi_tam
buffer = 9 / 10 -> tu_choi_tam
buffer = 10 / 10 -> chan_lai
buffer = 11 / 10 -> chan_lai
--- ham THUAN: khong doc/ghi gi ben ngoai ---
goi 2 lan voi cung tham so: tu_choi_tam tu_choi_tam true
```

`quyetDinhNhanSuKien` không hề "chặn" ai cả Ở chính bản thân nó — dù
tín hiệu LÀ `"chan_lai"`, hàm vẫn trả về BÌNH thường VÀ kết thúc ngay
lập tức. Nó không có vòng lặp chờ, không có `await`, không có tác dụng
phụ nào. Việc THỰC SỰ dừng nhận dữ liệu — nếu có — là việc của một lớp
KHÁC, đọc tín hiệu này rồi tự quyết định hành động.
::::

::::example{#cung-buffer-gioi-han-khac-nhau}
Vì `quyetDinhNhanSuKien` chỉ phụ thuộc vào ĐÚNG hai tham số truyền vào,
CÙNG một kích thước buffer có thể cho ra ba tín hiệu KHÁC nhau, tùy
`gioiHanBuffer` — không có trạng thái ẩn nào ảnh hưởng tới quyết định:

```typescript title=readonly
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";
function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}

// mo phong buffer TANG DAN tu 0 den 12, hoi quyetDinhNhanSuKien o MOI muc --
// day CHI la mot phep FOLD tren du lieu bia, khong co buffer THAT nao ca
const gioiHan = 10;
const cacMucBuffer = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
const cacTinHieu = cacMucBuffer.map((m) => quyetDinhNhanSuKien(m, gioiHan));
console.log("cac tin hieu theo tung muc buffer:", JSON.stringify(cacTinHieu));

const demTheoTinHieu = cacTinHieu.reduce(
  (dem, th) => ({ ...dem, [th]: (dem[th] ?? 0) + 1 }),
  {} as Record<TinHieuBackpressure, number>
);
console.log("dem theo tin hieu:", JSON.stringify(demTheoTinHieu));

console.log("--- gioiHanBuffer KHAC nhau cho CUNG kich thuoc buffer ---");
console.log("buffer=8, gioiHan=10:", quyetDinhNhanSuKien(8, 10));
console.log("buffer=8, gioiHan=100:", quyetDinhNhanSuKien(8, 100));
console.log("buffer=8, gioiHan=8:", quyetDinhNhanSuKien(8, 8));
```

```text title=readonly
cac tin hieu theo tung muc buffer: ["nhan","nhan","nhan","nhan","nhan","nhan","nhan","nhan","tu_choi_tam","tu_choi_tam","chan_lai","chan_lai","chan_lai"]
dem theo tin hieu: {"nhan":8,"tu_choi_tam":2,"chan_lai":3}
--- gioiHanBuffer KHAC nhau cho CUNG kich thuoc buffer ---
buffer=8, gioiHan=10: tu_choi_tam
buffer=8, gioiHan=100: nhan
buffer=8, gioiHan=8: chan_lai
```

Buffer Ở mức `8` cho ra BA tín hiệu khác nhau tùy `gioiHanBuffer`:
`"tu_choi_tam"` khi giới hạn là `10` (`80%`), `"nhan"` khi giới hạn là
`100` (`8%`, còn RẤT nhiều chỗ), `"chan_lai"` khi giới hạn CHÍNH LÀ `8`
(đã chạm đáy). Không có bộ nhớ "trước đó" nào được `quyetDinhNhanSuKien`
tham khảo — quyết định chỉ dựa vào hai con số Ở đúng lần gọi đó.
::::

::::predict{#doan-nguong-80-phan-tram commitOnce}
Gọi `quyetDinhNhanSuKien(4, 5)` (buffer Ở mức `4`, giới hạn `5`, tỉ lệ
ĐÚNG bằng `0.8`) VÀ tách biệt, `quyetDinhNhanSuKien(3, 5)` (tỉ lệ
`0.6`). Kết quả của HAI lệnh gọi này là gì?

:::opt{correct}
`quyetDinhNhanSuKien(4, 5)` trả về `"tu_choi_tam"` — tỉ lệ ĐÚNG bằng
`0.8` vẫn kích hoạt nhánh đó vì điều kiện dùng `>=`; `quyetDinhNhanSuKien(3,
5)` trả về `"nhan"` vì `0.6` chưa chạm ngưỡng `0.8`
:::
:::opt
Cả hai đều trả về `"nhan"` — `0.8` được xem LÀ mốc "gần đầy nhưng vẫn
còn ổn", chỉ khi tỉ lệ VƯỢT HẲN qua `0.8` (ví dụ `0.81` trở lên) tín
hiệu mới đổi thành `"tu_choi_tam"`
::why
Nhầm ranh giới của điều kiện `if (tiLe >= 0.8)` — toán tử LÀ `>=`,
không phải `>`, nên tỉ lệ ĐÚNG bằng `0.8` đã đủ để rơi vào nhánh
`"tu_choi_tam"`, không cần vượt qua nó.

Chỗ lệch: `4 / 5` tính ra ĐÚNG `0.8` — biểu thức `tiLe >= 0.8` là
`0.8 >= 0.8`, tức `true`. Đây là cùng dạng lỗi ranh giới đã gặp Ở
`quyetDinhGomLo` (bài trước, ngưỡng thời gian chờ) VÀ Ở `quyetDinhDem`
(quest "Nhật ký bất biến và fold"): mọi ngưỡng trong các hàm quyết
định của track này đều được viết để CHẠM ĐÚNG mốc là đủ điều kiện, trừ
khi văn xuôi nói rõ điều ngược lại.
::
:::
::::

::::code{#viet_quyet_dinh_nhan_su_kien}
Hoàn thiện `quyetDinhNhanSuKien` — nếu `kichThuocBufferHienTai >=
gioiHanBuffer`, trả về `"chan_lai"`. Nếu KHÔNG, tính `tiLe =
kichThuocBufferHienTai / gioiHanBuffer`; nếu `tiLe >= 0.8`, trả về
`"tu_choi_tam"`; ngược lại trả về `"nhan"`.

```typescript title=starter
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";

function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  ___
}

console.log(quyetDinhNhanSuKien(2, 10), quyetDinhNhanSuKien(9, 10), quyetDinhNhanSuKien(10, 10));
```

```typescript title=solution
type TinHieuBackpressure = "nhan" | "tu_choi_tam" | "chan_lai";

function quyetDinhNhanSuKien(kichThuocBufferHienTai: number, gioiHanBuffer: number): TinHieuBackpressure {
  if (kichThuocBufferHienTai >= gioiHanBuffer) return "chan_lai";
  const tiLe = kichThuocBufferHienTai / gioiHanBuffer;
  if (tiLe >= 0.8) return "tu_choi_tam";
  return "nhan";
}

console.log(quyetDinhNhanSuKien(2, 10), quyetDinhNhanSuKien(9, 10), quyetDinhNhanSuKien(10, 10));
```

```typescript title=test
if (quyetDinhNhanSuKien(0, 10) !== "nhan") throw new Error("buffer rong phai la nhan");
if (quyetDinhNhanSuKien(7, 10) !== "nhan") throw new Error("70% phai la nhan");
if (quyetDinhNhanSuKien(8, 10) !== "tu_choi_tam") throw new Error("dung 80% phai la tu_choi_tam (bien >=0.8)");
if (quyetDinhNhanSuKien(9, 10) !== "tu_choi_tam") throw new Error("90% phai la tu_choi_tam");
if (quyetDinhNhanSuKien(10, 10) !== "chan_lai") throw new Error("dung 100% phai la chan_lai");
if (quyetDinhNhanSuKien(15, 10) !== "chan_lai") throw new Error("vuot gioi han phai la chan_lai");

const tKq1 = quyetDinhNhanSuKien(5, 10);
const tKq2 = quyetDinhNhanSuKien(5, 10);
if (tKq1 !== tKq2) throw new Error("quyetDinhNhanSuKien phai THUAN -- cung dau vao phai cho cung ket qua");

if (quyetDinhNhanSuKien(4, 5) !== "tu_choi_tam") throw new Error("4/5 = 0.8 dung nguong phai la tu_choi_tam");
if (quyetDinhNhanSuKien(3, 5) !== "nhan") throw new Error("3/5 = 0.6 duoi nguong phai la nhan");
```

:::hints
- kind: attention
  body: "Kiem tra dieu kien CHAN LAI truoc: neu kichThuocBufferHienTai >= gioiHanBuffer, tra ve 'chan_lai' NGAY. Neu khong, tinh tiLe = kichThuocBufferHienTai / gioiHanBuffer; neu tiLe >= 0.8 tra ve 'tu_choi_tam'; con lai tra ve 'nhan'."
- kind: strategy
  body: "if (kichThuocBufferHienTai >= gioiHanBuffer) return 'chan_lai'; const tiLe = kichThuocBufferHienTai / gioiHanBuffer; if (tiLe >= 0.8) return 'tu_choi_tam'; return 'nhan';"
- kind: one-line
  body: "if (kichThuocBufferHienTai >= gioiHanBuffer) return \"chan_lai\"; const tiLe = kichThuocBufferHienTai / gioiHanBuffer; if (tiLe >= 0.8) return \"tu_choi_tam\"; return \"nhan\";"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "nhan tu_choi_tam chan_lai"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba tín hiệu, một hàm thuần — nhưng chưa có gì THỰC SỰ ngăn producer
gửi thêm cả. Ai đọc tín hiệu này VÀ biến nó thành một hành động chặn
thật?
::::

::::reflect{#nghi-lai}
`quyetDinhNhanSuKien` cố tình KHÔNG làm gì cả ngoài việc trả về một
chuỗi — đây chính LÀ điểm mấu chốt của "lõi quyết định thuần" áp dụng
vào backpressure: nếu hàm này TỰ chặn producer (ví dụ bằng cách
`await` một `Promise` không bao giờ resolve khi buffer đầy), nó sẽ
không còn thuần nữa, VÀ không thể test được chỉ bằng cách gọi hàm với
số liệu bịa. Tách "tín hiệu nên LÀ gì" khỏi "hành động chặn thật sự
diễn ra như thế nào" là điều cho phép đổi cơ chế chặn (dừng producer,
xếp hàng đợi, giảm tốc độ đọc) mà không cần sửa logic tính tín hiệu.
::::

::::checkpoint{mastery=0.76}
::::
