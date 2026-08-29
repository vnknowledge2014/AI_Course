---
id: nen-tang.ham-vien-gach.boss-may-tinh-hoa-don
title: BOSS — Máy tính hoá đơn
summary: Một tầng hàm thuần khiết lo tính toán, đúng một hàm ngoài cùng lo màn hình — ghép lại thành cái máy tính hoá đơn của quán.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 30
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 15
teaches: []
practices: [core.closed-signature, core.one-job-name, core.docstring, core.guard-clause, core.pure-function, core.nested-call, core.function-return, core.accumulator, core.list-index, core.int-cast, core.fstring]
requires: [core.one-job-name, core.closed-signature, core.docstring, core.guard-clause, core.pure-function, core.nested-call, core.function-return, core.function-parameter, core.list, core.list-index, core.len, core.accumulator, core.int-cast, core.floor-division, ctrl.for-each, ctrl.for-range, ctrl.if, core.fstring]
concepts: [core.ham, core.tra-ve, core.danh-sach]
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
Hôm nay không có viên gạch mới. Chỉ có những viên cũ và một bức tường phải xây.
::::

::::explain{#hinh-dang-cua-cai-may}
Bài trước kết bằng một câu hỏi: đủ viên gạch rồi, dựng hẳn một cái máy tính hoá
đơn được không?

Được. Và điều đáng nói là bạn sẽ không phải quyết định gì mới — hình dạng của
cái máy đã bị ba mươi bài vừa rồi ép sẵn.

Byte muốn cái máy làm được ba chuyện: in ra từng món khách gọi, tính ra số tiền
phải trả, và nói xem thối lại bao nhiêu.

Đọc câu ấy mà xem: nó có hai chữ "và". Theo bài 29, đó là ba việc, nên nó không
thể là một hàm. Tách ra thì mỗi việc một hàm, và đây là chỗ mọi thứ tự sắp:

- **Tầng tính toán.** Cộng tiền hàng, tính thuế, tính giảm giá, tính tiền thối.
  Mỗi việc một hàm, mỗi hàm một cái tên không có chữ "và", chữ ký kín, và một
  docstring nói rõ nhận gì trả gì. Cả tầng này thuần khiết theo đúng bài 26:
  không hàm nào chạm tới màn hình, không hàm nào chạm tới thứ nằm ngoài mình.
- **Đúng một hàm ngoài cùng.** Nó gọi tầng dưới, rồi in ra. Nó là chỗ duy nhất
  trong cả chương trình biết trên đời có cái màn hình.

Vì sao lại là **một**? Vì hàm thuần khiết thì thử được một mình, còn hàm chạm
màn hình thì phải chạy cả chương trình mới xem được nó nói gì. Dồn hết phần
chạm màn hình vào một chỗ nghĩa là mọi phần còn lại đều thử được một mình.

Bạn đã gặp hình dạng này rồi, chỉ chưa gọi tên: bài 29 tách `tinh_tien_va_in`
thành `thanh_tien` và `in_thanh_tien`. Cái máy hôm nay là đúng việc ấy làm ở
quy mô cả chương trình.
::::

::::example{#bang-thiet-ke}
Đây là bản thiết kế, viết ra trước khi gõ dòng code nào. Mỗi dòng là một hàm,
và mỗi hàm được tả bằng đúng một câu không có chữ "và" nối hai động từ:

```text title=readonly
cong_tien_hang(cac_gia)          → trả về tổng tiền hàng của cả bàn
tien_thue(tien_hang, phan_tram)  → trả về số tiền thuế
tien_giam(tien_hang)             → trả về số tiền được giảm
tinh_hoa_don(cac_gia, phan_tram) → trả về số tiền khách phải trả
tien_thoi_lai(phai_tra, khach_dua) → trả về số tiền thối lại
in_hoa_don(...)                  → in cả hoá đơn ra màn hình
```

Năm dòng đầu là tầng thuần khiết. Dòng cuối là cái vỏ.

Để ý `tinh_hoa_don`: nó không tự cộng, không tự tính thuế. Nó gọi ba hàm kia
rồi ghép kết quả — đúng chuyện bài 14 nói, hàm gọi hàm.

Còn dữ liệu của quán thì nằm trong hai danh sách viết song song nhau:

```python title=readonly
ten_mon = ["Phở bò", "Lẩu gà", "Trà đá"]
gia_mon = [45000, 60000, 5000]
```

Món thứ nhất tên nằm ở `ten_mon[0]`, giá nằm ở `gia_mon[0]`. Món thứ hai thì
`[1]`, món thứ ba thì `[2]`. Hai danh sách, một chỉ số dùng chung — cách duy
nhất bạn đang có để buộc một cái tên với một con số.
::::

::::predict{#doan-ai-in-ra commitOnce}
Trước khi ghép cả máy, một câu hỏi về ranh giới giữa tầng tính và cái vỏ.

Đoạn dưới định nghĩa hai hàm của tầng tính, rồi gọi `tinh_hoa_don` một lần —
nhưng không đặt kết quả vào cái tên nào, cũng không `print` nó.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
gia_mon = [45000, 60000, 5000]

def cong_tien_hang(cac_gia):
    tong = 0
    for gia in cac_gia:
        tong = tong + gia
    return tong

def tinh_hoa_don(cac_gia, phan_tram):
    tien_hang = cong_tien_hang(cac_gia)
    return tien_hang + tien_thue_gia_dinh(tien_hang, phan_tram)

def tien_thue_gia_dinh(tien_hang, phan_tram):
    return tien_hang * phan_tram // 100

tinh_hoa_don(gia_mon, 8)
print("Xong")
```

:::opt{correct}
Đúng một dòng: Xong
:::

:::opt
118800 rồi Xong
::why
Gần đúng ở chỗ con số: `118800` chính là thứ `tinh_hoa_don` tính ra, và bạn
tính lại không sai một đồng.

Chỗ lệch là đường đi của con số ấy. `return` đặt giá trị vào **tay người gọi**,
không đặt nó lên màn hình — bài 11 đã tách hẳn hai chuyện này ra. Ở đây người
gọi là dòng `tinh_hoa_don(gia_mon, 8)` đứng trơ một mình: nó nhận con số rồi
không làm gì với nó cả, nên con số biến mất lặng lẽ.

Và đó chính là điều làm cả tầng tính toán thử được một mình: nó không bao giờ tự
ý nói gì ra màn hình.
::
:::

:::opt
Máy báo lỗi, vì kết quả trả về không được ai nhận
::why
Gần đúng ở chỗ bạn thấy có gì đó bị bỏ phí — đúng là con số ấy bị vứt đi thật.

Chỗ lệch: vứt một giá trị đi là chuyện hợp lệ, máy không coi đó là hỏng hóc.
Bạn đã gặp đúng chuyện này từ Realm 0 mà không để ý: `thuc_don.append("Trà đá")`
cũng đưa ra một giá trị, và cũng không ai nhận nó.

Máy chỉ báo lỗi khi nó không hiểu bạn viết gì, hoặc khi nó không có quy ước nào
để làm việc bạn bảo. Ở đây không rơi vào cả hai.
::
:::

:::opt
Máy báo lỗi, vì `tien_thue_gia_dinh` được gọi ở dòng trên chỗ nó được định nghĩa
::why
Gần đúng ở chỗ bạn để ý tới thứ tự — và với những dòng chạy thẳng thì thứ tự
đúng là chuyện sống còn, Realm 0 đã nói vậy.

Chỗ lệch nằm ở khoảng thời gian giữa **định nghĩa** và **chạy**. Dòng
`def tinh_hoa_don(...)` chỉ dán một cái tên lên một thân hàm; máy chưa đọc bên
trong thân làm gì. Tới lúc `tinh_hoa_don` thật sự chạy — dòng gần cuối bài —
thì `tien_thue_gia_dinh` đã được định nghĩa từ lâu rồi.

Nói cách khác: các hàm phải có mặt đủ trước lời gọi ĐẦU TIÊN, chứ không phải
trước dòng `def` của nhau.
::
:::
::::

::::code{#chan-ngay-o-cua-mot-lan-nua}
Ghép từng viên gạch trước. Viên khó nhất của tầng tính là `tien_giam`, vì nó là
viên duy nhất có một trường hợp phải chặn ngay ở cửa.

Quán giảm 10 phần trăm cho bàn từ 100 nghìn trở lên. Bàn dưới mức đó không được
giảm đồng nào — và theo bài 13, trường hợp ấy được trả lời ngay dòng đầu rồi
thoát, để phần thân còn lại chỉ lo đúng việc chính.

Hai chỗ trống: một là câu hỏi ở cửa, một là con số đi ra khi đã qua cửa.

```python title=starter
def tien_giam(tien_hang):
    """Nhận tiền hàng, trả về số tiền được giảm. Bàn dưới 100 nghìn không được giảm."""
    if ___:
        return 0
    return ___


print(f"Bàn 60 nghìn được giảm: {tien_giam(60000)} đồng")
print(f"Bàn 110 nghìn được giảm: {tien_giam(110000)} đồng")
print(f"Bàn 300 nghìn được giảm: {tien_giam(300000)} đồng")
```

```python title=solution
def tien_giam(tien_hang):
    """Nhận tiền hàng, trả về số tiền được giảm. Bàn dưới 100 nghìn không được giảm."""
    if tien_hang < 100000:
        return 0
    return tien_hang * 10 // 100


print(f"Bàn 60 nghìn được giảm: {tien_giam(60000)} đồng")
print(f"Bàn 110 nghìn được giảm: {tien_giam(110000)} đồng")
print(f"Bàn 300 nghìn được giảm: {tien_giam(300000)} đồng")
```

```python title=test
# Ba bàn ở ba phía của cái ngưỡng, và bàn đúng 100 nghìn là bàn nói thật nhất:
# nó phân biệt "dưới 100 nghìn" với "từ 100 nghìn trở lên".
assert tien_giam(60000) == 0, "bàn 60 nghìn chưa tới ngưỡng 100 nghìn nên không được giảm đồng nào"
assert tien_giam(99000) == 0, "bàn 99 nghìn vẫn còn dưới ngưỡng, sát ngưỡng cũng là dưới"
assert tien_giam(100000) == 10000, "đúng 100 nghìn là đã tới ngưỡng, nên bàn này được giảm — mười phần trăm của nó là 10 nghìn"
assert tien_giam(110000) == 11000, "bàn 110 nghìn được giảm mười phần trăm, tức 11 nghìn"
assert tien_giam(300000) == 30000, "bàn 300 nghìn được giảm mười phần trăm, tức 30 nghìn — mức giảm phải tính theo tiền hàng chứ không phải một con số cố định"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất là câu hỏi ở cửa, và docstring nói rõ ai bị chặn lại: bàn dưới 100 nghìn. Chỗ trống thứ hai là dòng chỉ chạy khi đã qua cửa, nên ở đó bàn chắc chắn đủ điều kiện được giảm.
- kind: strategy
  body: Câu hỏi ở cửa so tiền hàng với ngưỡng 100 nghìn, và phải so sao cho bàn đúng 100 nghìn KHÔNG bị chặn. Dòng dưới cùng cần mười phần trăm của tiền hàng — nhân với 10 rồi chia lấy phần nguyên cho 100, để kết quả là số nguyên chứ không kéo theo cái đuôi thập phân.
- kind: one-line
  body: "Chỗ trống thứ nhất là `tien_hang < 100000`; chỗ trống thứ hai là `tien_hang * 10 // 100`."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  match: regex
  expect: ^Bàn 60 nghìn được giảm: 0 đồng\nBàn 110 nghìn được giảm: 11000 đồng\nBàn 300 nghìn được giảm: 30000 đồng\s*$
- tier: static
  onFail: mức giảm phải tính từ chính `tien_hang`, không phải một con số chép cứng
  requireAst:
  - kind: uses-name, target: tien_hang, min: 2
:::
::::

::::explain{#byte-dong-vai-khach}
Còn một chuyện thực tế phải nói rõ trước khi bạn ghép cả cái máy.

Ô chấm điểm của Byte chạy chương trình của bạn lúc không có ai ngồi trước bàn
phím. Gặp `input()`, chương trình sẽ dừng lại chờ một người không có mặt.

Nên ở bài này Byte đóng vai khách: câu trả lời được ghi sẵn trong một danh sách.

```python title=readonly
khach_noi = ["200000"]
```

`khach_noi[0]` là số tiền khách đưa, đứng đúng chỗ mà `input()` sẽ đứng khi có
người thật. Để ý nó nằm giữa hai dấu nháy: đó chính là thứ `input()` đưa về —
chữ, luôn luôn là chữ — nên cái vỏ vẫn phải gọi `int()` trước khi đem tính.

Nhờ vậy cả tầng tính toán bên dưới chẳng cần biết mình đang phục vụ người thật
hay một câu ghi sẵn. Nó chỉ nhận những con số đi vào qua cửa trước.
::::

::::assemble{#ghep-ca-may}
Đây là cả cái máy. Sáu hàm, hai danh sách song song, một lời gọi ở dòng cuối.

Ba chỗ trống nằm ở ba chỗ khác nhau của cùng một đường đi: một chỗ **hàm gọi
hàm**, một chỗ **con số đi ra**, một chỗ **cái vỏ nhận về rồi in ra**.

```python title=starter
ten_mon = ["Phở bò", "Lẩu gà", "Trà đá"]
gia_mon = [45000, 60000, 5000]
phan_tram_thue = 8

# Một câu Byte ghi sẵn, thay cho input(): số tiền khách đưa. Vẫn là chữ.
khach_noi = ["200000"]


def cong_tien_hang(cac_gia):
    """Nhận danh sách giá các món, trả về tổng tiền hàng của cả bàn."""
    tong = 0
    for gia in cac_gia:
        tong = tong + gia
    return tong


def tien_thue(tien_hang, phan_tram):
    """Nhận tiền hàng và phần trăm thuế, trả về số tiền thuế."""
    return tien_hang * phan_tram // 100


def tien_giam(tien_hang):
    """Nhận tiền hàng, trả về số tiền được giảm. Bàn dưới 100 nghìn không được giảm."""
    if tien_hang < 100000:
        return 0
    return tien_hang * 10 // 100


def tinh_hoa_don(cac_gia, phan_tram):
    """Nhận danh sách giá và phần trăm thuế, trả về số tiền khách phải trả."""
    tien_hang = ___
    return tien_hang + tien_thue(tien_hang, phan_tram) - tien_giam(tien_hang)


def tien_thoi_lai(phai_tra, khach_dua):
    """Nhận số tiền phải trả và số tiền khách đưa, trả về số tiền thối lại."""
    return ___


def in_hoa_don(cac_ten, cac_gia, phan_tram, cau_khach_noi):
    """In cả hoá đơn ra màn hình. Đây là hàm DUY NHẤT chạm tới màn hình."""
    print("PHỞ THÌN — HOÁ ĐƠN")
    for i in range(len(cac_gia)):
        print(f"- {cac_ten[i]}: {cac_gia[i]} đồng")
    phai_tra = tinh_hoa_don(cac_gia, phan_tram)
    khach_dua = int(cau_khach_noi[0])
    print(f"Phải trả: {phai_tra} đồng")
    print(f"Khách đưa: {khach_dua} đồng")
    print(f"Thối lại: {___} đồng")


in_hoa_don(ten_mon, gia_mon, phan_tram_thue, khach_noi)
```

```python title=solution
ten_mon = ["Phở bò", "Lẩu gà", "Trà đá"]
gia_mon = [45000, 60000, 5000]
phan_tram_thue = 8

# Một câu Byte ghi sẵn, thay cho input(): số tiền khách đưa. Vẫn là chữ.
khach_noi = ["200000"]


def cong_tien_hang(cac_gia):
    """Nhận danh sách giá các món, trả về tổng tiền hàng của cả bàn."""
    tong = 0
    for gia in cac_gia:
        tong = tong + gia
    return tong


def tien_thue(tien_hang, phan_tram):
    """Nhận tiền hàng và phần trăm thuế, trả về số tiền thuế."""
    return tien_hang * phan_tram // 100


def tien_giam(tien_hang):
    """Nhận tiền hàng, trả về số tiền được giảm. Bàn dưới 100 nghìn không được giảm."""
    if tien_hang < 100000:
        return 0
    return tien_hang * 10 // 100


def tinh_hoa_don(cac_gia, phan_tram):
    """Nhận danh sách giá và phần trăm thuế, trả về số tiền khách phải trả."""
    tien_hang = cong_tien_hang(cac_gia)
    return tien_hang + tien_thue(tien_hang, phan_tram) - tien_giam(tien_hang)


def tien_thoi_lai(phai_tra, khach_dua):
    """Nhận số tiền phải trả và số tiền khách đưa, trả về số tiền thối lại."""
    return khach_dua - phai_tra


def in_hoa_don(cac_ten, cac_gia, phan_tram, cau_khach_noi):
    """In cả hoá đơn ra màn hình. Đây là hàm DUY NHẤT chạm tới màn hình."""
    print("PHỞ THÌN — HOÁ ĐƠN")
    for i in range(len(cac_gia)):
        print(f"- {cac_ten[i]}: {cac_gia[i]} đồng")
    phai_tra = tinh_hoa_don(cac_gia, phan_tram)
    khach_dua = int(cau_khach_noi[0])
    print(f"Phải trả: {phai_tra} đồng")
    print(f"Khách đưa: {khach_dua} đồng")
    print(f"Thối lại: {tien_thoi_lai(phai_tra, khach_dua)} đồng")


in_hoa_don(ten_mon, gia_mon, phan_tram_thue, khach_noi)
```

```python title=test
# Cả tầng tính toán được chấm mà KHÔNG cần chạy cái vỏ lần nào — đó là món lợi
# của việc dồn hết phần chạm màn hình vào đúng một hàm.
assert cong_tien_hang([45000, 60000, 5000]) == 110000, "ba món của bàn này cộng lại là 110 nghìn tiền hàng"
assert cong_tien_hang([]) == 0, "bàn chưa gọi món nào thì tiền hàng là 0 — vòng cộng dồn không chạy lượt nào, và cái tên cộng dồn phải sinh ra từ 0 trước vòng"
assert tinh_hoa_don([45000, 60000, 5000], 8) == 107800, "110 nghìn tiền hàng, cộng 8 phần trăm thuế, trừ 10 phần trăm giảm giá thì còn 107.800 — `tinh_hoa_don` phải gọi cả ba hàm dưới nó chứ không tự cộng lấy"
assert tinh_hoa_don([45000, 5000], 8) == 54000, "bàn 50 nghìn chưa tới ngưỡng giảm giá, nên chỉ cộng thuế: 50 nghìn thành 54 nghìn"
assert tien_thoi_lai(107800, 200000) == 92200, "khách đưa 200 nghìn cho hoá đơn 107.800 thì thối lại 92.200 — thứ tự hai con số trong phép trừ quyết định dấu của kết quả"
assert tien_thoi_lai(50000, 50000) == 0, "khách đưa vừa đủ thì thối lại 0 đồng"
```

:::hints
- kind: attention
  body: Chỗ trống thứ nhất nằm trong `tinh_hoa_don`, và dòng ngay dưới nó đã dùng `tien_hang` ba lần — nên chỗ ấy phải làm cho `tien_hang` có giá trị. Chỗ thứ hai là dòng duy nhất trong thân `tien_thoi_lai`. Chỗ thứ ba nằm trong cái vỏ, ở dòng in cuối cùng, chỗ hai cái tên `phai_tra` và `khach_dua` đều đã sẵn sàng.
- kind: strategy
  body: Chỗ thứ nhất là chỗ hàm gọi hàm: tiền hàng của cả bàn đã có một hàm chuyên lo, và danh sách giá đang nằm trong tham số của `tinh_hoa_don`. Chỗ thứ hai là một phép trừ giữa hai tham số, và thứ tự phải sao cho khách đưa nhiều thì thối lại dương. Chỗ thứ ba đừng trừ lại lần nữa — phép trừ ấy đã có tên rồi, cái vỏ chỉ việc gọi nó.
- kind: one-line
  body: "Lần lượt ba chỗ trống là `cong_tien_hang(cac_gia)`, `khach_dua - phai_tra`, và `tien_thoi_lai(phai_tra, khach_dua)`."
:::

:::validate
- tier: run
  timeoutMs: 8000
- tier: tests
  timeoutMs: 8000
- tier: output
  match: regex
  expect: ^PHỞ THÌN — HOÁ ĐƠN\n- Phở bò: 45000 đồng\n- Lẩu gà: 60000 đồng\n- Trà đá: 5000 đồng\nPhải trả: 107800 đồng\nKhách đưa: 200000 đồng\nThối lại: 92200 đồng\s*$
- tier: output
  expect: Thối lại: 92200 đồng
- tier: static
  onFail: cái vỏ phải GỌI `tien_thoi_lai` chứ không tự trừ lại một lần nữa — mỗi việc đúng một cái tên
  requireAst:
  - kind: uses-call, target: tien_thoi_lai, min: 1
  - kind: uses-call, target: cong_tien_hang, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Sáu hàm, sáu cái tên thật thà. Mình đọc bảng thiết kế là chạy được cả cái máy.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi sang mạch sau.

Máy chạy ngon, nhưng nó đứng trên hai list song song `ten_mon` và `gia_mon`.

Quán thêm món. Bạn viết thêm `"Quẩy"` vào `ten_mon` — và quên viết `10000` vào
`gia_mon`. Chạy lại:

```text title=readonly
PHỞ THÌN — HOÁ ĐƠN
- Phở bò: 45000 đồng
- Lẩu gà: 60000 đồng
- Trà đá: 5000 đồng
Phải trả: 107800 đồng
Khách đưa: 200000 đồng
Thối lại: 92200 đồng
```

Không một tiếng báo lỗi. Không một dòng đỏ. Món quẩy chỉ đơn giản là không có
mặt, và `tinh_hoa_don` trả về một con số sai mà không báo lỗi nào cả.

Đây là loại lỗi tệ nhất, đúng loại mà cả track này đã đuổi theo từ bài 6: lỗi
không tự nói ra. Và lần này nó không nằm ở hàm nào cả — cả tầng tính đều kín
và thuần khiết, cái vỏ ngoài thì chỉ làm đúng việc in, và hàm nào cũng có tên
thật thà. Nó nằm ở chỗ tên món và giá món là hai thứ
rời nhau, chỉ được buộc lại bằng một cái chỉ số mà không ai canh.

Có cách nào buộc tên món dính chặt vào giá của nó?

Mạch sau (T1.4) nhận đúng câu hỏi này.
::::

::::checkpoint{mastery=0.9}
::::
