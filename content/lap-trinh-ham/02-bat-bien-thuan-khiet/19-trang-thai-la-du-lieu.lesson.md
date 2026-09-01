---
id: lap-trinh-ham.bat-bien-thuan-khiet.trang-thai-la-du-lieu
title: "Trạng thái LÀ dữ liệu — chuyển trạng thái LÀ một hàm"
summary: "Draft, Published, Archived là BA @dataclass(frozen=True) riêng biệt, không phải một object tự đổi self.trang_thai. Một hàm publish(bai) NHẬN trạng thái cũ, TRẢ VỀ trạng thái mới — không có gì tự đổi tại chỗ."
locale: vi
track: lap-trinh-ham
module: bat-bien-thuan-khiet
order: 19
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 14
teaches: [fp.state-as-data]
requires: [fp.review-data-structures]
concepts: [fp.state-as-data]
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
Bạn vừa CHỌN đúng công cụ bất biến cho từng loại dữ liệu TĨNH — một danh
sách, một bảng tra, một cấu hình. Nhưng dữ liệu ít khi đứng yên mãi. Một
bài viết bắt đầu là bản nháp, rồi được đăng, rồi bị gỡ. Cái ĐỔI QUA THỜI
GIAN đó gọi là **trạng thái** — và nó cũng bất biến được.
::::

