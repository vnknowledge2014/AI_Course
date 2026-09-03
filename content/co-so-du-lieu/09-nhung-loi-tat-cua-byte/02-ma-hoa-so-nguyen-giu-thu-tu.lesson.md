---
id: co-so-du-lieu.nhung-loi-tat-cua-byte.ma-hoa-so-nguyen-giu-thu-tu
title: "Mã hoá số nguyên giữ thứ tự"
summary: "ma_hoa_so(n) trả về Vec<u8> ĐỘ RỘNG CỐ ĐỊNH (2 byte, byte cao trước): cao = n/256, thap = n%256. Với n trong [0, 65535], so sánh Vec<u8> < Vec<u8> PHẢN ÁNH đúng so sánh số — khác hẳn biểu diễn 'chuỗi chữ số' của bài trước. Bí quyết: ĐỘ RỘNG cố định (không phụ thuộc giá trị) + so byte CAO nhất trước."
locale: vi
track: co-so-du-lieu
module: nhung-loi-tat-cua-byte
order: 2
tier: A
languages: [rust]
defaultLanguage: rust
level: intro
estimatedMinutes: 10
teaches: [db.int-key-encoding]
requires: [db.ordered-key-why]
concepts: [db.int-key-encoding]
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
Biểu diễn "chuỗi chữ số" (bài trước) hỏng vì ĐỘ dài chuỗi thay đổi
theo giá trị. Nếu MỌI khoá đều có ĐÚNG cùng một độ dài — CỐ định,
không phụ thuộc số — chuyện gì xảy ra?
::::

