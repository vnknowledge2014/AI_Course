---
id: khoa-hoc-may-tinh.cau-truc-du-lieu.chuoi-cung-la-mot-mang
title: "Chuỗi cũng là một mảng, nhưng khoá cứng"
summary: "Chuỗi cũng là một mảng — mảng ký tự khoá cứng, không sửa tại chỗ được; và với tiếng Việt, chỉ số Python đếm theo ký tự chứ không theo byte."
locale: vi
track: khoa-hoc-may-tinh
module: cau-truc-du-lieu
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [ds.string-as-array]
requires: [ds.array-contiguous, core.string-immutable, mem.utf8-bytes, core.string-slice, core.string-concat, core.len, core.fstring, err.type-error, err.try-except]
concepts: [ds.string-as-array]
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
Mỗi câu bạn gõ cũng là một mảng — chỉ là một mảng bị khoá cứng, không sửa
tại chỗ được.
::::

::::explain{#chuoi-la-mang-khoa-cung}
R1 đã nói một điều bạn học thuộc lòng từ đó: chuỗi (`str`) trong Python
KHÔNG sửa được — `core.string-immutable`. Bài này nói VÌ SAO, bằng đúng
ngôn ngữ track này vừa dựng: một chuỗi cũng là một MẢNG — một dãy ký tự
liền kề, đánh số từ 0 (bài 1) — chỉ khác `list` ở một điểm: nó bị KHOÁ
CỨNG ngay từ lúc sinh ra.

Với `list`, bạn ghi đè một ô bất kỳ: `a[0] = 99`. Với `str`, Python cấm
tuyệt đối thao tác đó — không phải vì thiếu cú pháp, mà vì cố ý khoá: một
chuỗi, một khi đã dựng, không ô nào trong nó được phép đổi. Muốn có một
chuỗi "đã sửa", bạn buộc phải dựng một mảng HOÀN TOÀN MỚI, chép những phần
giữ nguyên, ghép thêm phần đã đổi — đúng việc `mon_moi = "b" + mon[1:]`
từng làm ở nhiều bài R1, giờ nhìn lại dưới ánh sáng "mảng khoá cứng".

Và chuỗi tiếng Việt còn thêm một lớp phải cẩn thận. T3.1 bài 18 đã đo:
chữ `ở` mã hoá UTF-8 tốn **3 byte**, không phải 1. Nhưng khi bạn viết
`s[2]` trên một chuỗi Python, con số 2 đó đếm theo KÝ TỰ — "ở" vẫn là MỘT
ký tự, dù nó nặng ba byte khi mã hoá. Chỉ số Python không hề biết, và
không cần biết, một ký tự nặng bao nhiêu byte. Hai phép đếm này — đếm ký
tự (`len(s)`, `s[i]`) và đếm byte (`len(s.encode("utf-8"))`) — cho ra hai
con số khác hẳn nhau trên cùng một chuỗi tiếng Việt, và lẫn lộn giữa chúng
là lỗi rất dễ mắc.
::::

::::example{#dem-ky-tu-va-dem-byte}
```python title=readonly
mon = "phở"
print(len(mon))
print(mon[2])

ma_byte = mon.encode("utf-8")
print(len(ma_byte))
print(ma_byte)
```

```text title=readonly
3
ở
5
b'ph\xe1\xbb\x9f'
```

`"phở"` có 3 KÝ TỰ — `p`, `h`, `ở` — và `mon[2]` lấy đúng ký tự thứ ba,
`ở`, nguyên vẹn một khối, không phải một byte lẻ của nó. Nhưng mã hoá sang
UTF-8 thì dài tới 5 BYTE: `p` và `h` mỗi chữ 1 byte, còn `ở` một mình
chiếm 3 byte (đúng con số T3.1 bài 18 đã đo) — cộng lại: 1 + 1 + 3 = 5.

Giờ thử sửa một ký tự tại chỗ:

```python title=readonly
mon = "phở"
try:
    mon[0] = "b"
except TypeError as loi:
    print("Lỗi:", loi)

mon_moi = "b" + mon[1:]
print(mon_moi)
print(mon_moi is mon)
```

```text title=readonly
Lỗi: 'str' object does not support item assignment
bhở
False
```

`mon[0] = "b"` bị chặn ngay lập tức — thông báo nói thẳng: kiểu `str`
KHÔNG hỗ trợ gán vào một ô của nó. Muốn có "bhở", cách duy nhất là ghép
`"b"` với phần còn lại của `mon` (`mon[1:]`, tức "hở") thành một chuỗi
MỚI. `mon_moi is mon` cho `False` — hai mảng hoàn toàn khác nhau trong bộ
nhớ, một cái cũ giữ nguyên, một cái mới vừa dựng.
::::

::::predict{#doan-com-thanh-cam commitOnce}
Byte sửa một chữ khác — từ "cơm" thành "căm" — bằng đúng cách dựng chuỗi
mới vừa xem:

```python
mon_1 = "cơm"
mon_2 = mon_1[:1] + "ă" + mon_1[2:]

print(mon_1)
print(mon_2)
print(mon_1 is mon_2)
```

**Trước khi chạy**, bạn đoán ba dòng in ra là gì — đặc biệt chú ý dòng
đầu: `mon_1` có bị đổi theo không?

:::opt{correct}
`cơm`, rồi `căm`, rồi `False`
:::

:::opt
`cơm`, rồi `căm`, rồi `True` — vì `mon_2` chỉ sửa một ký tự của `mon_1`
nên chúng vẫn là cùng một chuỗi.
::why
Gần đúng ở việc bạn đoán đúng NỘI DUNG của cả hai dòng in đầu — "cơm" rồi
"căm" đúng là những gì hiện ra.

Chỗ lệch là bạn nghĩ "sửa một ký tự" có nghĩa là mảng CŨ bị đổi tại chỗ.
Chuỗi bị khoá cứng — không ô nào trong `mon_1` từng bị đụng tới. Biểu thức
`mon_1[:1] + "ă" + mon_1[2:]` luôn dựng ra một mảng HOÀN TOÀN MỚI, gán cho
`mon_2`; `mon_1` không hề hay biết chuyện đó.
::
:::

:::opt
`căm`, rồi `căm`, rồi `True` — vì lệnh gán `mon_2 = ...` đã ĐÈ luôn giá
trị của `mon_1`.
::why
Gần đúng ở việc bạn nhớ đúng "căm" là kết quả của phép ghép — con số đó
không sai.

Chỗ lệch là gán cho `mon_2` không hề đụng tới `mon_1`. Hai cái tên khác
nhau, `mon_1` vẫn giữ nguyên chuỗi cũ của nó suốt từ đầu tới cuối đoạn mã
này — không có "đè" nào xảy ra qua biên giới hai cái tên khác nhau.
::
:::

:::opt
`cơm`, rồi `cơm`, rồi `True` — vì `mon_1` và `mon_2` cùng trỏ một chuỗi
"cơm" chưa đổi.
::why
Gần đúng ở việc `mon_1` đúng là giữ nguyên "cơm" — dòng đầu bạn đoán không
sai.

Chỗ lệch là dòng thứ hai. Biểu thức gán cho `mon_2` có thật sự CHẠY —
`mon_1[:1]` lấy "c", cộng "ă", cộng `mon_1[2:]` (là "m"), ra "căm" hẳn
hoi. `mon_2` không "trỏ về" chuỗi cũ chưa đổi; nó là kết quả THẬT của phép
ghép vừa viết.
::
:::
::::

::::code{#go-thieu-dau}
Byte gõ vội trên bàn phím không dấu, quên mất dấu của món ăn — gõ ra
`"pho"` thay vì `"phở"`. Bạn cần: đếm xem chuỗi gõ vội nặng bao nhiêu byte,
sửa lại đúng chữ cuối bằng cách dựng một chuỗi MỚI (không sửa tại chỗ —
không sửa được), rồi đếm lại xem chuỗi đã sửa nặng bao nhiêu byte.

```python title=starter
mon = "pho"

so_ky_tu = len(mon)
so_byte_truoc = len(mon.encode("utf-8"))

mon_dung = ___                    # thay ký tự cuối (chỉ số 2) bằng "ở"

so_byte_sau = len(___)            # mã hoá mon_dung sang UTF-8 rồi đếm byte

print(f"'{mon}' có {so_ky_tu} ký tự, nặng {so_byte_truoc} byte")
print(f"Sửa lại: '{mon_dung}', vẫn {len(mon_dung)} ký tự, nhưng nặng {so_byte_sau} byte")
print(mon)
```

```python title=solution
mon = "pho"

so_ky_tu = len(mon)
so_byte_truoc = len(mon.encode("utf-8"))

mon_dung = mon[:2] + "ở"

so_byte_sau = len(mon_dung.encode("utf-8"))

print(f"'{mon}' có {so_ky_tu} ký tự, nặng {so_byte_truoc} byte")
print(f"Sửa lại: '{mon_dung}', vẫn {len(mon_dung)} ký tự, nhưng nặng {so_byte_sau} byte")
print(mon)
```

```python title=test
assert mon == "pho", "mon là bản gõ vội gốc — chỉ ĐỌC trong bài này, đừng sửa dòng gán đầu tiên"
assert so_ky_tu == 3, "'pho' có đúng 3 ký tự"
assert so_byte_truoc == 3, "'pho' toàn chữ không dấu — mỗi ký tự đúng 1 byte, tổng 3 byte"
assert mon_dung == "phở", f"mon_dung phải là 'phở' — đang ra {mon_dung!r}"
assert so_byte_sau == 5, f"'phở' phải nặng 5 byte khi mã hoá UTF-8 (p=1, h=1, ở=3 byte) — đang ra {so_byte_sau}"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất phải dựng ra chuỗi MỚI "phở" từ chuỗi gốc "pho" — chuỗi không sửa tại chỗ được, nên gán thẳng mon[2] không dùng được ở đây.
- kind: strategy
  body: 'Chỗ trống thứ nhất: giữ lại hai ký tự đầu của mon bằng mon[:2] (ra "ph"), rồi ghép thêm "ở" — đúng lối "b" + mon[1:] mà ví dụ phía trên đã làm. Chỗ trống thứ hai: mon_dung đã là chuỗi ĐÚNG rồi, chỉ cần .encode("utf-8") nó như dòng so_byte_truoc đã làm với mon.'
- kind: one-line
  body: 'Hai chỗ trống lần lượt là `mon[:2] + "ở"` và `mon_dung.encode("utf-8")`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: cả hai chỗ trống phải THẬT SỰ dùng dữ liệu từ mon (chỗ trống 1) và mon_dung (chỗ trống 2) để tính ra kết quả — không được gõ thẳng chuỗi "phở" hay con số 5 đã biết trước; bài này đang dạy cách DỰNG một mảng mới từ mảng cũ, không phải chép đáp án
  requireAst:
  - kind: uses-name, target: mon, min: 5
  - kind: uses-name, target: mon_dung, min: 3
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: "^'pho' có 3 ký tự, nặng 3 byte\\nSửa lại: 'phở', vẫn 3 ký tự, nhưng nặng 5 byte\\npho\\s*$"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Cùng ba ký tự, nhưng nặng khác hẳn nhau — 3 byte rồi 5 byte. Và `mon` gốc
vẫn y nguyên "pho", chưa ai đụng vào nó cả.
::::

::::reflect{#nghi-lai}
Một câu hỏi khép lại cả sáu bài vừa qua.

Bài 4 và bài 5 cho thấy: chèn hay xoá ở GIỮA một mảng luôn kéo theo cả một
dây chuyền dời chỗ — ít hay nhiều tuỳ có bao nhiêu phần tử đứng phía sau.
Không có ngoại lệ nào cho vị trí giữa cả.

Nhưng "giữa" không phải vị trí DUY NHẤT trong một mảng. Trong tất cả các
chỗ bạn có thể chèn vào hay xoá đi — đầu, giữa, cuối — có chỗ nào KHÔNG
kéo theo dây chuyền dời chỗ nào cả không? Một chỗ mà thêm vào hay bớt đi
không đụng tới bất kỳ phần tử nào khác?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
