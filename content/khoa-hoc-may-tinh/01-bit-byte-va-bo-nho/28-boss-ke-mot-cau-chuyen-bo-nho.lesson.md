---
id: khoa-hoc-may-tinh.bit-byte-va-bo-nho.boss-ke-mot-cau-chuyen-bo-nho
title: "BOSS — Kể lại đời một giá trị"
summary: "Ghép mọi con số của chặng vừa qua thành một câu chuyện: một giá trị sinh ra ở đâu, mấy tấm thẻ trỏ tới, chiếm mấy byte, và lúc nào không ai với tới nữa."
locale: vi
track: khoa-hoc-may-tinh
module: bit-byte-va-bo-nho
order: 28
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [mem.lifetime-story]
requires: [mem.memory-strategies, mem.utf8-bytes, mem.float-compare]
concepts: [mem.doi-mot-gia-tri, mem.dem-the, mem.chien-luoc-don-dep]
gradingMatrix:
  web-chrome: [static, run, tests, output]
  web-firefox: [static, run, tests, output]
  macos: [static, run, tests, output]
  windows: [static, run, tests, output]
  linux: [static, run, tests, output]
  android: [static, run, tests, output]
  ios: [static, run, tests, output]
provenance:
  authoredBy: llm-assisted
  reviewed: false
---

::::byte{trigger=enter mood=curious pose=lean-in}
Hai mươi bảy bài để tới đây. Giờ ghép hết lại: kể trọn đời một giá trị,
bằng chính chương trình của bạn.
::::

