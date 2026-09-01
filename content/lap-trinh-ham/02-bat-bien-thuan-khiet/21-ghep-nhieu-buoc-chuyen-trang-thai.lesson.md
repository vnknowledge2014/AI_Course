---
id: lap-trinh-ham.bat-bien-thuan-khiet.ghep-nhieu-buoc-chuyen-trang-thai
title: "Ghép nhiều bước chuyển — trạng thái CŨ không đổi sau mỗi bước"
summary: "Gọi publish(draft) rồi archive(pub) liên tiếp: pub vẫn LÀ Published sau khi archive() chạy xong — không tự đổi thành Archived. Đây là điểm khác cốt lõi với state machine hướng đối tượng thông thường."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.state-machine-compose]
requires: [fp.state-transition-match]
concepts: [fp.state-machine-compose]
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
Bài trước để lại một câu hỏi: gọi hai bước chuyển liên tiếp, bước giữa
có bị đổi gì không? Câu trả lời là điểm khác biệt cốt lõi với mọi state
machine hướng đối tượng bạn từng thấy.
::::

::::explain{#trang-thai-giua-khong-doi}
Trong một state machine hướng đối tượng quen thuộc, một object DUY NHẤT
đi qua nhiều trạng thái theo thời gian — `bai.trang_thai` đổi từ
`"nhap"` sang `"da_dang"` rồi sang `"luu_tru"`, NGAY TRÊN CÙNG một
object `bai`. Muốn biết "bài đang ở trạng thái nào", phải hỏi đúng thời
điểm, vì cùng một biến `bai` có thể cho câu trả lời khác nhau ở hai lúc
khác nhau.

Với `publish()` và `archive()` đã viết, chuyện khác hẳn. Gọi
`pub = publish(draft)` rồi `luu = archive(pub, ly_do)`: đây là HAI GIÁ
TRỊ RIÊNG, không phải một object đi qua hai giai đoạn. `archive(pub,
ly_do)` chỉ ĐỌC field của `pub` để dựng `luu` — nó không hề "tiêu thụ"
hay "nâng cấp" `pub`. Sau khi `archive()` chạy xong, biến `pub` vẫn
đứng đó, vẫn là `Published`, y hệt trước khi gọi.

Đây không phải một chi tiết cài đặt tình cờ — nó là HỆ QUẢ trực tiếp
của việc mỗi trạng thái là MỘT KIỂU DỮ LIỆU BẤT BIẾN riêng
(`fp.state-as-data`), và mọi hàm chuyển trạng thái chỉ TẠO MỚI, không
sửa tại chỗ (`fp.frozen-dataclass`, `fp.dataclasses-replace`). Ghép
nhiều bước lại không hề đánh mất tính chất đó — mỗi giá trị trung gian
đứng yên mãi mãi, kể cả sau khi có một giá trị MỚI được dựng từ nó.
::::

::::example{#ba-gia-tri-cung-ton-tai}
Byte đi qua CẢ HAI bước, rồi in ra cả BA giá trị cùng lúc — `draft`,
`pub`, `luu` — để xem chúng có "biến mất" hay đổi gì không:

```python title=readonly
from dataclasses import dataclass

@dataclass(frozen=True)
class Draft:
    tieu_de: str

@dataclass(frozen=True)
class Published:
    tieu_de: str
    ngay_dang: str

@dataclass(frozen=True)
class Archived:
    tieu_de: str
    ngay_dang: str
    ly_do: str

def publish(bai):
    match bai:
        case Draft(tieu_de=t):
            return Published(tieu_de=t, ngay_dang="2026-09-01")
        case _:
            raise ValueError("chỉ publish được từ Draft")

def archive(bai, ly_do):
    match bai:
        case Published(tieu_de=t, ngay_dang=nd):
            return Archived(tieu_de=t, ngay_dang=nd, ly_do=ly_do)
        case _:
            raise ValueError("chỉ archive được từ Published")

draft = Draft("Bài viết A")
pub = publish(draft)
luu = archive(pub, "hết thời hạn")

print(f"draft: {draft}")
print(f"pub:   {pub}")
print(f"luu:   {luu}")
```

```text title=readonly
draft: Draft(tieu_de='Bài viết A')
pub:   Published(tieu_de='Bài viết A', ngay_dang='2026-09-01')
luu:   Archived(tieu_de='Bài viết A', ngay_dang='2026-09-01', ly_do='hết thời hạn')
```

Cả BA biến cùng tồn tại, cùng đúng, sau khi mọi bước đã chạy xong.
`draft` không đổi sau `publish()`. `pub` không đổi sau `archive()`.
Không có bước nào "phá" bước trước — mỗi bước chỉ THÊM một giá trị
mới vào bức tranh, không XOÁ hay SỬA giá trị đã có.
::::

::::predict{#doan-goi-archive-hai-lan commitOnce}
Byte gọi `archive()` HAI LẦN, cùng một `pub`, cùng một `ly_do`:

```python
draft = Draft("Bài viết Z")
pub = publish(draft)
luu1 = archive(pub, "hết thời hạn")
luu2 = archive(pub, "hết thời hạn")

print(luu1 == luu2)
print(luu1 is luu2)
```

Hai dòng in ra gì?

:::opt{correct}
`True` rồi `False`
:::

:::opt
`False` rồi `False` — gọi hai lần thì phải ra hai kết quả khác nhau,
vì mỗi lần gọi hàm là một sự kiện riêng biệt
::why
Gần đúng ở việc bạn nghĩ "mỗi lần gọi là một sự kiện riêng" — đúng cho
những hàm KHÔNG THUẦN (ví dụ một hàm tự tăng một bộ đếm).

Chỗ lệch: `archive()` là một hàm THUẦN — CÙNG `pub`, CÙNG `ly_do`,
luôn cho ra một `Archived` có GIÁ TRỊ giống hệt. `luu1 == luu2` so
sánh GIÁ TRỊ (dataclass tự sinh `__eq__` so field-by-field), và hai
lần gọi cho ra field giống hệt nhau.
::
:::

:::opt
`True` rồi `True` — vì cùng input thì đương nhiên là "cùng một thứ"
::why
Gần đúng ở phần đầu: `luu1 == luu2` đúng là `True`, vì cùng input cho
cùng giá trị.

Chỗ lệch: "cùng giá trị" không phải "cùng object". `archive()` TẠO
MỘT `Archived` MỚI mỗi lần gọi — `luu1` và `luu2` là hai object khác
nhau trong bộ nhớ, dù nội dung bên trong giống hệt. `is` so sánh DANH
TÍNH object, không so sánh giá trị — đó là lý do `luu1 is luu2` phải
là `False`.
::
:::

:::opt
Máy báo lỗi — gọi `archive()` lần thứ hai trên cùng một `pub` là gọi
lại trên một giá trị "đã dùng"
::why
Gần đúng ở việc bạn cảnh giác về việc dùng lại một giá trị — phản xạ
đúng cho ngôn ngữ có luật sở hữu như Rust.

Chỗ lệch: Python không có khái niệm "dùng hết" một giá trị bất biến.
`archive(pub, ...)` chỉ ĐỌC field của `pub`, không hề làm `pub` mất
hiệu lực — gọi lại bao nhiêu lần cũng được, mỗi lần đọc y hệt lần
trước.
::
:::
::::

::::code{#quy-trinh-hai-buoc}
Viết `quy_trinh(bai, ly_do)` — gọi `publish()` rồi `archive()` liên
tiếp trên `bai`, trả về một `tuple` gồm CẢ HAI giá trị trung gian:
`(pub, luu)`.

```python title=starter
from dataclasses import dataclass

@dataclass(frozen=True)
class Draft:
    tieu_de: str

@dataclass(frozen=True)
class Published:
    tieu_de: str
    ngay_dang: str

@dataclass(frozen=True)
class Archived:
    tieu_de: str
    ngay_dang: str
    ly_do: str

def publish(bai):
    match bai:
        case Draft(tieu_de=t):
            return Published(tieu_de=t, ngay_dang="2026-09-01")
        case _:
            raise ValueError("chỉ publish được từ Draft")

def archive(bai, ly_do):
    match bai:
        case Published(tieu_de=t, ngay_dang=nd):
            return Archived(tieu_de=t, ngay_dang=nd, ly_do=ly_do)
        case _:
            raise ValueError("chỉ archive được từ Published")

def quy_trinh(bai, ly_do):
    ___

draft = Draft("Bài viết B")
pub, luu = quy_trinh(draft, "quá hạn")

print(f"draft: {draft}")
print(f"pub:   {pub}")
print(f"luu:   {luu}")
```

```python title=solution
from dataclasses import dataclass

@dataclass(frozen=True)
class Draft:
    tieu_de: str

@dataclass(frozen=True)
class Published:
    tieu_de: str
    ngay_dang: str

@dataclass(frozen=True)
class Archived:
    tieu_de: str
    ngay_dang: str
    ly_do: str

def publish(bai):
    match bai:
        case Draft(tieu_de=t):
            return Published(tieu_de=t, ngay_dang="2026-09-01")
        case _:
            raise ValueError("chỉ publish được từ Draft")

def archive(bai, ly_do):
    match bai:
        case Published(tieu_de=t, ngay_dang=nd):
            return Archived(tieu_de=t, ngay_dang=nd, ly_do=ly_do)
        case _:
            raise ValueError("chỉ archive được từ Published")

def quy_trinh(bai, ly_do):
    pub = publish(bai)
    luu = archive(pub, ly_do)
    return pub, luu

draft = Draft("Bài viết B")
pub, luu = quy_trinh(draft, "quá hạn")

print(f"draft: {draft}")
print(f"pub:   {pub}")
print(f"luu:   {luu}")
```

```python title=test
draft2 = Draft("Bài kiểm tra C")
pub2, luu2 = quy_trinh(draft2, "quá hạn")

assert isinstance(pub2, Published), "phần tử đầu trả về phải là Published — trạng thái SAU publish, TRƯỚC archive"
assert isinstance(luu2, Archived), "phần tử hai trả về phải là Archived — trạng thái sau archive"
assert pub2 == Published(tieu_de="Bài kiểm tra C", ngay_dang="2026-09-01"), "pub PHẢI GIỮ NGUYÊN giá trị này — archive() không được sửa pub tại chỗ"
assert luu2.ly_do == "quá hạn", "ly_do phải đúng giá trị truyền vào"
assert luu2.tieu_de == "Bài kiểm tra C", "tieu_de phải giữ nguyên từ draft2 qua cả hai bước"
assert isinstance(draft2, Draft), "draft2 không được đổi sau khi gọi quy_trinh"
```

:::hints
- kind: attention
  body: Chỗ trống là toàn bộ thân hàm quy_trinh — hai lời gọi hàm liên tiếp, cộng một dòng return trả về một tuple hai phần tử.
- kind: strategy
  body: 'Gọi publish(bai) trước, giữ kết quả lại bằng một tên (ví dụ pub). Gọi archive(pub, ly_do) sau, giữ kết quả bằng một tên khác (ví dụ luu). Trả về return pub, luu — không cần đóng ngoặc, Python tự hiểu đó là một tuple.'
- kind: one-line
  body: "pub = publish(bai)\nluu = archive(pub, ly_do)\nreturn pub, luu"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: quy_trinh() phải gọi publish() rồi archive() và trả về (pub, luu) — không được gán trực tiếp vào field của một object đã có sẵn (điều đó sẽ bị chặn bởi frozen=True, và cũng chính là thứ luật này canh)
  requireAst:
  - kind: uses-call, target: publish, min: 1
  - kind: uses-call, target: archive, min: 1
  - kind: no-mutation
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "pub:   Published(tieu_de='Bài viết B', ngay_dang='2026-09-01')"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`pub` đứng yên suốt từ lúc được tạo ra tới lúc chương trình kết thúc.
`archive()` chỉ MƯỢN nó để đọc, không hề đụng vào nó. Ghép bao nhiêu
bước cũng vậy — mỗi giá trị trung gian sống mãi, không ai âm thầm phá.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Suốt track này, mọi hàm bạn viết đều KHÔNG SỬA DỮ LIỆU TẠI CHỖ. Nhưng
có một chuyện khác, tách biệt hẳn với "sửa tại chỗ", mà bạn CHƯA từng
kiểm: nếu một hàm chuyển trạng thái, ngoài việc trả về giá trị mới, còn
lặng lẽ `print()` một dòng log, hay đọc giờ hệ thống để đặt `ngay_dang`
— hàm đó có còn ĐÁNG TIN theo đúng nghĩa "cùng input, cùng output"
không?

Bài sau quay lại đúng câu hỏi đó, trên một hàm KHÔNG THUẦN có thật —
và bạn viết lại nó thành THUẦN.
::::

::::checkpoint{mastery=0.8}
::::
