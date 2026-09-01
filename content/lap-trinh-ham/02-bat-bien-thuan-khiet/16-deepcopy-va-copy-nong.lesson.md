---
id: lap-trinh-ham.bat-bien-thuan-khiet.deepcopy-va-copy-nong
title: "`copy.deepcopy` và `copy.copy` — sao chép SÂU khác sao chép NÔNG"
summary: "copy.copy dựng object ngoài mới nhưng field lồng nhau vẫn dùng chung — sửa qua bản sao vẫn làm bản gốc đổi theo. copy.deepcopy sao chép trọn mọi lớp lồng nhau, bản gốc không đổi. Công cụ THỰC TẾ để né bẫy field mutable mà không cần đổi kiểu dữ liệu."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.deepcopy-vs-copy]
requires: [fp.nested-mutable-trap, mem.shallow-copy]
concepts: [fp.deepcopy-vs-copy]
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
Không cần đổi kiểu field. Module `copy` có sẵn công cụ đúng cho việc
này — bạn từng gặp một nửa của nó, hồi nói về cái nồi.
::::

::::explain{#nong-mot-lop-sau-toan-bo}
Hồi học về bộ nhớ, bạn đã gặp **chép nông** (`a[:]`): dựng ra một
`list` NGOÀI hoàn toàn mới, nhưng nếu một Ô trong đó lại là một `list`
CON, cả bản gốc lẫn bản sao vẫn cùng trỏ vào đúng một `list` con đó.
Module `copy` mang đúng ý này áp dụng cho MỌI kiểu dữ liệu, kể cả
`@dataclass`, qua hai hàm:

- `copy.copy(x)` — **chép nông**: dựng một `x` MỚI ở lớp ngoài, nhưng
  mọi field/phần tử lồng bên trong vẫn là CÙNG một địa chỉ với bản gốc.
- `copy.deepcopy(x)` — **chép sâu**: dựng một `x` MỚI, và đệ quy chép
  luôn MỌI lớp lồng bên trong thành các đối tượng ĐỘC LẬP hoàn toàn.

Đây chính là công cụ giải quyết bẫy bài trước theo một hướng KHÁC: thay
vì đổi field từ `list` sang `tuple` (đổi kiểu dữ liệu, không phải lúc
nào cũng làm được), `copy.deepcopy()` cho một bản độc lập thật sự mà
field vẫn cứ là `list` như cũ. Đánh đổi: `deepcopy` chỉ bảo vệ ĐÚNG BẢN
SAO bạn vừa tạo ra — bản gốc vẫn là `list`, vẫn sửa được tại chỗ bình
thường nếu ai đó cầm thẳng lấy nó.
::::

::::example{#chep-nong-van-dinh-nhau}
Field `mon` của `Gio` là `list` — đúng bẫy bài trước.

```python title=readonly
from dataclasses import dataclass
import copy

@dataclass(frozen=True)
class Gio:
    mon: list

gio_goc = Gio(["Phở", "Bún"])
gio_nong = copy.copy(gio_goc)
gio_nong.mon.append("Chả cá")

print(f"gio_goc:  {gio_goc.mon}")
print(f"gio_nong: {gio_nong.mon}")
```

```text title=readonly
gio_goc:  ['Phở', 'Bún', 'Chả cá']
gio_nong: ['Phở', 'Bún', 'Chả cá']
```

`copy.copy()` đã dựng ra một `Gio` khác (`gio_nong is gio_goc` ra
`False`), nhưng field `mon` của cả hai vẫn CÙNG một `list` — sửa qua
`gio_nong.mon` thì `gio_goc.mon` thấy ngay, y hệt chuyện chép nông hồi
trước với `list` lồng trong `list`.

Đổi đúng một chữ — `copy.deepcopy()` thay vì `copy.copy()`:

```python title=readonly
gio_goc = Gio(["Phở", "Bún"])
gio_sau = copy.deepcopy(gio_goc)
gio_sau.mon.append("Chả cá")

print(f"gio_goc:  {gio_goc.mon}")
print(f"gio_sau:  {gio_sau.mon}")
print(f"Cùng list không: {gio_goc.mon is gio_sau.mon}")
```

```text title=readonly
gio_goc:  ['Phở', 'Bún']
gio_sau:  ['Phở', 'Bún', 'Chả cá']
Cùng list không: False
```

Lần này `gio_goc.mon` không hề đổi. `gio_sau.mon` là một `list` HOÀN
TOÀN KHÁC (`is` ra `False`) — sửa nó không còn ảnh hưởng gì tới
`gio_goc` nữa.
::::

::::predict{#doan-nong-hay-sau commitOnce}
Một giỏ hàng khác, cùng cấu trúc — field `mon` kiểu `list`.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
from dataclasses import dataclass
import copy

@dataclass(frozen=True)
class Gio:
    mon: list

goc = Gio(["táo", "cam"])
sao = copy.copy(goc)
sao.mon.append("chuối")

print(f"goc: {goc.mon}")
print(f"sao: {sao.mon}")
```

:::opt{correct}
`goc: ['táo', 'cam', 'chuối']` và `sao: ['táo', 'cam', 'chuối']` — cả
hai đều có "chuối", vì `copy.copy()` không tách field `mon` ra
:::

:::opt
`goc: ['táo', 'cam']` và `sao: ['táo', 'cam', 'chuối']` — chỉ `sao`
đổi, `goc` giữ nguyên
::why
Gần đúng ở việc đây ĐÚNG LÀ kết quả bạn sẽ thấy — nhưng chỉ khi dùng
`copy.deepcopy()`, không phải `copy.copy()` như đoạn mã này.

Chỗ lệch: `copy.copy()` là chép NÔNG — nó chỉ dựng một `Gio` mới ở lớp
NGOÀI, còn field `mon` (một `list`) vẫn là CÙNG một địa chỉ giữa `goc`
và `sao`. Sửa `sao.mon` thì `goc.mon` phải thấy theo, đúng ví dụ vừa
xem ở trên.
::
:::

:::opt
Máy dừng lại, báo lỗi vì `Gio` là `frozen=True`, không cho `copy.copy()`
hoạt động
::why
Gần đúng ở việc bạn cảnh giác đúng chỗ — `frozen=True` THẬT SỰ chặn
nhiều thao tác trong track này.

Chỗ lệch: `copy.copy()` không đi qua cơ chế gán field mà `frozen=True`
canh gác — nó dựng đối tượng mới bằng một con đường khác hẳn. Không có
lỗi nào ở đây; `copy.copy()` chạy trót lọt trên cả object frozen.
::
:::

:::opt
`goc: ['táo', 'cam']` và `sao: ['táo', 'cam']` — cả hai đều không đổi,
vì `sao.mon.append(...)` không phải một GÁN LẠI field nên bị bỏ qua
::why
Gần đúng ở việc bạn nhớ đúng: đây không phải một phép GÁN LẠI field —
đúng, `.append()` không đụng tới cơ chế `frozen=True` canh gác.

Chỗ lệch: không bị chặn KHÔNG có nghĩa là không xảy ra. `.append(...)`
chạy bình thường, và nó THẬT SỰ thêm "chuối" vào đúng cái `list` mà cả
`goc.mon` lẫn `sao.mon` đang cùng trỏ tới.
::
:::
::::

::::code{#ban-sao-doc-lap}
Byte cần một bản "nháp" của giỏ hàng, thử thêm một món mà KHÔNG được
đụng tới giỏ gốc — kể cả field `mon`, dù nó là `list`. Điền chỗ trống
để `ban_sao` là một bản ĐỘC LẬP hoàn toàn của `goc`.

```python title=starter
from dataclasses import dataclass
import copy

@dataclass(frozen=True)
class GioHang:
    mon: list

goc = GioHang(["táo", "cam"])
ban_sao = ___

ban_sao.mon.append("chuối")

print(f"goc:      {goc.mon}")
print(f"ban_sao:  {ban_sao.mon}")
```

```python title=solution
from dataclasses import dataclass
import copy

@dataclass(frozen=True)
class GioHang:
    mon: list

goc = GioHang(["táo", "cam"])
ban_sao = copy.deepcopy(goc)

ban_sao.mon.append("chuối")

print(f"goc:      {goc.mon}")
print(f"ban_sao:  {ban_sao.mon}")
```

```python title=test
assert goc.mon == ["táo", "cam"], "goc.mon không được đổi — deepcopy phải dựng ra một list mon HOÀN TOÀN KHÁC cho ban_sao"
assert ban_sao.mon == ["táo", "cam", "chuối"], "ban_sao.mon phải mang đủ hai món gốc, cộng thêm 'chuối'"
assert goc.mon is not ban_sao.mon, "goc.mon và ban_sao.mon phải là hai list KHÁC NHAU — copy.copy() sẽ để chúng dùng chung, chỉ copy.deepcopy() mới tách hẳn"
assert goc is not ban_sao, "ban_sao phải là một GioHang khác goc, không phải chính goc"
```

:::hints
- kind: attention
  body: Chỗ trống là một lời gọi trên module copy, nhận goc làm đối số.
- kind: strategy
  body: copy.copy(goc) sẽ để field mon dùng chung giữa goc và ban_sao — đúng bẫy bài trước. Cần hàm chép SÂU, đệ quy chép cả field lồng bên trong.
- kind: one-line
  body: 'Điền `copy.deepcopy(goc)` vào chỗ trống.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống phải gọi copy.deepcopy(goc) — copy.copy(goc) chỉ chép NÔNG, để field mon dùng chung giữa goc và ban_sao, không tách được như bài này đòi hỏi
  requireAst:
  - kind: uses-call, target: deepcopy, min: 1
  forbidAst:
  - kind: uses-call, target: copy, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^goc:      \['táo', 'cam'\]\nban_sao:  \['táo', 'cam', 'chuối'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Không cần đổi kiểu dữ liệu nào cả. `deepcopy` cho bạn một bản độc lập
thật sự, ở mọi lớp lồng nhau.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`deepcopy` sao chép TOÀN BỘ, dù đôi khi bạn chỉ cần đổi đúng MỘT phần
nhỏ nằm sâu bên trong. Nếu dữ liệu của bạn là một `@dataclass(frozen=True)`
CHỨA cả một dãy `tuple` những `@dataclass(frozen=True)` khác — không
còn `list` nào để lo bẫy nữa — có cách nào đổi đúng một phần lồng sâu
bên trong, mà vẫn CHỈ tạo mới đúng phần cần đổi, không phải chép lại
toàn bộ?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
