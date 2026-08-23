---
id: nen-tang.ham-vien-gach.cho-dung-quyet-dinh-y-nghia
title: Chỗ đứng quyết định ý nghĩa
summary: Máy gán đối số vào tham số theo THỨ TỰ chứ không theo ý nghĩa — đảo chỗ hai con số thì chương trình vẫn chạy trơn tru.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 6
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 12
teaches: [core.positional-argument]
requires: [core.function-arg-count, core.function-parameter, core.function-argument, core.function-call, core.docstring, core.variable, core.fstring, err.type-error]
concepts: [core.ham, core.tham-so]
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
Mình không nhìn nghĩa của con số. Mình nhìn nó đứng thứ mấy.
::::

::::explain{#cho-dung-di-truoc-y-nghia}
Câu hỏi cuối bài trước: đưa đủ hai thứ nhưng đảo chỗ cho nhau, máy có phát hiện
ra không?

Máy không phát hiện ra. Nó không có cách nào phát hiện.

Cửa mà máy canh ở bài trước là một cái cửa đếm: hai chỗ trống, hai thứ đưa vào,
khớp thì mời vào. Qua khỏi cửa ấy rồi, việc còn lại là chia hàng vào chỗ, và
máy chia theo đúng một luật:

> Đối số thứ nhất đi vào tham số thứ nhất. Đối số thứ hai đi vào tham số thứ
> hai. Cứ thế cho hết.

Không có bước nào trong đó nhìn tới ý nghĩa. Máy không biết `so_to` nghĩa là số
tô hay `gia` nghĩa là giá tiền — với nó, hai cái tên ấy chỉ là nhãn dán lên chỗ
trống thứ nhất và chỗ trống thứ hai. Con số nào đứng thứ nhất thì mang tên
`so_to`, chấm hết.

Vì máy chia hàng theo **vị trí**, những đối số viết trần như vậy có tên riêng:
**đối số theo vị trí**.

Thử ngay trên `tinh_tien` của bài trước. Ba tô sáu chục nghìn một tô:

```python title=readonly
tinh_tien(3, 60000)     # so_to nhận 3, gia nhận 60000 — ra 180000
tinh_tien(60000, 3)     # so_to nhận 60000, gia nhận 3 — cũng ra 180000
```

Cả hai đều ra 180000, và không dòng nào báo lỗi. Lời gọi thứ hai nói một câu vô
nghĩa — *sáu chục nghìn tô phở, mỗi tô ba đồng* — mà vẫn cho ra con số đúng,
tại phép nhân thì đổi chỗ hai số không đổi kết quả.

Nghe như tin vui. Nó là tin xấu: bạn vừa viết một lời gọi sai mà không có gì
trên đời báo cho bạn biết. Lần này con số tình cờ vẫn đúng. Lần sau thì không.
::::

::::example{#lan-sau-thi-khong}
Đây là "lần sau". Hàm tính tiền thối lại cho khách — bên trong nó là một phép
**trừ**, mà phép trừ thì đổi chỗ là đổi kết quả:

```python title=readonly
def tien_thua(khach_dua, tien_hang):
    """Tính tiền thối lại cho khách.

    khach_dua: số tiền khách đưa, tính bằng đồng.
    tien_hang: số tiền của cả bàn, tính bằng đồng.
    Trả về số tiền phải thối lại.
    """
    return khach_dua - tien_hang

print(f"Thối lại: {tien_thua(200000, 135000)} đồng")
print(f"Thối lại: {tien_thua(135000, 200000)} đồng")
```

Máy in ra:

```text
Thối lại: 65000 đồng
Thối lại: -65000 đồng
```

Dòng thứ hai là một lời gọi đảo chỗ. Không có `TypeError` nào — số lượng vẫn
khớp, hai đối số cho hai chỗ trống. Không có cảnh báo nào. Chương trình chạy
trọn, in ra một dòng trông y hệt dòng trên, rồi kết thúc bình thường.

Chỉ có con số là sai, và nó sai theo kiểu đọc lướt qua thì không thấy.

Hãy đặt cạnh nhau hai loại sai bạn đã gặp:

- **Đếm lệch** (bài trước) — máy dừng ngay tại dòng gây ra, chỉ đúng tên chỗ
  trống còn thiếu. Bạn sửa trong nửa phút.
- **Đảo chỗ** (bài này) — máy chạy tiếp như không có gì. Con số sai đi vào hoá
  đơn, đi vào báo cáo cuối tháng, và tới lúc có người thấy lạ thì chẳng còn dấu
  vết nào chỉ về dòng đã gây ra.

Loại thứ hai tốn thời gian gấp trăm lần loại thứ nhất. Đó là lý do cả bài này
dành cho một chuyện nghe thì nhỏ xíu: thứ tự.
::::

::::predict{#ten-o-cho-goi commitOnce}
Byte cất sẵn hai con số vào hai cái tên trước khi gọi hàm — và hai cái tên ấy
trùng đúng tên hai chỗ trống trong chữ ký. Chỉ có điều lúc gõ lời gọi, Byte
viết chúng theo thứ tự đọc được ở hai dòng bên trên.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra gì?

```python title=readonly
def tien_thua(khach_dua, tien_hang):
    """Tiền thối lại cho khách: tiền khách đưa trừ tiền hàng."""
    return khach_dua - tien_hang

tien_hang = 135000
khach_dua = 200000

print(f"Thối lại: {tien_thua(tien_hang, khach_dua)} đồng")
```

:::opt{correct}
Thối lại: -65000 đồng
:::

:::opt
Thối lại: 65000 đồng, vì hai cái tên trùng đúng tên hai chỗ trống nên máy tự khớp
::why
Gần đúng ở chỗ đây là cách **người** đọc đoạn code ấy. Hai cái tên trùng khít
với hai chỗ trống, ai nhìn cũng thấy ngay cặp nào đi với cặp nào, nên chuyện
máy làm y như vậy nghe rất hợp lý.

Chỗ lệch: cái tên ở chỗ gọi không đi cùng giá trị vào trong hàm. Lúc máy tính
xong `tien_hang`, thứ nằm trong tay nó chỉ còn là con số 135000 — cái nhãn
`tien_hang` ở lại bên ngoài. Hàm nhận về hai con số trần, xếp hàng theo đúng
thứ tự bạn viết, và con số đứng đầu hàng thì mang tên `khach_dua`.

Chuyện tên trùng nhau vì thế còn nguy hơn tên khác nhau: nó làm dòng code sai
trông giống hệt dòng code đúng.
::
:::

:::opt
Máy báo `TypeError` vì hai đối số bị đặt nhầm chỗ
::why
Gần đúng ở chỗ bạn nhớ bài trước rất chắc: lời gọi mà lệch thì máy dừng ngay ở
cửa, không cho đi tiếp. Mong đợi ấy hợp lý.

Chỗ lệch nằm ở thứ máy kiểm được. Ở cửa nó **đếm**: hai chỗ trống, hai đối số,
khớp. Muốn biết 135000 là tiền hàng chứ không phải tiền khách đưa thì phải hiểu
nghĩa của tiền nong — mà nghĩa thì máy không nắm. Cả hai đều là số, phép trừ
làm được với số, nên mọi thứ chạy trót lọt.
::
:::

:::opt
Thối lại: 65000 đồng, vì tiền thối thì không bao giờ âm
::why
Gần đúng ở một hiểu biết thật ngoài đời: tiền thối lại đúng là không bao giờ
âm — khách đưa thiếu thì người ta đòi thêm chứ không ai "thối" số âm.

Chỗ lệch: hiểu biết ấy nằm trong đầu bạn, không nằm trong hàm. Thân hàm viết
đúng một việc, `khach_dua - tien_hang`, và nó làm việc ấy với bất cứ hai con số
nào được đưa vào. Máy không biết đây là tiền; nó trừ hai con số rồi đưa kết quả
ra, dương hay âm cũng vậy.

Chuyện này còn quay lại ở phía sau mạch: muốn hàm từ chối những đầu vào vô lý
thì chính bạn phải viết ra lời từ chối ấy.
::
:::
::::

::::explain{#doc-lai-truoc-khi-gui}
Máy chia hàng theo vị trí và sẽ không bao giờ đổi ý. Nên phần còn lại là việc
của người viết lời gọi, và có hai thói quen đáng tập ngay:

- **Đọc chữ ký ngay trước khi gõ lời gọi.** `help(tien_thua)` in ra
  `tien_thua(khach_dua, tien_hang)` — trái sang phải, đó là thứ tự bạn phải xếp
  hàng. Docstring bạn viết ở bài 4 lúc này mới trả công: nó nói cho bạn biết
  chỗ trống thứ nhất mong nhận cái gì.
- **Đọc lại lời gọi thành lời.** Đọc `tien_thua(tien_hang, khach_dua)` thành
  câu: *"tiền khách đưa là tiền hàng, tiền hàng là tiền khách đưa"*. Câu ấy vô
  lý, và bạn nghe ra được sự vô lý ngay — trong khi nhìn hai cái tên xếp cạnh
  nhau thì không.

Còn một chuyện nhỏ đáng dọn trước khi đi tiếp: những dấu phẩy trong lời gọi
**không** phải dấu phẩy ngăn cách trong một câu chữ, và cũng không phải phép
tính gì. Mỗi dấu phẩy đánh dấu ranh giới giữa chỗ trống này với chỗ trống kế
tiếp. Đếm dấu phẩy trong ngoặc, cộng một, ra số đối số bạn đang đưa.
::::

::::code{#xep-dung-hang}
Bàn ba ăn hết 135000 đồng và đưa tờ hai trăm nghìn. Hai con số ấy đã nằm sẵn
trong hai cái tên — nhưng chúng được viết ra theo thứ tự của **tờ hoá đơn**,
không phải theo thứ tự của **chữ ký hàm**.

Hãy điền vào chỗ trống để `thoi_lai` giữ đúng số tiền phải thối cho khách.

```python title=starter
def tien_thua(khach_dua, tien_hang):
    """Tính tiền thối lại cho khách.

    khach_dua: số tiền khách đưa, tính bằng đồng.
    tien_hang: số tiền của cả bàn, tính bằng đồng.
    Trả về số tiền phải thối lại.
    """
    return khach_dua - tien_hang

tien_hang = 135000
khach_dua = 200000

thoi_lai = tien_thua(___)

print(f"Tiền hàng: {tien_hang} đồng")
print(f"Khách đưa: {khach_dua} đồng")
print(f"Thối lại: {thoi_lai} đồng")
```

```python title=solution
def tien_thua(khach_dua, tien_hang):
    """Tính tiền thối lại cho khách.

    khach_dua: số tiền khách đưa, tính bằng đồng.
    tien_hang: số tiền của cả bàn, tính bằng đồng.
    Trả về số tiền phải thối lại.
    """
    return khach_dua - tien_hang

tien_hang = 135000
khach_dua = 200000

thoi_lai = tien_thua(khach_dua, tien_hang)

print(f"Tiền hàng: {tien_hang} đồng")
print(f"Khách đưa: {khach_dua} đồng")
print(f"Thối lại: {thoi_lai} đồng")
```

```python title=test
# Phép trừ là trọng tài ở đây. Xếp hàng đúng thì ra 65000; đảo chỗ hai thứ thì
# chương trình vẫn chạy trọn, vẫn in ra một dòng trông bình thường — và con số
# là -65000. Bài này chấm đúng chỗ đó, chứ không chấm việc gọi được hàm hay
# không.
assert thoi_lai == 65000, "khách đưa 200 nghìn cho bàn hết 135 nghìn thì thối lại 65 nghìn — ra số âm nghĩa là hai thứ đang đứng ngược chỗ nhau"
```

:::hints
- kind: attention
  body: Đọc dòng `def` từ trái sang phải xem chỗ trống nào đứng trước. Rồi nhìn hai dòng gán ngay trên lời gọi — chúng được viết theo thứ tự ngược lại, và đó là cái bẫy của bài này.
- kind: strategy
  body: Thứ tự trong ngoặc phải theo chữ ký, không theo thứ tự bạn đã viết hai dòng gán. Chỗ trống đầu tiên mong nhận tiền khách đưa, chỗ thứ hai mong nhận tiền của cả bàn. Đọc lời gọi thành lời trước khi bấm chạy: nghe có xuôi tai không.
- kind: one-line
  body: "Viết `khach_dua, tien_hang` vào chỗ trống — đúng thứ tự ấy, ngăn nhau bằng một dấu phẩy."
:::

:::validate
- tier: run
  timeoutMs: 6000
- tier: tests
  timeoutMs: 6000
- tier: output
  expect: Thối lại: 65000 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Xếp đúng hàng rồi. Chỗ trống thứ nhất nhận đúng thứ nó chờ.
::::

::::explain{#bon-con-so-trong-mot-ngoac}
Hai chỗ trống thì còn nhớ được. Nhưng quán mở rộng, và hàm tính tiền lớn theo.

Bây giờ một bàn có thể gọi thêm trà đá, và bà chủ thỉnh thoảng giảm cho khách
quen một khoản:

```python title=readonly
def tinh_tien(so_to, gia, so_tra_da, giam):
    """Tính tiền một bàn.

    so_to: mấy tô phở.
    gia: giá một tô, tính bằng đồng.
    so_tra_da: mấy ly trà đá, mỗi ly 5000 đồng.
    giam: giảm bao nhiêu đồng cho cả bàn.
    Trả về số tiền phải trả.
    """
    return so_to * gia + so_tra_da * 5000 - giam
```

Hàm vẫn đàng hoàng: chữ ký nói đủ bốn chỗ trống, docstring nói rõ từng chỗ mong
nhận gì. Rồi bạn gọi nó ở một file khác, cách đó hai trăm dòng:

```python title=readonly
tien_ban_ba = tinh_tien(2, 45000, 1, 0)
```

Con số ra đúng: 95000 đồng.

Nhưng hãy nhìn riêng dòng lời gọi ấy, như một người vừa mở file lên và chưa
từng đọc hàm. Bốn con số nằm trong một cặp ngoặc, cách nhau bằng ba dấu phẩy.
Số nào là số tô, số nào là giá, con số `1` kia là mấy ly trà đá hay là giảm một
đồng? Cái `0` cuối cùng nghĩa là không giảm gì, hay là không có ly trà đá nào?
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

`tinh_tien(2, 45000, 1, 0)` — bốn con số, không ai đọc ra số nào là gì, kể cả
chính bạn sau một tuần.

Đọc ngược lên chữ ký thì tra ra được, nhưng phải nhớ đường về chỗ hàm, và phải
đếm đúng chỗ thứ mấy. Máy thì không giúp gì: nó chia hàng theo vị trí, im lặng,
đúng cũng im mà sai cũng im.

Có cách nào nói rõ ngay lúc gọi — ngay trên chính dòng ấy — con số nào là gì?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
