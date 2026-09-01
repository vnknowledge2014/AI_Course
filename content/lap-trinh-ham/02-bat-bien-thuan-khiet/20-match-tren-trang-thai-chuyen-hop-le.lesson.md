---
id: lap-trinh-ham.bat-bien-thuan-khiet.match-tren-trang-thai-chuyen-hop-le
title: "`match` trên trạng thái — chỉ chuyển được từ đúng trạng thái nguồn"
summary: "match bai: case Draft(...): ...; case _: raise ValueError(...) — gọi publish() trên một Published (không phải Draft) ném lỗi rõ ràng, KHÔNG âm thầm chuyển sai."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.state-transition-match]
requires: [fp.state-as-data]
concepts: [fp.state-transition-match]
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
Bài trước để lại một lỗ hổng: gọi `publish()` trên một bài ĐÃ ĐĂNG vẫn
chạy trót lọt, âm thầm tạo ra rác. Hôm nay Byte vá lỗ đó — bằng `match`.
::::

::::explain{#match-tren-kieu-du-lieu}
`match` không chỉ so khớp giá trị (như `case "gio":`) — nó so khớp
đúng KIỂU DỮ LIỆU của một `@dataclass`, và còn lấy luôn field ra thành
biến, trong một bước:

```python
def publish(bai):
    match bai:
        case Draft(tieu_de=t):
            return Published(tieu_de=t, ngay_dang="2026-09-01")
        case _:
            raise ValueError("chỉ publish được từ Draft")
```

`case Draft(tieu_de=t):` chỉ khớp khi `bai` THẬT SỰ là một `Draft` —
đồng thời gán `tieu_de` của nó vào biến `t`, đọc dùng ngay trong nhánh
đó. `case _:` là nhánh "mọi thứ khác" — khớp một `Published`, một
`Archived`, hay bất kỳ thứ gì không phải `Draft`.

Khác bài trước: nhánh `case _` không âm thầm bỏ qua. Nó `raise
ValueError(...)` — NÉM một lỗi rõ ràng, dừng chương trình lại ngay tại
chỗ gọi sai, thay vì để hàm chạy tiếp và trả về một `Published` vô
nghĩa được dựng từ dữ liệu của một trạng thái không phải `Draft`. Lỗi
lộ ra NGAY, không đợi tới lúc ai đó phát hiện dữ liệu sai ở đâu đó xa
tít phía sau.
::::

::::example{#goi-dung-va-goi-sai}
Cùng một hàm `publish()`, gọi hai lần — một lần đúng, một lần sai:

```python title=readonly
from dataclasses import dataclass

@dataclass(frozen=True)
class Draft:
    tieu_de: str

@dataclass(frozen=True)
class Published:
    tieu_de: str
    ngay_dang: str

def publish(bai):
    match bai:
        case Draft(tieu_de=t):
            return Published(tieu_de=t, ngay_dang="2026-09-01")
        case _:
            raise ValueError("chỉ publish được từ Draft")

nhap = Draft("Bài đầu tiên")
da_dang = publish(nhap)
print(da_dang)

publish(da_dang)   # da_dang là Published, không phải Draft
```

```text title=readonly
Published(tieu_de='Bài đầu tiên', ngay_dang='2026-09-01')
Traceback (most recent call last):
  ...
ValueError: chỉ publish được từ Draft
```

Lần gọi đầu khớp `case Draft(tieu_de=t)`, chạy trót lọt. Lần gọi thứ
hai đưa `da_dang` (một `Published`) vào — không khớp nhánh `Draft`,
rơi xuống `case _`, và chương trình DỪNG LẠI với thông báo rõ ràng
thay vì tạo ra một `Published` chồng lên `Published` không có nghĩa
gì cả.
::::

::::predict{#doan-goi-sai-kieu commitOnce}
Một hàm khác, cùng khuôn `match`, xử lý việc HUỶ một bài viết — chỉ
huỷ được từ `Draft` (chưa đăng thì huỷ được, đã đăng rồi thì không):

```python
def huy(bai):
    match bai:
        case Draft(tieu_de=t):
            print(f"Đã huỷ: {t}")
        case _:
            raise ValueError("chỉ huỷ được bài chưa đăng")

nhap = Draft("Bản nháp")
da_dang = publish(nhap)
huy(da_dang)
```

Dòng cuối (`huy(da_dang)`) chạy ra gì?

:::opt{correct}
Chương trình dừng lại, `ValueError: chỉ huỷ được bài chưa đăng`
:::

:::opt
In ra `Đã huỷ: Bản nháp` — vì `da_dang` được TẠO TỪ `nhap`, nên field
`tieu_de` của nó vẫn là "Bản nháp", khớp `case Draft`
::why
Gần đúng ở việc bạn nhớ đúng `da_dang` giữ lại field `tieu_de` từ
`nhap` (qua `publish()`).

Chỗ lệch: `match` không so khớp theo GIÁ TRỊ field, mà so khớp theo
KIỂU DỮ LIỆU của cả object. `da_dang` là một `Published`, không phải
`Draft` — dù `tieu_de` bên trong có giống nhau, `case Draft(...)` chỉ
khớp khi object THẬT SỰ được tạo bằng `Draft(...)`.
::
:::

:::opt
Không in gì cả, hàm âm thầm trả về `None`
::why
Gần đúng ở việc bạn đoán đúng nhánh `case Draft` không khớp.

Chỗ lệch: nhánh `case _:` không phải một lối thoát im lặng — nó
`raise ValueError(...)`. Chương trình DỪNG LẠI với một thông báo,
không lặng lẽ trả về `None` rồi chạy tiếp.
::
:::

:::opt
Máy báo lỗi cú pháp, vì `match` không dùng được với `Published` khi
hàm chỉ khai `case Draft`
::why
Gần đúng ở việc bạn cảnh giác về việc `match` "chỉ biết" những case
đã khai.

Chỗ lệch: đây không phải lỗi CÚ PHÁP — Python parse `match` này hoàn
toàn bình thường. Vấn đề xảy ra LÚC CHẠY: không nhánh `case` nào khớp
`Published`, nên rơi xuống `case _`, và đó là một exception CÓ CHỦ Ý
(`ValueError`), không phải một lỗi cú pháp.
::
:::
::::

::::code{#archive-chi-tu-published}
Viết hàm `archive(bai, ly_do)` — chỉ ARCHIVE được một bài đã `Published`
(không archive được một `Draft`). Trả về một `Archived` mới, giữ
nguyên `tieu_de` và `ngay_dang`, thêm `ly_do`. Gọi sai — archive một
`Draft` — phải ném `ValueError`.

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
    ___

draft = Draft("Tin cũ")
pub = publish(draft)
luu = archive(pub, "hết thời hạn")

print(f"pub: {pub}")
print(f"luu: {luu}")
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

draft = Draft("Tin cũ")
pub = publish(draft)
luu = archive(pub, "hết thời hạn")

print(f"pub: {pub}")
print(f"luu: {luu}")
```

```python title=test
draft2 = Draft("Bài kiểm tra")
pub2 = publish(draft2)
luu2 = archive(pub2, "quá hạn")

assert isinstance(luu2, Archived), "archive(pub, ...) phải trả về một Archived"
assert luu2.tieu_de == "Bài kiểm tra", "tieu_de phải giữ nguyên từ pub"
assert luu2.ngay_dang == pub2.ngay_dang, "ngay_dang phải giữ nguyên từ pub"
assert luu2.ly_do == "quá hạn", "ly_do phải đúng giá trị truyền vào"
assert isinstance(pub2, Published), "pub2 không được đổi sau khi gọi archive"

da_nem_loi = False
try:
    archive(draft2, "gọi sai thứ tự")
except ValueError:
    da_nem_loi = True
assert da_nem_loi, "archive() gọi trên một Draft (không phải Published) phải ném ValueError"
```

:::hints
- kind: attention
  body: Chỗ trống là TOÀN BỘ thân hàm archive — viết một khối match, giống hệt cấu trúc publish() ở trên nhưng khớp Published thay vì Draft.
- kind: strategy
  body: 'case Published(tieu_de=t, ngay_dang=nd): lấy hai field từ bai, dùng để dựng Archived(tieu_de=t, ngay_dang=nd, ly_do=ly_do). Nhánh case _: phải raise ValueError(...), không phải return None hay bỏ qua.'
- kind: one-line
  body: "match bai:\n    case Published(tieu_de=t, ngay_dang=nd):\n        return Archived(tieu_de=t, ngay_dang=nd, ly_do=ly_do)\n    case _:\n        raise ValueError(\"chỉ archive được từ Published\")"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: archive() phải dùng match để khớp đúng kiểu Published, và nhánh còn lại phải raise ValueError — không phải return None, không phải assert, không phải bỏ qua việc kiểm tra kiểu
  requireAst:
  - kind: match-stmt, min: 1
  - kind: uses-call, target: Archived, min: 1
  - kind: no-mutation
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "luu: Archived(tieu_de='Tin cũ', ngay_dang='2026-09-01', ly_do='hết thời hạn')"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Gọi đúng thứ tự — chạy êm. Gọi sai thứ tự — Python DỪNG LẠI và nói rõ vì
sao, ngay tại chỗ gọi sai, không phải ở đâu đó xa tít phía sau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn vừa viết hai hàm chuyển trạng thái riêng lẻ: `publish()` và
`archive()`. Nhưng một bài viết thật thường đi qua NHIỀU bước liên
tiếp — nháp, rồi đăng, rồi lưu trữ — trong cùng một lần chạy chương
trình.

Nếu gọi `publish(draft)` rồi ngay lập tức `archive(pub)`, biến `pub`
đứng giữa hai lời gọi đó có bị đổi gì không, khi bước `archive` chạy
xong?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
