---
id: nen-tang.ham-vien-gach.ten-chi-song-trong-mot-luot-goi
title: Cái tên chỉ sống trong một lượt gọi
summary: Tham số và mọi tên gán trong thân hàm sinh ra mới ở từng lượt gọi rồi mất khi hàm xong — gọi chúng từ ngoài là `NameError`.
locale: vi
track: nen-tang
module: ham-vien-gach
order: 20
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 13
teaches: [core.local-variable]
requires: [func.recursive-trust, core.function-def, core.function-call, core.function-parameter, core.function-return, core.variable, core.assignment, core.reassign, core.name-lookup, err.name-error, err.traceback, core.fstring, core.floor-division]
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
Mỗi lượt gọi, mình lấy một tờ giấy nháp mới. Xong việc là vò nó bỏ đi.
::::

::::explain{#moi-luot-mot-to-giay}
Bài trước để lại một chuyện khó chịu: bốn tầng của `dem_nguoc(4)` cùng dùng
chữ `n`, mỗi tầng một con số khác nhau, mà không tầng nào làm hỏng tầng nào.

Nếu `n` là **một** cái tên duy nhất trong cả chương trình — kiểu cái tên bạn
đặt ở Realm 0 — thì chuyện đó không thể xảy ra. Ở Realm 0, dán tên lên một giá
trị mới là giá trị cũ mất chỗ dựa ngay. Bốn tầng dán đè lên nhau thì tầng
ngoài cùng đã mất số 4 từ lâu.

Vậy chúng không phải một cái tên. Chúng là bốn.

Hình dung thế này. Mỗi lần một hàm được gọi, máy đưa cho lượt gọi ấy **một tờ
giấy nháp mới tinh**. Cái tên `n` được viết lên tờ giấy của lượt gọi ấy, không
phải lên bảng chung của cả chương trình. Tầng ngoài viết số 4 lên tờ giấy của
nó; tầng trong được đưa một tờ khác và viết số 3 lên tờ ấy. Hai tờ giấy nằm
cạnh nhau trong chồng lời gọi mà bài 16 đã cho bạn thấy, không đè lên nhau.

Giới lập trình gọi những cái tên sống trên tờ giấy nháp ấy là **biến cục bộ**
— *cục bộ* nghĩa là chỉ thuộc về một chỗ, chỗ đó là một lượt gọi. Hai loại tên
đều là biến cục bộ:

- **tham số** — cái tên viết trong ngoặc ở dòng `def`;
- **mọi cái tên được gán trong thân hàm**, kể cả tên bạn nghĩ ra giữa chừng để
  giữ tạm một kết quả.

Và tờ giấy nháp có một tính chất thứ hai, quan trọng ngang tính chất đầu: khi
hàm chạy xong, **tờ giấy bị vò bỏ**. Mọi cái tên viết trên đó biến mất cùng
với nó.
::::

::::example{#to-giay-bi-vo-di}
Một hàm tính tiền thuế, dùng một cái tên riêng để giữ tạm con số vừa tính:

```python title=readonly
def tinh_thue(gia):
    """Tiền thuế 10 phần trăm của một món hàng."""
    thue = gia // 10
    return thue

print(tinh_thue(45000))
print(thue)
```

Máy in ra:

```text
4500
Traceback (most recent call last):
  File "hoa_don.py", line 7, in <module>
    print(thue)
NameError: name 'thue' is not defined
```

Dòng đầu chạy ngon lành: `tinh_thue(45000)` đưa ra 4500 đồng. Chuyện xảy ra ở
dòng sau mới là chuyện của bài này.

Đọc lỗi từ dòng cuối lên, đúng thói quen Realm 0 đã dựng: loại lỗi là
`NameError`, và đó chính là lỗi bạn gặp từ Realm 0 khi gõ một cái tên mà máy
không tìm ra. Dòng trên nó chỉ đúng chỗ vấp: dòng `print(thue)`.

Cái tên `thue` có tồn tại thật, và nó đã giữ đúng con số 4500 thật. Nhưng nó
tồn tại **trên tờ giấy nháp của lượt gọi `tinh_thue(45000)`**. Lượt gọi ấy kết
thúc ở dấu `return`, tờ giấy bị vò đi, và chữ `thue` biến mất cùng nó. Tới lúc
dòng `print(thue)` chạy thì trong cả chương trình không còn chỗ nào mang cái
tên đó nữa.

Tham số cũng vậy: thay `print(thue)` bằng `print(gia)` thì máy báo đúng loại
lỗi ấy, chỉ khác cái tên trong thông báo.

Nói cách khác: **hàm không cho cái tên nào của nó ra ngoài.** Muốn một con số
tính trong hàm đi ra tới chỗ gọi thì chỉ có một đường, và đó là `return` — cái
đường mà bài 11 đã phân biệt rạch ròi với chuyện in lên màn hình.
::::

::::predict{#hai-luot-goi-hai-to-giay commitOnce}
Byte viết một hàm chào, gọi nó hai lần, rồi tò mò muốn xem câu chào cuối cùng
còn nằm đâu đó không.

**Trước khi bấm chạy**, bạn đoán màn hình hiện ra những dòng nào?

```python
def chao(ten):
    """In một câu chào cho người có tên đưa vào."""
    loi_chao = f"Chào {ten}"
    print(loi_chao)

chao("Lan")
chao("Bình")
print(loi_chao)
```

:::opt{correct}
Chào Lan, rồi Chào Bình, rồi máy báo `NameError`
:::

:::opt
Chào Lan, rồi Chào Bình, rồi Chào Bình một lần nữa
::why
Gần đúng ở chỗ bạn theo rất sát thói quen Realm 0: một cái tên đã được gán thì
nó giữ giá trị gần nhất, và gán lại lần hai thì lần hai thắng. Với những cái
tên viết ở ngoài hàm, cách đọc đó chính xác từng chữ.

Chỗ lệch là chỗ cái tên ấy sống. `loi_chao` được gán trong **thân hàm**, nên
nó nằm trên tờ giấy nháp của lượt gọi. Lượt gọi `chao("Lan")` xong thì tờ giấy
thứ nhất bị vò; lượt `chao("Bình")` được đưa một tờ hoàn toàn mới, viết lên đó
rồi cũng vò nốt. Tới dòng cuối thì không còn tờ nào để đọc — không phải nó giữ
giá trị cũ, mà là nó không còn tồn tại.
::
:::

:::opt
Chào Lan, rồi Chào Bình, rồi một dòng trống
::why
Gần đúng ở chỗ bạn cảm nhận đúng rằng dòng cuối có gì đó không ổn: cái tên ấy
không còn giữ thứ gì đáng in.

Chỗ lệch là máy không có trạng thái "cái tên còn đó nhưng rỗng". Từ bài 8 của
Realm 0, một chữ không nháy chỉ có hai kết cục: máy tìm thấy và dùng giá trị,
hoặc máy tìm không ra và dừng ngay bằng `NameError`. Không có kết cục thứ ba
kiểu in ra khoảng trắng.
::
:::

:::opt
Máy báo lỗi ngay từ dòng `chao("Lan")`, vì `loi_chao` chưa được đặt tên từ trước
::why
Gần đúng ở chỗ bạn nhớ một luật có thật và rất quan trọng: máy đọc chương
trình từ trên xuống, và dùng một cái tên trước khi nó được đặt thì `NameError`
nổ ra ngay.

Chỗ lệch là thời điểm. Dòng `loi_chao = f"Chào {ten}"` nằm trong thân hàm, nên
nó không chạy lúc máy đọc tới `def` — nó chạy **mỗi lượt gọi**. Và trong lượt
gọi ấy, dòng gán chạy trước dòng `print(loi_chao)` nằm dưới nó, nên cái tên đã
có mặt trên tờ giấy nháp đúng lúc cần. Hai lời chào vì vậy in ra bình thường;
chỗ vấp chỉ đến ở dòng cuối cùng, sau khi tờ giấy đã bị vò.
::
:::
::::

::::explain{#hai-he-qua-cua-to-giay}
Một câu — *cái tên trong hàm chỉ sống trong một lượt gọi* — kéo theo hai chuyện
bạn dùng được ngay:

- **Bạn được đặt tên thoải mái trong thân hàm.** Muốn gọi một kết quả tạm là
  `tong` thì cứ gọi, kể cả khi ngoài kia hay trong một hàm khác cũng có cái
  tên `tong`. Chúng nằm trên những tờ giấy khác nhau. Chính nhờ vậy mà bốn
  tầng `dem_nguoc` dùng chung chữ `n` mà vẫn yên chuyện — câu hỏi bài trước
  bỏ ngỏ, tới đây là hết.
- **Muốn giữ lại thứ gì thì phải đưa nó ra bằng `return`.** Tính xong mà không
  đưa ra thì con số ấy bị vò cùng tờ giấy. Đây là lý do rất nhiều hàm kết thúc
  bằng một dòng `return` gom lại đúng thứ người gọi cần.

> Chỗ dễ vấp: một hàm chỉ `print` chứ không `return` thì nhìn trên màn hình
> vẫn "có kết quả", nên rất dễ tưởng là xong. Nhưng thứ hiện trên màn hình
> không đi vào được cái tên nào cả — bài 11 gọi tên chuyện đó rồi: người gọi
> nhận về `None`.
::::

::::code{#cho-con-so-di-ra-ngoai}
Byte viết hàm tính tiền phải trả gồm cả thuế, rồi ở ngoài gõ `print(thue)` để
xem tiền thuế là bao nhiêu — và ăn ngay `NameError`.

Byte sửa lại cho đúng đường: tính trong hàm, rồi **đưa** con số ra ngoài cho
người gọi dán vào một cái tên của riêng họ.

Hãy điền dòng cuối của thân hàm, sao cho `tong` ở ngoài nhận được tiền hàng
cộng tiền thuế.

```python title=starter
def tien_phai_tra(gia):
    """Tiền phải trả cho một món hàng, gồm cả thuế 10 phần trăm."""
    thue = gia // 10
    ___

tong = tien_phai_tra(45000)
print(f"Phải trả {tong} đồng")
```

```python title=solution
def tien_phai_tra(gia):
    """Tiền phải trả cho một món hàng, gồm cả thuế 10 phần trăm."""
    thue = gia // 10
    return gia + thue

tong = tien_phai_tra(45000)
print(f"Phải trả {tong} đồng")
```

```python title=test
# Ba mức giá khác nhau, vì một dòng chỉ in ra màn hình mà không đưa gì ra sẽ
# khiến hàm trả về None — và None thì trượt ngay ở phép so đầu tiên.
assert tien_phai_tra(0) == 0, "món hàng 0 đồng thì thuế cũng 0 đồng, nên phải trả 0 đồng"
assert tien_phai_tra(45000) == 49500, "tô phở 45 nghìn chịu 4500 tiền thuế, nên phải trả 49500 đồng"
assert tien_phai_tra(120000) == 132000, "món 120 nghìn chịu 12 nghìn tiền thuế, nên phải trả 132 nghìn — con số này phải TÍNH ra từ đối số, không phải một số ghi cứng trong thân hàm"
```

:::hints
- kind: attention
  body: Chỗ trống là dòng cuối thân hàm. Ngay trên nó, `thue` đang giữ tiền thuế của lượt gọi này — và chỉ trong lượt gọi này thôi.
- kind: strategy
  body: "Bên ngoài cần một con số để dán vào cái tên `tong`. Hàm chỉ có đúng một đường đưa giá trị ra ngoài, và đó không phải `print` — `print` chỉ hiện chữ lên màn hình chứ không đưa gì cho ai. Con số cần đưa ra là tiền hàng cộng tiền thuế."
- kind: one-line
  body: 'Viết `return gia + thue` vào chỗ trống, thụt vào bốn dấu cách cho thẳng hàng với dòng ngay trên nó.'
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Phải trả 49500 đồng
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Tính xong thì đưa ra. Không đưa ra thì tờ giấy nháp mang nó đi mất.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Hàm không cho cái tên nào ra ngoài. Còn chiều ngược lại — hàm có thấy cái tên
bạn viết ở ngoài không?

Nhìn lại cái máy trả lời tự động bạn dựng ở cuối Realm 0. Danh sách
`cac_co_to` viết sát lề trái, hàm `gia_to` viết ngay dưới nó, chung một trang.
Thân `gia_to` mà gõ chữ `cac_co_to` thì máy có với ra ngoài mà tìm được không,
hay lại `NameError` như dòng `print(thue)` lúc nãy?

Bài sau trả lời.
::::

::::checkpoint{mastery=0.8}
::::