::::explain{#doi-mot-gia-tri-co-may-doan}
Suốt chặng vừa qua, mỗi bài mở một mảnh khác nhau của cùng một câu chuyện —
**đời một giá trị** trong bộ nhớ. Ghép lại, câu chuyện đó có bốn đoạn:

1. **Sinh ra** — một giá trị xuất hiện, đúng vào dòng nó được gán cho một
   cái tên.
2. **Chiếm chỗ** — nó nằm gọn trong một số byte cụ thể. Với chữ, T3.1 bài
   18 đã chỉ ra: mã hoá UTF-8 rồi đếm số byte thật, không đếm số ký tự —
   một chữ tiếng Việt có dấu có thể tốn tới ba byte.
3. **Được trỏ tới** — một hay nhiều tấm thẻ buộc vào nó. Bài
   `khi-khong-con-the-nao-tro-toi` cho hỏi con số đó, và dặn chỉ tin vào
   HIỆU SỐ giữa hai lần hỏi, không tin con số tuyệt đối.
4. **Không ai với tới nữa** — tấm thẻ cuối cùng rời đi. Đếm thẻ dọn được
   ngay, trừ khi giá trị đó mắc trong một vòng tham chiếu — lúc đó phải chờ
   bộ dọn theo vòng, như hai bài trước vừa thấy.

Riêng phép so cân nặng trong bài này còn mượn thêm một mảnh nữa, từ trước
cả chặng này: bài `hai-so-float-bang-nhau-khong` dặn đừng hỏi hai số lẻ có
bằng nhau bằng `==`, mà hỏi "chênh nhau ít hơn một ngưỡng" — vì phép cộng
số lẻ trong máy hiếm khi ra đúng con số trên giấy.

Bốn đoạn cộng một mảnh mượn — đó là toàn bộ nguyên liệu để kể một đời giá
trị. Bài này ghép chúng thành một chương trình duy nhất.
::::

::::example{#doi-mot-cong-rau}
Cô Bảy ghi lại "lý lịch" ngắn cho một mớ rau vừa hái, trước khi đem lên bàn
cân:

```python title=readonly
import sys

ten_rau = "rau ngổ"
so_byte = len(ten_rau.encode("utf-8"))

can_1 = 0.1
can_2 = 0.2
tong_can = can_1 + can_2
chuan = 0.3
nguong = 1e-9
khop_chuan = abs(tong_can - chuan) < nguong

truoc = sys.getrefcount(ten_rau)
so_theo_doi = ten_rau          # thêm một thẻ để tiện xem lại
giua = sys.getrefcount(ten_rau)
so_theo_doi = None              # xong việc, gỡ thẻ đó ra
sau = sys.getrefcount(ten_rau)

print(f"'{ten_rau}' sinh ra, chiếm {so_byte} byte khi mã hoá UTF-8.")
print(f"Cân nặng đo được khớp chuẩn trong ngưỡng sai số: {khop_chuan}.")
print(f"Từng có thêm {giua - truoc} tấm thẻ trỏ vào nó, rồi trở lại {sau - truoc}.")
```

```text title=readonly
'rau ngổ' sinh ra, chiếm 9 byte khi mã hoá UTF-8.
Cân nặng đo được khớp chuẩn trong ngưỡng sai số: True.
Từng có thêm 1 tấm thẻ trỏ vào nó, rồi trở lại 0.
```

Ba dòng in ra, ba đoạn của câu chuyện:

- `"rau ngổ"` dài 7 ký tự, nhưng **9 byte** — chữ `ổ` mang cả dấu mũ lẫn
  dấu hỏi, tốn ba byte một mình nó, y hệt chữ `ở` mà bài 18 từng đếm.
- `0.1 + 0.2` không ra đúng `0.3` trên máy — nhưng chênh lệch đó nhỏ hơn
  `1e-9` rất nhiều, nên so bằng ngưỡng thì khớp. So bằng `==` thẳng thì đã
  ra `False`, sai một cách oan uổng.
- Thêm một thẻ, đếm thẻ tăng đúng một; gỡ thẻ, nó về lại đúng chỗ cũ. Không
  có vòng tham chiếu nào ở đây, nên đếm thẻ tự lo trọn vẹn — không cần gọi
  tới bộ dọn theo vòng của hai bài trước.
::::

::::predict{#doan-hai-dong-can commitOnce}
Byte đổi sang một cặp cân nặng khác — cố ý chọn để tổng bị LỆCH XUỐNG dưới
chuẩn, chứ không lệch lên như ví dụ trên:

```python title=readonly
can_1 = 0.7
can_2 = 0.1
tong_can = can_1 + can_2
chuan = 0.8
nguong = 1e-9

print(tong_can == chuan)
print(abs(tong_can - chuan) < nguong)
```

**Trước khi xem đáp án**, bạn đoán hai dòng in ra là gì?

:::opt{correct}
False, rồi True.
:::

:::opt
True, rồi True.
::why
Gần đúng ở phép tính trên giấy: `0.7 + 0.1` đúng là bằng `0.8`, không có gì
sai khi cộng bằng tay.

Chỗ lệch: máy không lưu `0.7` hay `0.1` đúng y hệt như trên giấy — cả hai
đều là số lẻ trong hệ hai, giống hệt chuyện `0.1 + 0.2` ở ví dụ trên. Tổng
tính ra hơi khác `0.8` một chút xíu, đủ để `==` (so khớp tuyệt đối) trả về
`False`, dù mắt người nhìn thấy chúng "bằng nhau".
::
:::

:::opt
False, rồi False.
::why
Gần đúng ở dòng đầu: bạn đúng khi đoán phép cộng không ra chính xác `0.8`,
nên `==` cho `False` — điều đó khớp với ví dụ trên.

Chỗ lệch nằm ở dòng thứ hai. Phần chênh lệch do số lẻ gây ra cực kỳ nhỏ —
nhỏ hơn một phần triệu tỷ — trong khi `nguong` ở đây là `1e-9`, lớn hơn
phần chênh đó rất nhiều. Ngưỡng đủ rộng để "nuốt" trọn phần lệch nhỏ xíu ấy,
nên phép so bằng ngưỡng vẫn cho `True`.
::
:::

:::opt
True, rồi False.
::why
Gần đúng ở việc bạn tách hai câu hỏi ra để trả lời riêng — đó đúng là điều
bài này muốn: `==` và so-bằng-ngưỡng là hai câu hỏi khác nhau, có thể ra
hai đáp án khác nhau.

Chỗ lệch là kết quả bị đảo ngược so với thực tế: `==` mới là câu trả lời
`False` (vì số lẻ không khớp tuyệt đối), còn so-bằng-ngưỡng mới là câu trả
lời `True` (vì phần chênh quá nhỏ so với `nguong`).
::
:::
::::

::::code{#viet-doi-mot-gia-tri}
Đến lượt bạn kể chuyện. Cô Bảy vừa hái một mớ `"rau muống"`, cân được hai
lần: `0.1` kg rồi `0.2` kg, so với chuẩn `0.3` kg. Điền ba chỗ trống để kể
đủ bốn đoạn: chiếm mấy byte, cân có khớp chuẩn không, và số thẻ trỏ vào nó
tăng giảm ra sao.

```python title=starter
import sys

ten_rau = "rau muống"
so_byte = len(___)                       # mã hoá ten_rau sang UTF-8 rồi đếm byte

can_1 = 0.1
can_2 = 0.2
tong_can = can_1 + can_2
chuan = 0.3
nguong = 1e-9
khop_chuan = ___                          # so tong_can với chuan, lệch dưới nguong

truoc = sys.getrefcount(ten_rau)
ten_phu = ten_rau
giua = sys.getrefcount(ten_rau)
ten_phu = ___                             # gỡ tấm thẻ ten_phu ra khỏi ten_rau
sau = sys.getrefcount(ten_rau)

print(f"'{ten_rau}' chiếm {so_byte} byte khi mã hoá UTF-8.")
print(f"Cân nặng đo được khớp chuẩn trong ngưỡng sai số: {khop_chuan}.")
print(f"Từng có thêm {giua - truoc} tấm thẻ trỏ vào nó, rồi trở lại {sau - truoc}.")
```

```python title=solution
import sys

ten_rau = "rau muống"
so_byte = len(ten_rau.encode("utf-8"))

can_1 = 0.1
can_2 = 0.2
tong_can = can_1 + can_2
chuan = 0.3
nguong = 1e-9
khop_chuan = abs(tong_can - chuan) < nguong

truoc = sys.getrefcount(ten_rau)
ten_phu = ten_rau
giua = sys.getrefcount(ten_rau)
ten_phu = None
sau = sys.getrefcount(ten_rau)

print(f"'{ten_rau}' chiếm {so_byte} byte khi mã hoá UTF-8.")
print(f"Cân nặng đo được khớp chuẩn trong ngưỡng sai số: {khop_chuan}.")
print(f"Từng có thêm {giua - truoc} tấm thẻ trỏ vào nó, rồi trở lại {sau - truoc}.")
```

```python title=test
# Chỗ trống thứ nhất phải ĐẾM BYTE thật, không phải đếm ký tự. "rau muống"
# dài 9 ký tự nhưng 11 byte, vì chữ "ố" mang hai dấu, tốn ba byte một mình.
assert so_byte == 11, f"'rau muống' phải chiếm 11 byte khi mã hoá UTF-8 (9 ký tự, nhưng 'ố' tốn ba byte) — đang ra {so_byte}; kiểm lại chỗ trống có mã hoá ten_rau trước khi đếm không"
# Chỗ trống thứ hai phải so bằng NGƯỠNG, không phải so bằng ==. 0.1 + 0.2
# không ra đúng 0.3 trên máy, nên == cho False; đúng ngưỡng thì cho True.
assert khop_chuan is True, "0.1 + 0.2 không khớp tuyệt đối với 0.3 trên máy (thử == sẽ ra False) — chỗ trống thứ hai phải so CHÊNH LỆCH với nguong, không phải so bằng =="
# Chỗ trống thứ ba phải thật sự gỡ ten_phu ra khỏi ten_rau.
assert giua - truoc == 1, "trước khi tới chỗ trống thứ ba, ten_phu đã được gán vào CÙNG ten_rau — nên refcount phải tăng đúng 1 ở đây"
assert sau - truoc == 0, f"sau chỗ trống thứ ba, refcount của ten_rau phải quay lại đúng mức ban đầu — đang lệch {sau - truoc}, nghĩa là ten_phu chưa thật sự rời khỏi ten_rau"
assert ten_rau == "rau muống", "ten_rau là thứ chỉ đọc trong bài này, không được đổi"
```

:::hints
- kind: attention
  body: Ba chỗ trống ứng với ba việc riêng biệt — đếm byte, so cân nặng, gỡ tấm thẻ. Đừng để việc này lẫn sang việc kia; mỗi chỗ chỉ đòi đúng MỘT biểu thức.
- kind: strategy
  body: 'Chỗ trống 1 dùng đúng công cụ bài 18 đã dạy để đổi chữ ra byte. Chỗ trống 2 dùng đúng dạng "khoảng cách nhỏ hơn ngưỡng" mà bài về so sánh số lẻ đã dạy, không phải `==`. Chỗ trống 3 làm đúng việc chị Hạnh làm ở R0 bài 12 — gán `ten_phu` sang một giá trị không liên quan gì tới `ten_rau` nữa.'
- kind: one-line
  body: 'Ba chỗ trống lần lượt là `ten_rau.encode("utf-8")`, `abs(tong_can - chuan) < nguong`, và `None`.'
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: static
  onFail: chỗ trống thứ nhất phải MÃ HOÁ ten_rau (dùng .encode) trước khi đếm độ dài — đếm thẳng len(ten_rau) chỉ ra số KÝ TỰ, không phải số byte; chỗ trống thứ hai phải dùng abs(...) để so KHOẢNG CÁCH, không so bằng ==
  requireAst:
  - kind: uses-call, target: encode, min: 1
  - kind: uses-call, target: abs, min: 1
- tier: output
  match: regex
  expect: "^'rau muống' chiếm 11 byte khi mã hoá UTF-8\\.\\nCân nặng đo được khớp chuẩn trong ngưỡng sai số: True\\.\\nTừng có thêm 1 tấm thẻ trỏ vào nó, rồi trở lại 0\\.\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sinh ra, chiếm chỗ, được trỏ tới, rồi không ai với tới nữa. Bốn đoạn, một
chương trình, và bạn vừa kể trọn.
::::

::::explain{#truoc-khi-chay-trinh-dich-da-ke-truoc}
Một điều đáng dừng lại, dù chưa học Rust — chỉ nhìn ý tưởng cuối cùng này.

Chương trình bạn vừa viết kể câu chuyện đó bằng cách CHẠY THẬT: gọi
`sys.getrefcount`, thật sự cộng `0.1` với `0.2`, thật sự mã hoá một chuỗi ra
byte. Không chạy thì không biết được đáp án.

```rust title=readonly
// Ý TƯỞNG, chưa phải cú pháp thật đã dạy:
// Rust không cần CHẠY để biết một giá trị còn được trỏ tới hay không.
// Trình dịch tự đọc mã nguồn, tự tính ai là chủ của mỗi giá trị, và tự biết
// CHÍNH XÁC dòng nào là dòng cuối cùng nó còn được dùng — tất cả TRƯỚC khi
// chương trình chạy lấy một bước nào.
```

Cùng bốn đoạn — sinh ra, chiếm chỗ, được trỏ tới, hết ai với tới — nhưng
Rust cho trình dịch kể trước, không phải chương trình kể trong lúc chạy. Đó
là câu hỏi mà cả track này dựng lên từ đầu: một cái tên GIỮ giá trị, hay nó
chỉ CHỈ TỚI giá trị? Từ bài 20 tới đây, câu trả lời luôn là Python đứng ở vế
sau — chỉ CHỈ TỚI, và đếm thẻ để biết khi nào thôi. Rust bắt bạn trả lời
đúng câu hỏi ấy một lần, tường minh, ngay trong mã nguồn — trước khi có
dòng nào được chạy.
::::

::::reflect{#nghi-lai}
Một câu hỏi cuối, khép lại cả track này.

Hai mươi tám bài, đi từ một cái ô chỉ chứa 0 hoặc 1, qua số âm, số lẻ,
chữ tiếng Việt, tới một cái tên chỉ CHỈ TỚI giá trị chứ không GIỮ nó — và
hôm nay, một chương trình tự kể được đời của chính giá trị nó đang cầm.

Nhưng mọi phép đo trong track này đều cần CHẠY chương trình lên mới biết
đáp án. Bạn vừa thấy Rust hứa một điều khác hẳn: kể được câu chuyện ấy TRƯỚC
khi chạy, ngay lúc biên dịch.

Muốn trình dịch làm được điều đó, nó phải hỏi bạn một câu mà Python chưa
từng bắt buộc phải trả lời: với MỖI giá trị trong chương trình, ai là chủ
của nó — và chỉ một chủ, hay chia sẻ được?

Đó là câu hỏi mở đầu cho Rust.
::::

::::checkpoint{mastery=0.85}
::::
