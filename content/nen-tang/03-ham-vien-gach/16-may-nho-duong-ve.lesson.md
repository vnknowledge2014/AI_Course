---
id: nen-tang.ham-vien-gach.may-nho-duong-ve
title: Máy nhớ đường về
summary: Mỗi lời gọi được xếp chồng lên lời gọi trước, xong thì gỡ ra và quay về đúng dòng đã gọi — và cái chồng ấy chính là những dòng bạn thấy trong traceback.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 16
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [func.call-stack]
requires: [core.eval-inside-out, core.nested-call, core.function-def, core.function-call, core.function-parameter, core.function-return, core.floor-division, core.fstring, core.output, err.traceback]
concepts: [func.ham, core.loi-khi-chay]
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
Mình đi sâu mấy tầng cũng được. Đường về mình có ghi lại đàng hoàng.
::::

::::explain{#ai-dung-doi-o-dau}
Bài trước chốt một điều chắc chắn — máy tính từ trong ra, đối số phải xong
trước khi hàm ngoài bắt đầu chạy. Nhưng câu ấy mới nói tới **thứ tự chạy
xong**. Nó chưa trả lời câu hỏi để hở ở cuối bài.

Lúc `tinh_thue` đang cộng dở, `dung_cau` nằm ở đâu?

Nó không biến mất, vì lát nữa nó vẫn phải dựng ra câu chữ. Nó cũng không chạy
song song, vì máy làm một việc một lúc. Nó đang **đứng đợi**, và đợi ở một chỗ
rất cụ thể.

Buổi trưa ở quán phở. Bà chủ đang làm dở tô của khách A thì con bà gọi ra sau
bếp nhờ tìm hộp hành phi. Bà không bỏ tô ấy đi. Bà đặt phiếu order của khách A
xuống mặt bàn — trên phiếu có ghi tô cỡ gì, và đang làm tới bước nào — rồi mới
đi ra sau. Xong việc sau bếp, bà quay vào, lật đúng tờ phiếu trên cùng lên, và
làm tiếp từ đúng bước đã dừng.

Nếu giữa lúc tìm hành lại có thêm một việc nữa xen vào, bà đặt thêm một tờ
phiếu nữa **lên trên** tờ cũ. Xong việc mới thì bỏ tờ trên cùng đi, và tờ nằm
ngay dưới lại thành tờ trên cùng.

Máy làm y hệt. Mỗi lần một hàm được gọi, máy đặt lên một cái chồng đúng một tờ
phiếu, ghi ba thứ:

- tên hàm đang chạy,
- những giá trị hàm ấy đang cầm trong tay,
- và quan trọng nhất — **dòng nào ở hàm gọi đang đứng đợi kết quả**.

Hàm chạy tới `return` thì máy vứt tờ phiếu trên cùng đi, nhìn tờ nằm ngay dưới,
rồi chạy tiếp đúng cái dòng ghi trên đó. Đó là đường về, và nó không phải trí
nhớ mơ hồ — nó là một tờ giấy có thật.

Cách gọi của giới lập trình: cái chồng ấy tên là **chồng lời gọi**.
::::

::::example{#chong-cao-len-roi-thap-xuong}
Ba hàm, mỗi hàm gọi hàm kế tiếp **trong thân mình**. Mỗi hàm báo một câu lúc
vào và một câu lúc ra, để cái chồng hiện lên thành chữ.

```python title=readonly
def tra_gia(co_to):
    print("vào tra_gia")
    tien = 45000
    print("ra tra_gia")
    return tien

def tinh_thue(co_to):
    print("vào tinh_thue")
    tien = tra_gia(co_to)
    tong = tien + tien // 10
    print("ra tinh_thue")
    return tong

def dung_cau(co_to):
    print("vào dung_cau")
    tong = tinh_thue(co_to)
    cau = f"Tất cả {tong} đồng"
    print("ra dung_cau")
    return cau

print(dung_cau("vừa"))
```

Máy in ra:

```text
vào dung_cau
vào tinh_thue
vào tra_gia
ra tra_gia
ra tinh_thue
ra dung_cau
Tất cả 49500 đồng
```

Đọc từ trên xuống, ba cặp vào/ra lồng vào nhau y như ba cặp ngoặc: cái mở sau
cùng là cái đóng trước nhất.

Đi lại từng bước, và nhìn cái chồng cao lên rồi thấp xuống:

- Dòng cuối chương trình gọi `dung_cau`. Máy đặt phiếu **dung_cau** lên chồng.
  Chồng cao 1.
- `dung_cau` chạy tới dòng `tong = tinh_thue(co_to)`. Nó chưa xong. Máy ghi lên
  phiếu của nó rằng nó đang đợi ở dòng ấy, rồi đặt phiếu **tinh_thue** lên
  trên. Chồng cao 2.
- `tinh_thue` chạy tới `tien = tra_gia(co_to)`, cũng chưa xong, cũng đứng đợi
  ở dòng ấy. Phiếu **tra_gia** lên trên cùng. Chồng cao 3.
- `tra_gia` không gọi ai nữa. Nó chạy trọn và `return 45000`. Máy bỏ phiếu
  **tra_gia** đi, nhìn tờ dưới, và nối con số 45000 vào đúng chỗ `tinh_thue`
  đang đợi. Chồng còn 2.
- `tinh_thue` cộng nốt phần thuế rồi trả 49500 về. Bỏ phiếu, chồng còn 1, và
  `dung_cau` chạy tiếp từ đúng dòng nó đang đợi.
- `dung_cau` dựng câu chữ rồi trả về. Chồng rỗng, chương trình hết việc.

Ba lần đặt phiếu, ba lần gỡ phiếu, và thứ tự gỡ ngược hẳn với thứ tự đặt. Con
số đi lên theo đúng con đường lúc nãy đi xuống.
::::

::::explain{#chong-lo-mat-luc-chuong-trinh-hong}
Cái chồng ấy nghe như chuyện xảy ra sau lưng bạn. Thật ra bạn đã nhìn thấy nó
rồi — ở Realm 0, lần đầu một chương trình hỏng.

Đổi `tra_gia` một chút cho nó vấp:

```python
def tra_gia(co_to):
    return int(co_to) * 1000
```

`co_to` đang là chữ `"vừa"`, mà `int()` thì không đổi được chữ ấy thành số. Máy
dừng lại và in ra:

```text
Traceback (most recent call last):
  File "quan_pho.py", line 22, in <module>
    print(dung_cau("vừa"))
  File "quan_pho.py", line 17, in dung_cau
    tong = tinh_thue(co_to)
  File "quan_pho.py", line 10, in tinh_thue
    tien = tra_gia(co_to)
  File "quan_pho.py", line 2, in tra_gia
    return int(co_to) * 1000
ValueError: invalid literal for int() with base 10: 'vừa'
```

Realm 0 dạy bạn đọc thông báo lỗi **từ dòng cuối lên**: dòng cuối nói loại lỗi,
dòng ngay trên nói lỗi xảy ra ở dòng số mấy. Lúc ấy mấy dòng ở giữa còn là một
đống chữ phải chấp nhận mà chưa hiểu.

Giờ thì chúng có tên. Mỗi cặp hai dòng trong traceback là **một tờ phiếu trên
chồng** — một lời gọi đang dở, chưa được gỡ ra. Đọc từ dưới lên chính là đọc từ
tờ trên cùng xuống tới tờ dưới đáy:

- `in tra_gia` — tờ trên cùng, chỗ chương trình đang đứng khi nó vấp.
- `in tinh_thue` — tờ dưới nó, đang đợi ở dòng `tien = tra_gia(co_to)`.
- `in dung_cau` — đang đợi ở dòng `tong = tinh_thue(co_to)`.
- `in <module>` — tờ dưới đáy, là chính chương trình của bạn, đang đợi ở dòng
  cuối cùng.

Traceback dài mấy dòng là vì lúc hỏng có mấy lời gọi đang chồng lên nhau. Nó
không phải một đống chữ thừa — nó là ảnh chụp cái chồng, in ra nguyên vẹn.

> Chỗ dễ vấp: dòng trên cùng của traceback **không** phải chỗ hỏng. Nó là chỗ
> xuất phát. Chỗ hỏng nằm ở cặp dòng sát ngay trên câu báo lỗi, và câu báo lỗi
> thì bao giờ cũng là dòng cuối cùng.
::::

::::predict{#doan-thu-tu-vao-ra commitOnce}
Byte viết ba hàm, mỗi hàm gọi hàm kế tiếp ngay trong thân mình, và mỗi hàm báo
một câu lúc vào, một câu lúc ra.

**Trước khi bấm chạy**, bạn đoán sáu dòng ấy hiện ra theo thứ tự nào?

```python
def ba(x):
    print("vào ba")
    print("ra ba")

def hai(x):
    print("vào hai")
    ba(x)
    print("ra hai")

def mot(x):
    print("vào mot")
    hai(x)
    print("ra mot")

mot(1)
```

:::opt{correct}
vào mot · vào hai · vào ba · ra ba · ra hai · ra mot
:::

:::opt
vào ba · ra ba · vào hai · ra hai · vào mot · ra mot
::why
Gần đúng ở chỗ bạn đang dùng lại đúng câu bài trước dạy — máy chạy từ trong
ra, nên thứ nằm sâu nhất chạy trước. Với `mot(hai(ba(1)))` thì suy luận ấy
chính xác từng chữ.

Chỗ lệch nằm ở chỗ lời gọi được viết. Ở đây `ba(x)` không nằm trong dấu ngoặc
của `hai` — nó nằm **trong thân** `hai`, ở dòng thứ hai. Muốn tới được dòng ấy
thì `hai` phải chạy trước, và muốn tới được `hai` thì `mot` phải chạy trước
nữa. Hàm ngoài cùng là hàm mở màn, không phải hàm sâu nhất.
::
:::

:::opt
vào mot · ra mot · vào hai · ra hai · vào ba · ra ba
::why
Gần đúng ở chỗ bạn giữ đúng thứ tự bắt đầu: `mot` vào trước, rồi `hai`, rồi
`ba`. Nửa đầu của bạn khớp hoàn toàn.

Chỗ lệch là ở chữ *ra*. `ra mot` được in bởi dòng cuối trong thân `mot`, mà
dòng ấy nằm **sau** lời gọi `hai(x)`. Máy không thể chạy tới nó khi `hai` còn
chưa trả về. Một tờ phiếu chỉ được gỡ khi mọi tờ nằm trên nó đã gỡ hết — nên
`mot` là tờ vào đầu tiên và cũng là tờ ra sau cùng.
::
:::

:::opt
vào mot · vào hai · vào ba · ra mot · ra hai · ra ba
::why
Gần đúng ở chỗ khó nhất, và bạn đã đi được ba phần tư đường: ba câu *vào* xếp
đúng thứ tự vì mỗi hàm phải bắt đầu chạy mới tới được lời gọi nằm trong thân
nó.

Chỗ lệch là chiều đi ra. Bạn đang hình dung một hàng người xếp trước cửa quán —
ai tới trước thì được phục vụ trước. Cái chồng phiếu không như vậy: bà chủ bao
giờ cũng lật tờ **trên cùng**, tức tờ đặt xuống muộn nhất. Vào sau thì ra
trước, nên `ba` ra đầu tiên và `mot` ra sau chót.
::
:::
::::

::::explain{#hai-thu-mot-to-phieu-giu-giup}
Một câu — *mỗi lời gọi có một tờ phiếu riêng, đặt lên rồi gỡ ra* — kéo theo hai
chuyện bạn dùng được ngay:

- **Hàm gọi không mất chỗ đứng.** Nó đang đợi ở đúng một dòng, và mọi cái tên
  nó đang cầm vẫn còn nguyên trên tờ phiếu của nó. Lời gọi bên trong chạy bao
  lâu, đi qua bao nhiêu hàm nữa cũng không xoá được chúng.
- **Traceback đọc được.** Số dòng trong traceback chính là số phiếu đang chồng
  lúc chương trình vấp, và chúng in theo thứ tự từ đáy lên đỉnh. Một chương
  trình vấp ở tầng sâu sẽ cho traceback dài — điều đó nói lên đường đi, không
  phải nói lên bạn viết tệ.

Cái chồng cũng có giới hạn, vì mỗi tờ phiếu chiếm chỗ nhớ thật. Chuyện gì xảy
ra khi nó cao mãi thì để dành cho phía trước.
::::

::::code{#bao-lai-luc-quay-ve}
Byte muốn nhìn tận mắt cái khoảnh khắc `tinh_thue` **được trả lại quyền chạy**.

Trong đoạn dưới, `tinh_thue` báo một câu lúc vào, nhưng lúc ra thì im lặng.
Byte muốn nó cũng báo một câu lúc ra, và câu ấy nói luôn con số nó đang cầm
trong tay ngay khi đó.

Hãy điền dòng còn thiếu, đặt đúng vào chỗ trống — sau khi `tra_gia` đã trả về
và phép cộng đã xong, nhưng trước khi `tinh_thue` trả kết quả đi.

Câu Byte muốn thấy có dạng: `ra tinh_thue, đang cầm <con số>`.

```python title=starter
def tra_gia(co_to):
    print("vào tra_gia")
    tien = 45000
    print("ra tra_gia")
    return tien

def tinh_thue(co_to):
    print("vào tinh_thue")
    tien = tra_gia(co_to)
    tong = tien + tien // 10
    ___
    return tong

def dung_cau(co_to):
    print("vào dung_cau")
    tong = tinh_thue(co_to)
    cau = f"Tất cả {tong} đồng"
    print("ra dung_cau")
    return cau

cau_cuoi = dung_cau("vừa")
print(cau_cuoi)
```

```python title=solution
def tra_gia(co_to):
    print("vào tra_gia")
    tien = 45000
    print("ra tra_gia")
    return tien

def tinh_thue(co_to):
    print("vào tinh_thue")
    tien = tra_gia(co_to)
    tong = tien + tien // 10
    print(f"ra tinh_thue, đang cầm {tong}")
    return tong

def dung_cau(co_to):
    print("vào dung_cau")
    tong = tinh_thue(co_to)
    cau = f"Tất cả {tong} đồng"
    print("ra dung_cau")
    return cau

cau_cuoi = dung_cau("vừa")
print(cau_cuoi)
```

```python title=test
# Câu chữ cuối cùng chỉ ra được nếu con số đi trọn cả ba tầng rồi quay ngược
# về: tra_gia đưa 45000 lên cho tinh_thue, tinh_thue cộng thuế thành 49500 rồi
# đưa lên cho dung_cau, dung_cau mới dựng nổi câu này.
assert cau_cuoi == "Tất cả 49500 đồng", "câu cuối phải là 'Tất cả 49500 đồng' — 45000 tiền phở cộng 4500 tiền thuế, và con số ấy phải leo ngược đủ ba tầng phiếu mới về tới dòng cuối chương trình"
# Gọi thẳng tầng giữa để chắc rằng nó vẫn trả ra con số, chứ không phải chỉ in
# ra màn hình rồi thôi.
assert tinh_thue("vừa") == 49500, "tinh_thue vẫn phải TRẢ VỀ 49500 — dòng bạn thêm vào chỉ báo lại con số đang cầm, không được thay chỗ của return"
```

:::hints
- kind: attention
  body: Chỗ trống nằm giữa hai dòng đã có sẵn. Dòng ngay trên nó vừa tính xong `tong`, dòng ngay dưới nó mới trả `tong` đi. Vậy tại đúng chỗ trống ấy, `tinh_thue` đang cầm con số đã cộng đủ thuế và vẫn còn quyền chạy.
- kind: strategy
  body: Câu cần in có kèm một con số đang nằm trong một cái tên, nên dùng đúng lối in kèm giá trị bạn đã quen từ Realm 0. Byte muốn thấy đúng chữ `ra tinh_thue, đang cầm ` rồi tới con số. Đừng gõ thẳng con số vào câu chữ — hãy để cái tên tự nói ra nó, vì đó mới là bằng chứng rằng tờ phiếu của `tinh_thue` còn giữ nguyên thứ nó cầm.
- kind: one-line
  body: 'Viết `print(f"ra tinh_thue, đang cầm {tong}")` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ngay trên nó.'
:::

:::validate
- tier: run
  timeoutMs: 5000
- tier: tests
  timeoutMs: 5000
- tier: output
  expect: ra tinh_thue, đang cầm 49500
- tier: output
  expect: ra tra_gia
- tier: static
  onFail: dòng bạn điền phải in ra con số mà `tong` đang giữ, chứ không phải một câu chữ cố định
  requireAst:
  # Khung đã có sẵn MỘT f-string (câu chữ trong `dung_cau`), nên dòng bạn điền
  # phải là cái thứ hai. Hỏi thẳng `has-literal: 49500` không chặn được kiểu
  # gõ cứng con số vào GIỮA câu chữ, vì lúc đó cả câu là một hằng chuỗi.
  - kind: uses-fstring, min: 2
  # `tong` được ĐỌC hai lần trong khung (`return tong` và câu chữ của
  # `dung_cau`). Dòng bạn điền phải là lần đọc thứ ba.
  - kind: uses-name, target: tong, min: 3
  - kind: uses-call, target: print, min: 7
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tờ phiếu của mình vẫn nằm đó suốt. Gỡ tờ trên ra là mình chạy tiếp ngay.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Cái chồng cao lên được là nhờ mỗi lời gọi đặt thêm một tờ phiếu, và thấp xuống
được là nhờ mỗi `return` gỡ một tờ. Trong mọi đoạn code tới giờ, hai chiều ấy
luôn cân nhau: hàm nào cũng gọi một hàm **khác**, và chuỗi gọi bao giờ cũng
chạm tới một hàm không gọi ai nữa — như `tra_gia` — rồi mới quay đầu.

Bây giờ thử hình dung một hàm mà trong thân nó, cái tên được gọi lại chính là
tên của nó.

Máy có cho phép viết như vậy không? Và nếu có, thì mỗi lần vào thân lại đặt
thêm một tờ phiếu nữa lên chồng — vậy nếu một hàm gọi lại chính nó, cái chồng
ấy có bao giờ gỡ ra không?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
