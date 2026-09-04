---
id: thiet-ke-he-thong.url-rut-gon-va-thu-thap-web.base62-giai-ma
title: "Base62: giải mã chuỗi về số nguyên"
summary: "giaiMaBase62(ma) đảo ngược maHoaBase62 (bài trước): duyệt từng ký tự TỪ TRÁI sang phải, mỗi bước nhân kết quả tích luỹ với 62 rồi cộng vị trí (indexOf) của ký tự trong bảng — y hệt cách đọc số thập phân từ trái sang phải. Roundtrip đúng trên mọi n đã thử (0, 1, 61, 62, 125, 3843, 3844, 999999). Vì bảng phân biệt hoa/thường, giaiMaBase62(\"aA\")=656 KHÁC giaiMaBase62(\"Aa\")=2242 — thứ tự ký tự quyết định giá trị, giống hệ thập phân \"21\" khác \"12\"."
locale: vi
track: thiet-ke-he-thong
module: url-rut-gon-va-thu-thap-web
order: 2
tier: A
languages: [typescript]
defaultLanguage: typescript
level: intro
estimatedMinutes: 8
teaches: [sd.base62-giai-ma]
requires: [sd.base62-ma-hoa]
concepts: [sd.base62-giai-ma]
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
Bài trước mã hoá số nguyên thành chuỗi. Một dịch vụ rút gọn URL còn
cần chiều NGƯỢC lại: ai đó gõ `http://vidu.com/21`, hệ thống PHẢI
đoán lại đúng số nguyên `125` để tra ra URL gốc.
::::

