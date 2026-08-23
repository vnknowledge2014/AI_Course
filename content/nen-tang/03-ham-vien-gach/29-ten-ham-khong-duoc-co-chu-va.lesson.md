---
id: nen-tang.ham-vien-gach.ten-ham-khong-duoc-co-chu-va
title: Tên hàm không được có chữ "và"
summary: Tên hàm phải nói trọn việc nó làm; phải thêm chữ "và" mới tả hết thì hàm đang làm hai việc.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 29
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.one-job-name]
requires: [core.closed-signature, core.docstring, core.guard-clause, core.pure-function, core.implicit-return-none, core.function-return, core.function-parameter, core.naming-convention, core.fstring, err.type-error]
concepts: [core.ham, core.tra-ve, core.ten]
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
Nếu mình phải thêm chữ "và" mới tả hết, thì mình đang tả hai việc chứ không một.
::::

::::explain{#xu-ly-a-b-c-d}
`xu_ly(a, b, c, d)` kín tuyệt đối. Không `global`, không sửa đồ của ai, không
nhớ lần gọi trước. Bốn thứ vào qua tham số, một thứ ra qua `return`. Theo đúng
mọi luật của bài trước, đây là một chữ ký mẫu mực.

Vậy mà bạn không gọi nổi nó. Vì bạn không biết `a` là gì, và tệ hơn, bạn không
biết **hàm này làm gì**.

Chữ ký kín lo phần máy: máy biết chính xác cái gì vào, cái gì ra. Nhưng người
đọc cần thêm một thứ nữa, và thứ đó nằm ở **cái tên**.

Cái tên là lời hứa duy nhất bạn đọc được mà không phải mở thân hàm. Bài 4 đã
thêm docstring cho phần mô tả dài; nhưng docstring nằm *trong* hàm, còn cái tên
thì hiện lên ngay tại chỗ gọi — giữa một trang code, chỗ người ta đọc nhanh
nhất và ít chịu dừng lại nhất.

Nên có một phép thử đơn sơ mà nghiêm khắc:

> Tả việc hàm làm bằng **một câu**. Nếu câu ấy cần chữ **"và"** mới tả hết, thì
> hàm đang làm hai việc — và nó cần hai cái tên.

Đây không phải chuyện văn phong. Chữ "và" là chỗ hai việc dính vào nhau, và
dính chỗ nào thì chỗ ấy không tách ra dùng riêng được.
::::

::::example{#cai-gia-cua-chu-va}
Byte viết một hàm tính thành tiền cho một món rồi in ra. Tả nó bằng một câu:
"tính thành tiền **và** in ra màn hình". Chữ "và" xuất hiện ngay, nên Byte đặt
tên thật thà luôn:

```python title=readonly
def tinh_tien_va_in(gia, so_luong):
    tien = gia * so_luong
    print(f"Thành tiền: {tien} đồng")

tinh_tien_va_in(45000, 2)
tinh_tien_va_in(25000, 3)
```

Máy in ra:

```text title=readonly
Thành tiền: 90000 đồng
Thành tiền: 75000 đồng
```

Chạy đúng. Vấn đề tới lúc quán cần **cộng cả bàn**.

Con số `90000` có tồn tại — nó nằm trong cái tên `tien`, sống được đúng một
lượt gọi rồi biến mất cùng lượt ấy, đúng như bài 20 đã nói. Ra khỏi hàm thì nó
không còn ai giữ. Hàm này in ra, chứ không đưa ra.

Muốn cộng cả bàn thì phải sửa hàm. Mà sửa thế nào cũng vướng: cho nó `return`
thêm con số thì cái tên `tinh_tien_va_in` càng lệch; bỏ dòng `print` đi thì mọi
chỗ đang gọi nó để in đều hỏng theo.

Tách đôi thì cả hai nửa đều gọn:

```python title=readonly
def thanh_tien(gia, so_luong):
    """Nhận giá một món và số lượng, trả về thành tiền của món đó."""
    return gia * so_luong

def in_thanh_tien(tien):
    """In một dòng thành tiền ra màn hình."""
    print(f"Thành tiền: {tien} đồng")

tien_pho = thanh_tien(45000, 2)
tien_com = thanh_tien(25000, 3)

in_thanh_tien(tien_pho)
in_thanh_tien(tien_com)
print(f"Cộng cả bàn: {tien_pho + tien_com} đồng")
```

Máy in ra:

```text title=readonly
Thành tiền: 90000 đồng
Thành tiền: 75000 đồng
Cộng cả bàn: 165000 đồng
```

Hai cái tên, hai câu tả, không câu nào cần chữ "và":

- `thanh_tien` — nhận giá và số lượng, trả về thành tiền. Nó thuần khiết theo
  đúng bài 26: gọi bao nhiêu lần cũng cho cùng một con số, thử được một mình.
- `in_thanh_tien` — in một dòng ra màn hình. Nó chạm vào màn hình, tức là có
  tác dụng phụ, nhưng nó chỉ làm mỗi việc ấy nên chỗ chạm nằm gọn một nơi.

Để ý chuyện vừa xảy ra: tách đôi theo chữ "và" thì phần **tính** và phần **in**
tự động rơi về hai hàm khác nhau, và phần tính trở thành hàm thuần khiết mà bạn
không phải cố ý làm gì thêm.
::::

::::predict{#doan-cong-hai-lan-goi commitOnce}
Byte thử một lối tắt: giữ nguyên hàm cũ có chữ "và", rồi cộng thẳng hai lời gọi
lại với nhau.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def tinh_tien_va_in(gia, so_luong):
    tien = gia * so_luong
    print(f"Thành tiền: {tien} đồng")

tong = tinh_tien_va_in(45000, 2) + tinh_tien_va_in(25000, 3)
print(f"Cộng cả bàn: {tong} đồng")
```

:::opt{correct}
Hai dòng "Thành tiền", rồi máy dừng lại vì lỗi `TypeError`
:::

:::opt
Hai dòng "Thành tiền", rồi Cộng cả bàn: 165000 đồng
::why
Gần đúng ở chỗ bạn theo dõi rất chuẩn phần đầu: hai lời gọi chạy trước, mỗi lời
gọi in ra một dòng, và hai con số ấy có thật.

Chỗ lệch là chuyện bài 11 đã nói: **in ra** khác hẳn **đưa ra**. `print` đẩy chữ
lên màn hình cho người xem; `return` mới là thứ đặt một giá trị vào tay người
gọi. `tinh_tien_va_in` không có dòng `return` nào, nên thứ nó đưa về là `None`.

Con số `90000` bạn nhìn thấy trên màn hình là thật — nhưng nó đã đi ra bằng
đường màn hình, và từ màn hình thì không cộng lại được.
::
:::

:::opt
Hai dòng "Thành tiền", rồi Cộng cả bàn: None đồng
::why
Gần đúng ở chỗ khó nhất: bạn nhận ra hàm này trả về `None`, và đó chính là điều
bài đang muốn bạn nhìn ra.

Chỗ lệch nằm ở dấu `+` đứng giữa hai lời gọi. Cộng hai `None` không phải là một
phép tính có nghĩa với Python — không có quy ước nào nói `None + None` bằng bao
nhiêu, y hệt chuyện cộng một câu chữ với một con số ở Realm 0. Máy dừng lại và
báo `TypeError`, nên dòng `Cộng cả bàn` không bao giờ được in.
::
:::

:::opt
Máy dừng ngay từ đầu vì `TypeError`, không dòng nào in ra
::why
Gần đúng ở chỗ bạn đoán trúng loại lỗi: `TypeError` đúng là thứ sắp xảy ra.

Chỗ lệch là **lúc nào** nó xảy ra. Bài 15 đã nói: đối số phải được tính xong
trước khi phép tính ngoài bắt đầu — máy chạy từ trong ra. Nên hai lời gọi chạy
trọn vẹn trước, in xong hai dòng của chúng, rồi máy mới cầm hai kết quả tới dấu
`+` và phát hiện ra chúng là `None`.

Lỗi khi chạy khác lỗi cú pháp ở đúng chỗ này: chương trình đã chạy được một
đoạn rồi mới gãy, nên thứ in ra trước đó vẫn nằm trên màn hình.
::
:::
::::

::::explain{#tach-o-dau}
Chữ "và" không phải luật duy nhất, nhưng nó là luật dễ dùng nhất, vì bạn không
phải phán đoán gì cả — bạn chỉ đọc lại câu mình vừa viết.

Vài cái tên và câu tả của chúng:

- `tinh_thue_va_lam_tron` — tính thuế **và** làm tròn. Hai việc.
- `doc_gia_va_bao_loi` — tra giá **và** báo lỗi khi không có món. Hai việc.
- `tien_thoi_lai` — trả về số tiền phải thối lại. Một việc, không cần chữ "và".
- `thanh_tien` — trả về thành tiền của một món. Một việc.

Có một chỗ dễ nhầm cần nói rõ. Câu **mô tả tham số** được phép có chữ "và":
"nhận giá **và** số lượng" là chuyện bình thường, vì hai thứ ấy cùng đi vào một
cửa để phục vụ đúng một việc. Chữ "và" đáng ngờ là chữ "và" nối hai **việc**:
tính rồi in, đọc rồi ghi, hỏi rồi tính.

Cách phân biệt: đọc phần sau chữ "và" xem nó có phải một động từ mới không.
"nhận giá và số lượng" — sau chữ "và" là một danh từ, đó vẫn là một việc. "tính
thành tiền và in ra" — sau chữ "và" là một động từ mới, đó là hai việc.

Và một mẹo cuối, mượn nguyên từ bài 4: viết docstring **trước** khi viết thân
hàm. Docstring là câu tả ấy. Nếu lúc viết nó bạn thấy tay mình gõ chữ "và" nối
hai động từ, thì bạn vừa bắt được hàm hai việc từ lúc nó còn chưa có dòng nào.
::::

::::code{#tach-doi-cai-ten}
Byte đưa bạn một hàm đang mang chữ "và" trong tên:

```python title=readonly
def tinh_va_in_dong_hoa_don(ten, gia, so_luong):
    tien = gia * so_luong
    print(f"{ten}: {tien} đồng")
```

Tách nó làm hai. Hai cái vỏ và hai docstring đã viết sẵn — mỗi vỏ nhận đúng thứ
việc của nó cần, không hơn. Việc của bạn là điền thân.

Dòng hoá đơn in ra theo đúng dạng `Tên món: số tiền đồng`, ví dụ
`Phở bò: 90000 đồng`.

```python title=starter
def thanh_tien(gia, so_luong):
    """Nhận giá một món và số lượng, trả về thành tiền của món đó."""
    ___


def in_dong_hoa_don(ten_mon, tien):
    """In một dòng hoá đơn cho một món ra màn hình."""
    ___


tien_pho = thanh_tien(45000, 2)
tien_com = thanh_tien(25000, 3)

in_dong_hoa_don("Phở bò", tien_pho)
in_dong_hoa_don("Cơm rang", tien_com)
print(f"Cộng cả bàn: {tien_pho + tien_com} đồng")
```

```python title=solution
def thanh_tien(gia, so_luong):
    """Nhận giá một món và số lượng, trả về thành tiền của món đó."""
    return gia * so_luong


def in_dong_hoa_don(ten_mon, tien):
    """In một dòng hoá đơn cho một món ra màn hình."""
    print(f"{ten_mon}: {tien} đồng")


tien_pho = thanh_tien(45000, 2)
tien_com = thanh_tien(25000, 3)

in_dong_hoa_don("Phở bò", tien_pho)
in_dong_hoa_don("Cơm rang", tien_com)
print(f"Cộng cả bàn: {tien_pho + tien_com} đồng")
```

```python title=test
# Nửa tính và nửa in được chấm bằng hai loại câu hỏi khác nhau, vì chúng đưa
# kết quả ra bằng hai đường khác nhau.
assert thanh_tien(20000, 4) == 80000, "bốn món hai mươi nghìn thì thành tiền là 80 nghìn — `thanh_tien` phải ĐƯA con số ấy ra qua `return`"
assert thanh_tien(45000, 2) == 90000, "hai tô phở 45 nghìn thì thành tiền là 90 nghìn, và cả bàn cộng được là nhờ con số này đi ra ngoài"
assert thanh_tien(30000, 0) == 0, "gọi không món nào thì thành tiền là 0 đồng, và hàm vẫn phải trả về một con số chứ không phải None"
assert in_dong_hoa_don("Trà đá", 5000) is None, "`in_dong_hoa_don` lo đúng một việc là in ra màn hình; phần tính đã là việc của `thanh_tien` rồi nên nửa này không đưa con số nào ra"
```

:::hints
- kind: attention
  body: Mỗi chỗ trống là dòng duy nhất trong thân một hàm, nên hãy đọc lại docstring ngay trên nó — docstring là câu tả việc, và mỗi câu tả ở đây chỉ có đúng một động từ. Hai động từ ấy khác nhau, nên hai dòng bạn điền cũng khác nhau.
- kind: strategy
  body: Hàm thứ nhất phải đưa một con số ra cho người gọi nhận được, vì ba dòng cuối bài còn cộng hai con số ấy lại. Hàm thứ hai đẩy chữ lên màn hình, và dòng chữ ấy ghép tên món với số tiền theo dạng `Tên món: số tiền đồng` — đúng loại câu chèn giá trị mà bạn dùng từ Realm 0.
- kind: one-line
  body: "Chỗ trống thứ nhất là `return gia * so_luong`; chỗ trống thứ hai là `print(f\"{ten_mon}: {tien} đồng\")`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: Phở bò: 90000 đồng
- tier: output
  expect: Cộng cả bàn: 165000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Hai cái tên, hai việc. Giờ đọc tên là biết gọi cái nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang bài cuối của mạch.

Bạn đã có đủ viên gạch: chữ ký kín, tên nói đúng việc, docstring, chặn sớm, hàm
gọi hàm, hàm thuần khiết.

Dựng hẳn một cái máy tính hoá đơn chỉ bằng những viên gạch ấy được không?
::::

::::checkpoint{mastery=0.85}
::::
