---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.base62-ma-hoa
title: "Base62: mã hoá số nguyên thành chuỗi ngắn"
summary: "maHoaBase62(n) mã hoá một số nguyên không âm thành chuỗi base62 (bảng đúng 62 ký tự 0-9,a-z,A-Z) bằng vòng lặp chia-lấy-dư liên tiếp cho 62, ghép ký tự từ PHẢI sang trái. Độ dài chuỗi KHÔNG tăng tuyến tính theo n — nó nhảy bậc đúng tại luỹ thừa của 62: n=61 (\"Z\") vẫn 1 ký tự, n=62 (\"10\") nhảy lên 2 ký tự, n=3843 (\"ZZ\") vẫn 2 ký tự, n=3844 (\"100\") nhảy lên 3 ký tự — ranh giới luôn đúng tại 62^k."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 1
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 9
teaches: [sd.base62-ma-hoa]
requires: [sd.boss-dinh-danh-va-toc-do]
concepts: [sd.base62-ma-hoa]
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
Quest trước ráp xong gateway: giới hạn tốc độ VÀ định danh. Realm 7 giờ
chuyển sang một bài toán khác — rút gọn URL. Mảnh ĐẦU tiên: biến một
số ĐẾM (counter) thành một chuỗi NGẮN, dễ dán vào tin nhắn.
::::