::::explain{#trang-thai-khong-tu-doi}
Lối viết quen thuộc cho "một thứ có trạng thái" thường là một class với
một field tự đổi:

```python
class BaiViet:
    def __init__(self, tieu_de):
        self.tieu_de = tieu_de
        self.trang_thai = "nhap"

    def dang_bai(self):
        self.trang_thai = "da_dang"   # SỬA TẠI CHỖ
```

`self.trang_thai = "da_dang"` là đúng thứ track này đã cảnh báo suốt sáu
bài đầu: sửa dữ liệu TẠI CHỖ. Mọi tên khác đang giữ tấm thẻ trỏ tới đúng
object đó (`mem.aliasing-explained`) đều thấy `trang_thai` đổi ngay, kể
cả những chỗ không ngờ đang giữ tấm thẻ ấy.

Cách khác: đừng để MỘT object tự đổi field bên trong nó. Thay vào đó,
mỗi trạng thái là một **kiểu dữ liệu riêng** — một `@dataclass(frozen=True)`
của chính nó — và **chuyển trạng thái là một hàm**: nhận trạng thái CŨ,
trả về trạng thái MỚI, không đụng gì vào trạng thái cũ.

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Draft:
    tieu_de: str

@dataclass(frozen=True)
class Published:
    tieu_de: str
    ngay_dang: str

def publish(bai: Draft) -> Published:
    return Published(tieu_de=bai.tieu_de, ngay_dang="2026-09-01")
```

`Draft` và `Published` là HAI KIỂU KHÁC NHAU, không phải cùng một
`BaiViet` với field `trang_thai` đổi giá trị. Gọi `publish(bai)` không hề
sửa `bai` — nó TRẢ VỀ một `Published` hoàn toàn mới, đúng tinh thần
"tạo mới, không sửa" mà mọi bài từ đầu track đã dựng.
::::

::::example{#nhap-van-la-nhap}
Byte tạo một bản nháp, đăng nó lên, rồi nhìn lại CẢ HAI biến:

```python title=readonly
from dataclasses import dataclass

@dataclass(frozen=True)
class Draft:
    tieu_de: str

@dataclass(frozen=True)
class Published:
    tieu_de: str
    ngay_dang: str

def publish(bai: Draft) -> Published:
    return Published(tieu_de=bai.tieu_de, ngay_dang="2026-09-01")

nhap = Draft("Ngày đầu tiên với Python")
da_dang = publish(nhap)

print(nhap)
print(da_dang)
print(nhap == da_dang)
```

```text title=readonly
Draft(tieu_de='Ngày đầu tiên với Python')
Published(tieu_de='Ngày đầu tiên với Python', ngay_dang='2026-09-01')
False
```

`nhap` vẫn là một `Draft` sau khi gọi `publish(nhap)` — không có dòng
nào biến nó thành `Published`. Chuyện "đăng bài" không phải một sự đổi
xảy ra BÊN TRONG `nhap`; nó là việc TẠO RA một giá trị mới (`da_dang`)
từ giá trị cũ. `nhap == da_dang` ra `False` dù cùng `tieu_de` — hai
`dataclass` khác kiểu nhau không bao giờ được coi là bằng nhau, dù trùng
hệt mọi field.
::::

::::predict{#doan-nhap-con-la-draft commitOnce}
Byte thêm hai dòng kiểm tra kiểu vào chương trình trên.

**Trước khi chạy**, bạn đoán hai dòng cuối in ra gì?

```python
from dataclasses import dataclass

@dataclass(frozen=True)
class Draft:
    tieu_de: str

@dataclass(frozen=True)
class Published:
    tieu_de: str
    ngay_dang: str

def publish(bai: Draft) -> Published:
    return Published(tieu_de=bai.tieu_de, ngay_dang="2026-09-01")

nhap = Draft("Ngày đầu tiên với Python")
da_dang = publish(nhap)

print(isinstance(nhap, Draft))
print(isinstance(nhap, Published))
```

:::opt{correct}
`True` rồi `False`
:::

:::opt
`True` rồi `True` — `nhap` vẫn là `Draft`, nhưng gọi `publish` cũng gắn
thêm "vai" `Published` cho nó, giống một object có thể vừa là cái này
vừa là cái kia
::why
Gần đúng ở việc bạn nhớ đúng dòng đầu: `nhap` chắc chắn vẫn là `Draft`.

Chỗ lệch: `publish(nhap)` không hề chạm vào `nhap` — nó chỉ ĐỌC field
của `nhap` rồi dựng một `Published` HOÀN TOÀN TÁCH BIỆT, gán cho tên
`da_dang`. `Draft` và `Published` không có quan hệ kế thừa hay "vai
trò kép" nào — `nhap` chỉ có thể là đúng MỘT kiểu, và nó vẫn là `Draft`.
::
:::

:::opt
`False` rồi `True` — sau khi đăng, `nhap` "trở thành" `Published`, y
hệt cách `self.trang_thai` đổi trong lối viết hướng đối tượng
::why
Gần đúng ở việc bạn nhớ đúng lối viết CŨ (`self.trang_thai = "da_dang"`)
— trong lối đó, object gốc THẬT SỰ đổi.

Chỗ lệch: bài này dạy đúng điều NGƯỢC LẠI với lối viết đó. `publish()`
không sửa `nhap` tại chỗ — nó trả về một giá trị MỚI. `nhap` không "trở
thành" gì cả sau khi gọi hàm; nó đứng nguyên là `Draft` mãi mãi, trừ khi
có một dòng gán lại rõ ràng như `nhap = publish(nhap)`.
::
:::

:::opt
Máy báo lỗi ở dòng `isinstance(nhap, Published)`, vì `nhap` chưa từng
được gán làm `Published`
::why
Gần đúng ở việc bạn tin chắc `nhap` không phải `Published` — đúng vậy.

Chỗ lệch: `isinstance(gia_tri, Kieu)` không bao giờ báo lỗi chỉ vì
`gia_tri` không thuộc `Kieu` đó — nó đơn giản trả về `False`. Không có
gì "chưa được gán làm" khiến hàm này nổ lỗi; nó luôn trả một `True`
hoặc `False` rõ ràng.
::
:::
::::

::::byte{trigger=success mood=happy pose=jump}
`nhap` không hề hay biết có một `da_dang` vừa được tạo ra từ nó. Trạng
thái, giờ đây, là một GIÁ TRỊ — không phải một field tự đổi bên trong
một object sống lâu dài.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`publish()` chỉ nhận đúng MỘT kiểu vào: `Draft`. Nhưng chương trình
Python không hề chặn bạn gọi `publish(mot_published)` hay
`publish(mot_archived)` — cú pháp vẫn hợp lệ, hàm vẫn CHẠY.

Nếu ai đó lỡ gọi `publish()` trên một bài ĐÃ ĐĂNG rồi, chuyện gì nên xảy
ra? Và bằng cách nào để Python thật sự BÁO cho họ biết, thay vì âm thầm
tạo ra một `Published` vô nghĩa?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
