---
id: onboarding.may-tinh-noi-gi.doc-thong-bao-loi
title: Đọc thông báo lỗi
summary: Khối chữ máy đổ ra khi vấp có một luật đọc riêng, và luật đó là đọc từ dưới lên.
locale: vi
track: onboarding
module: may-tinh-noi-gi
order: 17
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [err.traceback]
requires: [err.type-error]
concepts: [core.thong-bao-loi, core.loi-khi-chay]
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
Khối chữ đó không phải lời trách. Đó là mình kể lại đoạn đường vừa đi.
::::

::::explain{#ke-lai-doan-duong-vua-di}
Quán phở sai một người đi giao hàng. Nửa tiếng sau anh ta gọi về:

> Cháu đi từ quán, ra đường Lê Duẩn, rẽ vào ngõ 15, tới số nhà 27... cổng khoá
> ạ.

Anh ta kể theo đúng thứ tự đã đi: chỗ xuất phát trước, chỗ dừng lại sau cùng.
Nhưng câu bạn cần nghe nằm ở **cuối**: *cổng khoá*. Đó là thứ khiến chuyến đi
dừng lại. Ba câu trước chỉ là đường dẫn tới chỗ đó.

Thông báo lỗi của máy có đúng hình dạng ấy. Nó có tên riêng: **traceback**,
ghép từ *trace* (lần theo dấu vết) và *back* (ngược lại). Máy kể lại đoạn đường
nó vừa đi, theo thứ tự đã đi, và **chỗ nó vấp nằm ở dòng cuối cùng**.

Nên có một luật đọc, và luật này ngược với gần như mọi thứ bạn từng đọc trong
đời: **traceback đọc từ dưới lên.**
::::

::::example{#doc-tu-duoi-len}
Một chương trình bốn dòng. Byte đánh số cho dễ nói chuyện: dòng 1 là dòng trên
cùng.

```python title=readonly
gia_mot_to = 45000
tien_quay = "5000"
tong = gia_mot_to + tien_quay
print(tong)
```

Máy đổ ra:

```text
Traceback (most recent call last):
  File "quan_pho.py", line 3, in <module>
    tong = gia_mot_to + tien_quay
           ~~~~~~~~~~~^~~~~~~~~~~
TypeError: unsupported operand type(s) for +: 'int' and 'str'
```

Đọc **từ dưới lên**, ba nấc:

1. **Dòng dưới cùng — vấp cái gì.** `TypeError` là tên loại lỗi, đúng loại bạn gặp ở bài trước. Phần sau dấu hai chấm là lý do cụ thể: *không có cách dùng dấu `+` cho một `int` với một `str`*. Máy nói thẳng ra cả hai kiểu mà nó nhìn thấy.
2. **Dòng có chữ `line` — vấp ở đâu.** `line 3` là dòng số 3 trong file `quan_pho.py`. Ngay bên dưới, máy chép lại nguyên văn dòng 3 rồi gạch một hàng `~` và `^`; mũi nhọn `^` chỉ đúng vào dấu `+`.
3. **Dòng trên cùng — câu mở đầu cố định.** `Traceback (most recent call last):` lần nào cũng y hệt lần nào, và chỉ có nghĩa *dưới đây là đoạn đường tôi vừa đi*. Đọc một lần cho biết rồi thôi.

Gộp cả năm dòng lại, chúng nói đúng một câu: **"Ở dòng 3, tôi không cộng được
một con số với một câu chữ."**
::::

::::predict{#doan-may-dung-o-dau commitOnce}
Đoạn dưới đếm số lần một khách quen ghé quán. **Trước khi bấm chạy**, bạn đoán
màn hình hiện ra gì?

```python
ten_khach = "Cô Hà"
so_lan_ghe = 12
loi_chao = ten_khach + " đã ghé " + so_lan_ghe
print(loi_chao)
```

:::opt{correct}
Không có lời chào nào, chỉ có một traceback báo lỗi ở dòng 3
:::

:::opt
Cô Hà đã ghé 12
::why
Gần đúng ở chỗ bạn đọc ra được ý định của đoạn code và ghép đúng thứ tự ba
mảnh: tên khách, câu chữ ở giữa, rồi con số. Câu bạn viết ra chính là câu mà
chương trình này muốn nói.

Chỗ lệch nằm ở mảnh cuối. `so_lan_ghe` đang giữ `12` viết không nháy, nên nhãn
của nó là `int`, trong khi hai mảnh trước là `str`. Đúng tình huống bài trước:
dấu `+` không có quy ước nào cho `str` với `int`, nên máy dừng ngay tại dòng 3.
::
:::

:::opt
Có traceback, và nó báo lỗi ở dòng 4 — chỗ có print
::why
Gần đúng ở chỗ bạn để ý đến `print`, và bạn đúng một nửa: thứ bạn trông đợi
thấy trên màn hình đúng là do dòng 4 phụ trách, nên khi không thấy gì thì nghi
dòng 4 là chuyện tự nhiên.

Chỗ lệch là **thứ tự làm việc**. Dòng 3 phải ghép xong mới có thứ để giao cho
dòng 4. Phép ghép hỏng ngay ở dòng 3, nên dòng 4 chưa bao giờ tới lượt chạy.
Traceback ghi `line 3` chính vì vậy — nó chỉ vào chỗ máy vấp, không chỉ vào chỗ
bạn mong thấy kết quả.
::
:::

:::opt
Traceback báo lỗi ở dòng 1, vì đó là dòng đầu tiên của thông báo
::why
Gần đúng ở chỗ bạn làm điều tự nhiên nhất với một khối chữ: đọc từ trên xuống,
và coi dòng trên cùng là dòng quan trọng nhất. Với sách vở, tin nhắn, thư từ
thì thói quen đó phục vụ bạn rất tốt.

Chỗ lệch chỉ đúng ở riêng traceback. Dòng trên cùng luôn là câu mở đầu cố định
`Traceback (most recent call last):` — nó giống hệt nhau trong mọi thông báo
lỗi, nên nó không mang tin gì về bài của bạn. Tin nằm ở dưới đáy.
::
:::
::::

::::explain{#thoi-quen-ba-giay}
Từ giờ, mỗi lần một khối chữ dài đổ ra màn hình, hãy làm đúng ba việc sau, theo
đúng thứ tự:

1. Kéo mắt xuống **dòng dưới cùng**. Đọc tên lỗi và câu lý do.
2. Tìm chữ `line` ở phía trên. Đó là dòng cần mở ra xem.
3. Chỉ khi hai bước trên chưa đủ hiểu, mới đọc ngược lên phần giữa.

Đây là thói quen phân biệt người đọc được lỗi với người sợ lỗi. Khối chữ trông
đáng ngại vì nó dài và viết bằng tiếng Anh, chứ không phải vì nó khó. Gần hết
lượng thông tin bạn cần nằm gọn trong một dòng.

Một chi tiết nữa đáng để ý: câu lý do đổi theo tình huống. Bài trước bạn gặp
`can only concatenate str (not "int") to str` khi câu chữ đứng bên trái; ví dụ
hôm nay là `unsupported operand type(s) for +: 'int' and 'str'` khi con số đứng
bên trái. Vẫn cùng một loại `TypeError`, nhưng dòng cuối tả đúng cảnh máy vừa
gặp. Đó là lý do nó đáng đọc thay vì đáng lướt qua.
::::

::::code{#sua-dung-dong-may-chi}
Byte muốn in ra câu **Còn 4 bàn trống**, và viết thế này:

```python
print("Còn " + 4 + " bàn trống")
```

Máy trả lời:

```text
Traceback (most recent call last):
  File "quan_pho.py", line 1, in <module>
    print("Còn " + 4 + " bàn trống")
          ~~~~~~~^~~
TypeError: can only concatenate str (not "int") to str
```

Đọc dòng dưới cùng: chỉ ghép được `str` với `str`, mà ở đây có một `int` chen
vào giữa. Byte muốn **ghép** chứ không muốn tính. Hãy điền vào chỗ trống cho
phép ghép chạy được.

```python title=starter
print("Còn " + ___ + " bàn trống")
```

```python title=solution
print("Còn " + "4" + " bàn trống")
```

```python title=test
# Chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Dòng dưới cùng của traceback nói máy chỉ ghép được `str` với `str`. Hai mảnh nằm hai bên chỗ trống đều có dấu nháy — còn mảnh ở giữa thì sao?
- kind: strategy
  body: Con số 4 ở đây không đem đi tính, nó chỉ cần hiện lên màn hình như một chữ. Một ký tự chữ số vẫn là chữ, miễn nó nằm đúng chỗ.
- kind: one-line
  body: "Viết `\"4\"` vào chỗ trống — đủ cả hai dấu nháy kép."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: tests
  timeoutMs: 4000
- tier: output
  expect: Còn 4 bàn trống
:::
::::

::::byte{trigger=success mood=happy pose=jump}
Bạn vừa sửa lỗi bằng cách đọc, không phải bằng cách thử. Hai việc khác nhau lắm.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Để ý điểm chung của mọi thông báo bạn gặp hôm nay và bài trước: máy đều **đã
chạy được vài dòng** rồi mới vấp. Traceback tồn tại chính vì có một đoạn đường
để kể lại — dòng 1 và dòng 2 chạy xong xuôi, tới dòng 3 mới hỏng.

Vậy thử nghĩ ngược lại: có loại lỗi nào máy phát hiện ra **trước khi chạy dòng
đầu tiên** không? Nếu có, thông báo của nó sẽ trông thế nào — nó lấy đâu ra một
đoạn đường để kể?

Bài sau trả lời, bằng một chương trình mà dòng 1 viết đúng hoàn toàn nhưng vẫn
không được chạy.
::::

::::checkpoint{mastery=0.8}
::::