::::explain{#bang-base62}
Một mã ngắn cần NGẮN VÀ chỉ dùng ký tự an toàn cho URL. Base62 dùng
đúng `62` ký tự: mười chữ SỐ, hai mươi sáu chữ THƯỜNG, hai mươi sáu
chữ HOA — nhiều hơn base10 rất nhiều LẦN nên cùng một số nguyên biểu
diễn ra chuỗi NGẮN hơn hẳn. Thuật toán y hệt cách đổi cơ số vẫn học Ở
trường: chia LIÊN TỤC cho `62`, ghép PHẦN dư (là MỘT ký tự trong
bảng) từ phải sang trái, dừng khi thương SỐ về `0`:

```typescript title=readonly
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

console.log("bang co", BANG_BASE62.length, "ky tu");
console.log("maHoaBase62(0) =", maHoaBase62(0));
console.log("maHoaBase62(1) =", maHoaBase62(1));
console.log("maHoaBase62(10) =", maHoaBase62(10));
console.log("maHoaBase62(61) =", maHoaBase62(61));
console.log("maHoaBase62(62) =", maHoaBase62(62));
```

```text title=readonly
bang co 62 ky tu
maHoaBase62(0) = 0
maHoaBase62(1) = 1
maHoaBase62(10) = a
maHoaBase62(61) = Z
maHoaBase62(62) = 10
```

`n=0` LÀ trường hợp riêng (vòng lặp `while` không chạy LẦN nào nếu
không xử LÝ trước, vì `0 > 0` sai NGAY từ đầu). `n=10` rơi đúng VÀO
vùng chữ thường (`BANG_BASE62[10]` LÀ `"a"`, ký tự đầu tiên SAU mười
chữ số). `n=61` LÀ ký tự CUỐI của cả bảng (`"Z"`) — VÀ `n=62` (đúng
`62^1`) đã cần TỚI hai ký tự: `"10"`.
::::

::::example{#nhay-bac-luy-thua}
Độ dài chuỗi base62 không tăng ĐỀU theo `n` — nó giữ nguyên MỘT
khoảng dài rồi NHẢY lên đúng tại các mốc `62^1`, `62^2`, `62^3`:

```typescript title=readonly
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

const cacMocQuanTrong = [61, 62, 3843, 3844, 238327, 238328];
for (const n of cacMocQuanTrong) {
  const ma = maHoaBase62(n);
  console.log(`n=${n} -> "${ma}" (do dai ${ma.length})`);
}
```

```text title=readonly
n=61 -> "Z" (do dai 1)
n=62 -> "10" (do dai 2)
n=3843 -> "ZZ" (do dai 2)
n=3844 -> "100" (do dai 3)
n=238327 -> "ZZZ" (do dai 3)
n=238328 -> "1000" (do dai 4)
```

`61` LÀ số LỚN nhất còn biểu diễn được bằng MỘT ký tự (`62^1 - 1`).
`3843` LÀ số LỚN nhất còn vừa hai ký tự (`62^2 - 1`). `238327` LÀ số
LỚN nhất còn vừa BA ký tự (`62^3 - 1`). Ngay SAU mỗi mốc đó, `con`
(thương số) vẫn còn LỚN hơn `0` sau vòng chia ĐẦU tiên, nên vòng lặp
`while` buộc phải chạy THÊM một lần — VÀ độ dài chuỗi tăng thêm đúng
MỘT ký tự.
::::

::::predict{#doan-nhay-bac commitOnce}
Với bảng base62 gồm đúng `62` ký tự, `maHoaBase62(3843)` cho ra
`"ZZ"` (hai ký tự). `maHoaBase62(3844)` — SỐ nguyên ngay SAU đó — cho
ra chuỗi có mấy ký tự?

:::opt{correct}
BA ký tự (`"100"`) — `3844` đúng bằng `62^2`, vượt QUA phạm vi biểu
diễn được bằng hai ký tự (tối đa `62^2 - 1`, tức `3843`), nên vòng lặp
`while` phải chạy thêm MỘT lần nữa
:::
:::opt
Vẫn hai ký tự — vì `3844` chỉ hơn `3843` đúng MỘT đơn vị, một bước
tăng nhỏ không thể nào làm đổi cả ĐỘ dài chuỗi
::why
Nhầm "số nguyên tăng ĐỀU từng đơn vị một" VỚI "độ dài chuỗi base62
cũng tăng đều theo" — nhưng hai đại lượng NÀY không hề tỷ lệ tuyến
tính VỚI nhau.

Chỗ lệch: trong `maHoaBase62`, biến `con` bắt đầu bằng `3844`. Vòng
lặp ĐẦU: `3844 % 62 = 0`, `con = Math.floor(3844 / 62) = 62`. Vòng
lặp HAI: `62 % 62 = 0`, `con = Math.floor(62 / 62) = 1`. Vòng lặp BA:
`1 % 62 = 1`, `con = 0`, dừng. Ba vòng lặp đã chạy — NÊN chuỗi có
đúng BA ký tự (`"100"`), dù `n` chỉ tăng thêm `1` so VỚI `3843`.
::
:::
::::

::::code{#viet_ma_hoa_base62}
Hoàn thiện `maHoaBase62` — bên trong vòng lặp `while`, lấy CHỮ số
base62 hiện tại (`con % 62`) VÀ ghép vào ĐẦU chuỗi kết quả.

```typescript title=starter
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ___
    con = Math.floor(con / 62);
  }
  return ketQua;
}

console.log(maHoaBase62(125));
```

```typescript title=solution
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function maHoaBase62(n: number): string {
  if (n === 0) return "0";
  let ketQua = "";
  let con = n;
  while (con > 0) {
    ketQua = BANG_BASE62[con % 62]! + ketQua;
    con = Math.floor(con / 62);
  }
  return ketQua;
}

console.log(maHoaBase62(125));
```

```typescript title=test
if (maHoaBase62(0) !== "0") throw new Error("n=0 phai ma hoa thanh \"0\"");
if (maHoaBase62(61) !== "Z") throw new Error("n=61 (ky tu cuoi bang) phai la \"Z\"");
if (maHoaBase62(62) !== "10") throw new Error("n=62 (dung 62^1) phai la \"10\", danh dau do dai tang len 2");
if (maHoaBase62(125) !== "21") throw new Error("n=125 phai ma hoa thanh \"21\" (125 = 2*62 + 1)");
if (maHoaBase62(3843).length !== 2) throw new Error("n=3843 (62^2 - 1) van phai co do dai 2");
if (maHoaBase62(3844).length !== 3) throw new Error("n=3844 (dung 62^2) phai co do dai 3");
if (maHoaBase62(10) !== "a") throw new Error("n=10 phai la \"a\" (ky tu dau tien sau cac chu so)");
```

:::hints
- kind: attention
  body: "Ben trong vong lap, moi vong sinh ra DUNG mot ky tu base62 tu con % 62, roi ghep vao DAU ketQua (khong phai cuoi)."
- kind: strategy
  body: "BANG_BASE62[con % 62] la ky tu can them; ghep no truoc ketQua hien tai bang phep cong chuoi."
- kind: one-line
  body: "ketQua = BANG_BASE62[con % 62]! + ketQua;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "21"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Số nguyên giờ đã thành chuỗi NGẮN. Nhưng một dịch vụ rút gọn URL cần
đi NGƯỢC lại được — từ chuỗi đoán RA đúng số nguyên gốc.
::::

::::reflect{#nghi-lai}
`maHoaBase62` không hề "nén" dữ liệu — nó chỉ đổi CƠ số biểu diễn,
giống hệt cách `0xFF` VÀ `255` LÀ cùng một số. Cái LÀM chuỗi NGẮN đi
LÀ có `62` ký hiệu thay VÌ `10`, nên mỗi VỊ trí "gánh" được nhiều
thông tin hơn — VÀ độ dài chuỗi chỉ tăng đúng tại những mốc luỹ thừa
của `62`, không hề tăng đều.
::::

::::checkpoint{mastery=0.65}
::::