::::explain{#do-rong-co-dinh}
`ma_hoa_so` mã hoá một `i64` (giả sử nằm trong [0, 65535]) thành
ĐÚNG hai byte — byte CAO (hàng 256) TRƯỚC, byte THẤP (phần dư) SAU:

```rust title=readonly
fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

fn main() {
    let a = ma_hoa_so(9);
    let b = ma_hoa_so(10);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a < b);
}
```

```text title=readonly
[0, 9]
[0, 10]
true
```

Với `9` VÀ `10`, byte cao ĐỀU LÀ `0` — so sánh `Vec<u8>` rơi xuống
byte THỨ hai, `9 < 10` ĐÚNG như mong đợi. KHÁC hẳn bài trước: ĐỘ dài
`Vec<u8>` LUÔN LÀ hai, bất kể `n` lớn nhỏ CỠ nào (trong phạm vi cho
phép) — không còn chuyện "chữ số ĐẦU quyết định, độ dài LỆCH nhau"
nữa.
::::

::::example{#so-lech-tram-va-nghin}
Số có byte CAO khác `0` — `300` so với `5`:

```rust title=readonly
let c = ma_hoa_so(300);
let d = ma_hoa_so(5);
println!("{:?}", c);
println!("{:?}", d);
println!("{}", c < d);
```

```text title=readonly
[1, 44]
[0, 5]
false
```

`300 // 256 = 1` (byte cao, chia LẤY phần nguyên), `300 % 256 = 44`
(byte thấp) → `[1,
44]`. `5` có byte cao `0` → `[0, 5]`. So `Vec<u8>`: byte ĐẦU `1 > 0`
— dừng NGAY ở đó, kết luận `[1,44] > [0,5]`, ĐÚNG với `300 > 5`. Byte
CAO đóng vai trò "chữ số hàng LỚN nhất" — SO nó trước LUÔN cho kết
quả đúng trước khi cần nhìn tới byte thấp.
::::

::::predict{#doan-hai-dau-mut commitOnce}
`ma_hoa_so(0)` VÀ `ma_hoa_so(65535)` — hai đầu MÚT của phạm vi hai
byte (`65535 = 255 * 256 + 255`). So sánh
`ma_hoa_so(0) < ma_hoa_so(65535)` cho kết quả gì?

:::opt{correct}
`true` — `[0, 0] < [255, 255]`, đúng thứ tự số `0 < 65535`
:::

:::opt
Không xác định được — `65535` VƯỢT quá một byte (`255` LÀ giá trị
LỚN nhất của `u8`) nên `ma_hoa_so` sẽ tràn số HOẶC panic
::why
Gần đúng ở việc bạn để Ý ĐÚNG rằng một byte chỉ chứa được `0..=255`
— MỘT quan sát chính XÁC về giới hạn của `u8`.

Chỗ lệch: `ma_hoa_so` không hề nhồi `65535` VÀO một byte DUY nhất —
nó TÁCH thành `65535 // 256 = 255` (byte cao) VÀ `65535 % 256 = 255`
(byte thấp), MỖI phần đều nằm gọn trong `0..=255`. HAI byte cùng
nhau biểu diễn được TỚI `65535` — đây chính LÀ lý do "độ rộng CỐ
định hai byte" giới hạn phạm vi mã hoá được Ở `[0, 65535]`, không
phải LÀ một giới hạn gây lỗi.
::
:::
::::

::::code{#viet_ma_hoa_so}
Hoàn thiện `ma_hoa_so` — tính byte THẤP (phần dư sau khi chia cho
`256`).

```rust title=starter
fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = ___;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

fn main() {
    let a = ma_hoa_so(9);
    let b = ma_hoa_so(10);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a < b);
}
```

```rust title=solution
fn ma_hoa_so(n: i64) -> Vec<u8> {
    let cao: u8 = (n / 256) as u8;
    let thap: u8 = (n % 256) as u8;
    let mut ra: Vec<u8> = Vec::new();
    ra.push(cao);
    ra.push(thap);
    ra
}

fn main() {
    let a = ma_hoa_so(9);
    let b = ma_hoa_so(10);
    println!("{:?}", a);
    println!("{:?}", b);
    println!("{}", a < b);
}
```

```rust title=test
fn main() {
    let a = ma_hoa_so(9);
    let b = ma_hoa_so(10);
    println!("{}", a < b);
    assert_eq!(a < b, true, "9 phai nho hon 10 sau khi ma hoa");

    let c = ma_hoa_so(300);
    let d = ma_hoa_so(5);
    assert_eq!(c < d, false, "300 khong duoc nho hon 5 sau khi ma hoa");

    let e = ma_hoa_so(0);
    let f = ma_hoa_so(65535);
    assert_eq!(e < f, true, "0 phai nho hon 65535 sau khi ma hoa");

    println!("tat ca test qua");
}
```

:::hints
- kind: attention
  body: "thap la phan du cua n khi chia cho 256 -- dung phep %, ep kieu ve u8, mot dong."
- kind: strategy
  body: "(n % 256) as u8"
- kind: one-line
  body: "let thap: u8 = (n % 256) as u8;"
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: contains
  expect: "true"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Khoá số ĐÃ mã hoá giữ đúng thứ tự. Nhưng khoá GHÉP từ HAI trường —
nối trực tiếp hai `Vec<char>` lại — có an TOÀN không?
::::

::::reflect{#nghi-lai}
Bí quyết của `ma_hoa_so` LÀ hai điều CỘNG lại: ĐỘ rộng CỐ định (LUÔN
đúng hai byte, không phụ thuộc `n`) VÀ so byte có TRỌNG số LỚN nhất
(byte cao) TRƯỚC. Cách MÃ hoá này CHỈ đúng trong phạm vi đã CHỌN
([0, 65535] cho hai byte) — số ÂM hoặc số lớn HƠN cần độ rộng khác
(hoặc kỹ thuật lật bit dấu, ngoài phạm vi bài NÀY). Một khoá SỐ đã
mã hoá được — khoá GHÉP từ nhiều trường (tên bảng + tên cột, VÍ dụ)
thì SAO?
::::

::::checkpoint{mastery=0.8}
::::
