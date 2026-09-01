---
id: lap-trinh-ham.ham-bac-cao-hop-thanh.method-chaining-pipe-kieu-oop
title: "Method chaining — `pipe` kiểu hướng đối tượng"
summary: "Một @dataclass(frozen=True) có method TRẢ VỀ MỘT BẢN SAO MỚI (dùng replace()) cho phép viết TruyVan('users').loc('tuoi>18').gioi_han(10) — đọc như một chuỗi bước, nhưng MỖI bước tạo object MỚI. Đây là pipe khoác áo OOP."
locale: vi
track: lap-trinh-ham
module: ham-bac-cao-hop-thanh
order: 23
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.method-chaining]
requires: [fp.pipe-uses-reduce]
concepts: [fp.method-chaining]
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
`pipe(value, f, g, h)` ghép hàm bằng cách TRUYỀN chúng vào một lời gọi.
Nhiều thư viện thật ghép bước bằng cú pháp khác hẳn: `.method()` nối
tiếp nhau. Liên quan gì tới `pipe` không?
::::

::::explain{#chain-la-pipe-khoac-ao-oop}
Nhớ lại T4.1: một `@dataclass(frozen=True)` chặn SỬA field, nhưng
`replace()` tạo được một bản sao MỚI đã đổi field. Ghép điều đó với
`self` — một METHOD có thể trả về `replace(self, ...)` thay vì SỬA
`self` tại chỗ:

```python
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TruyVan:
    bang: str
    dieu_kien: tuple = ()
    gioi_han_so: int = None

    def loc(self, dk):
        return replace(self, dieu_kien=self.dieu_kien + (dk,))

    def gioi_han(self, n):
        return replace(self, gioi_han_so=n)

q = TruyVan("users").loc("tuoi>18").gioi_han(10)
print(q)
```

```text
TruyVan(bang='users', dieu_kien=('tuoi>18',), gioi_han_so=10)
```

`TruyVan("users")` tạo một `TruyVan` gốc. `.loc("tuoi>18")` KHÔNG sửa
nó — trả về một `TruyVan` MỚI (đã thêm điều kiện). `.gioi_han(10)` gọi
TIẾP trên object MỚI đó, lại trả về một `TruyVan` MỚI KHÁC (đã đặt giới
hạn). Đọc như một CHUỖI bước liền mạch — nhưng bên dưới, MỖI dấu chấm là
MỘT lần TẠO MỚI, không phải một lần SỬA.

Đây CHÍNH LÀ `pipe` (bài 21) — chỉ khoác áo cú pháp KHÁC:
`pipe(value, f, g, h)` truyền các bước làm ĐỐI SỐ của một lời gọi;
`value.f().g().h()` viết các bước làm LỜI GỌI METHOD nối tiếp. Cùng Ý
(mỗi bước nhận kết quả của bước trước, tạo ra giá trị mới), khác cú
pháp gọi.
::::

::::example{#goc-khong-doi}
Điểm mấu chốt: object GỐC không hề bị ảnh hưởng bởi chuỗi method:

```python title=readonly
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TruyVan:
    bang: str
    dieu_kien: tuple = ()
    gioi_han_so: int = None

    def loc(self, dk):
        return replace(self, dieu_kien=self.dieu_kien + (dk,))

    def gioi_han(self, n):
        return replace(self, gioi_han_so=n)

goc = TruyVan("users")
da_loc = goc.loc("tuoi>18")

print(goc)
print(da_loc)
```

```text title=readonly
TruyVan(bang='users', dieu_kien=(), gioi_han_so=None)
TruyVan(bang='users', dieu_kien=('tuoi>18',), gioi_han_so=None)
```

`goc` vẫn `dieu_kien=()` — KHÔNG bị `.loc(...)` sửa. `da_loc` là một
object HOÀN TOÀN KHÁC, đã thêm điều kiện. Đây là hệ quả TRỰC TIẾP của
`frozen=True` + `replace()` (T4.1): mọi "bước" trong chuỗi đều TẠO MỚI,
không có object nào bị sửa tại chỗ, kể cả object gốc bị "gọi lên" đầu
tiên.
::::

::::predict{#doan-chain-doc-lap commitOnce}
```python
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TruyVan:
    bang: str
    dieu_kien: tuple = ()
    gioi_han_so: int = None

    def loc(self, dk):
        return replace(self, dieu_kien=self.dieu_kien + (dk,))

    def gioi_han(self, n):
        return replace(self, gioi_han_so=n)

q1 = TruyVan("orders").loc("gia>50")
q2 = q1.gioi_han(3)

print(q1.gioi_han_so)
print(q2.gioi_han_so)
```

Hai dòng cuối in ra gì?

:::opt{correct}
`None` rồi `3`
:::

:::opt
`3` rồi `3` — vì `q2 = q1.gioi_han(3)` sửa `q1` tại chỗ, nên cả hai
biến trỏ tới CÙNG một object đã đổi
::why
Gần đúng ở việc bạn tin giá trị CUỐI CÙNG (`gioi_han_so=3`) đúng — số
đó đúng cho `q2`.

Chỗ lệch: `gioi_han()` KHÔNG sửa `self` (`q1`) — nó gọi `replace(self,
gioi_han_so=n)`, TẠO MỘT OBJECT MỚI, gán vào `q2`. `q1` vẫn là object
CŨ, chưa từng bị đổi — `q1.gioi_han_so` vẫn là `None` (giá trị mặc định
lúc khởi tạo), không phải `3`.
::
:::

:::opt
Máy báo lỗi — `q1.gioi_han_so` không truy cập được vì `TruyVan` là
`frozen`, chặn cả việc ĐỌC field, không chỉ SỬA
::why
Gần đúng ở việc bạn nhớ ĐÚNG `frozen=True` có chặn một hành vi thật
(T4.1 đã dạy) — có sự chặn thật xảy ra ở đây.

Chỗ lệch: `frozen=True` CHỈ chặn GÁN LẠI field (`q1.gioi_han_so = 3`
mới báo lỗi) — ĐỌC field (`q1.gioi_han_so`) hoàn toàn bình thường,
không bị chặn. Đọc field của một object bất biến luôn hợp lệ; chỉ sửa
mới bị cấm.
::
:::
::::

::::code{#truy_van_chain}
Hoàn thiện `TruyVan` — viết method `loc(dk)` (thêm MỘT điều kiện vào
`dieu_kien`) và `gioi_han(n)` (đặt `gioi_han_so`). CẢ HAI phải trả về
một `TruyVan` MỚI bằng `replace()`, không sửa `self`.

```python title=starter
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TruyVan:
    bang: str
    dieu_kien: tuple = ()
    gioi_han_so: int = None

    ___

q = TruyVan("users").loc("tuoi>18").gioi_han(10)
print(q)
```

```python title=solution
from dataclasses import dataclass, replace

@dataclass(frozen=True)
class TruyVan:
    bang: str
    dieu_kien: tuple = ()
    gioi_han_so: int = None

    def loc(self, dk):
        return replace(self, dieu_kien=self.dieu_kien + (dk,))

    def gioi_han(self, n):
        return replace(self, gioi_han_so=n)

q = TruyVan("users").loc("tuoi>18").gioi_han(10)
print(q)
```

```python title=test
q0 = TruyVan("orders")
q1 = q0.loc("gia>100")
assert q1.dieu_kien == ("gia>100",), "loc phải THÊM điều kiện vào dieu_kien"
assert q0.dieu_kien == (), "q0 KHÔNG được bị sửa — loc phải trả về bản sao MỚI"
q2 = q1.gioi_han(5)
assert q2.gioi_han_so == 5, "gioi_han phải đặt đúng gioi_han_so"
assert q1.gioi_han_so is None, "q1 KHÔNG được bị sửa — gioi_han phải trả về bản sao MỚI"
q3 = q2.loc("con_hang=True")
assert q3.dieu_kien == ("gia>100", "con_hang=True"), "loc thêm lần thứ hai phải GIỮ điều kiện cũ, thêm điều kiện mới vào cuối"
```

:::hints
- kind: attention
  body: "Cả hai method PHẢI dùng replace(self, ...) để trả về một TruyVan MỚI — không được gán lại self.dieu_kien hay self.gioi_han_so (frozen sẽ báo lỗi, và đó không phải Ý bài học)."
- kind: strategy
  body: 'loc: return replace(self, dieu_kien=self.dieu_kien + (dk,)) — nối thêm dk vào tuple cũ. gioi_han: return replace(self, gioi_han_so=n) — đặt lại đúng field đó.'
- kind: one-line
  body: "def loc(self, dk):\n    return replace(self, dieu_kien=self.dieu_kien + (dk,))\n\ndef gioi_han(self, n):\n    return replace(self, gioi_han_so=n)"
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: Cả loc và gioi_han đều phải dùng replace() để tạo một TruyVan MỚI — không được sửa self tại chỗ (self.dieu_kien = ... sẽ báo lỗi vì frozen=True, và không phải cách Ý bài này dạy).
  requireAst:
  - kind: uses-call, target: replace, min: 2
- tier: tests
  timeoutMs: 4000
- tier: output
  match: contains
  expect: "TruyVan"
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`.method().method()` không phải cú pháp lạ của hướng đối tượng — nó là
`pipe` mặc áo khác, cùng Ý: mỗi bước tạo giá trị mới, không sửa cái cũ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cụm này đã đi từ `compose` (bài 19) tới `pipe` (bài 21-22) tới method
chaining (bài này) — ba cách viết CÙNG một Ý ghép hàm. Ghép TẤT CẢ lại,
cùng `map`/`filter`/`reduce` từ cụm 2, trông thế nào?

Bài sau chốt cụm.
::::

::::checkpoint{mastery=0.8}
::::