::::explain{#giai-ma-base62}
Giải mã đi NGƯỢC đúng thứ tự mã hoá: thay VÌ chia liên tục, ta duyệt
chuỗi TỪ TRÁI sang phải, mỗi bước NHÂN kết quả tích luỹ VỚI `62` rồi
CỘNG giá trị (vị trí trong bảng) của ký tự hiện tại — giống hệt cách
đọc một số thập phân TỪ trái sang phải (`"21"` = `2*10 + 1`):

```typescript title=readonly
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function giaiMaBase62(ma: string): number {
  let ketQua = 0;
  for (const kyTu of ma) {
    const giaTri = BANG_BASE62.indexOf(kyTu);
    ketQua = ketQua * 62 + giaTri;
  }
  return ketQua;
}

console.log('giaiMaBase62("0") =', giaiMaBase62("0"));
console.log('giaiMaBase62("Z") =', giaiMaBase62("Z"));
console.log('giaiMaBase62("10") =', giaiMaBase62("10"));
console.log('giaiMaBase62("21") =', giaiMaBase62("21"));
```

```text title=readonly
giaiMaBase62("0") = 0
giaiMaBase62("Z") = 61
giaiMaBase62("10") = 62
giaiMaBase62("21") = 125
```

Chuỗi `"10"` giải mã ra `62` — ĐÚNG như bài TRƯỚC đã mã hoá `62`
thành `"10"`. Vòng lặp `for...of` xử LÝ `"1"` trước (`ketQua = 0*62 +
1 = 1`), rồi `"0"` (`ketQua = 1*62 + 0 = 62`) — mỗi ký tự MỚI "đẩy"
kết quả cũ sang trái đúng MỘT bậc base62, giống hệt cách thêm một chữ
số VÀO cuối một số thập phân.
::::

::::example{#roundtrip-va-phan-biet-hoa-thuong}
Mã hoá RỒI giải mã một số bất KỲ phải luôn trả về đúng số BAN đầu —
VÀ vì bảng base62 chứa CẢ chữ hoa lẫn chữ thường như hai KÝ hiệu tách
biệt, thứ tự VÀ hoa/thường trong chuỗi đều mang Ý nghĩa:

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

function giaiMaBase62(ma: string): number {
  let ketQua = 0;
  for (const kyTu of ma) {
    const giaTri = BANG_BASE62.indexOf(kyTu);
    ketQua = ketQua * 62 + giaTri;
  }
  return ketQua;
}

const cacSo = [0, 1, 61, 62, 125, 3843, 3844, 999999];
for (const n of cacSo) {
  const ma = maHoaBase62(n);
  const giaiMa = giaiMaBase62(ma);
  console.log(`${n} -> "${ma}" -> ${giaiMa} (khop: ${giaiMa === n})`);
}

console.log('giaiMaBase62("aA") =', giaiMaBase62("aA"));
console.log('giaiMaBase62("Aa") =', giaiMaBase62("Aa"));
```

```text title=readonly
0 -> "0" -> 0 (khop: true)
1 -> "1" -> 1 (khop: true)
61 -> "Z" -> 61 (khop: true)
62 -> "10" -> 62 (khop: true)
125 -> "21" -> 125 (khop: true)
3843 -> "ZZ" -> 3843 (khop: true)
3844 -> "100" -> 3844 (khop: true)
999999 -> "4c91" -> 999999 (khop: true)
giaiMaBase62("aA") = 656
giaiMaBase62("Aa") = 2242
```

Mọi cặp mã-hoá-rồi-giải-mã đều KHỚP lại đúng số ban ĐẦU. VÀ
`"aA"` (`656`) hoàn toàn KHÁC `"Aa"` (`2242`) — dù dùng đúng HAI ký
tự giống nhau, chỉ ĐỔI chỗ, giá trị đổi HẲN, y hệt cách `"21"` khác
`"12"` trong hệ thập phân.
::::

::::predict{#doan-chuoi-rong commitOnce}
`giaiMaBase62` nhận VÀO một chuỗi RỖNG (`""`) — vòng lặp `for...of`
không chạy LẦN nào. Hàm trả VỀ giá trị gì?

:::opt{correct}
`0` — `ketQua` được khởi tạo BẰNG `0` VÀ không hề bị thay đổi vì vòng
lặp không có ký tự NÀO để duyệt qua
:::
:::opt
`NaN` — một chuỗi RỖNG không tương ứng VỚI số nguyên nào cả, nên phép
tính PHẢI cho ra một giá trị "không phải số"
::why
Nhầm "không có ĐẦU vào hợp lệ về mặt Ý nghĩa" VỚI "hàm PHẢI tạo ra
`NaN`" — nhưng `giaiMaBase62` không hề có phép toán nào SINH ra
`NaN` khi chuỗi rỗng.

Chỗ lệch: `NaN` chỉ xuất hiện khi có phép TÍNH trên giá trị không
phải số (như `undefined * 62`). Ở ĐÂY, vòng lặp `for (const kyTu of
ma)` với `ma = ""` đơn giản KHÔNG chạy thân vòng lặp lần NÀO —
`ketQua` giữ nguyên giá trị khởi tạo LÀ `0`, VÀ hàm trả về đúng `0`
đó, không hề chạm tới phép nhân/cộng NÀO cả.
::
:::
::::

::::code{#viet_giai_ma_base62}
Hoàn thiện `giaiMaBase62` — sau khi TRA ra vị trí của ký tự hiện tại
trong bảng, cập nhật kết quả tích luỹ (nhân `62` rồi cộng).

```typescript title=starter
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function giaiMaBase62(ma: string): number {
  let ketQua = 0;
  for (const kyTu of ma) {
    const giaTri = BANG_BASE62.indexOf(kyTu);
    ___
  }
  return ketQua;
}

console.log(giaiMaBase62("21"));
```

```typescript title=solution
const BANG_BASE62 = "0123456789abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

function giaiMaBase62(ma: string): number {
  let ketQua = 0;
  for (const kyTu of ma) {
    const giaTri = BANG_BASE62.indexOf(kyTu);
    ketQua = ketQua * 62 + giaTri;
  }
  return ketQua;
}

console.log(giaiMaBase62("21"));
```

```typescript title=test
if (giaiMaBase62("0") !== 0) throw new Error('"0" phai giai ma thanh 0');
if (giaiMaBase62("Z") !== 61) throw new Error('"Z" (ky tu cuoi bang) phai giai ma thanh 61');
if (giaiMaBase62("10") !== 62) throw new Error('"10" phai giai ma thanh 62');
if (giaiMaBase62("21") !== 125) throw new Error('"21" phai giai ma thanh 125');
if (giaiMaBase62("aA") !== 656) throw new Error('"aA" phai giai ma thanh 656 (10*62+36)');
if (giaiMaBase62("Aa") !== 2242) throw new Error('"Aa" phai giai ma thanh 2242 (36*62+10), khac "aA"');
if (giaiMaBase62("") !== 0) throw new Error('chuoi rong phai giai ma thanh 0');
```

:::hints
- kind: attention
  body: "Moi ky tu MOI 'day' ketQua sang trai dung mot bac base62 (nhan 62), roi cong them vi tri cua ky tu hien tai."
- kind: strategy
  body: "ketQua = ketQua * 62 + giaTri;"
- kind: one-line
  body: "ketQua = ketQua * 62 + giaTri;"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "125"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Mã hoá VÀ giải mã đã khớp nhau hoàn hảo. Giờ CẦN một nguồn số nguyên
để nuôi cả HAI hàm này — một bộ đếm không bao giờ cấp trùng.
::::

::::reflect{#nghi-lai}
`giaiMaBase62` VÀ `maHoaBase62` LÀ một cặp phép biến đổi NGƯỢC nhau
hoàn hảo — mỗi số nguyên ứng VỚI đúng một chuỗi, VÀ ngược lại. Tính
CHẤT "song ánh" NÀY (mỗi giá trị chỉ tương ứng đúng một giá trị KHÁC)
chính LÀ điều sẽ làm bài SAU trở nên đơn giản: một bộ đếm KHÔNG BAO
GIỜ cần kiểm tra trùng lặp.
::::

::::checkpoint{mastery=0.67}
::::
