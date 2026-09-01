---
id: lap-trinh-ham.bat-bien-thuan-khiet.do-tong-hop-cau-truc-du-lieu
title: "Đo tổng hợp trước khi sang trạng thái"
summary: "Bốn công cụ bất biến — tuple, frozenset, frozen=True, MappingProxyType — mỗi cái hợp với một kiểu dữ liệu khác nhau. Bài này đo khả năng CHỌN đúng công cụ cho dữ liệu cụ thể, không phải chỉ dùng đúng một công cụ đã chỉ định sẵn."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 18
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: [fp.review-data-structures]
requires: [fp.compose-immutable]
concepts: [fp.review-data-structures]
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
Không khái niệm mới hôm nay. Byte muốn xem bạn tự CHỌN được công cụ
đúng, khi không ai chỉ định sẵn cho bạn nữa.
::::

::::explain{#bon-cong-cu-bon-cau-hoi}
Sáu bài vừa qua cho bạn bốn công cụ bất biến, mỗi cái ứng đúng một kiểu
dữ liệu:

| Dữ liệu cần bất biến               | Công cụ            |
|-------------------------------------|---------------------|
| Một DÃY có thứ tự                   | `tuple`              |
| Một TẬP HỢP, không quan tâm thứ tự  | `frozenset`          |
| Một ĐỐI TƯỢNG có tên trường riêng   | `@dataclass(frozen=True)` |
| Một ÁNH XẠ (tra cứu theo khoá)      | `MappingProxyType`   |

Việc khó không phải NHỚ bốn công cụ này — bạn vừa luyện từng cái một,
riêng lẻ. Việc khó là khi có một bài toán THẬT, không ai nói trước "bài
này dùng `frozenset`" — bạn phải tự hỏi đúng câu hỏi: dữ liệu này có
THỨ TỰ không? Có TÊN TRƯỜNG riêng không, hay chỉ là nhiều giá trị cùng
loại? Tra cứu theo KHOÁ hay chỉ kiểm tra CÓ MẶT?
::::

::::example{#ba-du-lieu-mot-ca-lam}
Một ca làm việc buổi sáng có ba loại dữ liệu cố định, không đổi suốt
ca:

```python title=readonly
from types import MappingProxyType

nhan_vien = ("An", "Bình", "Chi")
bang_gia = MappingProxyType({"trà đá": 5000, "phở": 45000})
vai_tro_hop_le = frozenset({"quan_ly", "thu_ngan"})
```

`nhan_vien` là một DÃY CÓ THỨ TỰ (An luôn đứng trước Bình trong danh
sách ca) — `tuple`. `bang_gia` là một ÁNH XẠ, tra theo tên món —
`MappingProxyType`. `vai_tro_hop_le` chỉ dùng để kiểm tra "vai trò này
có được phép không", không quan tâm thứ tự — `frozenset`. Ba dữ liệu,
ba câu hỏi khác nhau, ba công cụ khác nhau — không công cụ nào dùng
chung cho cả ba được.
::::

::::predict{#chon-dung-cong-cu commitOnce}
Một quán trà sữa có danh sách "ngày mở cửa trong tuần" — cố định, có
thứ tự (Thứ Hai luôn đứng trước Thứ Ba trong lịch làm việc in ra cho
nhân viên).

**Trước khi viết code**, công cụ nào hợp nhất để giữ danh sách này bất
biến?

:::opt{correct}
`tuple` — một dãy có thứ tự cố định, không cần tên trường riêng cho
từng phần tử, không cần tra theo khoá
:::

:::opt
`frozenset` — cũng là một tập hợp giá trị không đổi, giống hệt tinh
thần cần ở đây
::why
Gần đúng ở việc bạn nhận ra ĐÚNG đây là một tập giá trị cố định, không
sửa được — tinh thần bất biến đúng hướng.

Chỗ lệch: `frozenset` KHÔNG giữ thứ tự — hai lần duyệt qua nó không
đảm bảo ra cùng một thứ tự phần tử. Danh sách ngày mở cửa thì thứ tự
QUAN TRỌNG (Thứ Hai phải đứng trước Thứ Ba khi in lịch) — `frozenset`
không giữ được cam kết đó, chỉ `tuple` giữ được.
::
:::

:::opt
`@dataclass(frozen=True)` — đóng băng nó lại thành một đối tượng, chắc
chắn nhất trong bốn công cụ
::why
Gần đúng ở việc `frozen=True` đúng là công cụ chặn "chắc" nhất bạn đã
học — bằng cả một ngoại lệ riêng.

Chỗ lệch: `@dataclass` sinh ra field có TÊN riêng
(`.ten`, `.gia`...) cho một đối tượng có cấu trúc khác nhau ở mỗi
field. Bảy ngày mở cửa lại là BẢY GIÁ TRỊ CÙNG LOẠI (đều là tên ngày),
không phải bảy field khác ý nghĩa — không có lý do gì để đặt tên riêng
cho từng ngày. `tuple` đã đủ và gọn hơn nhiều.
::
:::

:::opt
`MappingProxyType` — bọc nó lại thành một ánh xạ chỉ đọc để chắc chắn
không ai sửa được
::why
Gần đúng ở việc bạn đúng hướng muốn CHẶN sửa — cả bốn công cụ đều làm
được điều đó.

Chỗ lệch: `MappingProxyType` dành cho ÁNH XẠ — tra cứu một GIÁ TRỊ
bằng một KHOÁ (`bang_gia["phở"]`). Danh sách ngày mở cửa không tra
theo khoá nào cả — chỉ là một dãy, đọc tuần tự hoặc theo chỉ số. Không
có khoá nghĩa là không có gì để `MappingProxyType` bọc quanh.
::
:::
::::

::::code{#ba-du-lieu-mot-ca}
Byte cần khai ba dữ liệu bất biến cho ca làm việc buổi sáng: một danh
sách TÊN nhân viên có thứ tự, một BẢNG GIÁ tra theo tên món, và một tập
VAI TRÒ được phép sửa bảng giá. Điền đúng công cụ cho từng chỗ trống.

```python title=starter
from types import MappingProxyType

# Ba nhân viên cố định trong ca sáng — có thứ tự (An luôn đứng đầu
# trong lịch phân ca in ra).
nhan_vien = ___                                     # "An", "Bình", "Chi"

# Bảng giá món — tra theo tên món, không ai được sửa giữa ca.
bang_gia = ___                                      # {"trà đá": 5000, "phở": 45000}

# Vai trò được phép sửa bảng giá — chỉ kiểm tra có mặt, không quan
# tâm thứ tự.
vai_tro_hop_le = ___                                # "quan_ly", "thu_ngan"

print(f"Nhân viên: {nhan_vien}")
print(f"Giá trà đá: {bang_gia['trà đá']}")
print(f"Vai trò hợp lệ: {sorted(vai_tro_hop_le)}")
```

```python title=solution
from types import MappingProxyType

# Ba nhân viên cố định trong ca sáng — có thứ tự (An luôn đứng đầu
# trong lịch phân ca in ra).
nhan_vien = ("An", "Bình", "Chi")

# Bảng giá món — tra theo tên món, không ai được sửa giữa ca.
bang_gia = MappingProxyType({"trà đá": 5000, "phở": 45000})

# Vai trò được phép sửa bảng giá — chỉ kiểm tra có mặt, không quan
# tâm thứ tự.
vai_tro_hop_le = frozenset({"quan_ly", "thu_ngan"})

print(f"Nhân viên: {nhan_vien}")
print(f"Giá trà đá: {bang_gia['trà đá']}")
print(f"Vai trò hợp lệ: {sorted(vai_tro_hop_le)}")
```

```python title=test
assert isinstance(nhan_vien, tuple), "nhan_vien phải là tuple — một dãy CÓ THỨ TỰ"
assert nhan_vien == ("An", "Bình", "Chi"), "nhan_vien phải đúng ba tên, đúng thứ tự"

assert type(bang_gia).__name__ == "mappingproxy", "bang_gia phải là MappingProxyType — một ÁNH XẠ tra theo tên món"
assert bang_gia["trà đá"] == 5000 and bang_gia["phở"] == 45000, "bang_gia phải tra đúng giá từng món"

assert isinstance(vai_tro_hop_le, frozenset), "vai_tro_hop_le phải là frozenset — một TẬP HỢP không quan tâm thứ tự"
assert vai_tro_hop_le == frozenset({"quan_ly", "thu_ngan"}), "vai_tro_hop_le phải đúng hai vai trò"

da_chan = {"nhan_vien": False, "bang_gia": False, "vai_tro": False}
try:
    nhan_vien.append("Dũng")
except AttributeError:
    da_chan["nhan_vien"] = True
try:
    bang_gia["trà đá"] = 1
except TypeError:
    da_chan["bang_gia"] = True
try:
    vai_tro_hop_le.add("bao_ve")
except AttributeError:
    da_chan["vai_tro"] = True
assert all(da_chan.values()), f"cả ba dữ liệu đều phải chặn được việc sửa tại chỗ — kết quả: {da_chan}"
```

:::hints
- kind: attention
  body: Ba chỗ trống ứng với ba câu hỏi khác nhau — dãy CÓ THỨ TỰ, ánh xạ tra theo KHOÁ, hay tập hợp chỉ kiểm tra CÓ MẶT. Đọc lại comment cạnh mỗi dòng trước khi chọn công cụ.
- kind: strategy
  body: 'nhan_vien: một dãy tên có thứ tự — dùng dấu ngoặc tròn (tuple). bang_gia: tra cứu theo tên món — bọc dict bằng MappingProxyType(). vai_tro_hop_le: chỉ kiểm tra có mặt, không thứ tự — bọc bằng frozenset().'
- kind: one-line
  body: 'Điền lần lượt `("An", "Bình", "Chi")`, `MappingProxyType({"trà đá": 5000, "phở": 45000})`, và `frozenset({"quan_ly", "thu_ngan"})`.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: static
  onFail: chỗ trống thứ hai phải gọi MappingProxyType(...) để bọc bảng giá thành ánh xạ chỉ đọc; chỗ trống thứ ba phải gọi frozenset(...) để đóng băng tập vai trò — dict/set thường vẫn sửa được tại chỗ
  requireAst:
  - kind: uses-call, target: MappingProxyType, min: 1
  - kind: uses-call, target: frozenset, min: 1
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Nhân viên: \('An', 'Bình', 'Chi'\)\nGiá trà đá: 5000\nVai trò hợp lệ: \['quan_ly', 'thu_ngan'\]\s*$
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Ba dữ liệu, ba câu hỏi, ba công cụ đúng — không ai chỉ định sẵn, và bạn
vẫn chọn đúng. Cụm này khép lại ở đây.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bạn giờ chọn đúng công cụ bất biến cho dữ liệu NẰM YÊN. Nhưng dữ liệu
thực tế thường THAY ĐỔI qua thời gian — một đơn hàng đi từ "chờ xác
nhận" sang "đã giao", một bài viết đi từ "bản nháp" sang "đã đăng". Lối
viết quen thuộc là để chính đối tượng đó TỰ ĐỔI trạng thái
(`self.trang_thai = "..."`, sửa tại chỗ). Nếu không sửa tại chỗ được
nữa, một thứ đang ĐỔI TRẠNG THÁI được biểu diễn thế nào?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
