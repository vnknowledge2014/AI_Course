---
id: nen-tang.ham-vien-gach.return-la-dau-cham-het
title: Return là dấu chấm hết
summary: Gặp `return` là lượt chạy của hàm kết thúc ngay tại chỗ — mọi dòng phía sau nó, kể cả dòng nằm ngoài `if`, không bao giờ tới lượt.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 12
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.return-exits]
requires: [core.implicit-return-none, core.function-return, core.function-call, core.function-parameter, ctrl.if, ctrl.block-indent, ctrl.break, core.boolean, core.variable, core.assignment, core.fstring]
concepts: [core.ham, ctrl.re-nhanh, core.thu-tu-buoc]
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
  reviewed: true
---

::::byte{trigger=enter mood=curious pose=lean-in}
Mình đưa hoá đơn xong là mình đi. Việc còn lại trong bếp không tới lượt mình.
::::

::::explain{#dua-hoa-don-xong-la-di}
Bài trước để lại một câu hỏi có hai cách trả lời, và cả hai đều nghe lọt tai:

- `return` đưa giá trị ra ngoài rồi hàm vẫn chạy nốt những dòng còn lại;
- `return` đưa giá trị ra ngoài và lượt chạy của hàm kết thúc luôn tại đó.

Quay lại quán phở. Bà chủ tính tiền bàn bạn, xé tờ hoá đơn, đặt vào tay bạn — rồi
bà quay sang bàn khác. Những việc còn lại của bàn bạn không tới lượt bà làm nữa,
vì với bà, bàn bạn **xong rồi**. Tờ hoá đơn không phải một bước giữa chừng; nó là
dấu chấm hết của cả lượt phục vụ.

Máy làm đúng như vậy. Gặp `return`, nó làm hai việc liền nhau và không có việc
thứ ba:

- lấy giá trị viết sau chữ `return` đem về chỗ gọi;
- **kết thúc lượt chạy của hàm ngay tại dòng đó**.

Mọi dòng viết phía sau, trong cùng thân hàm ấy, không bao giờ tới lượt chạy. Vậy
trong hai cách hiểu lúc nãy, cách thứ hai mới đúng.

Chuyện này khác một chuyện bạn đã gặp ở mạch trước. `break` cũng là một lệnh
thoát, nhưng nó chỉ bước ra khỏi **vòng lặp**, và chương trình chạy tiếp dòng nằm
dưới vòng. `return` bước ra khỏi **cả lượt gọi hàm** — không còn dòng nào của hàm
để chạy tiếp nữa.
::::

::::example{#mot-dong-khong-bao-gio-toi-luot}
Quán có hôm còn phở, có hôm hết. Hàm dưới đây trả lời khách, và có bốn dòng đáng
theo dõi từng dòng một.

```python title=readonly
def goi_mon(con_pho):
    if con_pho:
        print("Bếp bắt đầu làm")
        return "Một tô phở"
    print("Xin lỗi khách")
    return "Hết phở rồi"

print(goi_mon(True))
print(goi_mon(False))
```

Máy in ra:

```text
Bếp bắt đầu làm
Một tô phở
Xin lỗi khách
Hết phở rồi
```

Nhìn kỹ dòng `print("Xin lỗi khách")`. Nó viết **sát lề của thân hàm**, không
thụt vào trong `if`. Theo bài thụt đầu dòng ở Realm 0, một dòng nằm ngoài `if`
thì chạy trong mọi trường hợp.

Vậy mà lượt gọi đầu tiên — lượt còn phở — không in nó ra. Bốn dòng trên màn hình,
không dòng nào là câu xin lỗi của lượt ấy.

Đó là bằng chứng nhìn thấy được. Đi lại đúng đường máy đi trong lượt gọi thứ nhất:

- `con_pho` là `True`, máy vào thân `if`.
- In "Bếp bắt đầu làm".
- Gặp `return "Một tô phở"`. Máy đem chuỗi ấy về chỗ gọi và **đóng lượt chạy tại
  đây**. Hai dòng cuối thân hàm không tới lượt.

Lượt gọi thứ hai đi đường khác: `con_pho` là `False`, máy bỏ qua thân `if`, rơi
xuống dòng xin lỗi, rồi `return "Hết phở rồi"`.

Cùng một thân hàm, hai lượt gọi, hai đường đi khác nhau — và cái quyết định
đường nào dừng ở đâu chính là chỗ đặt `return`.
::::

::::predict{#doan-dong-nam-sau-return commitOnce}
Byte thêm một dòng báo cáo vào cuối thân hàm cho dễ theo dõi.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
def bao_gia(so_to):
    tien = so_to * 45000
    return tien
    print(f"Đã tính xong: {tien} đồng")

print(bao_gia(2))
```

:::opt{correct}
Chỉ một dòng: 90000
:::

:::opt
Đã tính xong: 90000 đồng — rồi 90000
::why
Gần đúng ở chỗ bạn giữ một luật rất chắc và rất đúng cho phần lớn chương trình:
máy đọc từ trên xuống, dòng nào cũng tới lượt. Bốn dòng trong thân hàm thì bốn
dòng đều được viết ra để chạy — cách nghĩ ấy hợp lý.

Chỗ lệch nằm ở chữ `return`. Nó không phải một dòng như những dòng khác: gặp nó
là lượt chạy của hàm đóng lại ngay tại đó. Dòng `print` viết phía sau vẫn nằm
trong thân hàm, vẫn thụt lề đúng, cú pháp không sai gì cả — nó chỉ không bao giờ
tới lượt.
::
:::

:::opt
90000 — rồi Đã tính xong: 90000 đồng
::why
Gần đúng ở chỗ bạn đặt `return` đúng vị trí trong dòng thời gian: giá trị đi ra
trước, những chuyện khác sau. Thứ tự ấy đúng với `print` — in xong thì chạy tiếp
dòng dưới.

Chỗ lệch: `return` vừa đưa giá trị ra vừa đóng lượt chạy, hai việc trong một, và
không có khoảng nào giữa chúng để nhét thêm dòng thứ ba. Cũng để ý dòng
`print(bao_gia(2))` ở ngoài: nó chỉ in được sau khi hàm đã chạy xong hẳn, nên
nếu dòng trong thân hàm có chạy thật thì nó phải hiện ra **trước** con số, không
phải sau.
::
:::

:::opt
Máy báo lỗi, vì có một dòng nằm sau `return` thì không bao giờ chạy được
::why
Gần đúng ở chỗ bạn đòi hỏi đúng thứ nên đòi hỏi: một dòng không bao giờ chạy được
gần như chắc chắn là chỗ tác giả viết nhầm, và một cái máy chu đáo thì nên nói ra.

Chỗ lệch: Python nhận dòng ấy không một lời nào. Nó đúng cú pháp, nên chương trình
chạy bình thường và cái dòng kia lặng lẽ nằm đó. Người ta gọi những dòng như vậy
là **mã chết**. Chính sự im lặng ấy làm nó đáng ngại: bạn sửa nội dung dòng đó,
chạy lại, và không thấy gì đổi khác — vì nó chưa bao giờ chạy lần nào.
::
:::
::::

::::explain{#dat-return-o-dau-la-quyet-dinh}
Một câu mang đi: **`return` là dấu chấm hết của lượt chạy, không phải một bước
giữa chừng.** Từ đó có hai hệ quả bạn dùng được ngay.

**Một hàm được phép có nhiều `return`.** Mỗi nhánh một cái, và lượt chạy dừng ở
cái nào gặp trước. Điều đó không mâu thuẫn với chuyện mỗi lượt gọi chỉ đưa ra
đúng một giá trị: nhiều `return` trong thân hàm, nhưng mỗi **lượt chạy** chỉ đi
qua đúng một trong số chúng.

**Dòng nằm sau `return` là mã chết.** Không lỗi, không cảnh báo, không chạy. Khi
một hàm cư xử lạ và bạn thề rằng dòng ấy phải chạy, hãy đọc ngược lên xem phía
trên nó có `return` nào không.

> Chỗ dễ vấp: `return` chỉ đóng lượt chạy của **hàm đang chứa nó**, không đóng cả
> chương trình. Sau khi hàm kết thúc, máy quay về đúng dòng đã gọi hàm và chạy
> tiếp từ đó — đúng như hai dòng `print` ở ví dụ trên, dòng thứ hai vẫn gọi hàm
> lần nữa bình thường.
::::

::::code{#ket-thuc-som-cho-hoc-sinh}
Quán tính thêm **5000** đồng tiền phục vụ cho khách thường. Học sinh được miễn
khoản đó: trả đúng giá gốc, không cộng gì thêm.

Đoạn dưới đã có sẵn hai dòng lo phần khách thường, viết sát lề thân hàm nên
chúng chạy trong mọi trường hợp. Nhánh `if` của học sinh đang bỏ trống.

Hãy điền dòng còn thiếu sao cho lượt gọi của học sinh **kết thúc ngay trong nhánh
ấy** và đưa ra đúng giá gốc.

```python title=starter
def gia_phai_tra(gia_goc, la_hoc_sinh):
    if la_hoc_sinh:
        ___
    tien_phuc_vu = 5000
    return gia_goc + tien_phuc_vu

print(f"Học sinh: {gia_phai_tra(45000, True)} đồng")
print(f"Khách thường: {gia_phai_tra(45000, False)} đồng")
```

```python title=solution
def gia_phai_tra(gia_goc, la_hoc_sinh):
    if la_hoc_sinh:
        return gia_goc
    tien_phuc_vu = 5000
    return gia_goc + tien_phuc_vu

print(f"Học sinh: {gia_phai_tra(45000, True)} đồng")
print(f"Khách thường: {gia_phai_tra(45000, False)} đồng")
```

```python title=test
# Bốn phép kiểm trên hai bộ giá khác nhau.
# Hai dòng học sinh chỉ đúng khi lượt gọi ấy DỪNG trong nhánh `if` — để nó rơi
# xuống hai dòng cuối là tiền phục vụ bị cộng vào.
# Hai bộ giá khác nhau để một lời giải chép cứng con số không qua nổi.
assert gia_phai_tra(45000, True) == 45000, "học sinh được miễn tiền phục vụ, nên lượt gọi ấy phải kết thúc ngay trong nhánh `if`; rơi xuống hai dòng cuối là bị cộng thêm 5 nghìn"
assert gia_phai_tra(45000, False) == 50000, "khách thường trả giá gốc cộng 5 nghìn tiền phục vụ, nên 45 nghìn thành 50 nghìn"
assert gia_phai_tra(55000, True) == 55000, "giá gốc đổi thành 55 nghìn thì học sinh trả đúng 55 nghìn — con số đưa ra phải lấy từ thứ hàm nhận vào chứ không phải một số chép sẵn"
assert gia_phai_tra(55000, False) == 60000, "khách thường với giá gốc 55 nghìn trả 60 nghìn"
```

:::hints
- kind: attention
  body: Chỗ trống là dòng duy nhất nằm trong nhánh `if`. Hai dòng ngay dưới nhánh ấy viết sát lề thân hàm, nên bình thường chúng chạy cho mọi lượt gọi — kể cả lượt của học sinh.
- kind: strategy
  body: Bỏ qua một dòng thì không đủ, vì hai dòng cuối vẫn sẽ tới lượt và cộng thêm tiền phục vụ. Lượt gọi của học sinh phải kết thúc hẳn ngay tại nhánh này. Từ khoá làm việc đó cũng chính là từ khoá đưa giá trị ra ngoài, và giá trị cần đưa ra ở đây là đúng cái giá gốc hàm vừa nhận.
- kind: one-line
  body: "Viết `return gia_goc` vào chỗ trống, thụt vào tám dấu cách cho nằm gọn bên trong nhánh `if`."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  match: regex
  expect: ^Học sinh: 45000 đồng\nKhách thường: 50000 đồng\s*$
- tier: output
  expect: Khách thường: 50000 đồng
- tier: static
  onFail: nhánh của học sinh phải đóng lượt chạy ngay tại đó và đưa ra chính giá gốc hàm nhận vào
  requireAst:
  # Mã khởi đầu ĐỌC `gia_goc` đúng một lần, ở dòng `return` cuối thân hàm. Lời
  # giải đọc nó hai lần. Hỏi `min: 2` là hỏi đúng chỗ khác nhau giữa hai bản, và
  # `has-literal` thì hỏi không được: con số 45000 nằm sẵn trong hai dòng print.
  - kind: uses-name, target: gia_goc, min: 2
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Một lượt gọi, một đường đi, một chỗ dừng. Dừng rồi là dừng hẳn.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Byte viết một hàm tính hoá đơn, và trước khi tính thì phải loại ba trường hợp
không tính được: số tô nhỏ hơn 1, giá một tô nhỏ hơn 1, và tên món để trống.
Chưa biết `return` thoát ngay, Byte gói việc chính vào trong ba tầng `if`:

```python
def tinh_hoa_don(ten_mon, gia_mot_to, so_to):
    if so_to >= 1:
        if gia_mot_to >= 1:
            if ten_mon != "":
                hang = gia_mot_to * so_to
                thue = hang // 10
                return hang + thue
    return 0
```

Đoạn này chạy đúng. Nhưng nhìn hình dạng của nó: bốn dòng làm việc chính bị đẩy
sang tít bên phải, cách lề mười sáu dấu cách, và muốn biết chúng chạy trong điều
kiện nào thì phải ngước lên đọc ngược cả ba dòng `if`. Thêm một trường hợp xấu
nữa là thêm một tầng, và lề còn lùi xa hơn.

Hàm của bạn kiểm ba trường hợp xấu rồi mới làm việc chính, viết `if` lồng ba tầng
nên dòng chính thụt vào tít bên phải. Biết `return` thoát ngay rồi — sửa được
không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
