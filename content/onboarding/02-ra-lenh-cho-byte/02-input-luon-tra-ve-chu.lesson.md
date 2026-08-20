---
id: onboarding.ra-lenh-cho-byte.input-luon-tra-ve-chu
title: Thứ bạn gõ luôn là chữ
summary: Gõ 25 vào ô hỏi tuổi, máy nhận về hai ký tự chứ không phải con số — và nó có lý do.
locale: vi
track: onboarding
module: ra-lenh-cho-byte
order: 2
tier: A
languages: [python]
defaultLanguage: python
level: intro
estimatedMinutes: 11
teaches: [core.input-returns-str]
requires: [core.input, core.type-fn]
concepts: [core.kieu-gia-tri, core.nhap-lieu]
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
Bạn gõ 25. Mình cầm về hai ký tự. Nghe lạ, nhưng mình có lý do đàng hoàng.
::::

::::explain{#net-muc-tren-to-giay}
Câu hỏi cuối bài trước: người ta gõ `25`, trong `tuoi` là con số 25 hay hai ký
tự `2` và `5`?

Câu trả lời: **hai ký tự**. Luôn luôn là chữ, không có ngoại lệ.

Hình dung người phụ quán đứng nghe khách rồi ghi lên tờ giấy. Khách nói "hai
lăm", người ấy viết hai nét mực: *2* rồi *5*. Tờ giấy bây giờ có hai nét mực —
nó chưa phải một con số để đem cộng vào sổ tiền. Muốn cộng, phải có người **đọc**
tờ giấy ấy ra thành số trước đã.

`input` làm đúng việc của người phụ quán: nó hứng lấy những phím bạn bấm, theo
đúng thứ tự bạn bấm, rồi đưa lại nguyên vẹn ngần ấy ký tự.

Vì sao nó không tự đổi giúp, khi thấy toàn chữ số? Vì đổi giúp sẽ hỏng việc.
Thử nghĩ tới ô hỏi số điện thoại: người ta gõ `0912345678`. Nếu `input` tự ý coi
đó là một con số, số `0` đứng đầu biến mất và bạn còn `912345678` — một số điện
thoại không gọi được. Máy không đoán ý bạn; nó giao lại đúng thứ bạn gõ và để
bạn quyết định.
::::

::::example{#hoi-thang-cai-nhan}
Không cần tin lời Byte. Hỏi thẳng máy bằng `type` — cái lệnh đọc nhãn của một
giá trị:

```python title=readonly
tuoi = input("Bác bao nhiêu tuổi? ")
print(type(tuoi))
```

Người ta gõ `25` rồi bấm Enter. Màn hình:

```text
Bác bao nhiêu tuổi? 25
<class 'str'>
```

Nhãn là `str` — chuỗi ký tự, tức là chữ.

Gõ gì cũng vậy. Gõ `Lan` thì nhãn là `str`. Gõ `25` thì nhãn là `str`. Gõ
`45000` cũng `str`. Không có kiểu gõ nào làm `input` đưa về một giá trị mang
nhãn `int`.

Nói cách khác, hai dòng dưới đây đặt vào `tuoi` **cùng một thứ**:

```python
tuoi = input("Bác bao nhiêu tuổi? ")   # người ta gõ 25
tuoi = "25"                            # bạn viết sẵn trong code
```
::::

::::predict{#doan-cong-them-mot commitOnce}
Quán muốn in ra tuổi của khách vào sang năm. Byte sắp chạy đoạn dưới, và người
ngồi trước máy sẽ gõ `25` rồi bấm Enter.

**Trước khi bấm chạy**, bạn đoán máy làm gì?

```python title=readonly
tuoi = input("Bác bao nhiêu tuổi? ")
print(tuoi + 1)
```

:::opt{correct}
Máy dừng lại và báo TypeError
:::

:::opt
In ra 26
::why
Gần đúng ở chỗ trong đầu bạn, 25 là một con số và cộng thêm 1 thì được 26 —
phép tính ấy không sai chút nào, người nào cũng làm vậy.

Chỗ lệch nằm ở thứ đang nằm trong `tuoi`. Đó không phải con số 25 mà là chữ
`"25"`, mang nhãn `str` như bạn vừa thấy ở trên. Còn `1` viết không nháy là một
con số, nhãn `int`. Chữ một bên, số một bên — và bài về hai kiểu không đi cùng
nhau đã cho thấy máy dừng ngay ở chỗ này.
::
:::

:::opt
In ra 251
::why
Gần đúng, và gần nhất trong ba đáp án. Bạn nhớ đúng luật ghép chuỗi: dấu `+`
giữa hai câu chữ là **nối** chứ không phải cộng, nên `"25"` nối với `"1"` sẽ ra
`"251"`.

Chỗ lệch chỉ ở một chi tiết: luật ấy đòi **cả hai vế** đều là chữ. Ở đây vế phải
là `1` viết trần, không nháy — một con số. Máy không có quy ước nào để nối chữ
với số, nên nó không nối, cũng không cộng. Nó dừng.

Nếu dòng đó viết là `print(tuoi + "1")` thì bạn đã đúng hoàn toàn: máy in `251`.
::
:::

:::opt
In ra 25, còn phần `+ 1` bị bỏ qua
::why
Gần đúng ở chỗ bạn nghĩ máy sẽ cố làm cho xong việc — nhiều chương trình bạn
dùng hằng ngày quả thật hay bỏ qua phần nó không hiểu.

Python thì không. Khi gặp một việc nó không có quy ước để làm, nó **dừng và nói
ra** thay vì đoán bừa rồi chạy tiếp. Một kết quả sai lặng lẽ khó tìm hơn nhiều
so với một thông báo lỗi ngay tại dòng gây ra nó.
::
:::
::::

::::explain{#doc-lai-dong-cuoi}
Đây là nguyên văn thứ máy in ra khi gặp đoạn trên:

```text
Traceback (most recent call last):
  File "quan_pho.py", line 2, in <module>
    print(tuoi + 1)
TypeError: can only concatenate str (not "int") to str
```

Đọc **từ dòng cuối lên**, đúng như bài đọc thông báo lỗi:

- Dòng cuối là loại lỗi và lời giải thích: `TypeError` — sai kiểu. Câu tiếng
  Anh dịch ra là *"chỉ nối được str với str, không nối được với int"*.
- Dòng trên nó chỉ thẳng vào dòng code có vấn đề: `print(tuoi + 1)`.
- Dòng trên nữa nói lỗi nằm ở dòng số 2 của tệp.

Điều đáng nhớ: đây **không phải lỗi của `input`**. `input` đã làm đúng việc của
nó — giao lại đúng thứ người ta gõ, dưới dạng chữ. Lỗi nảy sinh ở chỗ bạn đem
thứ chữ ấy đi cộng như thể nó là số.

Từ giờ, mỗi lần một giá trị đi ra từ `input`, hãy tự nhắc: *cái này đang là chữ*.
::::

::::code{#hai-buoi-ban-hang}
Đến lượt bạn nhìn tận mắt.

Quán ghi số tô bán được hai buổi. Cả hai con số đều do người ta gõ vào, nên cả
hai đều là chữ — Byte gõ hộ bàn phím như bài trước:

```python
so_to_sang = input("Sáng bán mấy tô? ")
so_to_chieu = input("Chiều bán mấy tô? ")
```

Hãy nối hai thứ vừa gõ bằng dấu `+` rồi in ra. Báo trước cho bạn khỏi hoang
mang: kết quả sẽ **không** phải 19. Thứ hiện ra chính là điều bài này muốn bạn
tận mắt thấy một lần.

```python title=starter
# Byte gõ hộ bàn phím: đây đúng là thứ input() đưa về khi người ta gõ 12 và 7.
so_to_sang = "12"
so_to_chieu = "7"

print(___)
```

```python title=solution
# Byte gõ hộ bàn phím: đây đúng là thứ input() đưa về khi người ta gõ 12 và 7.
so_to_sang = "12"
so_to_chieu = "7"

print(so_to_sang + so_to_chieu)
```

```python title=test
# Chấm bằng OUTPUT: người học chưa biết hàm nên chưa assert được gì.
# Khối này chỉ khẳng định chương trình chạy được tới dòng cuối.
pass
```

:::hints
- kind: attention
  body: Chỗ trống nằm trong `print`. Bạn cần đặt vào đó hai thứ vừa gõ, có dấu `+` ở giữa.
- kind: strategy
  body: Hai thứ ấy đang nằm trong hai cái tên ở dòng trên. Viết tên chứ đừng viết lại con số, và không dùng dấu nháy — bạn muốn giá trị mà cái tên đang giữ.
- kind: one-line
  body: "Viết `so_to_sang + so_to_chieu` vào chỗ trống."
:::

:::validate
- tier: run
  timeoutMs: 4000
- tier: output
  expect: 127
:::
::::

::::byte{trigger=success mood=happy pose=point-stage}
127 chứ không phải 19. Máy không cộng sai — nó ghép, vì cả hai đều là chữ.
::::

::::reflect{#nghi-lai}
Một câu hỏi trước khi đi tiếp.

Bây giờ bạn biết chắc: thứ đi ra từ `input` là chữ. Nhưng quán vẫn cần in cho
được câu *"Sang năm bác 26 tuổi"* — mà muốn có 26 thì phải cộng thật, và cộng
thật thì máy vừa từ chối ngay trước mắt bạn.

Vậy có cách nào bảo máy: *chữ này chứa toàn chữ số, coi nó là một con số thật
đi*?

Đừng trả lời vội. Bài sau có đúng một lệnh làm việc ấy — và tên nó bạn đã nhìn
thấy rồi, nó nằm ngay trong cái nhãn `<class 'int'>`.
::::

::::checkpoint{mastery=0.8}
::::
