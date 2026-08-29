---
id: nen-tang.ham-vien-gach.ham-nhin-thay-ten-o-ngoai
title: Hàm nhìn thấy tên ở ngoài
summary: Gặp một cái tên trong thân hàm, máy tìm trong lượt gọi trước; không thấy mới bước ra ngoài — cái tên ngoài đó gọi là biến toàn cục.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 21
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.global-variable, core.scope-lookup]
requires: [core.local-variable, core.function-def, core.function-call, core.function-parameter, core.function-return, core.variable, core.assignment, core.name-lookup, err.name-error, ctrl.comparison, core.arithmetic, core.fstring]
concepts: [core.ham, core.bien, core.pham-vi]
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
Không thấy cái tên trên tờ giấy nháp à? Vậy mình bước ra ngoài tìm tiếp.
::::

::::explain{#hoi-thang-cai-may}
Bài trước đóng chặt một chiều: cái tên sinh ra trong hàm không ra ngoài được,
gọi nó từ ngoài là `NameError`.

Chiều ngược lại thì chưa ai trả lời. Trong thân hàm gõ một cái tên viết ở
ngoài — máy có với tới được không, hay tờ giấy nháp cũng bịt luôn đường vào?

Hai câu trả lời đều nghe lọt tai. Nếu tờ giấy nháp là một chỗ kín, hàm chỉ
biết những gì được đưa vào qua tham số, thì đường vào cũng phải chặn. Nếu nó
chỉ là một chỗ *thêm*, thì hàm vẫn đọc được cả trang giấy lớn bên ngoài.

Loại câu hỏi này không cần đoán. Bài 3 đã dựng cho bạn thói quen hỏi thẳng cái
máy; ở đây thì viết đúng một đoạn ngắn rồi bấm chạy là biết.
::::

::::example{#thu-cai-la-biet}
Một cái tên viết sát lề trái, một hàm dùng lại chính cái tên ấy mà không nhận
nó qua tham số nào:

```python title=readonly
nguong = 200000

def vuot_nguong(tien):
    """Nói xem một khoản chi có vượt ngưỡng của tháng không."""
    return tien > nguong

print(vuot_nguong(310000))
print(vuot_nguong(85000))
```

Máy in ra:

```text
True
False
```

Không có `NameError` nào cả. Hàm `vuot_nguong` nhận đúng một thứ qua tham số —
`tien` — nhưng nó đọc được cả `nguong`, cái tên nó chưa bao giờ được đưa cho.

Đây là luật máy đang chạy theo, và nó là một thứ tự chứ không phải một chỗ:

> Gặp một cái tên trong thân hàm, máy hỏi trước một câu: **thân hàm này có
> gán cho cái tên ấy ở đâu không?**
>
> - **Có** — thì suốt lượt gọi, cái tên ấy là tên cục bộ, và máy không ra
>   ngoài tìm nữa. Đọc nó trước dòng gán thì máy dừng ngay và báo lỗi.
> - **Không** — thì máy **bước ra ngoài**, tìm trong những cái tên viết sát lề
>   trái của file. Thấy thì dùng. Ra tới đó vẫn không thấy thì mới `NameError`.

Câu hỏi ấy máy trả lời **trước khi chạy**, bằng cách đọc trọn thân hàm một
lượt — chứ không phải vừa chạy vừa dò. Nên nó không phụ thuộc dòng nào chạy
trước dòng nào.

Cái tên nằm ở tầng ngoài ấy gọi là **biến toàn cục** — *toàn cục* vì nó không
thuộc riêng lượt gọi nào, nên mọi hàm trong file đều với tới được. Nó đứng đối
diện với biến cục bộ của bài trước, cái chỉ sống trong một lượt gọi.

Nhìn theo tờ giấy nháp thì dễ hình dung: tờ giấy nháp không phải một cái hộp
kín, nó là một tờ giấy đặt **lên trên** trang giấy lớn của cả chương trình.
Máy đọc tờ nháp trước vì nó nằm trên; chỗ nào tờ nháp không có chữ thì nhìn
xuyên xuống trang bên dưới.

Có một chỗ tờ nháp **không** trong suốt: những cái tên mà thân hàm có gán. Máy
đã chừa sẵn ô cho chúng trên tờ nháp ngay từ đầu lượt, nên nhìn xuống không
thấy gì cả — ô ấy chỉ chưa có chữ. Bài 22 và bài 23 sống hẳn ở chỗ này.

Chuyện này giải thích luôn một điều bạn đã dùng suốt mà chưa để ý: hàm gọi
được hàm khác (bài 14) chính vì tên của hàm kia cũng là một cái tên ở tầng
ngoài. Không có luật tìm-ra-ngoài này thì `dung_cau` không thể gọi nổi
`tinh_thue`.
::::

::::predict{#trong-truoc-ngoai-sau commitOnce}
Bây giờ tới ca đáng nhìn: một cái tên có mặt ở **cả hai** chỗ. Ngoài kia có
`gia_pho`, mà tham số của hàm cũng tên `gia_pho`.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra con số nào?

```python
gia_pho = 45000

def bao_gia(gia_pho):
    """Nói lại mức giá đang được hỏi."""
    return gia_pho

print(bao_gia(60000))
```

:::opt{correct}
60000
:::

:::opt
45000
::why
Gần đúng ở chỗ bạn vừa học một luật thật và áp nó thật: hàm với ra ngoài đọc
được biến toàn cục, và ngoài kia `gia_pho` đúng là đang giữ 45000.

Chỗ lệch là thứ tự. Máy chỉ bước ra ngoài khi **không tìm thấy** cái tên ở
trong. Ở đây `gia_pho` là tham số, nghĩa là nó đã có mặt sẵn trên tờ giấy nháp
của lượt gọi, mang con số 60000 vừa được đưa vào. Máy tìm thấy ngay ở bước thứ
nhất nên dừng luôn tại đó — chuyến đi ra ngoài không bao giờ xảy ra.
::
:::

:::opt
Máy báo lỗi, vì một cái tên không được dùng ở hai chỗ cùng lúc
::why
Gần đúng ở chỗ bạn thấy chuyện trùng tên này dễ gây nhầm cho người đọc — cảm
giác đó đúng, và về sau bạn sẽ gặp những lời khuyên đặt tên sinh ra chính vì
nó.

Chỗ lệch: với máy thì không hề trùng. Hai cái tên nằm trên hai tờ giấy khác
nhau, một tờ nháp của lượt gọi và một trang chung của chương trình, nên chúng
không tranh chỗ của nhau. Máy chỉ báo `NameError` khi tìm hết cả hai tầng mà
không thấy chữ nào — chứ không báo lỗi vì thấy tới hai.
::
:::

:::opt
105000, vì máy dùng cả hai con số
::why
Gần đúng ở chỗ bạn để ý rằng có hai giá trị đang cùng mang một cái tên, và
muốn biết số phận của cả hai — đó là câu hỏi đáng hỏi.

Chỗ lệch: một cái tên trong một lượt chạy chỉ dẫn tới **một** giá trị. Máy đi
tìm và dừng ở thứ đầu tiên nó gặp, chứ không gom góp mọi thứ trùng tên rồi
cộng lại. `return gia_pho` vì vậy đưa ra đúng một con số, và đó là con số nằm
trên tờ giấy nháp.
::
:::
::::

::::explain{#doc-duoc-thi-tien-toi-dau}
Ghép hai bài lại, luật đã đủ hình dạng:

- Cái tên sinh ra **trong** hàm thì không đi ra ngoài được (bài trước).
- Cái tên viết **ở ngoài** thì hàm đọc được, miễn là trong lượt gọi không có
  cái tên nào cùng chữ che mất nó.

Đọc được là một tiện lợi có thật. Một mức ngưỡng, một bảng giá, một tên quán —
những thứ cả chương trình dùng chung — viết một lần ở ngoài rồi mọi hàm đều
tra được, không phải chuyền qua tham số hết lần này tới lần khác.

Nhưng để ý kỹ thì tới đây bạn mới chỉ thấy hàm **đọc**. Cả hai đoạn mã trong
bài đều chỉ lấy giá trị của cái tên ngoài ra dùng, không đoạn nào chạm vào nó.

> Chỗ dễ vấp: một hàm đọc biến toàn cục thì chữ ký của nó không còn nói hết
> mọi thứ nó cần. `vuot_nguong(310000)` nhìn vào chỉ thấy một con số đi vào,
> trong khi kết quả còn phụ thuộc một cái tên nằm tận đâu đó phía trên. Đây là
> một sợi dây khuất, và cả track này rồi sẽ quay lại đúng chỗ đó.
::::

::::code{#tra-bang-gia-o-ngoai}
Quán phở niêm yết đúng một giá cho mọi tô, và giá ấy được viết sát lề trái để
cả chương trình cùng dùng.

Hãy viết thân hàm `tien_may_to` sao cho nó nói ra tiền của mấy tô — bằng cách
**tra cái giá đang niêm yết ở ngoài** chứ không chép lại con số vào trong hàm.

```python title=starter
gia_pho = 45000

def tien_may_to(so_to):
    """Tiền của mấy tô phở, theo giá quán đang niêm yết."""
    return ___

print(f"Ba tô hết {tien_may_to(3)} đồng")
```

```python title=solution
gia_pho = 45000

def tien_may_to(so_to):
    """Tiền của mấy tô phở, theo giá quán đang niêm yết."""
    return so_to * gia_pho

print(f"Ba tô hết {tien_may_to(3)} đồng")
```

```python title=test
# Hai phép kiểm đầu chỉ nói hàm tính đúng. Phép kiểm cuối mới nói hàm có THẬT
# SỰ đi tra cái tên ở ngoài hay không: quán đổi giá, và một lời giải chép cứng
# con số 45000 vào thân hàm sẽ vẫn báo giá cũ.
assert tien_may_to(0) == 0, "không lấy tô nào thì hết 0 đồng"
assert tien_may_to(4) == 180000, "bốn tô phở 45 nghìn hết 180 nghìn"

gia_pho = 50000
assert tien_may_to(2) == 100000, "quán vừa đổi giá lên 50 nghìn một tô, nên hai tô phải thành 100 nghìn — hàm đi tìm cái tên `gia_pho` ở ngoài vào mỗi lượt gọi chứ không giữ sẵn con số cũ trong thân"
```

:::hints
- kind: attention
  body: Dòng sát lề trái ở trên cùng đặt tên cho giá một tô. Thân hàm được phép nhìn xuống trang giấy ấy, nên nó không cần ai đưa giá vào qua tham số.
- kind: strategy
  body: "Hàm cần hai thứ để nói ra tổng tiền: số tô và giá một tô. Số tô đi vào qua tham số; giá một tô thì đã có sẵn một cái tên ở ngoài đang giữ. Nhân hai thứ ấy với nhau."
- kind: one-line
  body: 'Viết `so_to * gia_pho` vào chỗ trống, giữ nguyên chữ `return` ở đầu dòng.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Ba tô hết 135000 đồng
- tier: static
  onFail: thân hàm phải TRA cái tên `gia_pho` ở ngoài, không được chép con số của nó vào trong
  requireAst:
  # `uses-name` chỉ đếm chỗ ĐỌC một cái tên, không đếm dòng gán — nên
  # `gia_pho = 45000` ở đầu file không tự làm luật này thoả. Lần đọc duy nhất
  # có thể có nằm đúng trong thân hàm.
  - kind: uses-name, target: gia_pho, min: 1
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Giá niêm yết viết một chỗ, mọi hàm cùng tra. Mình không phải chép lại lần nào.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Đọc thì được. Vậy GÁN cho chính cái tên ngoài đó, ngay trong thân hàm — cái
tên ở ngoài có đổi theo không?

Chuyện này rất dễ gặp chứ không phải bịa ra để hỏi. Quán đổi giá giữa buổi, và
bạn muốn một hàm `doi_gia` làm đúng việc ấy: trong thân hàm viết
`gia_pho = 50000`, rồi ở ngoài in `gia_pho` ra xem.

Bạn có sẵn hai mảnh để suy đoán, và chúng kéo về hai phía khác nhau. Bài này
vừa nói hàm với ra ngoài đọc được cái tên đó. Bài trước lại nói mọi cái tên
gán trong thân hàm đều nằm trên tờ giấy nháp.

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
